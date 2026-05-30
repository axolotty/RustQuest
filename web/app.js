// RustQuest — logique de l'interface (sans dépendance externe).

// ----------------------------------------------------------------------------
// Internationalisation de l'interface
// ----------------------------------------------------------------------------
const I18N = {
  fr: {
    levels: "Niveaux",
    welcome: "Choisis un niveau à gauche pour commencer ton aventure Rust.",
    run: "Lancer",
    hint: "Indice",
    solution: "Solution",
    resetCode: "Réinitialiser",
    compiling: "Compilation…",
    locked: "Termine le niveau précédent pour débloquer celui-ci.",
    confirmSolution: "Afficher la solution remplacera ton code actuel. Continuer ?",
    confirmResetCode: "Réinitialiser remettra le code de départ. Continuer ?",
    confirmReset: "Remettre toute ta progression à zéro ?",
    noMoreHints: "Plus d'indices disponibles — regarde la leçon à gauche !",
    hintLabel: "Indice",
    // titres de sortie
    passed: "✅ Bravo, niveau réussi !",
    compile_error: "❌ Ça ne compile pas encore",
    runtime_error: "💥 Le programme a planté à l'exécution",
    timeout: "⏱️ Trop long — exécution interrompue",
    wrong_output: "🤔 Presque ! La sortie ne correspond pas",
    yourOutput: "Ta sortie",
    expectedOutput: "Sortie attendue",
    compilerSays: "Le compilateur dit",
    programSays: "Message du programme",
    newLevelUnlocked: "Niveau suivant débloqué !",
    allDone: "🎉 Tu as terminé tous les niveaux disponibles !",
    tipsTitle: "💡 Ça marche ! Mais on peut faire plus idiomatique (Clippy) :",
    suggestionLabel: "Suggestion",
    differsNote: "✔️ Ta solution est correcte, même si elle diffère de la solution proposée. Clippy n'a rien à redire — bravo ! (Tu peux comparer avec « 👁 Solution ».)",
    clippyNote: "Remarques de Clippy, le linter officiel de Rust (en anglais). Elles n'enlèvent pas ta réussite : ce sont des conseils de style.",
    resetAll: "Recommencer",
    palierSubtitle: "Palier de révision",
    palierLocked: "Termine le bloc de 10 niveaux pour débloquer ce palier.",
    palierLabel: "Palier",
  },
  en: {
    levels: "Levels",
    welcome: "Pick a level on the left to start your Rust adventure.",
    run: "Run",
    hint: "Hint",
    solution: "Solution",
    resetCode: "Reset",
    compiling: "Compiling…",
    locked: "Finish the previous level to unlock this one.",
    confirmSolution: "Showing the solution will replace your current code. Continue?",
    confirmResetCode: "Reset will restore the starter code. Continue?",
    confirmReset: "Reset all your progress to zero?",
    noMoreHints: "No more hints — check the lesson on the left!",
    hintLabel: "Hint",
    passed: "✅ Well done, level complete!",
    compile_error: "❌ It doesn't compile yet",
    runtime_error: "💥 The program crashed at runtime",
    timeout: "⏱️ Too slow — execution stopped",
    wrong_output: "🤔 Almost! The output doesn't match",
    yourOutput: "Your output",
    expectedOutput: "Expected output",
    compilerSays: "The compiler says",
    programSays: "Program message",
    newLevelUnlocked: "Next level unlocked!",
    allDone: "🎉 You finished every available level!",
    tipsTitle: "💡 It works! But here's a more idiomatic way (Clippy):",
    suggestionLabel: "Suggestion",
    differsNote: "✔️ Your solution is correct, even though it differs from the proposed one. Clippy has nothing to add — nice! (You can compare with \"👁 Solution\".)",
    clippyNote: "Notes from Clippy, Rust's official linter. They don't take away your success: they're style tips.",
    resetAll: "Reset",
    palierSubtitle: "Review checkpoint",
    palierLocked: "Finish the block of 10 levels to unlock this checkpoint.",
    palierLabel: "Checkpoint",
  },
};

