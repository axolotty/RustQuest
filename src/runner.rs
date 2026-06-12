//! Moteur d'exécution : compile et lance le code de l'apprenant, vérifie le
//! résultat attendu, puis — si c'est réussi — passe Clippy pour suggérer des
//! améliorations idiomatiques.
//!
//! ## Sécurité
//!
//! Ce moteur compile et exécute du code arbitraire. Plusieurs garde-fous :
//! - un **timeout** sur la compilation **et** sur l'exécution ;
//! - sous Linux, des **limites de ressources** (`setrlimit`) appliquées au
//!   programme de l'apprenant : temps CPU, mémoire, taille des fichiers écrits.
//!
//! En production, on le fait en plus tourner dans un conteneur Docker non-root
//! avec un nombre de processus limité (`pids_limit`) — voir le docker-compose.

use crate::content::{Check, Level, SUCCESS_MARKER};
use serde::Serialize;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

/// Durée maximale d'exécution du programme compilé.
const RUN_TIMEOUT: Duration = Duration::from_secs(5);
/// Durée maximale de compilation (anti code pathologique).
const COMPILE_TIMEOUT: Duration = Duration::from_secs(20);
/// Durée maximale d'analyse Clippy.
const CLIPPY_TIMEOUT: Duration = Duration::from_secs(15);

/// Une suggestion d'amélioration issue de Clippy.
#[derive(Serialize, Clone)]
pub struct LintTip {
    /// Identifiant du lint (ex. "clippy::needless_range_loop"), si connu.
    pub lint: String,
    /// Message lisible.
    pub message: String,
    /// Suggestion concrète, le cas échéant.
    pub suggestion: Option<String>,
}

/// Résultat d'une tentative, renvoyé tel quel au navigateur (en JSON).
#[derive(Serialize)]
pub struct RunOutcome {
    pub success: bool,
    /// "compile_error", "runtime_error", "timeout", "wrong_output" ou "passed".
    pub stage: &'static str,
    pub compile_output: String,
    pub stdout: String,
    pub stderr: String,
    pub expected: Option<String>,
    /// Suggestions Clippy (seulement quand c'est réussi mais perfectible).
    pub tips: Vec<LintTip>,
    /// Vrai si la solution diffère de la solution de référence (et est correcte).
    pub differs: bool,
}

impl RunOutcome {
    fn fail(stage: &'static str) -> Self {
        RunOutcome {
            success: false,
            stage,
            compile_output: String::new(),
            stdout: String::new(),
            stderr: String::new(),
            expected: None,
            tips: Vec::new(),
            differs: false,
        }
    }
}

/// Compile puis exécute le code de l'apprenant pour le niveau donné.
pub fn run_check(level: &Level, user_code: &str) -> RunOutcome {
    // 1. Construire le code source complet selon le mode de vérification.
    let (full_source, expected) = match level.check {
        Check::Stdout { expected } => (user_code.to_string(), Some(expected.to_string())),
        Check::Harness { harness } => (format!("{user_code}\n{harness}"), None),
    };

    // 2. Préparer un dossier temporaire unique.
    let work = match TempWork::new() {
        Ok(w) => w,
        Err(e) => {
            let mut o = RunOutcome::fail("compile_error");
            o.compile_output = format!("Erreur interne (dossier temporaire) : {e}");
            return o;
        }
    };
    let src_path = work.dir.join("solution.rs");
    let bin_path = work.dir.join("solution_bin");
    if let Err(e) = std::fs::write(&src_path, &full_source) {
        let mut o = RunOutcome::fail("compile_error");
        o.compile_output = format!("Erreur interne (écriture source) : {e}");
        return o;
    }

    // 3. Compiler avec rustc (édition 2021, avertissements masqués, timeout).
    let mut rustc = Command::new("rustc");
    rustc
        .arg("--edition")
        .arg("2021")
        .arg("-A")
        .arg("warnings")
        .arg("-o")
        .arg(&bin_path)
        .arg(&src_path);

    let compiled = match run_cmd(rustc, COMPILE_TIMEOUT) {
        Ok(c) => c,
        Err(e) => {
            let mut o = RunOutcome::fail("compile_error");
            o.compile_output = format!(
                "Impossible de lancer rustc : {e}\n\
                 Vérifie que Rust est installé (rustc --version)."
            );
            return o;
        }
    };

    if compiled.timed_out {
        let mut o = RunOutcome::fail("compile_error");
        o.compile_output = format!(
            "La compilation a dépassé {} secondes et a été interrompue.",
            COMPILE_TIMEOUT.as_secs()
        );
        return o;
    }
    if !compiled.success {
        let mut o = RunOutcome::fail("compile_error");
        o.compile_output = clean_compiler_output(&compiled.stderr, &src_path);
        return o;
    }

    // 4. Exécuter le binaire avec timeout + limites de ressources (Linux).
    let mut bin = Command::new(&bin_path);
    apply_sandbox(&mut bin);
    let exec = match run_cmd(bin, RUN_TIMEOUT) {
        Ok(e) => e,
        Err(e) => {
            let mut o = RunOutcome::fail("runtime_error");
            o.stderr = format!("Impossible de lancer le programme : {e}");
            return o;
        }
    };

    if exec.timed_out {
        let mut o = RunOutcome::fail("timeout");
        o.stderr = format!(
            "Le programme a dépassé {} secondes (ou ses limites de ressources) \
             et a été arrêté. Boucle infinie ?",
            RUN_TIMEOUT.as_secs()
        );
        return o;
    }

    let stdout = exec.stdout;
    let stderr = exec.stderr;

    // 5. Le programme a-t-il planté (panic) ?
    if !exec.success {
        let mut o = RunOutcome::fail("runtime_error");
        o.stdout = stdout;
        o.stderr = stderr;
        o.expected = expected;
        return o;
    }

    // 6. Vérifier le résultat selon le mode.
    let success = match level.check {
        Check::Stdout { expected } => stdout.trim() == expected.trim(),
        Check::Harness { .. } => stdout.contains(SUCCESS_MARKER),
    };

    let display_stdout = stdout.replace(SUCCESS_MARKER, "").trim_end().to_string();

    if !success {
        return RunOutcome {
            success: false,
            stage: "wrong_output",
            compile_output: String::new(),
            stdout: display_stdout,
            stderr,
            expected,
            tips: Vec::new(),
            differs: false,
        };
    }

    // 7. Réussi ! On compare à la référence et on passe Clippy pour suggérer
    //    des améliorations idiomatiques.
    let differs = normalize(user_code) != normalize(level.solution);
    let tips = if differs {
        idiomatic_tips(&work.dir, &src_path, level)
    } else {
        Vec::new()
    };

    RunOutcome {
        success: true,
        stage: "passed",
        compile_output: String::new(),
        stdout: display_stdout,
        stderr,
        expected,
        tips,
        differs,
    }
}

