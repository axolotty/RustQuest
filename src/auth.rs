//! Authentification HTTP Basic, optionnelle.
//!
//! Si les variables d'environnement `RUSTQUEST_AUTH_USER` et
//! `RUSTQUEST_AUTH_PASSWORD` sont définies, **toutes** les routes sont
//! protégées : le navigateur demande un identifiant et un mot de passe.
//!
//! Si elles ne sont pas définies (cas du `cargo run` local), l'authentification
//! est désactivée — pratique pour développer sur sa machine.
//!
//! ⚠️ À n'exposer sur Internet **qu'au travers de HTTPS** (voir le reverse
//! proxy Caddy fourni). En clair, le mot de passe Basic circulerait en clair.

use axum::{
    body::Body,
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
};
use base64::Engine;

/// Identifiants attendus, lus dans l'environnement.
#[derive(Clone)]
pub struct AuthConfig {
    user: String,
    password: String,
}

impl AuthConfig {
    /// Construit la config depuis l'environnement, ou `None` si l'auth est
    /// désactivée (variables absentes ou vides).
    pub fn from_env() -> Option<Self> {
        let user = std::env::var("RUSTQUEST_AUTH_USER").ok()?;
        let password = std::env::var("RUSTQUEST_AUTH_PASSWORD").ok()?;
        if user.is_empty() || password.is_empty() {
            return None;
        }
        Some(AuthConfig { user, password })
    }
}

/// Middleware axum : vérifie l'en-tête `Authorization: Basic ...`.
pub async fn require_basic_auth(
    State(cfg): State<AuthConfig>,
    req: Request,
    next: Next,
) -> Response {
    if let Some(value) = req.headers().get(header::AUTHORIZATION) {
        if let Some(credentials) = value
            .to_str()
            .ok()
            .and_then(|v| v.strip_prefix("Basic "))
            .and_then(decode_basic)
        {
            let (user, password) = credentials;
            // Comparaison à temps constant pour éviter les attaques temporelles.
            let ok = constant_time_eq(user.as_bytes(), cfg.user.as_bytes())
                & constant_time_eq(password.as_bytes(), cfg.password.as_bytes());
            if ok {
                return next.run(req).await;
            }
        }
    }
    unauthorized()
}

/// Décode `base64(user:password)` en `(user, password)`.
fn decode_basic(encoded: &str) -> Option<(String, String)> {
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(encoded.trim())
        .ok()?;
    let decoded = String::from_utf8(bytes).ok()?;
    let (user, password) = decoded.split_once(':')?;
    Some((user.to_string(), password.to_string()))
}

/// Réponse 401 demandant une authentification Basic.
fn unauthorized() -> Response {
    Response::builder()
        .status(StatusCode::UNAUTHORIZED)
        .header(header::WWW_AUTHENTICATE, "Basic realm=\"RustQuest\", charset=\"UTF-8\"")
        .body(Body::from("Authentification requise."))
        .unwrap()
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
