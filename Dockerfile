# =============================================================================
# RustQuest — image Docker
#
# Particularité : l'application compile et exécute le code des apprenants. Son
# image d'exécution doit donc embarquer la chaîne d'outils Rust (rustc + clippy),
# pas seulement le binaire. C'est pour ça que l'étape finale part de `rust:slim`.
# =============================================================================

# ---- Étape 1 : compilation du serveur ----------------------------------------
FROM rust:1.95-slim AS builder
WORKDIR /app

# 1) On copie d'abord les manifestes pour mettre en cache la compilation des
#    dépendances (tant que Cargo.toml/Cargo.lock ne changent pas).
COPY Cargo.toml Cargo.lock ./
RUN mkdir src \
    && echo "fn main() {}" > src/main.rs \
    && cargo build --release \
    && rm -rf src

# 2) Puis le vrai code (y compris le dossier web/ embarqué via include_str!).
COPY . .
# L'étape de cache ci-dessus a compilé un main.rs FACTICE. On « touche » les
# sources pour que Cargo recompile bien le VRAI binaire : sans ça, selon les
# dates de fichiers issues de COPY, Cargo pourrait garder le binaire factice
# (qui se contente de se terminer immédiatement avec le code 0).
RUN touch src/*.rs && cargo build --release

# ---- Étape 2 : image d'exécution ---------------------------------------------
FROM rust:1.95-slim AS runtime

# Clippy est nécessaire au runtime (l'app l'invoque pour les conseils
# idiomatiques). On crée un lien sûr vers clippy-driver pour qu'il soit dans le
# PATH de tous les utilisateurs.
RUN rustup component add clippy \
    && ln -sf "$(rustc --print sysroot)/bin/clippy-driver" /usr/local/bin/clippy-driver

# Utilisateur non privilégié + dossier de données inscriptible.
RUN useradd --create-home --uid 10001 rustquest \
    && mkdir -p /data \
    && chown rustquest:rustquest /data

COPY --from=builder /app/target/release/rust_quest /usr/local/bin/rustquest

USER rustquest
WORKDIR /data

# L'app écrit sa progression dans le répertoire courant (/data, monté en volume).
ENV RUSTQUEST_BIND=0.0.0.0:3000
EXPOSE 3000

CMD ["rustquest"]
