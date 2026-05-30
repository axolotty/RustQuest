//! Progression de l'apprenant : quels niveaux sont réussis, combien d'XP.
//!
//! On sauvegarde dans un simple fichier JSON (`progress.json`) à la racine du
//! projet. Un `Mutex` protège l'accès concurrent depuis le serveur.

use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::path::PathBuf;
use std::sync::Mutex;

const PROGRESS_FILE: &str = "progress.json";

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Progress {
    /// Identifiants des niveaux réussis.
    pub completed: BTreeSet<u32>,
}

/// Magasin protégé par un Mutex, partagé entre toutes les requêtes.
pub struct Store {
    inner: Mutex<Progress>,
    path: PathBuf,
}

impl Store {
    /// Charge la progression depuis le disque, ou démarre à zéro.
    pub fn load() -> Self {
        let path = PathBuf::from(PROGRESS_FILE);
        let progress = std::fs::read_to_string(&path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default();
        Store {
            inner: Mutex::new(progress),
            path,
        }
    }

    /// Renvoie une copie de la progression actuelle.
    pub fn snapshot(&self) -> Progress {
        self.inner.lock().unwrap().clone()
    }

    /// Marque un niveau comme réussi et sauvegarde.
    pub fn mark_completed(&self, level_id: u32) {
        {
            let mut p = self.inner.lock().unwrap();
            p.completed.insert(level_id);
        }
        self.save();
    }

    /// Remet la progression à zéro (utile pour recommencer).
    pub fn reset(&self) {
        {
            let mut p = self.inner.lock().unwrap();
            p.completed.clear();
        }
        self.save();
    }

    fn save(&self) {
        let p = self.inner.lock().unwrap();
        if let Ok(json) = serde_json::to_string_pretty(&*p) {
            let _ = std::fs::write(&self.path, json);
        }
    }
}
