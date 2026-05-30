//! RustQuest — un serveur web local pour apprendre le Rust par niveaux.
//!
//! Lance `cargo run`, puis ouvre http://127.0.0.1:3000 dans ton navigateur.

mod auth;
mod content;
mod progress;
mod runner;

use axum::{
    extract::{Path, Query, State},
    http::header,
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// État partagé entre toutes les requêtes.
struct AppState {
    store: progress::Store,
}

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState {
        store: progress::Store::load(),
    });

    let app = Router::new()
        .route("/", get(index))
        .route("/style.css", get(style))
        .route("/app.js", get(app_js))
        .route("/api/levels", get(list_levels))
        .route("/api/levels/{id}", get(level_detail))
        .route("/api/paliers", get(list_paliers))
        .route("/api/run", post(run))
        .route("/api/reset", post(reset))
        .with_state(state);

    // Authentification HTTP Basic, activée uniquement si les variables
    // d'environnement RUSTQUEST_AUTH_USER / RUSTQUEST_AUTH_PASSWORD sont posées.
    let (app, auth_active) = match auth::AuthConfig::from_env() {
        Some(cfg) => (
            app.layer(axum::middleware::from_fn_with_state(
                cfg,
                auth::require_basic_auth,
            )),
            true,
        ),
        None => (app, false),
    };

    // Adresse d'écoute configurable (127.0.0.1:3000 par défaut en local,
    // 0.0.0.0:3000 typiquement dans Docker).
    let addr = std::env::var("RUSTQUEST_BIND").unwrap_or_else(|_| "127.0.0.1:3000".to_string());
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("\n  🦀 RustQuest est lancé !");
    println!("  ➜  Écoute sur http://{addr}");
    if auth_active {
        println!("  🔒 Authentification : ACTIVÉE (Basic Auth)");
    } else {
        println!("  🔓 Authentification : désactivée (aucun RUSTQUEST_AUTH_* défini)");
    }
    println!();

    axum::serve(listener, app).await.unwrap();
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