// ---------------------------------------------------------------------------
// Sandbox : limites de ressources appliquées au programme de l'apprenant
// ---------------------------------------------------------------------------

/// Sous Linux, applique des limites de ressources juste avant l'exécution du
/// programme. Ailleurs (macOS de dev…), seul le timeout protège.
fn apply_sandbox(cmd: &mut Command) {
    #[cfg(target_os = "linux")]
    {
        use std::os::unix::process::CommandExt;
        // SAFETY : `pre_exec` s'exécute dans l'enfant après `fork`, avant
        // `exec`. On n'y appelle que `setrlimit`, qui est async-signal-safe.
        unsafe {
            cmd.pre_exec(|| {
                set_resource_limits();
                Ok(())
            });
        }
    }
    let _ = cmd; // évite l'avertissement "unused" hors Linux
}

#[cfg(target_os = "linux")]
fn set_resource_limits() {
    use libc::{rlimit, setrlimit, RLIMIT_AS, RLIMIT_CPU, RLIMIT_FSIZE, RLIMIT_NOFILE};
    let limit = |resource, value: libc::rlim_t| {
        let rl = rlimit {
            rlim_cur: value,
            rlim_max: value,
        };
        // On ignore les erreurs : au pire, la limite n'est pas posée.
        unsafe {
            setrlimit(resource, &rl);
        }
    };
    limit(RLIMIT_CPU, 10); // 10 s de temps CPU
    limit(RLIMIT_AS, 512 * 1024 * 1024); // 512 Mo de mémoire (adresse virtuelle)
    limit(RLIMIT_FSIZE, 16 * 1024 * 1024); // 16 Mo max écrits dans un fichier
    limit(RLIMIT_NOFILE, 256); // 256 descripteurs de fichiers max (anti épuisement)
}

// ---------------------------------------------------------------------------
// Exécution d'une commande avec timeout
// ---------------------------------------------------------------------------

struct CmdResult {
    timed_out: bool,
    success: bool,
    stdout: String,
    stderr: String,
}

/// Lance une commande et l'arrête si elle dépasse `timeout`.
fn run_cmd(mut cmd: Command, timeout: Duration) -> std::io::Result<CmdResult> {
    cmd.stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = cmd.spawn()?;
    let start = Instant::now();
    loop {
        if let Some(status) = child.try_wait()? {
            let out = child.wait_with_output()?;
            return Ok(CmdResult {
                timed_out: false,
                success: status.success(),
                stdout: String::from_utf8_lossy(&out.stdout).into_owned(),
                stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
            });
        }
        if start.elapsed() >= timeout {
            let _ = child.kill();
            let _ = child.wait();
            return Ok(CmdResult {
                timed_out: true,
                success: false,
                stdout: String::new(),
                stderr: String::new(),
            });
        }
        std::thread::sleep(Duration::from_millis(40));
    }
}