// ----------------------------------------------------------------------------
// État global
// ----------------------------------------------------------------------------
let lang = localStorage.getItem("rustquest_lang") || "fr";
let currentLevelId = null;
let currentDetail = null;
let revealedHints = 0;
let lastLevels = [];
let lastPaliers = [];
let currentPalierAfter = null; // si un palier de révision est ouvert

const t = (k) => I18N[lang][k];
const $ = (id) => document.getElementById(id);

// ----------------------------------------------------------------------------
// Glossaire FR des remarques Clippy les plus fréquentes.
// Clé = nom du lint (sans le préfixe "clippy::"). Affiché sous le message
// anglais d'origine, en français, quand la langue de l'interface est FR.
// ----------------------------------------------------------------------------
const GLOSSAIRE_CLIPPY = {
  needless_range_loop:
    "Tu boucles sur des indices juste pour accéder aux éléments : itère directement (for x in &v).",
  bool_comparison: "Comparer à true/false est inutile : écris directement la condition.",
  useless_vec: "Un vec! n'est pas nécessaire ici : un tableau [..] suffit.",
  needless_return: "Le return final est superflu : la dernière expression est déjà renvoyée.",
  let_and_return:
    "Inutile de stocker dans une variable juste pour la renvoyer : renvoie l'expression directement.",
  assign_op_pattern: "Utilise un opérateur composé : x += y plutôt que x = x + y.",
  ptr_arg: "Préfère &[T] à &Vec<T> (ou &str à &String) en paramètre : c'est plus général.",
  redundant_clone: "Ce .clone() est inutile : la valeur n'a pas besoin d'être copiée.",
  clone_on_copy: "Inutile de .clone() un type Copy : il se copie tout seul.",
  needless_borrow: "Cet emprunt & est superflu : Rust le fait pour toi.",
  redundant_field_names: "Champ redondant : écris `x` au lieu de `x: x`.",
  len_zero: "Utilise .is_empty() plutôt que .len() == 0.",
  comparison_to_empty: "Compare avec .is_empty() plutôt qu'à une valeur vide.",
  single_char_pattern: "Utilise un caractère 'x' plutôt qu'une chaîne \"x\" comme motif.",
  manual_map: "Ce match peut se réécrire avec .map() sur l'Option/Result.",
  manual_filter: "Ce match/if peut se réécrire avec .filter().",
  collapsible_if: "Ces if imbriqués peuvent fusionner en une seule condition.",
  collapsible_else_if: "Le else { if ... } peut s'écrire else if ....",
  needless_lifetimes: "Cette annotation de durée de vie est superflue (élision possible).",
  unused_variables: "Variable inutilisée : préfixe-la d'un _ ou supprime-la.",
  unused_mut: "mut inutile : cette variable n'est jamais modifiée.",
  while_let_on_iterator: "Utilise une boucle for au lieu de while let sur un itérateur.",
  redundant_closure:
    "Closure superflue : passe directement la fonction (ex. .map(f) au lieu de .map(|x| f(x))).",
  identity_op: "Opération neutre inutile (ex. x + 0, x * 1).",
  needless_bool:
    "Renvoie directement la condition plutôt que if cond { true } else { false }.",
  question_mark: "Tu peux simplifier avec l'opérateur ?.",
  unnecessary_cast: "Conversion (as) inutile : le type est déjà le bon.",
  explicit_counter_loop: "Utilise .enumerate() plutôt qu'un compteur manuel.",
  manual_is_ascii_check: "Utilise une méthode is_ascii_* dédiée.",
  char_lit_as_u8: "Convertis proprement le caractère plutôt que via 'x' as u8.",
  too_many_arguments:
    "Beaucoup d'arguments : regroupe-les peut-être dans une struct dédiée.",
};

