//! RustQuest — un serveur web local pour apprendre le Rust par niveaux.
//!
//! Lance `cargo run`, puis ouvre http://127.0.0.1:3000 dans ton navigateur.

mod auth;
mod content;
mod progress;
mod runner;

use axum::{
    extract::{Path, Query, Request, State},
    http::{header, HeaderMap, StatusCode},
    middleware::Next,
    response::{Html, IntoResponse, Redirect, Response},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// État partagé entre toutes les requêtes.
struct AppState {
    store: progress::Store,
    /// Authentification (page de connexion) — `None` si désactivée.
    auth: Option<auth::Auth>,
}

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState {
        store: progress::Store::load(),
        auth: auth::Auth::from_env(),
    });
    let auth_active = state.auth.is_some();

    let app = Router::new()
        .route("/", get(index))
        .route("/login", get(login_page))
        .route("/style.css", get(style))
        .route("/app.js", get(app_js))
        .route("/api/auth-status", get(auth_status))
        .route("/api/login", post(api_login))
        .route("/api/logout", post(api_logout))
        .route("/api/levels", get(list_levels))
        .route("/api/levels/{id}", get(level_detail))
        .route("/api/paliers", get(list_paliers))
        .route("/api/run", post(run))
        .route("/api/reset", post(reset))
        // Garde d'authentification : laisse tout passer si l'auth est désactivée,
        // sinon exige une session valide (sauf /login et /api/login).
        .layer(axum::middleware::from_fn_with_state(state.clone(), auth_guard))
        .with_state(state);

    // Adresse d'écoute configurable (127.0.0.1:3000 par défaut en local,
    // 0.0.0.0:3000 typiquement dans Docker).
    let addr = std::env::var("RUSTQUEST_BIND").unwrap_or_else(|_| "127.0.0.1:3000".to_string());
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("\n  🦀 RustQuest est lancé !");
    println!("  ➜  Écoute sur http://{addr}");
    if auth_active {
        println!("  🔒 Authentification : ACTIVÉE (page de connexion)");
    } else {
        println!("  🔓 Authentification : désactivée (aucun RUSTQUEST_AUTH_* défini)");
    }
    println!();

    axum::serve(listener, app).await.unwrap();
}

// ---------------------------------------------------------------------------
// Authentification : garde de session + page de connexion
// ---------------------------------------------------------------------------

/// Middleware : exige une session valide, sauf pour les routes publiques.
async fn auth_guard(State(state): State<Arc<AppState>>, req: Request, next: Next) -> Response {
    let Some(auth) = &state.auth else {
        return next.run(req).await; // auth désactivée : tout passe
    };

    let path = req.uri().path();
    if path == "/login" || path == "/api/login" {
        return next.run(req).await; // routes publiques (sinon on ne pourrait pas se connecter)
    }

    if let Some(token) = auth::read_session_cookie(req.headers()) {
        if auth.sessions.is_valid(&token) {
            return next.run(req).await;
        }
    }

    // Non authentifié : 401 pour l'API, redirection vers /login pour une page.
    if path.starts_with("/api/") {
        StatusCode::UNAUTHORIZED.into_response()
    } else {
        Redirect::to("/login").into_response()
    }
}

/// GET /login — la page de connexion (ou redirection si l'auth est désactivée).
async fn login_page(State(state): State<Arc<AppState>>) -> Response {
    if state.auth.is_none() {
        return Redirect::to("/").into_response();
    }
    Html(include_str!("../web/login.html")).into_response()
}

#[derive(Deserialize)]
struct LoginRequest {
    user: String,
    password: String,
}

/// POST /api/login — vérifie les identifiants et ouvre une session.
async fn api_login(State(state): State<Arc<AppState>>, Json(body): Json<LoginRequest>) -> Response {
    let Some(auth) = &state.auth else {
        return Json(serde_json::json!({ "ok": true })).into_response();
    };

    if auth.config.verify(&body.user, &body.password) {
        let token = auth.sessions.create();
        let mut resp = Json(serde_json::json!({ "ok": true })).into_response();
        resp.headers_mut().insert(
            header::SET_COOKIE,
            auth::set_cookie_header(&token).parse().unwrap(),
        );
        resp
    } else {
        (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({ "ok": false })),
        )
            .into_response()
    }
}

/// POST /api/logout — ferme la session et efface le cookie.
async fn api_logout(State(state): State<Arc<AppState>>, headers: HeaderMap) -> Response {
    if let Some(auth) = &state.auth {
        if let Some(token) = auth::read_session_cookie(&headers) {
            auth.sessions.remove(&token);
        }
    }
    let mut resp = Json(serde_json::json!({ "ok": true })).into_response();
    resp.headers_mut().insert(
        header::SET_COOKIE,
        auth::clear_cookie_header().parse().unwrap(),
    );
    resp
}

