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
use std::collections::HashMap;
use std::io::Read;
use std::sync::Mutex;
use std::time::{Duration, Instant};

const COOKIE_NAME: &str = "rustquest_session";

/// Durée de vie d'une session côté serveur — alignée sur le `Max-Age` du cookie
/// (7 jours). Au-delà, le jeton est considéré invalide et purgé.
const SESSION_TTL: Duration = Duration::from_secs(604800);

/// Nombre maximal de tentatives de connexion échouées par client et par fenêtre.
const LOGIN_MAX_ATTEMPTS: u32 = 10;
/// Fenêtre glissante du limiteur de connexion.
const LOGIN_WINDOW: Duration = Duration::from_secs(900); // 15 min

/// Regroupe la config (identifiants), les sessions actives et le limiteur de
/// tentatives de connexion.
pub struct Auth {
    pub config: AuthConfig,
    pub sessions: Sessions,
    pub throttle: LoginThrottle,
}

impl Auth {
    /// Construit l'auth depuis l'environnement, ou `None` si désactivée.
    pub fn from_env() -> Option<Self> {
        Some(Auth {
            config: AuthConfig::from_env()?,
            sessions: Sessions::new(),
            throttle: LoginThrottle::new(),
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

/// L'ensemble des jetons de session valides (en mémoire), avec leur instant de
/// création pour appliquer une expiration (TTL) et purger les sessions périmées.
pub struct Sessions {
    inner: Mutex<HashMap<String, Instant>>,
}

impl Default for Sessions {
    fn default() -> Self {
        Self::new()
    }
}

impl Sessions {
    pub fn new() -> Self {
        Sessions {
            inner: Mutex::new(HashMap::new()),
        }
    }

    /// Crée une nouvelle session et renvoie son jeton.
    /// En profite pour purger au passage les jetons expirés (pas de croissance
    /// mémoire non bornée).
    pub fn create(&self) -> String {
        let token = random_token();
        let now = Instant::now();
        let mut map = self.inner.lock().unwrap();
        map.retain(|_, created| now.duration_since(*created) < SESSION_TTL);
        map.insert(token.clone(), now);
        token
    }

    pub fn is_valid(&self, token: &str) -> bool {
        let mut map = self.inner.lock().unwrap();
        match map.get(token) {
            Some(created) if Instant::now().duration_since(*created) < SESSION_TTL => true,
            Some(_) => {
                // Session expirée : on la retire et on refuse.
                map.remove(token);
                false
            }
            None => false,
        }
    }

    pub fn remove(&self, token: &str) {
        self.inner.lock().unwrap().remove(token);
    }
}

/// Limiteur de tentatives de connexion par client (fenêtre glissante en
/// mémoire). Protège `/api/login` du brute-force.
pub struct LoginThrottle {
    inner: Mutex<HashMap<String, Attempts>>,
}

struct Attempts {
    count: u32,
    window_start: Instant,
}

impl Default for LoginThrottle {
    fn default() -> Self {
        Self::new()
    }
}

impl LoginThrottle {
    pub fn new() -> Self {
        LoginThrottle {
            inner: Mutex::new(HashMap::new()),
        }
    }

    /// Renvoie `true` si une tentative est encore autorisée pour ce client.
    /// Purge au passage les fenêtres expirées.
    pub fn allowed(&self, key: &str) -> bool {
        let now = Instant::now();
        let mut map = self.inner.lock().unwrap();
        map.retain(|_, a| now.duration_since(a.window_start) < LOGIN_WINDOW);
        match map.get(key) {
            Some(a) => a.count < LOGIN_MAX_ATTEMPTS,
            None => true,
        }
    }

    /// Enregistre une tentative échouée pour ce client.
    pub fn record_failure(&self, key: &str) {
        let now = Instant::now();
        let mut map = self.inner.lock().unwrap();
        let entry = map.entry(key.to_string()).or_insert(Attempts {
            count: 0,
            window_start: now,
        });
        if now.duration_since(entry.window_start) >= LOGIN_WINDOW {
            entry.count = 0;
            entry.window_start = now;
        }
        entry.count += 1;
    }

    /// Réinitialise le compteur après une connexion réussie.
    pub fn record_success(&self, key: &str) {
        self.inner.lock().unwrap().remove(key);
    }
}

/// Identifie le client pour le limiteur de connexion : première IP de
/// `X-Forwarded-For` (renseignée par le reverse proxy), sinon `X-Real-IP`,
/// sinon une clé constante (limiteur global de repli).
pub fn client_key(headers: &HeaderMap) -> String {
    if let Some(xff) = headers.get("x-forwarded-for").and_then(|v| v.to_str().ok()) {
        if let Some(first) = xff.split(',').next() {
            let ip = first.trim();
            if !ip.is_empty() {
                return ip.to_string();
            }
        }
    }
    if let Some(xr) = headers.get("x-real-ip").and_then(|v| v.to_str().ok()) {
        let ip = xr.trim();
        if !ip.is_empty() {
            return ip.to_string();
        }
    }
    "unknown".to_string()
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
