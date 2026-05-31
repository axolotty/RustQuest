//! Authentification par **page de connexion** + cookie de session, optionnelle.
//!
//! Si `RUSTQUEST_AUTH_USER` et `RUSTQUEST_AUTH_PASSWORD` sont définis, l'accès
//! passe par une jolie page `/login` : on vérifie les identifiants, on crée une
//! **session côté serveur** (jeton aléatoire) et on pose un **cookie**.
//! Sinon (pas de variables), l'authentification est désactivée (dév local).
//!
//! ⚠️ À n'exposer sur Internet **qu'au travers de HTTPS** (reverse proxy).
//! Pose `RUSTQUEST_SECURE_COOKIE=true` pour ajouter l'attribut `Secure` au
//! cookie quand tu es bien derrière HTTPS.

use axum::http::{header, HeaderMap};
use std::collections::HashSet;
use std::io::Read;
use std::sync::Mutex;

const COOKIE_NAME: &str = "rustquest_session";

/// Regroupe la config (identifiants) et les sessions actives.
pub struct Auth {
    pub config: AuthConfig,
    pub sessions: Sessions,
}

impl Auth {
    /// Construit l'auth depuis l'environnement, ou `None` si désactivée.
    pub fn from_env() -> Option<Self> {
        Some(Auth {
            config: AuthConfig::from_env()?,
            sessions: Sessions::new(),
        })
    }
}

/// Identifiants attendus, lus dans l'environnement.
pub struct AuthConfig {
    user: String,
    password: String,
}

impl AuthConfig {
    fn from_env() -> Option<Self> {
        let user = std::env::var("RUSTQUEST_AUTH_USER").ok()?;
        let password = std::env::var("RUSTQUEST_AUTH_PASSWORD").ok()?;
        if user.is_empty() || password.is_empty() {
            return None;
        }
        Some(AuthConfig { user, password })
    }

    /// Vérifie un couple identifiant/mot de passe (comparaison à temps constant).
    pub fn verify(&self, user: &str, password: &str) -> bool {
        constant_time_eq(user.as_bytes(), self.user.as_bytes())
            & constant_time_eq(password.as_bytes(), self.password.as_bytes())
    }
}

/// L'ensemble des jetons de session valides (en mémoire).
pub struct Sessions {
    inner: Mutex<HashSet<String>>,
}

impl Default for Sessions {
    fn default() -> Self {
        Self::new()
    }
}

impl Sessions {
    pub fn new() -> Self {
        Sessions {
            inner: Mutex::new(HashSet::new()),
        }
    }

    /// Crée une nouvelle session et renvoie son jeton.
    pub fn create(&self) -> String {
        let token = random_token();
        self.inner.lock().unwrap().insert(token.clone());
        token
    }

    pub fn is_valid(&self, token: &str) -> bool {
        self.inner.lock().unwrap().contains(token)
    }

    pub fn remove(&self, token: &str) {
        self.inner.lock().unwrap().remove(token);
    }
}

/// Lit le jeton de session dans l'en-tête `Cookie`, s'il est présent.
pub fn read_session_cookie(headers: &HeaderMap) -> Option<String> {
    let raw = headers.get(header::COOKIE)?.to_str().ok()?;
    let prefix = format!("{COOKIE_NAME}=");
    for part in raw.split(';') {
        if let Some(value) = part.trim().strip_prefix(&prefix) {
            if !value.is_empty() {
                return Some(value.to_string());
            }
        }
    }
    None
}

/// En-tête `Set-Cookie` qui installe la session (7 jours).
pub fn set_cookie_header(token: &str) -> String {
    let base = format!("{COOKIE_NAME}={token}; HttpOnly; SameSite=Strict; Path=/; Max-Age=604800");
    finish_cookie(base)
}

/// En-tête `Set-Cookie` qui efface la session.
pub fn clear_cookie_header() -> String {
    let base = format!("{COOKIE_NAME}=; HttpOnly; SameSite=Strict; Path=/; Max-Age=0");
    finish_cookie(base)
}

fn finish_cookie(base: String) -> String {
    let secure = std::env::var("RUSTQUEST_SECURE_COOKIE")
        .map(|v| v == "true" || v == "1")
        .unwrap_or(false);
    if secure {
        format!("{base}; Secure")
    } else {
        base
    }
}

/// Jeton aléatoire de 256 bits (lu depuis /dev/urandom), en hexadécimal.
fn random_token() -> String {
    let mut buf = [0u8; 32];
    if let Ok(mut f) = std::fs::File::open("/dev/urandom") {
        if f.read_exact(&mut buf).is_ok() {
            return buf.iter().map(|b| format!("{b:02x}")).collect();
        }
    }
    // Repli (rare) si /dev/urandom est indisponible : horloge + compteur.
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let t = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    format!("{t:032x}{n:016x}")
}

/// Comparaison d'octets à temps constant (ne court-circuite pas).
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}