/// GET /api/auth-status — indique au frontend si l'auth est active (pour
/// afficher le bouton de déconnexion). N'est atteint que par un utilisateur
/// authentifié (ou quand l'auth est désactivée).
async fn auth_status(State(state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    Json(serde_json::json!({ "enabled": state.auth.is_some() }))
}

// ---------------------------------------------------------------------------
// Fichiers statiques (embarqués dans le binaire)
// ---------------------------------------------------------------------------

async fn index() -> Html<&'static str> {
    Html(include_str!("../web/index.html"))
}

async fn style() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "text/css; charset=utf-8")],
        include_str!("../web/style.css"),
    )
}

async fn app_js() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "application/javascript; charset=utf-8")],
        include_str!("../web/app.js"),
    )
}

// ---------------------------------------------------------------------------
// API
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
struct LangQuery {
    lang: Option<String>,
}

impl LangQuery {
    fn lang(&self) -> &str {
        match self.lang.as_deref() {
            Some("en") => "en",
            _ => "fr",
        }
    }
}

#[derive(Serialize)]
struct LevelCard {
    id: u32,
    slug: &'static str,
    title: &'static str,
    subtitle: &'static str,
    xp: u32,
    completed: bool,
    unlocked: bool,
}

#[derive(Serialize)]
struct LevelsResponse {
    levels: Vec<LevelCard>,
    earned_xp: u32,
    total_xp: u32,
    completed_count: usize,
    total_count: usize,
}

/// GET /api/levels?lang=fr — la carte des niveaux + la progression.
async fn list_levels(
    State(state): State<Arc<AppState>>,
    Query(q): Query<LangQuery>,
) -> Json<LevelsResponse> {
    let lang = q.lang();
    let progress = state.store.snapshot();

    let mut levels = Vec::new();
    let mut earned_xp = 0;
    let mut previous_completed = true; // le premier niveau est toujours déverrouillé

    for level in content::LEVELS {
        let completed = progress.completed.contains(&level.id);
        let unlocked = previous_completed || completed;
        if completed {
            earned_xp += level.xp;
        }
        let s = level.summary(lang);
        levels.push(LevelCard {
            id: s.id,
            slug: s.slug,
            title: s.title,
            subtitle: s.subtitle,
            xp: s.xp,
            completed,
            unlocked,
        });
        previous_completed = completed;
    }

    Json(LevelsResponse {
        levels,
        earned_xp,
        total_xp: content::total_xp(),
        completed_count: progress.completed.len(),
        total_count: content::LEVELS.len(),
    })
}

/// GET /api/paliers?lang=fr — les paliers de révision et leur statut.
async fn list_paliers(
    State(state): State<Arc<AppState>>,
    Query(q): Query<LangQuery>,
) -> Json<Vec<content::PalierView>> {
    let lang = q.lang();
    let progress = state.store.snapshot();
    let views = content::PALIERS
        .iter()
        .map(|p| {
            let unlocked = progress.completed.contains(&p.after_level);
            p.view(lang, unlocked)
        })
        .collect();
    Json(views)
}

/// GET /api/levels/{id}?lang=fr — le détail d'un niveau (leçon + exercice).
async fn level_detail(Path(id): Path<u32>, Query(q): Query<LangQuery>) -> impl IntoResponse {
    match content::level_by_id(id) {
        Some(level) => Json(level.detail(q.lang())).into_response(),
        None => (axum::http::StatusCode::NOT_FOUND, "niveau introuvable").into_response(),
    }
}

#[derive(Deserialize)]
struct RunRequest {
    level_id: u32,
    code: String,
}

#[derive(Serialize)]
struct RunResponse {
    /// Résultat de la tentative (imbriqué sous `outcome` côté JSON).
    outcome: runner::RunOutcome,
    /// XP gagné jusqu'ici (mis à jour si on vient de réussir).
    earned_xp: u32,
    /// Vrai si ce niveau vient juste d'être validé pour la première fois.
    newly_completed: bool,
}

/// POST /api/run — compile, exécute et vérifie le code soumis.
async fn run(
    State(state): State<Arc<AppState>>,
    Json(req): Json<RunRequest>,
) -> impl IntoResponse {
    let Some(level) = content::level_by_id(req.level_id) else {
        return (axum::http::StatusCode::NOT_FOUND, "niveau introuvable").into_response();
    };

    let already_done = state.store.snapshot().completed.contains(&level.id);

    // La compilation/exécution est bloquante : on la sort du runtime async.
    let code = req.code;
    let outcome = tokio::task::spawn_blocking(move || runner::run_check(level, &code))
        .await
        .unwrap();

    let newly_completed = outcome.success && !already_done;
    if outcome.success {
        state.store.mark_completed(level.id);
    }

    // Recalcule l'XP total gagné.
    let progress = state.store.snapshot();
    let earned_xp = content::LEVELS
        .iter()
        .filter(|l| progress.completed.contains(&l.id))
        .map(|l| l.xp)
        .sum();

    Json(RunResponse {
        outcome,
        earned_xp,
        newly_completed,
    })
    .into_response()
}

/// POST /api/reset — remet la progression à zéro.
async fn reset(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    state.store.reset();
    Json(serde_json::json!({ "ok": true }))
}