/// Renvoie l'explication FR d'un lint, ou null si inconnue.
function frenchGloss(lint) {
  if (!lint) return null;
  const key = lint.replace(/^clippy::/, "");
  return GLOSSAIRE_CLIPPY[key] || null;
}

// ----------------------------------------------------------------------------
// Appels API
// ----------------------------------------------------------------------------
async function fetchLevels() {
  const r = await fetch(`/api/levels?lang=${lang}`);
  return r.json();
}
async function fetchDetail(id) {
  const r = await fetch(`/api/levels/${id}?lang=${lang}`);
  return r.json();
}
async function fetchPaliers() {
  const r = await fetch(`/api/paliers?lang=${lang}`);
  return r.json();
}
async function postRun(levelId, code) {
  const r = await fetch("/api/run", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ level_id: levelId, code }),
  });
  return r.json();
}

// ----------------------------------------------------------------------------
// Rendu de la carte des niveaux + XP
// ----------------------------------------------------------------------------
async function refreshLevels() {
  const [data, paliers] = await Promise.all([fetchLevels(), fetchPaliers()]);
  lastLevels = data.levels;
  lastPaliers = paliers;

  // Barre d'XP
  const pct = data.total_xp ? (data.earned_xp / data.total_xp) * 100 : 0;
  $("xp-fill").style.width = pct + "%";
  $("xp-text").textContent = `${data.earned_xp} / ${data.total_xp} XP`;
  $("progress-count").textContent = `${data.completed_count}/${data.total_count}`;

  // Liste des niveaux, entrecoupée des paliers de révision.
  const list = $("level-list");
  list.innerHTML = "";
  for (const lv of data.levels) {
    const li = document.createElement("li");
    li.className = "level-item";
    if (lv.completed) li.classList.add("completed");
    if (!lv.unlocked) li.classList.add("locked");
    if (lv.id === currentLevelId) li.classList.add("active");

    const num = lv.completed ? "✓" : lv.unlocked ? lv.id : "🔒";
    li.innerHTML = `
      <div class="level-num">${num}</div>
      <div class="level-info">
        <div class="t">${escapeHtml(lv.title)}</div>
        <div class="s">${escapeHtml(lv.subtitle)} · ${lv.xp} XP</div>
      </div>`;

    li.addEventListener("click", () => {
      if (!lv.unlocked) {
        flashToast(t("locked"), false);
        return;
      }
      openLevel(lv.id);
    });
    list.appendChild(li);

    // Un palier de révision après ce niveau ?
    const palier = paliers.find((p) => p.after_level === lv.id);
    if (palier) {
      list.appendChild(buildPalierItem(palier));
    }
  }

  // Si un palier est ouvert, on le re-rend (langue / déblocage à jour).
  if (currentPalierAfter !== null) {
    const p = lastPaliers.find((x) => x.after_level === currentPalierAfter);
    if (p) renderPalier(p);
  }
}

function buildPalierItem(palier) {
  const li = document.createElement("li");
  li.className = "palier-item";
  if (!palier.unlocked) li.classList.add("locked");
  if (palier.after_level === currentPalierAfter) li.classList.add("active");

  const icon = palier.unlocked ? "★" : "🔒";
  li.innerHTML = `
    <div class="palier-star">${icon}</div>
    <div class="level-info">
      <div class="t">${escapeHtml(palier.title)}</div>
      <div class="s">${t("palierSubtitle")}</div>
    </div>`;

  li.addEventListener("click", () => {
    if (!palier.unlocked) {
      flashToast(t("palierLocked"), false);
      return;
    }
    openPalier(palier);
  });
  return li;
}