/// Retire le chemin temporaire absolu des messages du compilateur.
fn clean_compiler_output(raw: &str, src_path: &Path) -> String {
    let full = src_path.to_string_lossy();
    raw.replace(full.as_ref(), "solution.rs")
}

// ---------------------------------------------------------------------------
// Suggestions idiomatiques via Clippy
// ---------------------------------------------------------------------------

/// Lance Clippy sur le code de l'apprenant et ne garde que les remarques que la
/// solution de référence ne déclenche **pas**.
fn idiomatic_tips(dir: &Path, user_src: &Path, level: &Level) -> Vec<LintTip> {
    let user_tips = run_clippy(dir, user_src, "user");
    if user_tips.is_empty() {
        return Vec::new();
    }

    let ref_src = dir.join("reference.rs");
    let reference_full = match level.check {
        Check::Stdout { .. } => level.solution.to_string(),
        Check::Harness { harness } => format!("{}\n{}", level.solution, harness),
    };
    let ref_ignored: HashSet<String> = if std::fs::write(&ref_src, &reference_full).is_ok() {
        run_clippy(dir, &ref_src, "reference")
            .into_iter()
            .map(|t| t.key())
            .collect()
    } else {
        HashSet::new()
    };

    user_tips
        .into_iter()
        .filter(|t| !ref_ignored.contains(&t.key()))
        .collect()
}

impl LintTip {
    fn key(&self) -> String {
        if self.lint.is_empty() {
            self.message.clone()
        } else {
            self.lint.clone()
        }
    }
}

/// Exécute `clippy-driver` sur un fichier. Si Clippy est absent ou trop lent,
/// renvoie une liste vide (fonctionnalité simplement désactivée).
fn run_clippy(dir: &Path, src: &Path, tag: &str) -> Vec<LintTip> {
    let out_bin = dir.join(format!("clippy_{tag}_bin"));
    let mut cmd = Command::new("clippy-driver");
    cmd.arg("--edition")
        .arg("2021")
        .arg("-o")
        .arg(&out_bin)
        .arg(src);

    match run_cmd(cmd, CLIPPY_TIMEOUT) {
        Ok(r) if !r.timed_out => parse_clippy(&r.stderr),
        _ => Vec::new(),
    }
}

/// Transforme la sortie texte de Clippy en une liste de suggestions.
fn parse_clippy(stderr: &str) -> Vec<LintTip> {
    let lines: Vec<&str> = stderr.lines().collect();
    let mut tips = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        let Some(rest) = lines[i].strip_prefix("warning: ") else {
            i += 1;
            continue;
        };
        if rest.contains("warning emitted") || rest.contains("warnings emitted") {
            i += 1;
            continue;
        }

        let message = rest.trim().to_string();
        let mut lint = String::new();
        let mut suggestion: Option<String> = None;

        let mut j = i + 1;
        while j < lines.len() {
            let line = lines[j];
            if line.starts_with("warning: ") || line.starts_with("error") {
                break;
            }
            let t = line.trim();

            if lint.is_empty() {
                if let Some(idx) = t.find("clippy::") {
                    lint = take_ident(&t[idx..]);
                } else if let Some(idx) = t.find("#[warn(") {
                    lint = take_ident(&t[idx + 7..]);
                }
            }

            if suggestion.is_none() {
                if let Some(p) = t.find("try: `") {
                    let after = &t[p + 6..];
                    if let Some(end) = after.find('`') {
                        suggestion = Some(after[..end].to_string());
                    }
                } else if let Some(s) = t.strip_prefix("help: ") {
                    if !s.contains("for further information") {
                        suggestion = Some(s.to_string());
                    }
                }
            }

            j += 1;
        }

        tips.push(LintTip {
            lint,
            message,
            suggestion,
        });
        i = j;
    }
    tips
}

/// Lit un identifiant de lint (lettres, chiffres, `_`, `:`) au début de `s`.
fn take_ident(s: &str) -> String {
    s.chars()
        .take_while(|c| c.is_alphanumeric() || *c == '_' || *c == ':')
        .collect()
}

/// Normalise du code pour comparer deux solutions à l'espace près.
fn normalize(code: &str) -> String {
    code.split_whitespace().collect::<Vec<_>>().join(" ")
}

// ---------------------------------------------------------------------------
// Dossier temporaire auto-nettoyé
// ---------------------------------------------------------------------------

struct TempWork {
    dir: PathBuf,
}

impl TempWork {
    fn new() -> std::io::Result<Self> {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let n = COUNTER.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!("rustquest_{nanos}_{n}"));
        std::fs::create_dir_all(&dir)?;
        Ok(TempWork { dir })
    }
}

impl Drop for TempWork {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.dir);
    }
}
