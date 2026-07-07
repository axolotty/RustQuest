# 🦀 RustQuest

[![CI](https://github.com/Axolotty/RustQuest/actions/workflows/ci.yml/badge.svg)](https://github.com/Axolotty/RustQuest/actions/workflows/ci.yml)

Une plateforme web locale pour apprendre le **Rust** de façon ludique :
des niveaux qui se débloquent un à un, des leçons illustrées (tirées de la
logique du *Rust Book*), des exercices que tu corriges dans le navigateur, et
de l'XP à gagner. Disponible en **français** et en **anglais** (bouton FR/EN).

## Lancer

```bash
cargo run
```

Puis ouvre **http://127.0.0.1:3000** dans ton navigateur.

> La première compilation télécharge les dépendances et prend ~1 minute.
> Les suivantes sont quasi instantanées.

## Comment ça marche

- Choisis un niveau dans la colonne de gauche.
- Lis la **leçon** (panneau de gauche), puis fais l'**exercice** (panneau de
  droite).
- Clique sur **▶ Lancer** (ou `Ctrl`/`Cmd` + `Entrée`) : ton code est
  **réellement compilé et exécuté** par le serveur, puis vérifié.
- En cas d'erreur, tu vois le message exact du compilateur Rust - c'est lui
  ton meilleur prof.
- Si ta solution **marche mais n'est pas idiomatique**, elle est quand même
  validée, et **Clippy** (le linter officiel) te suggère comment l'améliorer.
- Bloqué ? Utilise **💡 Indice** (révélés un par un), ou **👁 Solution**.
- Réussis un niveau pour gagner de l'XP et **débloquer le suivant**.

Ta progression est sauvegardée dans `progress.json`. Le bouton **⟳** en haut à
droite remet tout à zéro.

## Le parcours - 100 niveaux, du débutant à l'avancé

Les niveaux se débloquent un à un et montent en difficulté. Grandes étapes :

- **Bases (1–6)** : `println!`, variables & `mut`, types, fonctions, `if`,
  boucles.
- **Ownership (7–9)** : propriété, références & emprunt (`&`/`&mut`), structs.
- **Modélisation (10–17)** : enums & `match`, `Option`, `Vec`, tuples, tableaux,
  `String`/`&str`, `HashMap`, `Result`.
- **Erreurs & génériques (18–25)** : opérateur `?`, paniques, génériques
  (fonctions & structs), traits, méthodes par défaut, *trait bounds*, `derive`.
- **Itérateurs & combinateurs (26–38)** : closures, `map`/`filter`/`collect`,
  `sum`/`fold`, `enumerate`/`zip`, combinateurs `Option`/`Result`, `? `avec
  `Option`, filtrage avancé, `if let`/`while let`, `let else`, modules,
  `From`/`Into`.
- **Traits & pointeurs (39–50)** : `Display`, `PartialEq`, tri, `HashSet`,
  `BTreeMap`, `Box`, `Rc`, `RefCell`, objets-traits `dyn`, durées de vie,
  itérateur personnalisé.
- **Concurrence & avancé (51–62)** : threads, `Arc<Mutex>`, channels, `parse`,
  newtype, `impl Trait`, `Default`, surcharge d'opérateurs, récursivité, slices,
  `collect` vers `Result`.
- **Itérateurs experts & texte (63–75)** : `find`, `all`/`any`, `filter_map`,
  `flat_map`, `chain`/`rev`, `take_while`, `windows`/`chunks`, `split`/`join`,
  `chars`, arithmétique sûre, `match` sur tuples, closures `FnMut`.
- **Traits & outils avancés (76–87)** : erreurs personnalisées, `Box<dyn Error>`,
  `Drop`, `Deref`, `BinaryHeap`, `VecDeque`, tri sur mesure, `macro_rules!`,
  `impl Trait` en argument, supertraits, constantes associées, tests.
- **Projets (88–100)** : une **calculatrice RPN** (tokeniser → évaluer), le
  **Jeu de la Vie de Conway** en ASCII, puis des défis-finales (FizzBuzz,
  chiffre de César, palindrome, histogramme, et une pyramide de célébration 🏆).

Disponible intégralement en **français** et en **anglais**.

## Ajouter un niveau

Tout le contenu est dans [`src/content.rs`](src/content.rs). Ajoute une entrée
`Level { ... }` à la fin de la liste `LEVELS` :

- `id` : numéro suivant.
- `title`, `subtitle`, `lesson`, `task`, `hints` : bilingues via `Bi { fr, en }`.
- `lesson` est du **Markdown** (rendu automatiquement en HTML).
- `starter` : le code de départ affiché dans l'éditeur.
- `check` : comment vérifier la réussite -
  - `Check::Stdout { expected: "..." }` : on compare la sortie du programme.
  - `Check::Harness { harness: "..." }` : on colle du code de test (avec un
    `fn main` qui fait des `assert!` puis imprime `SUCCESS_MARKER`) à la suite
    du code de l'apprenant.

Pas besoin de toucher au reste : relance `cargo run` et le niveau apparaît.

## Architecture

| Fichier | Rôle |
|---|---|
| `src/main.rs` | Serveur web **axum** + routes API |
| `src/auth.rs` | Authentification optionnelle (page de connexion + session) |
| `src/content.rs` | Le curriculum (données + leçons bilingues) |
| `src/runner.rs` | Compile/exécute le code soumis (timeout + limites + Clippy) |
| `src/progress.rs` | Sauvegarde de la progression (XP, niveaux réussis) |
| `web/` | Interface (HTML/CSS/JS, sans dépendance externe) |

---

## Déploiement (Docker) - accessible depuis n'importe où 🔒

Le projet peut tourner **soit** en local avec `cargo run`, **soit** dans Docker
derrière un reverse proxy HTTPS, protégé par un compte.

### 1. Configurer les secrets

```bash
cp .env.example .env
```

Édite `.env` :

- `RUSTQUEST_AUTH_USER` / `RUSTQUEST_AUTH_PASSWORD` : ton compte d'accès. **Mets
  un mot de passe long et aléatoire** (`openssl rand -base64 24`).
- `COMPOSE_PROFILES` : **`caddy`** pour utiliser le reverse proxy fourni, ou
  **vide** si tu as déjà ton propre reverse proxy (voir mode B).
- `RUSTQUEST_DOMAIN` / `ACME_EMAIL` : ton domaine et ton e-mail (mode A, Caddy).
- `RUSTQUEST_PORT` : le port publié côté hôte (mode B, ton proxy).

> Le fichier `.env` n'est **jamais** committé (il est dans `.gitignore`).

### 2. Lancer

```bash
docker compose up -d --build
```

#### Mode A - avec le Caddy fourni (`COMPOSE_PROFILES=caddy`)

- `caddy` expose les ports **80/443** et obtient/renouvelle le **certificat
  HTTPS** tout seul (si `RUSTQUEST_DOMAIN` est un vrai domaine pointant vers le
  serveur).
- `rustquest` n'est **pas** exposé à Internet : Caddy lui parle par le réseau
  interne.

Va sur `https://ton-domaine` → une **page de connexion** s'affiche ; saisis ton
**identifiant + mot de passe**. C'est tout.

#### Mode B - tu as déjà un reverse proxy (`COMPOSE_PROFILES=` vide)

Le service Caddy est **ignoré**. L'app est publiée sur **`127.0.0.1:RUSTQUEST_PORT`**
(par défaut `127.0.0.1:3000`) - donc pas exposée directement à Internet. Fais
pointer ton reverse proxy (nginx, Traefik, Apache…) vers cette adresse :

```nginx
# Exemple nginx
location / {
    proxy_pass http://127.0.0.1:3000;
}
```

> Si ton reverse proxy tourne **lui-même dans un conteneur**, fais plutôt
> rejoindre `rustquest` à son réseau Docker (au lieu du port hôte).

### Lancer à la main (sans Docker)

Toujours possible. L'auth s'active dès que les variables sont présentes :

```bash
# Sans auth (dev local)
cargo run

# Avec auth
RUSTQUEST_AUTH_USER=valentin RUSTQUEST_AUTH_PASSWORD='motDePasseCostaud' \
RUSTQUEST_BIND=0.0.0.0:3000 cargo run
```

## Sécurité

Le serveur **compile et exécute du code arbitraire** : c'est l'objet même de
l'outil. Voici les garde-fous mis en place.

**Accès**
- Authentification par **page de connexion** + **session par cookie**
  (jeton aléatoire 256 bits, cookie `HttpOnly` + `SameSite=Strict`, comparaison
  des identifiants à temps constant) dès que `RUSTQUEST_AUTH_*` sont définis.
- **HTTPS** via ton reverse proxy (le mot de passe ne circule jamais en clair).
  N'expose l'app **que** derrière HTTPS. Pose `RUSTQUEST_SECURE_COOKIE=true`
  pour marquer le cookie `Secure` quand tu es bien en HTTPS.

**Exécution du code (sandbox)**
- **Timeouts** : compilation (20 s) et exécution (5 s) interrompues si trop
  longues.
- **Limites de ressources** (Linux, `setrlimit`) appliquées au programme de
  l'apprenant : temps CPU (10 s), mémoire (512 Mo), taille des fichiers écrits.
- **Conteneur durci** : utilisateur **non-root**, `no-new-privileges`,
  `cap_drop: ALL`, `pids_limit` (anti fork-bomb), `mem_limit`, `cpus`, et
  compilation/exécution dans un **tmpfs** `/tmp` de taille limitée.
- **Correctifs système** : l'image applique `apt-get upgrade` à la construction
  pour patcher les CVE Debian connues. (L'avertissement éventuel d'un scanner
  sur la ligne `FROM` porte sur l'image de base *publiée*, pas sur l'image
  finale patchée.) Option `read_only: true` disponible (commentée) dans le
  compose pour une racine en lecture seule.

> 🔎 Pour scanner l'image construite : `docker scout cves rustquest:latest`
> (ou `trivy image rustquest:latest`). Rebuilds réguliers = derniers correctifs.

**Limite connue** - le code exécuté peut encore tenter des accès réseau sortants
(le conteneur a besoin du réseau pour servir l'app). Pour aller plus loin :
restreindre le réseau sortant au niveau du pare-feu de l'hôte, ou isoler
l'exécution dans un sandbox dédié (gVisor, nsjail, microVM…). Pour un usage
**personnel et authentifié**, les protections ci-dessus suffisent largement.