// ----------------------------------------------------------------------------
// Ouverture d'un niveau
// ----------------------------------------------------------------------------
async function openLevel(id) {
  currentLevelId = id;
  currentPalierAfter = null;
  revealedHints = 0;
  closeSidebar();
  const detail = await fetchDetail(id);
  currentDetail = detail;

  $("welcome").classList.add("hidden");
  $("level-view").classList.remove("hidden");
  $("exercise-pane").classList.remove("hidden"); // au cas où un palier était ouvert
  $("level-badge").classList.remove("palier-badge");

  $("level-badge").textContent = id;
  $("level-title").textContent = detail.title;
  $("level-subtitle").textContent = detail.subtitle;
  $("lesson").innerHTML = detail.lesson_html;
  $("task").textContent = detail.task;

  // Code : on garde ce que l'apprenant avait tapé, sinon le code de départ.
  const saved = localStorage.getItem("rustquest_code_" + id);
  $("editor").value = saved !== null ? saved : detail.starter;

  $("hints").innerHTML = "";
  $("output").innerHTML = "";

  // Met à jour la surbrillance dans la liste.
  document.querySelectorAll(".level-item").forEach((el) => el.classList.remove("active"));
  refreshLevels();
}

// Ouvre un palier de révision (pas d'exercice, juste le récapitulatif).
function openPalier(p) {
  currentLevelId = null;
  currentPalierAfter = p.after_level;
  closeSidebar();
  renderPalier(p);
  refreshLevels(); // met à jour la surbrillance dans la liste
}

function renderPalier(p) {
  $("welcome").classList.add("hidden");
  $("level-view").classList.remove("hidden");
  $("exercise-pane").classList.add("hidden"); // un palier n'a pas d'exercice
  $("level-badge").textContent = "★";
  $("level-badge").classList.add("palier-badge");
  $("level-title").textContent = p.title;
  $("level-subtitle").textContent = t("palierSubtitle");
  $("lesson").innerHTML = p.recap_html;
  $("lesson").scrollTop = 0;
}

// Re-rend la leçon dans la nouvelle langue sans toucher au code tapé.
async function reloadLessonLanguage() {
  if (currentLevelId === null) return;
  const detail = await fetchDetail(currentLevelId);
  currentDetail = detail;
  $("level-title").textContent = detail.title;
  $("level-subtitle").textContent = detail.subtitle;
  $("lesson").innerHTML = detail.lesson_html;
  $("task").textContent = detail.task;
  // Ré-affiche les indices déjà révélés, traduits.
  const shown = revealedHints;
  $("hints").innerHTML = "";
  revealedHints = 0;
  for (let i = 0; i < shown; i++) revealHint(true);
}

// ----------------------------------------------------------------------------
// Lancer le code
// ----------------------------------------------------------------------------
async function runCode() {
  if (currentLevelId === null) return;
  const code = $("editor").value;
  localStorage.setItem("rustquest_code_" + currentLevelId, code);

  const btn = $("run-btn");
  btn.disabled = true;
  const label = $("run-label").textContent;
  $("run-label").textContent = t("compiling");

  try {
    const res = await postRun(currentLevelId, code);
    renderOutcome(res);

    // Met à jour l'XP et la carte.
    await refreshLevels();

    if (res.outcome.success) {
      if (res.newly_completed) {
        flashToast("🎉 +" + currentDetail.xp + " XP", true);
        maybeAdvance();
      }
    }
  } catch (e) {
    $("output").innerHTML = `<div class="out-box err"><div class="out-title">Erreur réseau</div><div class="out-pre">${escapeHtml(String(e))}</div></div>`;
  } finally {
    btn.disabled = false;
    $("run-label").textContent = label;
  }
}

// Propose de passer au niveau suivant après une réussite.
function maybeAdvance() {
  const idx = lastLevels.findIndex((l) => l.id === currentLevelId);
  const next = lastLevels[idx + 1];
  if (next) {
    setTimeout(() => flashToast("➜ " + t("newLevelUnlocked"), true), 1600);
  } else {
    setTimeout(() => flashToast(t("allDone"), true), 1600);
  }
}

function renderOutcome(res) {
  const o = res.outcome;
  const ok = o.success;
  const box = document.createElement("div");
  box.className = "out-box " + (ok ? "ok" : "err");

  let html = `<div class="out-title">${t(o.stage)}</div>`;

  if (o.stage === "compile_error" && o.compile_output) {
    html += `<div class="out-label">${t("compilerSays")}</div>`;
    html += `<div class="out-pre">${escapeHtml(o.compile_output.trim())}</div>`;
  } else if (o.stage === "wrong_output") {
    html += `<div class="diff-row">
      <div class="diff-col">
        <div class="out-label">${t("yourOutput")}</div>
        <div class="out-pre">${escapeHtml(o.stdout) || "—"}</div>
      </div>
      <div class="diff-col">
        <div class="out-label">${t("expectedOutput")}</div>
        <div class="out-pre">${escapeHtml(o.expected || "")}</div>
      </div>
    </div>`;
  } else if (o.stage === "runtime_error" || o.stage === "timeout") {
    if (o.stdout) {
      html += `<div class="out-label">${t("yourOutput")}</div>`;
      html += `<div class="out-pre">${escapeHtml(o.stdout)}</div>`;
    }
    if (o.stderr) {
      html += `<div class="out-label">${t("programSays")}</div>`;
      html += `<div class="out-pre">${escapeHtml(o.stderr.trim())}</div>`;
    }
  } else if (o.stage === "passed") {
    if (o.stdout) {
      html += `<div class="out-label">${t("yourOutput")}</div>`;
      html += `<div class="out-pre">${escapeHtml(o.stdout)}</div>`;
    }
    if (o.tips && o.tips.length) {
      html += `<div class="tips-block">`;
      html += `<div class="tips-title">${t("tipsTitle")}</div>`;
      for (const tip of o.tips) {
        html += `<div class="tip-card">`;
        html += `<div class="tip-msg">${escapeHtml(tip.message)}</div>`;
        const fr = lang === "fr" ? frenchGloss(tip.lint) : null;
        if (fr) {
          html += `<div class="tip-fr">🇫🇷 ${escapeHtml(fr)}</div>`;
        }
        if (tip.suggestion) {
          html += `<div class="tip-sugg"><span>${t("suggestionLabel")} :</span> <code>${escapeHtml(tip.suggestion)}</code></div>`;
        }
        html += `</div>`;
      }
      html += `<div class="tips-foot">${t("clippyNote")}</div>`;
      html += `</div>`;
    } else if (o.differs) {
      html += `<div class="differs-note">${t("differsNote")}</div>`;
    }
  }

  box.innerHTML = html;
  const out = $("output");
  out.innerHTML = "";
  out.appendChild(box);
}

// ----------------------------------------------------------------------------
// Indices & solution
// ----------------------------------------------------------------------------
function revealHint(silent) {
  if (!currentDetail) return;
  const hints = currentDetail.hints || [];
  if (revealedHints >= hints.length) {
    if (!silent) flashToast(t("noMoreHints"), false);
    return;
  }
  const i = revealedHints;
  const div = document.createElement("div");
  div.className = "hint";
  div.innerHTML = `<strong>${t("hintLabel")} ${i + 1} :</strong> ${escapeHtml(hints[i])}`;
  $("hints").appendChild(div);
  revealedHints++;
}

function showSolution() {
  if (!currentDetail) return;
  if (!confirm(t("confirmSolution"))) return;
  $("editor").value = currentDetail.solution;
  localStorage.setItem("rustquest_code_" + currentLevelId, currentDetail.solution);
}

function resetCode() {
  if (!currentDetail) return;
  if (!confirm(t("confirmResetCode"))) return;
  $("editor").value = currentDetail.starter;
  localStorage.removeItem("rustquest_code_" + currentLevelId);
}

// ----------------------------------------------------------------------------
// Langue & remise à zéro
// ----------------------------------------------------------------------------
function setLang(newLang) {
  lang = newLang;
  localStorage.setItem("rustquest_lang", lang);
  document.documentElement.lang = lang;
  document.querySelectorAll("#lang-toggle button").forEach((b) =>
    b.classList.toggle("active", b.dataset.lang === lang)
  );
  applyStaticLabels();
  refreshLevels();
  reloadLessonLanguage();
}

function applyStaticLabels() {
  $("sidebar-title").textContent = t("levels");
  $("welcome-text").textContent = t("welcome");
  $("run-label").textContent = t("run");
  $("hint-label").textContent = t("hint");
  $("solution-label").textContent = t("solution");
  $("resetcode-label").textContent = t("resetCode");
  $("reset-label").textContent = t("resetAll");
}

async function resetProgress() {
  if (!confirm(t("confirmReset"))) return;
  await fetch("/api/reset", { method: "POST" });
  // On efface aussi les codes sauvegardés localement.
  Object.keys(localStorage)
    .filter((k) => k.startsWith("rustquest_code_"))
    .forEach((k) => localStorage.removeItem(k));
  currentLevelId = null;
  currentDetail = null;
  currentPalierAfter = null;
  $("level-view").classList.add("hidden");
  $("welcome").classList.remove("hidden");
  refreshLevels();
}

// ----------------------------------------------------------------------------
// Utilitaires
// ----------------------------------------------------------------------------
function escapeHtml(s) {
  return String(s)
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;");
}

let toastTimer = null;
function flashToast(msg, good) {
  const el = $("toast");
  el.textContent = msg;
  el.style.background = good
    ? "linear-gradient(135deg, #6cc04a, #4e9e30)"
    : "linear-gradient(135deg, #e0524d, #b23f3a)";
  el.classList.remove("hidden");
  // reflow pour relancer la transition
  void el.offsetWidth;
  el.classList.add("show");
  clearTimeout(toastTimer);
  toastTimer = setTimeout(() => el.classList.remove("show"), 2600);
}

// Menu latéral (drawer) sur mobile.
function toggleSidebar() {
  const open = $("sidebar").classList.toggle("open");
  $("scrim").classList.toggle("show", open);
}
function closeSidebar() {
  $("sidebar").classList.remove("open");
  $("scrim").classList.remove("show");
}

// Tabulation = 4 espaces dans l'éditeur.
function handleEditorKeys(e) {
  if (e.key === "Tab") {
    e.preventDefault();
    const ta = e.target;
    const start = ta.selectionStart;
    const end = ta.selectionEnd;
    ta.value = ta.value.slice(0, start) + "    " + ta.value.slice(end);
    ta.selectionStart = ta.selectionEnd = start + 4;
  }
  // Ctrl/Cmd + Entrée = lancer
  if ((e.ctrlKey || e.metaKey) && e.key === "Enter") {
    e.preventDefault();
    runCode();
  }
}

// ----------------------------------------------------------------------------
// Initialisation
// ----------------------------------------------------------------------------
function init() {
  document.documentElement.lang = lang;
  document.querySelectorAll("#lang-toggle button").forEach((b) => {
    b.classList.toggle("active", b.dataset.lang === lang);
    b.addEventListener("click", () => setLang(b.dataset.lang));
  });

  $("run-btn").addEventListener("click", runCode);
  $("hint-btn").addEventListener("click", () => revealHint(false));
  $("solution-btn").addEventListener("click", showSolution);
  $("reset-code-btn").addEventListener("click", resetCode);
  $("reset-btn").addEventListener("click", resetProgress);
  $("menu-btn").addEventListener("click", toggleSidebar);
  $("scrim").addEventListener("click", closeSidebar);
  $("editor").addEventListener("keydown", handleEditorKeys);
  $("editor").addEventListener("input", () => {
    if (currentLevelId !== null)
      localStorage.setItem("rustquest_code_" + currentLevelId, $("editor").value);
  });

  applyStaticLabels();
  refreshLevels();
}

init();
