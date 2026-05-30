//! Curriculum de RustQuest.
//!
//! Tout le contenu pédagogique vit ici, sous forme de données statiques.
//! Pour ajouter un niveau, il suffit d'ajouter une entrée dans `LEVELS`.
//!
//! The whole curriculum lives here as static data. Add a level by adding an
//! entry to `LEVELS`.

use serde::Serialize;

/// Marqueur imprimé par un harnais de test quand tous les `assert!` passent.
/// Le moteur d'exécution cherche cette chaîne dans la sortie standard.
pub const SUCCESS_MARKER: &str = "__RUSTQUEST_OK__";

/// Une chaîne disponible en français et en anglais.
#[derive(Clone, Copy)]
pub struct Bi {
    pub fr: &'static str,
    pub en: &'static str,
}

impl Bi {
    /// Renvoie la version dans la langue demandée ("fr" ou autre = anglais).
    pub fn pick(&self, lang: &str) -> &'static str {
        if lang == "fr" {
            self.fr
        } else {
            self.en
        }
    }
}

/// Comment vérifier la réussite d'un exercice.
pub enum Check {
    /// On exécute le programme complet de l'apprenant et on compare la sortie
    /// standard (espaces de début/fin ignorés) à `expected`.
    Stdout { expected: &'static str },
    /// On colle `harness` (qui contient un `fn main`) à la suite du code de
    /// l'apprenant, on compile et on exécute. Réussite si le programme se
    /// termine bien et imprime `SUCCESS_MARKER`.
    /// (Prévu pour de futurs niveaux à base de tests automatiques.)
    #[allow(dead_code)]
    Harness { harness: &'static str },
}

pub struct Level {
    pub id: u32,
    pub slug: &'static str,
    pub title: Bi,
    pub subtitle: Bi,
    pub xp: u32,
    /// Leçon au format Markdown.
    pub lesson: Bi,
    /// Consigne courte de l'exercice.
    pub task: Bi,
    /// Code de départ affiché dans l'éditeur.
    pub starter: &'static str,
    pub check: Check,
    pub hints: &'static [Bi],
    pub solution: &'static str,
}

// ===========================================================================
// Vue sérialisable d'un niveau, résolue dans une langue donnée.
// ===========================================================================

#[derive(Serialize)]
pub struct LevelSummary {
    pub id: u32,
    pub slug: &'static str,
    pub title: &'static str,
    pub subtitle: &'static str,
    pub xp: u32,
}

#[derive(Serialize)]
pub struct LevelDetail {
    pub id: u32,
    pub slug: &'static str,
    pub title: &'static str,
    pub subtitle: &'static str,
    pub xp: u32,
    /// Leçon déjà convertie de Markdown vers HTML.
    pub lesson_html: String,
    pub task: &'static str,
    pub starter: &'static str,
    pub hints: Vec<&'static str>,
    pub solution: &'static str,
}

impl Level {
    pub fn summary(&self, lang: &str) -> LevelSummary {
        LevelSummary {
            id: self.id,
            slug: self.slug,
            title: self.title.pick(lang),
            subtitle: self.subtitle.pick(lang),
            xp: self.xp,
        }
    }

    pub fn detail(&self, lang: &str) -> LevelDetail {
        LevelDetail {
            id: self.id,
            slug: self.slug,
            title: self.title.pick(lang),
            subtitle: self.subtitle.pick(lang),
            xp: self.xp,
            lesson_html: md_to_html(self.lesson.pick(lang)),
            task: self.task.pick(lang),
            starter: self.starter,
            hints: self.hints.iter().map(|h| h.pick(lang)).collect(),
            solution: self.solution,
        }
    }
}

fn md_to_html(md: &str) -> String {
    use pulldown_cmark::{html, Options, Parser};
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(md, options);
    let mut out = String::new();
    html::push_html(&mut out, parser);
    out
}

/// Renvoie le niveau d'identifiant `id`, s'il existe.
pub fn level_by_id(id: u32) -> Option<&'static Level> {
    LEVELS.iter().find(|l| l.id == id)
}

/// XP total atteignable, pour calculer la progression globale.
pub fn total_xp() -> u32 {
    LEVELS.iter().map(|l| l.xp).sum()
}

// ===========================================================================
// LE CURRICULUM
// ===========================================================================

pub static LEVELS: &[Level] = &[
    // -------------------------------------------------------------------
    Level {
        id: 1,
        slug: "hello",
        title: Bi { fr: "Bonjour, Rust !", en: "Hello, Rust!" },
        subtitle: Bi { fr: "Ton tout premier programme", en: "Your very first program" },
        xp: 50,
        lesson: Bi {
            fr: r#"
## Bienvenue dans RustQuest 🦀

Rust est un langage qui vise **trois choses à la fois** : la rapidité, la
sûreté de la mémoire, et la productivité. On va commencer par le rituel de
tout apprenti programmeur : afficher du texte à l'écran.

### Le programme minimal

Tout programme Rust exécutable commence par une **fonction** nommée `main`.
C'est le point d'entrée : quand tu lances le programme, Rust appelle `main`.

```rust
fn main() {
    println!("Bonjour, Rust !");
}
```

Décortiquons :

- `fn` introduit une **fonction**.
- `main` est son nom (spécial : c'est le point de départ).
- `()` : la liste des paramètres, ici vide.
- `{ ... }` : le **corps** de la fonction, entre accolades.
- `println!` affiche une ligne de texte. Le `!` indique que c'est une
  **macro**, pas une fonction classique (on y reviendra, retiens juste le `!`).
- Le texte est entre guillemets `"..."` : c'est une **chaîne de caractères**.
- Chaque instruction se termine par un point-virgule `;`.

> 💡 En Rust, l'indentation standard est de **4 espaces**, et on termine les
> instructions par `;`.
"#,
            en: r#"
## Welcome to RustQuest 🦀

Rust aims for **three things at once**: speed, memory safety, and
productivity. Let's start with every programmer's ritual: printing text.

### The minimal program

Every runnable Rust program starts with a **function** called `main`. It is
the entry point: when you run the program, Rust calls `main`.

```rust
fn main() {
    println!("Hello, Rust!");
}
```

Breaking it down:

- `fn` introduces a **function**.
- `main` is its name (special: the starting point).
- `()` is the parameter list, empty here.
- `{ ... }` is the function **body**, inside curly braces.
- `println!` prints a line of text. The `!` means it's a **macro**, not a
  regular function (more on that later — just remember the `!`).
- The text is inside quotes `"..."`: a **string**.
- Every statement ends with a semicolon `;`.

> 💡 In Rust the standard indentation is **4 spaces**, and statements end
> with `;`.
"#,
        },
        task: Bi {
            fr: "Complète le programme pour qu'il affiche exactement : Bonjour, Rust !",
            en: "Complete the program so it prints exactly: Hello, Rust!",
        },
        starter: r#"fn main() {
    // Écris ici un println! qui affiche : Bonjour, Rust !
    // Write a println! here that prints: Hello, Rust!
}
"#,
        check: Check::Stdout { expected: "Bonjour, Rust !" },
        hints: &[
            Bi {
                fr: "Utilise la macro println! avec le texte entre guillemets doubles.",
                en: "Use the println! macro with the text in double quotes.",
            },
            Bi {
                fr: "N'oublie pas le point-virgule à la fin : println!(\"Bonjour, Rust !\");",
                en: "Don't forget the semicolon: println!(\"Bonjour, Rust !\");",
            },
        ],
        solution: r#"fn main() {
    println!("Bonjour, Rust !");
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 2,
        slug: "variables",
        title: Bi { fr: "Variables & mutabilité", en: "Variables & mutability" },
        subtitle: Bi { fr: "let, mut et l'immuabilité par défaut", en: "let, mut and default immutability" },
        xp: 60,
        lesson: Bi {
            fr: r#"
## Les variables

On déclare une variable avec le mot-clé `let` :

```rust
let x = 5;
println!("x vaut {x}");
```

Le `{x}` à l'intérieur du texte est un **emplacement** : `println!` y insère
la valeur de `x`. (On peut aussi écrire `println!("x vaut {}", x);`.)

### Immuable par défaut

Surprise venant d'autres langages : en Rust une variable est **immuable** par
défaut. Ce code **ne compile pas** :

```rust
let x = 5;
x = 6; // ❌ erreur : cannot assign twice to immutable variable `x`
```

C'est une **fonctionnalité**, pas une contrainte gratuite : ça t'évite de
modifier par erreur une valeur que tu croyais fixe.

### `mut` : rendre modifiable

Pour pouvoir changer la valeur, ajoute `mut` (pour *mutable*) :

```rust
let mut x = 5;
x = 6; // ✅ ok
```

### Le *shadowing*

On peut aussi **redéclarer** une variable avec le même nom. La nouvelle
« masque » l'ancienne (et peut même changer de type) :

```rust
let x = 5;
let x = x + 1; // x vaut maintenant 6
let x = "coucou"; // et là c'est devenu une chaîne !
```

> 💡 `mut` modifie **la même** variable. Le shadowing en crée **une nouvelle**.
"#,
            en: r#"
## Variables

You declare a variable with the `let` keyword:

```rust
let x = 5;
println!("x is {x}");
```

The `{x}` inside the text is a **placeholder**: `println!` inserts the value
of `x`. (You can also write `println!("x is {}", x);`.)

### Immutable by default

A surprise coming from other languages: in Rust a variable is **immutable**
by default. This code **does not compile**:

```rust
let x = 5;
x = 6; // ❌ error: cannot assign twice to immutable variable `x`
```

It's a **feature**, not a pointless restriction: it stops you from
accidentally changing a value you thought was fixed.

### `mut`: make it mutable

To allow changing the value, add `mut`:

```rust
let mut x = 5;
x = 6; // ✅ ok
```

### Shadowing

You can also **redeclare** a variable with the same name. The new one
"shadows" the old one (and may even change type):

```rust
let x = 5;
let x = x + 1; // x is now 6
let x = "hi"; // now it's a string!
```

> 💡 `mut` changes **the same** variable. Shadowing creates **a new** one.
"#,
        },
        task: Bi {
            fr: "Le code ne compile pas car on modifie une variable immuable. Corrige la déclaration pour que le compteur puisse passer de 0 à 10.",
            en: "The code does not compile because it modifies an immutable variable. Fix the declaration so the counter can go from 0 to 10.",
        },
        starter: r#"fn main() {
    let compteur = 0;
    compteur = 10; // on veut modifier la valeur
    println!("compteur = {compteur}");
}
"#,
        check: Check::Stdout { expected: "compteur = 10" },
        hints: &[
            Bi {
                fr: "Une variable est immuable par défaut. Quel mot-clé la rend modifiable ?",
                en: "A variable is immutable by default. Which keyword makes it mutable?",
            },
            Bi {
                fr: "Remplace `let compteur` par `let mut compteur`.",
                en: "Replace `let compteur` with `let mut compteur`.",
            },
        ],
        solution: r#"fn main() {
    let mut compteur = 0;
    compteur = 10;
    println!("compteur = {compteur}");
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 3,
        slug: "types",
        title: Bi { fr: "Les types de base", en: "Basic types" },
        subtitle: Bi { fr: "Entiers, flottants, booléens, caractères", en: "Integers, floats, booleans, chars" },
        xp: 60,
        lesson: Bi {
            fr: r#"
## Les types scalaires

Rust est **typé statiquement** : chaque valeur a un type connu à la
compilation. Souvent, le compilateur **devine** le type tout seul
(*inférence*), mais on peut aussi l'écrire explicitement avec `: Type`.

### Les entiers

`i32` (entier signé sur 32 bits) est le type par défaut. Il existe aussi
`i8, i16, i64, i128` et leurs versions non signées `u8, u16, u32, u64`…

```rust
let age: u32 = 30;
let temperature: i32 = -5;
```

### Les flottants

Pour les nombres à virgule : `f64` (par défaut) ou `f32`.

```rust
let pi: f64 = 3.14159;
```

### Les booléens

Le type `bool` vaut `true` ou `false`.

```rust
let majeur: bool = age >= 18;
```

### Les caractères

Le type `char` représente **un** caractère Unicode, entre **apostrophes
simples** `'a'` (≠ des chaînes `"..."` qui utilisent des guillemets doubles).

```rust
let initiale: char = 'V';
let emoji: char = '🦀';
```

### Opérations

```rust
let somme = 5 + 7;        // 12
let produit = 4 * 3;      // 12
let reste = 10 % 3;       // 1  (modulo)
let division = 7.0 / 2.0; // 3.5
```

> ⚠️ On ne peut pas mélanger les types : `5 + 2.0` ne compile pas. Il faut
> des opérandes du **même** type.
"#,
            en: r#"
## Scalar types

Rust is **statically typed**: every value has a type known at compile time.
Often the compiler **infers** the type for you, but you can also write it
explicitly with `: Type`.

### Integers

`i32` (signed 32-bit integer) is the default. There are also
`i8, i16, i64, i128` and unsigned versions `u8, u16, u32, u64`…

```rust
let age: u32 = 30;
let temperature: i32 = -5;
```

### Floats

For decimal numbers: `f64` (default) or `f32`.

```rust
let pi: f64 = 3.14159;
```

### Booleans

The `bool` type is `true` or `false`.

```rust
let adult: bool = age >= 18;
```

### Characters

The `char` type is **one** Unicode character, in **single quotes** `'a'`
(unlike strings `"..."` which use double quotes).

```rust
let initial: char = 'V';
let emoji: char = '🦀';
```

### Operations

```rust
let sum = 5 + 7;          // 12
let product = 4 * 3;      // 12
let remainder = 10 % 3;   // 1  (modulo)
let division = 7.0 / 2.0; // 3.5
```

> ⚠️ You cannot mix types: `5 + 2.0` does not compile. Operands must be the
> **same** type.
"#,
        },
        task: Bi {
            fr: "Déclare les variables demandées avec les bons types et calcule l'aire d'un rectangle (largeur × hauteur). Le programme doit afficher : Aire = 15",
            en: "Declare the requested variables with the right types and compute a rectangle's area (width × height). The program must print: Aire = 15",
        },
        starter: r#"fn main() {
    let largeur: u32 = 3;
    // Déclare `hauteur` (u32) qui vaut 5
    // Déclare `aire` = largeur * hauteur

    // Décommente quand c'est prêt :
    // println!("Aire = {aire}");
}
"#,
        check: Check::Stdout { expected: "Aire = 15" },
        hints: &[
            Bi {
                fr: "let hauteur: u32 = 5; puis let aire = largeur * hauteur;",
                en: "let hauteur: u32 = 5; then let aire = largeur * hauteur;",
            },
        ],
        solution: r#"fn main() {
    let largeur: u32 = 3;
    let hauteur: u32 = 5;
    let aire = largeur * hauteur;
    println!("Aire = {aire}");
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 4,
        slug: "functions",
        title: Bi { fr: "Les fonctions", en: "Functions" },
        subtitle: Bi { fr: "Paramètres, valeurs de retour, expressions", en: "Parameters, return values, expressions" },
        xp: 70,
        lesson: Bi {
            fr: r#"
## Écrire ses propres fonctions

Au-delà de `main`, tu peux définir tes propres fonctions pour organiser ton
code.

```rust
fn saluer(nom: &str) {
    println!("Salut, {nom} !");
}

fn main() {
    saluer("Valentin");
}
```

### Les paramètres ont un type obligatoire

Contrairement aux variables, le type des **paramètres** doit toujours être
écrit : `nom: &str` (une chaîne empruntée — on verra `&` plus tard).

### Renvoyer une valeur

On annonce le type de retour après une flèche `->` :

```rust
fn carre(x: i32) -> i32 {
    x * x
}
```

⚠️ Remarque : **pas de `return`, pas de `;`** sur la dernière ligne ! En Rust,
un bloc renvoie la valeur de sa **dernière expression**. C'est un point
central :

- `x * x` (sans `;`) est une **expression** : sa valeur est renvoyée.
- `x * x;` (avec `;`) est une **instruction** : elle ne renvoie rien (`()`).

On *peut* utiliser `return` pour sortir plus tôt, mais le style idiomatique
est l'expression finale sans `;` :

```rust
fn maximum(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}
```

> 💡 Règle d'or : `;` transforme une expression en instruction et **jette** sa
> valeur.
"#,
            en: r#"
## Writing your own functions

Beyond `main`, you can define your own functions to organize your code.

```rust
fn greet(name: &str) {
    println!("Hi, {name}!");
}

fn main() {
    greet("Valentin");
}
```

### Parameters require a type

Unlike variables, **parameter** types must always be written: `name: &str`
(a borrowed string — more on `&` later).

### Returning a value

You declare the return type after an arrow `->`:

```rust
fn square(x: i32) -> i32 {
    x * x
}
```

⚠️ Note: **no `return`, no `;`** on the last line! In Rust a block returns
the value of its **last expression**. This is central:

- `x * x` (no `;`) is an **expression**: its value is returned.
- `x * x;` (with `;`) is a **statement**: it returns nothing (`()`).

You *can* use `return` to exit early, but the idiomatic style is the trailing
expression without `;`:

```rust
fn maximum(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}
```

> 💡 Golden rule: `;` turns an expression into a statement and **throws away**
> its value.
"#,
        },
        task: Bi {
            fr: "Complète la fonction `double` pour qu'elle renvoie le double de son argument. `main` est déjà écrit et doit afficher : double(21) = 42",
            en: "Complete the `double` function so it returns twice its argument. `main` is already written and must print: double(21) = 42",
        },
        starter: r#"fn double(n: i32) -> i32 {
    // Renvoie n multiplié par 2 (sans return, sans point-virgule)
}

fn main() {
    let r = double(21);
    println!("double(21) = {r}");
}
"#,
        check: Check::Stdout { expected: "double(21) = 42" },
        hints: &[
            Bi {
                fr: "La dernière expression d'un bloc est sa valeur de retour. Écris simplement `n * 2` sans point-virgule.",
                en: "A block's last expression is its return value. Just write `n * 2` with no semicolon.",
            },
        ],
        solution: r#"fn double(n: i32) -> i32 {
    n * 2
}

fn main() {
    let r = double(21);
    println!("double(21) = {r}");
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 5,
        slug: "if",
        title: Bi { fr: "Conditions", en: "Conditions" },
        subtitle: Bi { fr: "if / else if / else, et if comme expression", en: "if / else if / else, and if as an expression" },
        xp: 70,
        lesson: Bi {
            fr: r#"
## Prendre des décisions avec `if`

```rust
let n = 7;
if n % 2 == 0 {
    println!("pair");
} else {
    println!("impair");
}
```

Points importants :

- La condition doit être un **`bool`**. Pas de « 0 = faux » comme en C : un
  entier n'est **pas** une condition valide.
- **Pas de parenthèses** obligatoires autour de la condition.
- Les **accolades sont obligatoires**, même pour une seule ligne.

### Plusieurs branches

```rust
let note = 75;
if note >= 90 {
    println!("Excellent");
} else if note >= 50 {
    println!("Réussi");
} else {
    println!("À retravailler");
}
```

### `if` est une **expression**

Comme presque tout en Rust, `if` produit une valeur. On peut donc l'utiliser à
droite d'un `let` :

```rust
let abonnement = "premium";
let prix = if abonnement == "premium" { 10 } else { 0 };
```

⚠️ Les deux branches doivent renvoyer le **même type**, et il faut un `else`.

> 💡 C'est l'équivalent du « ternaire » `cond ? a : b` d'autres langages, mais
> en plus lisible.
"#,
            en: r#"
## Making decisions with `if`

```rust
let n = 7;
if n % 2 == 0 {
    println!("even");
} else {
    println!("odd");
}
```

Key points:

- The condition must be a **`bool`**. No "0 means false" like in C: an
  integer is **not** a valid condition.
- **No parentheses** required around the condition.
- **Braces are mandatory**, even for a single line.

### Multiple branches

```rust
let grade = 75;
if grade >= 90 {
    println!("Excellent");
} else if grade >= 50 {
    println!("Pass");
} else {
    println!("Retry");
}
```

### `if` is an **expression**

Like almost everything in Rust, `if` produces a value. So you can use it on
the right of a `let`:

```rust
let plan = "premium";
let price = if plan == "premium" { 10 } else { 0 };
```

⚠️ Both branches must return the **same type**, and you need an `else`.

> 💡 It's the equivalent of the `cond ? a : b` ternary from other languages,
> but more readable.
"#,
        },
        task: Bi {
            fr: "Complète la fonction `categorie` : elle renvoie \"enfant\" si age < 13, \"ado\" si age < 18, sinon \"adulte\". Le programme teste age = 16 et doit afficher : ado",
            en: "Complete the `categorie` function: it returns \"enfant\" if age < 13, \"ado\" if age < 18, otherwise \"adulte\". The program tests age = 16 and must print: ado",
        },
        starter: r#"fn categorie(age: u32) -> &'static str {
    // Renvoie "enfant", "ado" ou "adulte" selon l'âge
}

fn main() {
    println!("{}", categorie(16));
}
"#,
        check: Check::Stdout { expected: "ado" },
        hints: &[
            Bi {
                fr: "Enchaîne if / else if / else. Chaque branche est une expression dont la valeur est une chaîne.",
                en: "Chain if / else if / else. Each branch is an expression whose value is a string.",
            },
            Bi {
                fr: "if age < 13 { \"enfant\" } else if age < 18 { \"ado\" } else { \"adulte\" }",
                en: "if age < 13 { \"enfant\" } else if age < 18 { \"ado\" } else { \"adulte\" }",
            },
        ],
        solution: r#"fn categorie(age: u32) -> &'static str {
    if age < 13 {
        "enfant"
    } else if age < 18 {
        "ado"
    } else {
        "adulte"
    }
}

fn main() {
    println!("{}", categorie(16));
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 6,
        slug: "loops",
        title: Bi { fr: "Les boucles", en: "Loops" },
        subtitle: Bi { fr: "loop, while et for", en: "loop, while and for" },
        xp: 80,
        lesson: Bi {
            fr: r#"
## Répéter des actions

Rust propose trois boucles.

### `loop` — boucle infinie (qu'on arrête avec `break`)

```rust
let mut n = 0;
loop {
    n += 1;
    if n == 3 {
        break;
    }
}
```

Astuce : `loop` peut **renvoyer une valeur** via `break valeur` :

```rust
let resultat = loop {
    break 42;
};
```

### `while` — tant que la condition est vraie

```rust
let mut n = 3;
while n > 0 {
    println!("{n}");
    n -= 1;
}
```

### `for` — parcourir une suite de valeurs

C'est la boucle la plus utilisée. On parcourt un **intervalle** ou une
collection :

```rust
for i in 1..=5 {
    println!("{i}"); // 1, 2, 3, 4, 5
}
```

- `1..5` : intervalle **exclusif** (1, 2, 3, 4).
- `1..=5` : intervalle **inclusif** (1, 2, 3, 4, 5).

```rust
let fruits = ["pomme", "poire", "kiwi"];
for fruit in fruits {
    println!("{fruit}");
}
```

> 💡 Le `for` de Rust est sûr : impossible de sortir des bornes d'un tableau,
> contrairement à un index manuel.
"#,
            en: r#"
## Repeating actions

Rust offers three loops.

### `loop` — infinite loop (stopped with `break`)

```rust
let mut n = 0;
loop {
    n += 1;
    if n == 3 {
        break;
    }
}
```

Tip: `loop` can **return a value** via `break value`:

```rust
let result = loop {
    break 42;
};
```

### `while` — as long as the condition holds

```rust
let mut n = 3;
while n > 0 {
    println!("{n}");
    n -= 1;
}
```

### `for` — iterate over a sequence

The most used loop. You iterate over a **range** or a collection:

```rust
for i in 1..=5 {
    println!("{i}"); // 1, 2, 3, 4, 5
}
```

- `1..5`: **exclusive** range (1, 2, 3, 4).
- `1..=5`: **inclusive** range (1, 2, 3, 4, 5).

```rust
let fruits = ["apple", "pear", "kiwi"];
for fruit in fruits {
    println!("{fruit}");
}
```

> 💡 Rust's `for` is safe: you cannot go out of an array's bounds, unlike
> manual indexing.
"#,
        },
        task: Bi {
            fr: "Complète la fonction `somme_jusqu_a` qui additionne tous les entiers de 1 à n inclus. Le programme teste n = 100 et doit afficher : 5050",
            en: "Complete the `somme_jusqu_a` function that sums all integers from 1 to n inclusive. The program tests n = 100 and must print: 5050",
        },
        starter: r#"fn somme_jusqu_a(n: u32) -> u32 {
    let mut total = 0;
    // Utilise une boucle for sur l'intervalle inclusif 1..=n
    total
}

fn main() {
    println!("{}", somme_jusqu_a(100));
}
"#,
        check: Check::Stdout { expected: "5050" },
        hints: &[
            Bi {
                fr: "for i in 1..=n { total += i; }",
                en: "for i in 1..=n { total += i; }",
            },
            Bi {
                fr: "Pense à l'intervalle INCLUSIF avec `..=` pour bien compter n.",
                en: "Use the INCLUSIVE range `..=` so n is counted.",
            },
        ],
        solution: r#"fn somme_jusqu_a(n: u32) -> u32 {
    let mut total = 0;
    for i in 1..=n {
        total += i;
    }
    total
}

fn main() {
    println!("{}", somme_jusqu_a(100));
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 7,
        slug: "ownership",
        title: Bi { fr: "La propriété (ownership)", en: "Ownership" },
        subtitle: Bi { fr: "Le concept signature de Rust", en: "Rust's signature concept" },
        xp: 100,
        lesson: Bi {
            fr: r#"
## Le cœur de Rust : l'*ownership*

C'est **l'idée la plus importante** du langage, et celle qui surprend le plus
au début. Elle permet à Rust de garantir la sûreté mémoire **sans ramasse-
miettes** (garbage collector).

### Trois règles

1. Chaque valeur a **un propriétaire** (une variable).
2. Il ne peut y avoir **qu'un seul** propriétaire à la fois.
3. Quand le propriétaire sort de portée (`scope`), la valeur est **libérée**.

### Le *move* (déplacement)

Pour les types stockés sur le tas comme `String`, affecter une variable à une
autre **déplace** la propriété :

```rust
let s1 = String::from("salut");
let s2 = s1;       // la propriété est DÉPLACÉE vers s2
println!("{s1}");  // ❌ erreur : s1 n'est plus valide !
```

Après `let s2 = s1;`, `s1` ne peut plus être utilisé. Ça évite que deux
variables libèrent la même mémoire (un bug classique en C).

### `clone` : copier explicitement

Si tu veux vraiment deux copies indépendantes :

```rust
let s1 = String::from("salut");
let s2 = s1.clone(); // copie complète
println!("{s1} et {s2}"); // ✅ ok
```

### Et les entiers ?

Les types simples (`i32`, `bool`, `char`…) sont **`Copy`** : ils sont copiés,
pas déplacés. Donc ceci marche très bien :

```rust
let a = 5;
let b = a;
println!("{a} et {b}"); // ✅ ok, a est toujours valide
```

> 💡 Règle mentale : « petit et de taille fixe » → copié ; « possède de la
> mémoire sur le tas » (comme `String`) → déplacé.
"#,
            en: r#"
## The heart of Rust: *ownership*

This is the language's **most important idea**, and the most surprising at
first. It lets Rust guarantee memory safety **without a garbage collector**.

### Three rules

1. Each value has **one owner** (a variable).
2. There can be **only one** owner at a time.
3. When the owner goes out of `scope`, the value is **freed**.

### Move

For heap-stored types like `String`, assigning one variable to another
**moves** ownership:

```rust
let s1 = String::from("hi");
let s2 = s1;       // ownership is MOVED to s2
println!("{s1}");  // ❌ error: s1 is no longer valid!
```

After `let s2 = s1;`, `s1` can no longer be used. This prevents two variables
from freeing the same memory (a classic C bug).

### `clone`: copy explicitly

If you really want two independent copies:

```rust
let s1 = String::from("hi");
let s2 = s1.clone(); // full copy
println!("{s1} and {s2}"); // ✅ ok
```

### What about integers?

Simple types (`i32`, `bool`, `char`…) are **`Copy`**: they are copied, not
moved. So this works fine:

```rust
let a = 5;
let b = a;
println!("{a} and {b}"); // ✅ ok, a is still valid
```

> 💡 Mental rule: "small and fixed-size" → copied; "owns heap memory" (like
> `String`) → moved.
"#,
        },
        task: Bi {
            fr: "Ce code ne compile pas : on utilise `message` après l'avoir déplacé dans `copie`. Corrige-le pour que les DEUX variables soient utilisables. Sortie attendue : salut / salut",
            en: "This code does not compile: `message` is used after being moved into `copie`. Fix it so BOTH variables are usable. Expected output: salut / salut",
        },
        starter: r#"fn main() {
    let message = String::from("salut");
    let copie = message; // déplacement (move)
    println!("{message} / {copie}");
}
"#,
        check: Check::Stdout { expected: "salut / salut" },
        hints: &[
            Bi {
                fr: "Pour garder `message` utilisable, il faut une copie indépendante de la String.",
                en: "To keep `message` usable, you need an independent copy of the String.",
            },
            Bi {
                fr: "Utilise la méthode .clone() : let copie = message.clone();",
                en: "Use the .clone() method: let copie = message.clone();",
            },
        ],
        solution: r#"fn main() {
    let message = String::from("salut");
    let copie = message.clone();
    println!("{message} / {copie}");
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 8,
        slug: "borrowing",
        title: Bi { fr: "Références & emprunt", en: "References & borrowing" },
        subtitle: Bi { fr: "Prêter une valeur avec &", en: "Lending a value with &" },
        xp: 100,
        lesson: Bi {
            fr: r#"
## Emprunter au lieu de déplacer

Déplacer une valeur à chaque appel de fonction serait pénible. Les
**références** permettent de **prêter** l'accès à une valeur sans en transférer
la propriété. On parle d'**emprunt** (*borrowing*).

### Référence partagée `&`

```rust
fn longueur(s: &String) -> usize {
    s.len()
} // s sort de portée, mais ne possède rien : rien n'est libéré

fn main() {
    let nom = String::from("Valentin");
    let n = longueur(&nom); // on PRÊTE nom
    println!("{nom} fait {n} caractères"); // nom est toujours valide ✅
}
```

`&nom` crée une référence. La fonction lit la valeur mais n'en devient pas
propriétaire ; après l'appel, `nom` reste utilisable.

### Référence mutable `&mut`

Pour **modifier** la valeur empruntée :

```rust
fn ajoute_point(s: &mut String) {
    s.push('!');
}

fn main() {
    let mut texte = String::from("Bravo");
    ajoute_point(&mut texte);
    println!("{texte}"); // "Bravo!"
}
```

### La règle d'or de l'emprunt

À un instant donné, tu peux avoir **soit** :
- autant de références **partagées** (`&`) que tu veux (lecture seule),
- **soit** une **seule** référence **mutable** (`&mut`),

mais **jamais les deux en même temps**. C'est ce qui élimine, à la compilation,
toute une classe de bugs (data races, modifications concurrentes).

> 💡 Plusieurs lecteurs OU un seul écrivain. Jamais les deux.
"#,
            en: r#"
## Borrowing instead of moving

Moving a value on every function call would be painful. **References** let you
**lend** access to a value without transferring ownership. This is called
**borrowing**.

### Shared reference `&`

```rust
fn length(s: &String) -> usize {
    s.len()
} // s goes out of scope, but owns nothing: nothing is freed

fn main() {
    let name = String::from("Valentin");
    let n = length(&name); // we LEND name
    println!("{name} is {n} chars"); // name is still valid ✅
}
```

`&name` creates a reference. The function reads the value but does not own it;
after the call, `name` is still usable.

### Mutable reference `&mut`

To **modify** the borrowed value:

```rust
fn add_bang(s: &mut String) {
    s.push('!');
}

fn main() {
    let mut text = String::from("Nice");
    add_bang(&mut text);
    println!("{text}"); // "Nice!"
}
```

### The golden rule of borrowing

At any given time you can have **either**:
- as many **shared** references (`&`) as you want (read only),
- **or** a **single** **mutable** reference (`&mut`),

but **never both at once**. This eliminates, at compile time, a whole class of
bugs (data races, concurrent modification).

> 💡 Many readers OR one writer. Never both.
"#,
        },
        task: Bi {
            fr: "Complète la fonction `ajoute_exclamation` pour qu'elle ajoute '!' à la fin de la String empruntée mutablement. Sortie attendue : Bonjour!",
            en: "Complete the `ajoute_exclamation` function so it appends '!' to the mutably borrowed String. Expected output: Bonjour!",
        },
        starter: r#"fn ajoute_exclamation(s: &mut String) {
    // Ajoute le caractère '!' à la fin de s
}

fn main() {
    let mut salutation = String::from("Bonjour");
    ajoute_exclamation(&mut salutation);
    println!("{salutation}");
}
"#,
        check: Check::Stdout { expected: "Bonjour!" },
        hints: &[
            Bi {
                fr: "La méthode push ajoute un seul caractère : s.push('!');",
                en: "The push method appends one character: s.push('!');",
            },
        ],
        solution: r#"fn ajoute_exclamation(s: &mut String) {
    s.push('!');
}

fn main() {
    let mut salutation = String::from("Bonjour");
    ajoute_exclamation(&mut salutation);
    println!("{salutation}");
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 9,
        slug: "structs",
        title: Bi { fr: "Les structures", en: "Structs" },
        subtitle: Bi { fr: "Regrouper des données et leur ajouter des méthodes", en: "Group data and give it methods" },
        xp: 110,
        lesson: Bi {
            fr: r#"
## Créer ses propres types : les `struct`

Une `struct` regroupe plusieurs valeurs liées sous un même nom.

```rust
struct Personne {
    nom: String,
    age: u32,
}
```

### Créer une instance

```rust
let p = Personne {
    nom: String::from("Valentin"),
    age: 30,
};
println!("{} a {} ans", p.nom, p.age);
```

On accède aux champs avec un **point** : `p.nom`.

### Ajouter des méthodes avec `impl`

Un bloc `impl` attache des fonctions au type. Le premier paramètre `&self`
représente l'instance (comme `this` ailleurs) :

```rust
impl Personne {
    // méthode : prend &self
    fn se_presenter(&self) -> String {
        format!("Je suis {} et j'ai {} ans", self.nom, self.age)
    }

    // fonction associée (pas de self) : souvent un constructeur
    fn nouvelle(nom: &str, age: u32) -> Personne {
        Personne { nom: String::from(nom), age }
    }
}
```

- `personne.se_presenter()` → on appelle avec un **point**.
- `Personne::nouvelle("Léa", 25)` → fonction associée, appelée avec `::`.

> 💡 `format!` fonctionne comme `println!` mais **renvoie** la chaîne au lieu
> de l'afficher. Et `age` tout court est un raccourci pour `age: age`.
"#,
            en: r#"
## Defining your own types: `struct`

A `struct` groups several related values under one name.

```rust
struct Personne {
    nom: String,
    age: u32,
}
```

### Creating an instance

```rust
let p = Personne {
    nom: String::from("Valentin"),
    age: 30,
};
println!("{} is {}", p.nom, p.age);
```

You access fields with a **dot**: `p.nom`.

### Adding methods with `impl`

An `impl` block attaches functions to the type. The first parameter `&self` is
the instance (like `this` elsewhere):

```rust
impl Personne {
    // method: takes &self
    fn se_presenter(&self) -> String {
        format!("I'm {} and I'm {}", self.nom, self.age)
    }

    // associated function (no self): often a constructor
    fn nouvelle(nom: &str, age: u32) -> Personne {
        Personne { nom: String::from(nom), age }
    }
}
```

- `personne.se_presenter()` → call with a **dot**.
- `Personne::nouvelle("Léa", 25)` → associated function, called with `::`.

> 💡 `format!` works like `println!` but **returns** the string instead of
> printing it. And bare `age` is shorthand for `age: age`.
"#,
        },
        task: Bi {
            fr: "On a une struct `Rectangle`. Complète la méthode `aire` (largeur × hauteur). Le programme crée un 4×6 et doit afficher : 24",
            en: "We have a `Rectangle` struct. Complete the `aire` method (width × height). The program builds a 4×6 and must print: 24",
        },
        starter: r#"struct Rectangle {
    largeur: u32,
    hauteur: u32,
}

impl Rectangle {
    fn aire(&self) -> u32 {
        // Renvoie largeur * hauteur en utilisant self
    }
}

fn main() {
    let r = Rectangle { largeur: 4, hauteur: 6 };
    println!("{}", r.aire());
}
"#,
        check: Check::Stdout { expected: "24" },
        hints: &[
            Bi {
                fr: "Accède aux champs via self : self.largeur et self.hauteur.",
                en: "Access fields through self: self.largeur and self.hauteur.",
            },
            Bi {
                fr: "self.largeur * self.hauteur (sans point-virgule pour renvoyer la valeur).",
                en: "self.largeur * self.hauteur (no semicolon, to return the value).",
            },
        ],
        solution: r#"struct Rectangle {
    largeur: u32,
    hauteur: u32,
}

impl Rectangle {
    fn aire(&self) -> u32 {
        self.largeur * self.hauteur
    }
}

fn main() {
    let r = Rectangle { largeur: 4, hauteur: 6 };
    println!("{}", r.aire());
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 10,
        slug: "enums",
        title: Bi { fr: "Énumérations & match", en: "Enums & match" },
        subtitle: Bi { fr: "Modéliser des choix avec un filtrage exhaustif", en: "Model choices with exhaustive matching" },
        xp: 120,
        lesson: Bi {
            fr: r#"
## Les énumérations

Une `enum` définit un type qui peut prendre **l'une parmi plusieurs**
variantes.

```rust
enum Direction {
    Nord,
    Sud,
    Est,
    Ouest,
}
```

Une variante peut aussi **transporter des données** :

```rust
enum Message {
    Quitter,
    Deplacer { x: i32, y: i32 },
    Ecrire(String),
}
```

### `match` : le filtrage par motif

`match` compare une valeur à des motifs. Il est **exhaustif** : tu dois traiter
**tous** les cas (le compilateur vérifie !).

```rust
fn nom_fr(d: Direction) -> &'static str {
    match d {
        Direction::Nord => "nord",
        Direction::Sud => "sud",
        Direction::Est => "est",
        Direction::Ouest => "ouest",
    }
}
```

Chaque branche s'écrit `motif => expression`. On peut extraire les données
transportées :

```rust
match msg {
    Message::Ecrire(texte) => println!("Texte : {texte}"),
    Message::Deplacer { x, y } => println!("Va en ({x}, {y})"),
    Message::Quitter => println!("Bye"),
}
```

Le motif `_` capture « tout le reste » :

```rust
match n {
    1 => "un",
    2 => "deux",
    _ => "autre",
}
```

> 💡 L'exhaustivité de `match` est une superpuissance : ajoute une variante à
> ton enum, et le compilateur te montre **tous** les endroits à mettre à jour.
"#,
            en: r#"
## Enumerations

An `enum` defines a type that can be **one of several** variants.

```rust
enum Direction {
    Nord,
    Sud,
    Est,
    Ouest,
}
```

A variant can also **carry data**:

```rust
enum Message {
    Quitter,
    Deplacer { x: i32, y: i32 },
    Ecrire(String),
}
```

### `match`: pattern matching

`match` compares a value against patterns. It is **exhaustive**: you must
handle **all** cases (the compiler checks!).

```rust
fn nom_fr(d: Direction) -> &'static str {
    match d {
        Direction::Nord => "nord",
        Direction::Sud => "sud",
        Direction::Est => "est",
        Direction::Ouest => "ouest",
    }
}
```

Each arm is `pattern => expression`. You can extract carried data:

```rust
match msg {
    Message::Ecrire(text) => println!("Text: {text}"),
    Message::Deplacer { x, y } => println!("Go to ({x}, {y})"),
    Message::Quitter => println!("Bye"),
}
```

The `_` pattern captures "everything else":

```rust
match n {
    1 => "one",
    2 => "two",
    _ => "other",
}
```

> 💡 `match` exhaustiveness is a superpower: add a variant to your enum and the
> compiler shows you **every** place to update.
"#,
        },
        task: Bi {
            fr: "Complète la fonction `feu` qui, selon la variante de `Feu`, renvoie l'action : Rouge → \"stop\", Orange → \"ralentir\", Vert → \"passer\". Le programme teste Feu::Orange et doit afficher : ralentir",
            en: "Complete the `feu` function: depending on the `Feu` variant it returns the action: Rouge → \"stop\", Orange → \"ralentir\", Vert → \"passer\". The program tests Feu::Orange and must print: ralentir",
        },
        starter: r#"enum Feu {
    Rouge,
    Orange,
    Vert,
}

fn feu(f: Feu) -> &'static str {
    // Utilise un match exhaustif sur f
}

fn main() {
    println!("{}", feu(Feu::Orange));
}
"#,
        check: Check::Stdout { expected: "ralentir" },
        hints: &[
            Bi {
                fr: "match f { Feu::Rouge => \"stop\", ... } — traite les trois variantes.",
                en: "match f { Feu::Rouge => \"stop\", ... } — handle all three variants.",
            },
        ],
        solution: r#"enum Feu {
    Rouge,
    Orange,
    Vert,
}

fn feu(f: Feu) -> &'static str {
    match f {
        Feu::Rouge => "stop",
        Feu::Orange => "ralentir",
        Feu::Vert => "passer",
    }
}

fn main() {
    println!("{}", feu(Feu::Orange));
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 11,
        slug: "option",
        title: Bi { fr: "Option : gérer l'absence", en: "Option: handling absence" },
        subtitle: Bi { fr: "Rust n'a pas de null", en: "Rust has no null" },
        xp: 120,
        lesson: Bi {
            fr: r#"
## Le milliardaire d'erreurs : `null`

Beaucoup de langages ont `null`/`nil`, source d'innombrables plantages. Rust
**n'a pas de null**. À la place, l'absence éventuelle d'une valeur est rendue
**explicite** dans le type, avec l'enum `Option<T>` :

```rust
enum Option<T> {
    Some(T), // il y a une valeur
    None,    // il n'y en a pas
}
```

`Option` est dans le prélude : tu peux écrire `Some(...)` et `None`
directement.

```rust
let present: Option<i32> = Some(5);
let absent: Option<i32> = None;
```

### Le compilateur t'oblige à gérer le `None`

Tu ne peux pas utiliser un `Option<i32>` comme un `i32` : il faut d'abord
**ouvrir l'enveloppe**. Avec `match` :

```rust
fn decrire(x: Option<i32>) -> String {
    match x {
        Some(v) => format!("valeur : {v}"),
        None => String::from("rien"),
    }
}
```

### Méthodes pratiques

- `x.unwrap_or(0)` → la valeur, ou `0` si `None`.
- `x.is_some()` / `x.is_none()` → des booléens.
- `if let Some(v) = x { ... }` → raccourci quand on ne veut traiter qu'un cas :

```rust
if let Some(v) = trouver() {
    println!("trouvé : {v}");
}
```

> 💡 `Option` transforme un risque de plantage à l'exécution en une
> vérification **à la compilation**. Le « null pointer » disparaît.
"#,
            en: r#"
## The billion-dollar mistake: `null`

Many languages have `null`/`nil`, the source of countless crashes. Rust **has
no null**. Instead, the possible absence of a value is made **explicit** in the
type, with the `Option<T>` enum:

```rust
enum Option<T> {
    Some(T), // there is a value
    None,    // there is none
}
```

`Option` is in the prelude: you can write `Some(...)` and `None` directly.

```rust
let present: Option<i32> = Some(5);
let absent: Option<i32> = None;
```

### The compiler forces you to handle `None`

You cannot use an `Option<i32>` as an `i32`: you must first **open the
wrapper**. With `match`:

```rust
fn decrire(x: Option<i32>) -> String {
    match x {
        Some(v) => format!("value: {v}"),
        None => String::from("nothing"),
    }
}
```

### Handy methods

- `x.unwrap_or(0)` → the value, or `0` if `None`.
- `x.is_some()` / `x.is_none()` → booleans.
- `if let Some(v) = x { ... }` → shortcut when you only care about one case:

```rust
if let Some(v) = find() {
    println!("found: {v}");
}
```

> 💡 `Option` turns a runtime crash risk into a **compile-time** check. The
> "null pointer" disappears.
"#,
        },
        task: Bi {
            fr: "Complète `premier_pair` : elle renvoie Some du premier nombre pair du tableau, ou None s'il n'y en a aucun. Le programme cherche dans [3, 7, 4, 9] et, via unwrap_or(-1), doit afficher : 4",
            en: "Complete `premier_pair`: it returns Some of the first even number in the slice, or None if there is none. The program searches [3, 7, 4, 9] and, via unwrap_or(-1), must print: 4",
        },
        starter: r#"fn premier_pair(nombres: &[i32]) -> Option<i32> {
    for &n in nombres {
        // Si n est pair, renvoie Some(n)
    }
    None
}

fn main() {
    let r = premier_pair(&[3, 7, 4, 9]);
    println!("{}", r.unwrap_or(-1));
}
"#,
        check: Check::Stdout { expected: "4" },
        hints: &[
            Bi {
                fr: "Un nombre est pair si n % 2 == 0.",
                en: "A number is even if n % 2 == 0.",
            },
            Bi {
                fr: "Dans la boucle : if n % 2 == 0 { return Some(n); }",
                en: "In the loop: if n % 2 == 0 { return Some(n); }",
            },
        ],
        solution: r#"fn premier_pair(nombres: &[i32]) -> Option<i32> {
    for &n in nombres {
        if n % 2 == 0 {
            return Some(n);
        }
    }
    None
}

fn main() {
    let r = premier_pair(&[3, 7, 4, 9]);
    println!("{}", r.unwrap_or(-1));
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 12,
        slug: "vectors",
        title: Bi { fr: "Les vecteurs", en: "Vectors" },
        subtitle: Bi { fr: "Des listes qui grandissent : Vec<T>", en: "Growable lists: Vec<T>" },
        xp: 130,
        lesson: Bi {
            fr: r#"
## `Vec<T>` : une liste dynamique

Un tableau `[1, 2, 3]` a une taille **fixe**. Un **vecteur** `Vec<T>` peut
grandir et rétrécir. C'est la collection la plus courante.

### Créer et remplir

```rust
let mut nombres: Vec<i32> = Vec::new();
nombres.push(10);
nombres.push(20);

// ou directement avec la macro vec!
let mut autres = vec![1, 2, 3];
```

### Parcourir

```rust
for n in &nombres {
    println!("{n}");
}
```

On écrit `&nombres` (emprunt) pour **ne pas** déplacer/consommer le vecteur :
on veut juste le lire.

### Accéder à un élément

```rust
let v = vec![10, 20, 30];
let deuxieme = v[1];          // 20 — panique si hors bornes
let sur = v.get(1);           // Some(20) — renvoie une Option, plus sûr
```

### Méthodes utiles

- `v.len()` → le nombre d'éléments.
- `v.is_empty()` → vrai si vide.
- `v.contains(&20)` → vrai si 20 est présent.
- `v.iter().sum::<i32>()` → la somme des éléments.

```rust
let v = vec![1, 2, 3, 4];
let total: i32 = v.iter().sum();
println!("{total}"); // 10
```

> 💡 `Vec<T>` possède ses éléments. Quand le vecteur est libéré, tous ses
> éléments le sont aussi — automatiquement, grâce à l'ownership.
"#,
            en: r#"
## `Vec<T>`: a dynamic list

An array `[1, 2, 3]` has a **fixed** size. A **vector** `Vec<T>` can grow and
shrink. It's the most common collection.

### Create and fill

```rust
let mut numbers: Vec<i32> = Vec::new();
numbers.push(10);
numbers.push(20);

// or directly with the vec! macro
let mut others = vec![1, 2, 3];
```

### Iterate

```rust
for n in &numbers {
    println!("{n}");
}
```

We write `&numbers` (borrow) so we **don't** move/consume the vector: we just
want to read it.

### Access an element

```rust
let v = vec![10, 20, 30];
let second = v[1];            // 20 — panics if out of bounds
let safe = v.get(1);         // Some(20) — returns an Option, safer
```

### Useful methods

- `v.len()` → number of elements.
- `v.is_empty()` → true if empty.
- `v.contains(&20)` → true if 20 is present.
- `v.iter().sum::<i32>()` → the sum of the elements.

```rust
let v = vec![1, 2, 3, 4];
let total: i32 = v.iter().sum();
println!("{total}"); // 10
```

> 💡 `Vec<T>` owns its elements. When the vector is freed, so are all its
> elements — automatically, thanks to ownership.
"#,
        },
        task: Bi {
            fr: "Complète `moyenne` : elle calcule la moyenne (entière) des éléments d'un vecteur. Le programme teste vec![10, 20, 30, 40] et doit afficher : 25",
            en: "Complete `moyenne`: it computes the (integer) average of a vector's elements. The program tests vec![10, 20, 30, 40] and must print: 25",
        },
        starter: r#"fn moyenne(nombres: &Vec<i32>) -> i32 {
    let mut total = 0;
    // Additionne tous les éléments avec une boucle for
    // Puis divise par le nombre d'éléments (nombres.len() as i32)
    total
}

fn main() {
    let v = vec![10, 20, 30, 40];
    println!("{}", moyenne(&v));
}
"#,
        check: Check::Stdout { expected: "25" },
        hints: &[
            Bi {
                fr: "for n in nombres { total += n; } puis return total / (nombres.len() as i32);",
                en: "for n in nombres { total += n; } then return total / (nombres.len() as i32);",
            },
            Bi {
                fr: "`as i32` convertit le usize renvoyé par len() en i32 pour la division.",
                en: "`as i32` converts the usize from len() into i32 for the division.",
            },
        ],
        solution: r#"fn moyenne(nombres: &Vec<i32>) -> i32 {
    let mut total = 0;
    for n in nombres {
        total += n;
    }
    total / (nombres.len() as i32)
}

fn main() {
    let v = vec![10, 20, 30, 40];
    println!("{}", moyenne(&v));
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 13,
        slug: "tuples",
        title: Bi { fr: "Les tuples", en: "Tuples" },
        subtitle: Bi { fr: "Regrouper des valeurs de types différents", en: "Group values of different types" },
        xp: 130,
        lesson: Bi {
            fr: r#"
## Les tuples

Un **tuple** regroupe un nombre **fixe** de valeurs, possiblement de types
**différents**.

```rust
let point = (3, 7);
let melange = (1, 'a', true);
```

### Accéder aux éléments

Par leur **position**, avec un point suivi de l'index :

```rust
let t = (10, 20);
println!("{}", t.0); // 10
println!("{}", t.1); // 20
```

### Déstructurer

On peut « ouvrir » un tuple dans des variables d'un coup :

```rust
let (x, y) = (3, 7);
println!("{x} et {y}");
```

### Renvoyer plusieurs valeurs

Les tuples sont parfaits pour qu'une fonction renvoie **plusieurs** résultats :

```rust
fn diviser(a: i32, b: i32) -> (i32, i32) {
    (a / b, a % b) // quotient et reste
}
```

> 💡 `{:?}` (format *Debug*) affiche un tuple joliment : `(3, 7)`.
"#,
            en: r#"
## Tuples

A **tuple** groups a **fixed** number of values, possibly of **different**
types.

```rust
let point = (3, 7);
let mixed = (1, 'a', true);
```

### Accessing elements

By their **position**, with a dot followed by the index:

```rust
let t = (10, 20);
println!("{}", t.0); // 10
println!("{}", t.1); // 20
```

### Destructuring

You can "open" a tuple into variables at once:

```rust
let (x, y) = (3, 7);
println!("{x} and {y}");
```

### Returning multiple values

Tuples are perfect for a function returning **several** results:

```rust
fn divide(a: i32, b: i32) -> (i32, i32) {
    (a / b, a % b) // quotient and remainder
}
```

> 💡 `{:?}` (Debug format) prints a tuple nicely: `(3, 7)`.
"#,
        },
        task: Bi {
            fr: "Complète `min_max` : elle renvoie un tuple (le plus petit, le plus grand) des deux nombres. Le programme teste (8, 3) et doit afficher : (3, 8)",
            en: "Complete `min_max`: it returns a tuple (smallest, largest) of the two numbers. The program tests (8, 3) and must print: (3, 8)",
        },
        starter: r#"fn min_max(a: i32, b: i32) -> (i32, i32) {
    // Renvoie un tuple (plus petit, plus grand)
}

fn main() {
    println!("{:?}", min_max(8, 3));
}
"#,
        check: Check::Stdout { expected: "(3, 8)" },
        hints: &[
            Bi { fr: "Compare a et b avec if, et renvoie le tuple dans le bon ordre.", en: "Compare a and b with if, and return the tuple in the right order." },
            Bi { fr: "if a < b { (a, b) } else { (b, a) }", en: "if a < b { (a, b) } else { (b, a) }" },
        ],
        solution: r#"fn min_max(a: i32, b: i32) -> (i32, i32) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}

fn main() {
    println!("{:?}", min_max(8, 3));
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 14,
        slug: "arrays",
        title: Bi { fr: "Les tableaux", en: "Arrays" },
        subtitle: Bi { fr: "Des suites de taille fixe : [T; N]", en: "Fixed-size sequences: [T; N]" },
        xp: 130,
        lesson: Bi {
            fr: r#"
## Les tableaux `[T; N]`

Un **tableau** contient un nombre **fixe** d'éléments du **même** type. Sa
taille fait partie de son type : `[i32; 5]` = 5 entiers.

```rust
let notes = [12, 15, 9, 18, 7];
let zeros = [0; 3]; // [0, 0, 0]
```

- `notes.len()` → 5
- `notes[0]` → premier élément (panique si hors bornes)

### Parcourir

```rust
let mut total = 0;
for n in notes {
    total += n;
}
```

### Tableau vs vecteur

| | Tableau `[T; N]` | Vecteur `Vec<T>` |
|---|---|---|
| Taille | **fixe** | **dynamique** |
| Stockage | sur la pile | sur le tas |

Utilise un tableau quand tu connais la taille à l'avance et qu'elle ne change
pas.

> 💡 On peut prêter un tableau comme une **slice** `&[T]` à une fonction — on
> verra les slices plus tard.
"#,
            en: r#"
## Arrays `[T; N]`

An **array** holds a **fixed** number of elements of the **same** type. Its
size is part of its type: `[i32; 5]` = 5 integers.

```rust
let grades = [12, 15, 9, 18, 7];
let zeros = [0; 3]; // [0, 0, 0]
```

- `grades.len()` → 5
- `grades[0]` → first element (panics if out of bounds)

### Iterating

```rust
let mut total = 0;
for n in grades {
    total += n;
}
```

### Array vs vector

| | Array `[T; N]` | Vector `Vec<T>` |
|---|---|---|
| Size | **fixed** | **dynamic** |
| Storage | on the stack | on the heap |

Use an array when you know the size ahead of time and it won't change.

> 💡 You can lend an array as a **slice** `&[T]` to a function — more on slices
> later.
"#,
        },
        task: Bi {
            fr: "Complète `somme` qui additionne les 5 éléments d'un tableau. Le programme teste [2, 4, 6, 8, 10] et doit afficher : 30",
            en: "Complete `somme` which adds the 5 elements of an array. The program tests [2, 4, 6, 8, 10] and must print: 30",
        },
        starter: r#"fn somme(arr: [i32; 5]) -> i32 {
    let mut total = 0;
    // Parcours le tableau avec un for et additionne
    total
}

fn main() {
    println!("{}", somme([2, 4, 6, 8, 10]));
}
"#,
        check: Check::Stdout { expected: "30" },
        hints: &[
            Bi { fr: "for n in arr { total += n; }", en: "for n in arr { total += n; }" },
        ],
        solution: r#"fn somme(arr: [i32; 5]) -> i32 {
    let mut total = 0;
    for n in arr {
        total += n;
    }
    total
}

fn main() {
    println!("{}", somme([2, 4, 6, 8, 10]));
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 15,
        slug: "strings",
        title: Bi { fr: "String et &str", en: "String and &str" },
        subtitle: Bi { fr: "Les deux faces du texte en Rust", en: "The two sides of text in Rust" },
        xp: 140,
        lesson: Bi {
            fr: r#"
## `String` vs `&str`

Rust a **deux** types de texte, et c'est déroutant au début :

- **`&str`** (« string slice ») : une vue **empruntée**, immuable, sur du
  texte. Les littéraux `"bonjour"` sont des `&str`.
- **`String`** : du texte **possédé**, sur le tas, qui peut **grandir**.

```rust
let emprunte: &str = "salut";
let possede: String = String::from("salut");
let aussi: String = "salut".to_string();
```

### Méthodes utiles (sur les deux)

```rust
let s = "Rust";
s.len();              // 4 (octets)
s.to_uppercase();     // "RUST" (renvoie une String)
s.to_lowercase();     // "rust"
s.contains("us");     // true
s.starts_with("Ru");  // true
s.replace("u", "*");  // "R*st"
```

### Construire une String

```rust
let mut nom = String::from("Val");
nom.push('!');          // ajoute un char
nom.push_str("entin");  // ajoute un &str
```

### Concaténer

```rust
let phrase = format!("{} {}", "bonjour", "le monde");
```

> 💡 Règle pratique : prends `&str` en **paramètre** de fonction (plus
> souple), et renvoie/stocke une `String` quand tu **possèdes** le texte.
"#,
            en: r#"
## `String` vs `&str`

Rust has **two** text types, confusing at first:

- **`&str`** (string slice): a **borrowed**, immutable view of text. Literals
  `"hello"` are `&str`.
- **`String`**: **owned** text, on the heap, that can **grow**.

```rust
let borrowed: &str = "hi";
let owned: String = String::from("hi");
let also: String = "hi".to_string();
```

### Useful methods (on both)

```rust
let s = "Rust";
s.len();              // 4 (bytes)
s.to_uppercase();     // "RUST" (returns a String)
s.to_lowercase();     // "rust"
s.contains("us");     // true
s.starts_with("Ru");  // true
s.replace("u", "*");  // "R*st"
```

### Building a String

```rust
let mut name = String::from("Val");
name.push('!');          // adds a char
name.push_str("entin");  // adds a &str
```

### Concatenating

```rust
let sentence = format!("{} {}", "hello", "world");
```

> 💡 Rule of thumb: take `&str` as a function **parameter** (more flexible),
> and return/store a `String` when you **own** the text.
"#,
        },
        task: Bi {
            fr: "Complète `crier` : elle prend un &str et renvoie une String en MAJUSCULES suivie d'un '!'. Le programme teste \"bravo\" et doit afficher : BRAVO!",
            en: "Complete `crier`: it takes a &str and returns a String in UPPERCASE followed by a '!'. The program tests \"bravo\" and must print: BRAVO!",
        },
        starter: r#"fn crier(texte: &str) -> String {
    // Mets le texte en majuscules et ajoute "!" à la fin
}

fn main() {
    println!("{}", crier("bravo"));
}
"#,
        check: Check::Stdout { expected: "BRAVO!" },
        hints: &[
            Bi { fr: "to_uppercase() renvoie une String. format! permet d'y coller le '!'.", en: "to_uppercase() returns a String. format! lets you append the '!'." },
            Bi { fr: "format!(\"{}!\", texte.to_uppercase())", en: "format!(\"{}!\", texte.to_uppercase())" },
        ],
        solution: r#"fn crier(texte: &str) -> String {
    format!("{}!", texte.to_uppercase())
}

fn main() {
    println!("{}", crier("bravo"));
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 16,
        slug: "hashmap",
        title: Bi { fr: "Les HashMap", en: "HashMaps" },
        subtitle: Bi { fr: "Associer des clés à des valeurs", en: "Map keys to values" },
        xp: 150,
        lesson: Bi {
            fr: r#"
## `HashMap<K, V>` : un dictionnaire

Une `HashMap` associe des **clés** à des **valeurs**. Il faut l'importer :

```rust
use std::collections::HashMap;

let mut ages = HashMap::new();
ages.insert("Ana", 30);
ages.insert("Bob", 25);
```

### Lire une valeur

`get` renvoie une **`Option`** (la clé peut être absente) :

```rust
match ages.get("Ana") {
    Some(age) => println!("{age}"),
    None => println!("inconnu"),
}
```

### Le motif `entry` (très utile pour compter)

`entry(clé).or_insert(défaut)` renvoie une **référence mutable** vers la valeur,
en l'insérant d'abord si elle manque :

```rust
let mut compteur = HashMap::new();
for mot in ["a", "b", "a"] {
    *compteur.entry(mot).or_insert(0) += 1;
}
// compteur["a"] == 2, compteur["b"] == 1
```

L'étoile `*` **déréférence** la référence pour modifier la valeur pointée.

> 💡 L'ordre des éléments d'une `HashMap` est **imprévisible**. Si tu veux un
> ordre trié, utilise `BTreeMap` (niveau plus avancé).
"#,
            en: r#"
## `HashMap<K, V>`: a dictionary

A `HashMap` maps **keys** to **values**. You must import it:

```rust
use std::collections::HashMap;

let mut ages = HashMap::new();
ages.insert("Ana", 30);
ages.insert("Bob", 25);
```

### Reading a value

`get` returns an **`Option`** (the key might be missing):

```rust
match ages.get("Ana") {
    Some(age) => println!("{age}"),
    None => println!("unknown"),
}
```

### The `entry` pattern (great for counting)

`entry(key).or_insert(default)` returns a **mutable reference** to the value,
inserting it first if missing:

```rust
let mut counter = HashMap::new();
for word in ["a", "b", "a"] {
    *counter.entry(word).or_insert(0) += 1;
}
// counter["a"] == 2, counter["b"] == 1
```

The `*` **dereferences** the reference to change the pointed-to value.

> 💡 A `HashMap`'s element order is **unpredictable**. For sorted order use
> `BTreeMap` (a more advanced level).
"#,
        },
        task: Bi {
            fr: "Complète `occurrences` : elle compte combien de fois `cible` apparaît dans la liste, à l'aide d'une HashMap. Le programme teste [1, 2, 2, 3, 2] avec cible 2 et doit afficher : 3",
            en: "Complete `occurrences`: it counts how many times `cible` appears in the list, using a HashMap. The program tests [1, 2, 2, 3, 2] with target 2 and must print: 3",
        },
        starter: r#"use std::collections::HashMap;

fn occurrences(nombres: &[i32], cible: i32) -> i32 {
    let mut compteur: HashMap<i32, i32> = HashMap::new();
    for &n in nombres {
        // Incrémente le compteur de n :
        // *compteur.entry(n).or_insert(0) += 1;
    }
    // Renvoie le compteur de `cible`, ou 0 si absent
    *compteur.get(&cible).unwrap_or(&0)
}

fn main() {
    println!("{}", occurrences(&[1, 2, 2, 3, 2], 2));
}
"#,
        check: Check::Stdout { expected: "3" },
        hints: &[
            Bi { fr: "Dans la boucle : *compteur.entry(n).or_insert(0) += 1;", en: "In the loop: *compteur.entry(n).or_insert(0) += 1;" },
        ],
        solution: r#"use std::collections::HashMap;

fn occurrences(nombres: &[i32], cible: i32) -> i32 {
    let mut compteur: HashMap<i32, i32> = HashMap::new();
    for &n in nombres {
        *compteur.entry(n).or_insert(0) += 1;
    }
    *compteur.get(&cible).unwrap_or(&0)
}

fn main() {
    println!("{}", occurrences(&[1, 2, 2, 3, 2], 2));
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 17,
        slug: "result",
        title: Bi { fr: "Result : gérer les erreurs", en: "Result: handling errors" },
        subtitle: Bi { fr: "Ok ou Err, sans exceptions", en: "Ok or Err, no exceptions" },
        xp: 150,
        lesson: Bi {
            fr: r#"
## `Result<T, E>`

Rust n'a **pas d'exceptions**. Une opération qui peut échouer renvoie un
`Result`, un enum à deux variantes :

```rust
enum Result<T, E> {
    Ok(T),  // succès, avec la valeur
    Err(E), // échec, avec l'erreur
}
```

`Ok` et `Err` sont dans le prélude (pas besoin de les importer).

```rust
fn racine(x: f64) -> Result<f64, String> {
    if x < 0.0 {
        Err(String::from("nombre négatif"))
    } else {
        Ok(x.sqrt())
    }
}
```

### Traiter le résultat avec `match`

```rust
match racine(16.0) {
    Ok(v) => println!("racine = {v}"),
    Err(e) => println!("erreur : {e}"),
}
```

### Méthodes pratiques

- `r.is_ok()` / `r.is_err()`
- `r.unwrap_or(0)` → la valeur, ou `0` en cas d'`Err`
- `r.unwrap()` → la valeur, mais **panique** si `Err` (à éviter en vrai)

> 💡 `Option` dit « il y a / il n'y a pas ». `Result` dit « ça a marché / ça a
> échoué **et voici pourquoi** ».
"#,
            en: r#"
## `Result<T, E>`

Rust has **no exceptions**. An operation that may fail returns a `Result`, an
enum with two variants:

```rust
enum Result<T, E> {
    Ok(T),  // success, with the value
    Err(E), // failure, with the error
}
```

`Ok` and `Err` are in the prelude (no import needed).

```rust
fn sqrt(x: f64) -> Result<f64, String> {
    if x < 0.0 {
        Err(String::from("negative number"))
    } else {
        Ok(x.sqrt())
    }
}
```

### Handling the result with `match`

```rust
match sqrt(16.0) {
    Ok(v) => println!("root = {v}"),
    Err(e) => println!("error: {e}"),
}
```

### Handy methods

- `r.is_ok()` / `r.is_err()`
- `r.unwrap_or(0)` → the value, or `0` on `Err`
- `r.unwrap()` → the value, but **panics** on `Err` (avoid in real code)

> 💡 `Option` says "there is / there isn't". `Result` says "it worked / it
> failed **and here's why**".
"#,
        },
        task: Bi {
            fr: "Complète `diviser` : elle renvoie Err(\"division par zéro\") si b vaut 0, sinon Ok(a / b). Le programme teste 10 / 2 et doit afficher : 5",
            en: "Complete `diviser`: it returns Err(\"division par zéro\") if b is 0, otherwise Ok(a / b). The program tests 10 / 2 and must print: 5",
        },
        starter: r#"fn diviser(a: i32, b: i32) -> Result<i32, String> {
    // Si b == 0, renvoie Err(String::from("division par zéro"))
    // Sinon renvoie Ok(a / b)
}

fn main() {
    match diviser(10, 2) {
        Ok(v) => println!("{v}"),
        Err(e) => println!("{e}"),
    }
}
"#,
        check: Check::Stdout { expected: "5" },
        hints: &[
            Bi { fr: "if b == 0 { Err(...) } else { Ok(a / b) }", en: "if b == 0 { Err(...) } else { Ok(a / b) }" },
        ],
        solution: r#"fn diviser(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("division par zéro"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    match diviser(10, 2) {
        Ok(v) => println!("{v}"),
        Err(e) => println!("{e}"),
    }
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 18,
        slug: "question-mark",
        title: Bi { fr: "L'opérateur ?", en: "The ? operator" },
        subtitle: Bi { fr: "Propager les erreurs élégamment", en: "Propagate errors elegantly" },
        xp: 160,
        lesson: Bi {
            fr: r#"
## Propager une erreur avec `?`

Souvent, quand une sous-opération échoue, on veut juste **transmettre**
l'erreur à l'appelant. Écrire un `match` à chaque fois est lourd :

```rust
let valeur = match diviser(a, b) {
    Ok(v) => v,
    Err(e) => return Err(e), // on remonte l'erreur
};
```

L'opérateur **`?`** fait exactement ça, en un caractère :

```rust
let valeur = diviser(a, b)?;
```

`?` signifie : « si c'est `Ok(v)`, donne-moi `v` ; si c'est `Err(e)`, **sors de
la fonction** en renvoyant `Err(e)` ».

### Exemple complet

```rust
fn calcul() -> Result<i32, String> {
    let a = diviser(20, 2)?; // a = 10
    let b = diviser(a, 5)?;  // b = 2
    Ok(b)
}
```

⚠️ `?` ne s'utilise que dans une fonction qui **renvoie elle-même** un `Result`
(ou une `Option`), puisqu'il peut faire un `return Err(...)`.

> 💡 `?` rend le code « chemin heureux » lisible : on enchaîne les étapes, et la
> gestion d'erreur est implicite mais rigoureuse.
"#,
            en: r#"
## Propagating an error with `?`

Often, when a sub-operation fails, you just want to **forward** the error to
the caller. Writing a `match` every time is heavy:

```rust
let value = match divide(a, b) {
    Ok(v) => v,
    Err(e) => return Err(e), // bubble the error up
};
```

The **`?`** operator does exactly that, in one character:

```rust
let value = divide(a, b)?;
```

`?` means: "if it's `Ok(v)`, give me `v`; if it's `Err(e)`, **return from the
function** with `Err(e)`".

### Full example

```rust
fn compute() -> Result<i32, String> {
    let a = divide(20, 2)?; // a = 10
    let b = divide(a, 5)?;  // b = 2
    Ok(b)
}
```

⚠️ `?` can only be used in a function that **itself returns** a `Result` (or an
`Option`), since it may do a `return Err(...)`.

> 💡 `?` keeps the "happy path" readable: you chain the steps, and error
> handling is implicit yet rigorous.
"#,
        },
        task: Bi {
            fr: "Utilise l'opérateur ? dans `calcul` pour enchaîner deux divisions : 20 / 2, puis le résultat / 5. Le programme affiche le résultat, attendu : 2",
            en: "Use the ? operator in `calcul` to chain two divisions: 20 / 2, then the result / 5. The program prints the result, expected: 2",
        },
        starter: r#"fn diviser(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("division par zéro"))
    } else {
        Ok(a / b)
    }
}

fn calcul() -> Result<i32, String> {
    // Utilise ? pour récupérer 20 / 2, puis diviser ce résultat par 5
    let a = diviser(20, 2)?;
    // ... à toi de jouer pour b, puis renvoie Ok(b)
}

fn main() {
    println!("{}", calcul().unwrap());
}
"#,
        check: Check::Stdout { expected: "2" },
        hints: &[
            Bi { fr: "let b = diviser(a, 5)?; puis Ok(b)", en: "let b = diviser(a, 5)?; then Ok(b)" },
        ],
        solution: r#"fn diviser(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("division par zéro"))
    } else {
        Ok(a / b)
    }
}

fn calcul() -> Result<i32, String> {
    let a = diviser(20, 2)?;
    let b = diviser(a, 5)?;
    Ok(b)
}

fn main() {
    println!("{}", calcul().unwrap());
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 19,
        slug: "panic-unwrap",
        title: Bi { fr: "Paniques & accès sûrs", en: "Panics & safe access" },
        subtitle: Bi { fr: "unwrap, expect et leurs alternatives", en: "unwrap, expect and their alternatives" },
        xp: 150,
        lesson: Bi {
            fr: r#"
## Quand un programme « panique »

Une **panique** (`panic!`) interrompt brutalement le programme. Elle survient
sur une erreur irrécupérable : accès hors d'un tableau, `unwrap()` sur `None`,
division entière par zéro…

```rust
let v = vec![1, 2, 3];
let x = v[10];      // 💥 panique : index out of bounds
let y: Option<i32> = None;
let z = y.unwrap(); // 💥 panique : called unwrap on a None
```

### Éviter la panique

Plutôt que d'accéder « à l'aveugle », utilise les versions **sûres** qui
renvoient une `Option` :

```rust
let v = vec![1, 2, 3];
let x = v.get(10);            // None (pas de panique)
let sur = v.get(10).copied().unwrap_or(-1); // -1
```

- `unwrap_or(défaut)` : valeur, ou un défaut.
- `unwrap_or_else(|| ...)` : pareil, mais le défaut est calculé seulement si
  besoin.
- `expect("message")` : comme `unwrap`, mais avec un **message** clair en cas
  de panique (pratique pour déboguer).

> 💡 Dans un vrai programme, réserve `unwrap`/`expect` aux cas où une `Err`/
> `None` est **logiquement impossible**. Sinon, gère le cas proprement.
"#,
            en: r#"
## When a program "panics"

A **panic** (`panic!`) abruptly stops the program. It happens on an
unrecoverable error: out-of-bounds array access, `unwrap()` on `None`, integer
division by zero…

```rust
let v = vec![1, 2, 3];
let x = v[10];      // 💥 panic: index out of bounds
let y: Option<i32> = None;
let z = y.unwrap(); // 💥 panic: called unwrap on a None
```

### Avoiding the panic

Instead of accessing "blindly", use the **safe** versions returning an
`Option`:

```rust
let v = vec![1, 2, 3];
let x = v.get(10);            // None (no panic)
let safe = v.get(10).copied().unwrap_or(-1); // -1
```

- `unwrap_or(default)`: value, or a default.
- `unwrap_or_else(|| ...)`: same, but the default is computed only if needed.
- `expect("message")`: like `unwrap`, but with a clear **message** on panic
  (handy for debugging).

> 💡 In real code, reserve `unwrap`/`expect` for cases where an `Err`/`None` is
> **logically impossible**. Otherwise, handle the case cleanly.
"#,
        },
        task: Bi {
            fr: "Complète `troisieme` : elle renvoie l'élément d'indice 2 du tableau, ou -1 s'il n'existe pas, SANS jamais paniquer. Le programme teste [10, 20] (pas d'indice 2) et doit afficher : -1",
            en: "Complete `troisieme`: it returns the element at index 2 of the slice, or -1 if it doesn't exist, WITHOUT ever panicking. The program tests [10, 20] (no index 2) and must print: -1",
        },
        starter: r#"fn troisieme(v: &[i32]) -> i32 {
    // Utilise .get(2) (qui renvoie une Option) puis transforme-la
    // en i32 avec une valeur par défaut de -1.
}

fn main() {
    println!("{}", troisieme(&[10, 20]));
}
"#,
        check: Check::Stdout { expected: "-1" },
        hints: &[
            Bi { fr: ".get(2) renvoie Option<&i32>. .copied() la transforme en Option<i32>.", en: ".get(2) returns Option<&i32>. .copied() turns it into Option<i32>." },
            Bi { fr: "v.get(2).copied().unwrap_or(-1)", en: "v.get(2).copied().unwrap_or(-1)" },
        ],
        solution: r#"fn troisieme(v: &[i32]) -> i32 {
    v.get(2).copied().unwrap_or(-1)
}

fn main() {
    println!("{}", troisieme(&[10, 20]));
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 20,
        slug: "generics-fn",
        title: Bi { fr: "Les génériques (fonctions)", en: "Generics (functions)" },
        subtitle: Bi { fr: "Du code qui marche pour plusieurs types", en: "Code that works for many types" },
        xp: 160,
        lesson: Bi {
            fr: r#"
## Les génériques

Imagine une fonction qui renvoie le dernier élément d'une liste. Tu ne veux pas
la réécrire pour les `i32`, puis les `char`, puis… Les **génériques**
permettent d'écrire le code **une seule fois**, pour un type abstrait `T`.

```rust
fn dernier<T>(v: &[T]) -> &T {
    &v[v.len() - 1]
}
```

- `<T>` après le nom déclare un **paramètre de type**. `T` est un nom (on
  pourrait écrire `U`, `Element`…).
- On l'utilise ensuite comme n'importe quel type dans la signature.

À l'appel, Rust **déduit** `T` :

```rust
dernier(&[1, 2, 3]);        // T = i32
dernier(&['a', 'b']);       // T = char
```

### Les contraintes (*trait bounds*)

Parfois `T` doit savoir faire quelque chose. Par exemple, pour **copier** la
valeur, on exige que `T` soit `Copy` :

```rust
fn dernier_copie<T: Copy>(v: &[T]) -> T {
    v[v.len() - 1]
}
```

`T: Copy` se lit « T doit être un type copiable ». On verra les *traits* au
niveau suivant — pour l'instant, retiens la syntaxe `<T: Contrainte>`.

> 💡 Les génériques n'ont **aucun coût** à l'exécution : Rust génère une version
> spécialisée pour chaque type utilisé (*monomorphisation*).
"#,
            en: r#"
## Generics

Imagine a function returning the last element of a list. You don't want to
rewrite it for `i32`, then `char`, then… **Generics** let you write the code
**once**, for an abstract type `T`.

```rust
fn last<T>(v: &[T]) -> &T {
    &v[v.len() - 1]
}
```

- `<T>` after the name declares a **type parameter**. `T` is a name (could be
  `U`, `Element`…).
- You then use it like any type in the signature.

At the call site, Rust **infers** `T`:

```rust
last(&[1, 2, 3]);        // T = i32
last(&['a', 'b']);       // T = char
```

### Constraints (*trait bounds*)

Sometimes `T` must be able to do something. For example, to **copy** the
value, require `T` to be `Copy`:

```rust
fn last_copy<T: Copy>(v: &[T]) -> T {
    v[v.len() - 1]
}
```

`T: Copy` reads "T must be a copyable type". We'll see *traits* in the next
level — for now, remember the `<T: Constraint>` syntax.

> 💡 Generics have **zero** runtime cost: Rust generates a specialized version
> for each used type (*monomorphization*).
"#,
        },
        task: Bi {
            fr: "Complète `dernier` (générique avec contrainte Copy) qui renvoie le dernier élément d'une slice. Le programme teste [1, 2, 3] et doit afficher : 3",
            en: "Complete `dernier` (generic with a Copy bound) returning the last element of a slice. The program tests [1, 2, 3] and must print: 3",
        },
        starter: r#"fn dernier<T: Copy>(v: &[T]) -> T {
    // Renvoie le dernier élément : indice v.len() - 1
}

fn main() {
    println!("{}", dernier(&[1, 2, 3]));
}
"#,
        check: Check::Stdout { expected: "3" },
        hints: &[
            Bi { fr: "v[v.len() - 1]", en: "v[v.len() - 1]" },
        ],
        solution: r#"fn dernier<T: Copy>(v: &[T]) -> T {
    v[v.len() - 1]
}

fn main() {
    println!("{}", dernier(&[1, 2, 3]));
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 21,
        slug: "generics-struct",
        title: Bi { fr: "Structures génériques", en: "Generic structs" },
        subtitle: Bi { fr: "Des types paramétrés", en: "Parameterized types" },
        xp: 170,
        lesson: Bi {
            fr: r#"
## Des `struct` génériques

Comme les fonctions, les structures peuvent être **génériques**. `Vec<T>` et
`Option<T>` en sont des exemples que tu utilises déjà !

```rust
struct Paire<T> {
    a: T,
    b: T,
}
```

`Paire<i32>` contient deux entiers ; `Paire<String>` deux chaînes.

### Méthodes sur un type générique

Le bloc `impl` doit aussi déclarer le paramètre :

```rust
impl<T> Paire<T> {
    fn nouvelle(a: T, b: T) -> Paire<T> {
        Paire { a, b }
    }
}
```

### Exiger une contrainte dans l'impl

Si une méthode a besoin que `T` sache faire quelque chose — par exemple
s'afficher avec `{}` — on ajoute la contrainte `Display` :

```rust
use std::fmt::Display;

impl<T: Display> Paire<T> {
    fn afficher(&self) -> String {
        format!("({}, {})", self.a, self.b)
    }
}
```

> 💡 `Display` est le trait « peut s'afficher joliment avec `{}` ». Les nombres,
> `&str`, `String`… l'implémentent déjà.
"#,
            en: r#"
## Generic `struct`s

Like functions, structs can be **generic**. `Vec<T>` and `Option<T>` are
examples you already use!

```rust
struct Paire<T> {
    a: T,
    b: T,
}
```

`Paire<i32>` holds two integers; `Paire<String>` two strings.

### Methods on a generic type

The `impl` block must also declare the parameter:

```rust
impl<T> Paire<T> {
    fn nouvelle(a: T, b: T) -> Paire<T> {
        Paire { a, b }
    }
}
```

### Requiring a constraint in the impl

If a method needs `T` to do something — e.g. print with `{}` — add the
`Display` constraint:

```rust
use std::fmt::Display;

impl<T: Display> Paire<T> {
    fn afficher(&self) -> String {
        format!("({}, {})", self.a, self.b)
    }
}
```

> 💡 `Display` is the "can be printed nicely with `{}`" trait. Numbers, `&str`,
> `String`… already implement it.
"#,
        },
        task: Bi {
            fr: "Complète la méthode `afficher` de `Paire<T>` : elle renvoie la chaîne \"(a, b)\". Le programme crée Paire { a: 1, b: 2 } et doit afficher : (1, 2)",
            en: "Complete the `afficher` method of `Paire<T>`: it returns the string \"(a, b)\". The program builds Paire { a: 1, b: 2 } and must print: (1, 2)",
        },
        starter: r#"use std::fmt::Display;

struct Paire<T> {
    a: T,
    b: T,
}

impl<T: Display> Paire<T> {
    fn afficher(&self) -> String {
        // Renvoie une String de la forme "(a, b)" avec format!
    }
}

fn main() {
    let p = Paire { a: 1, b: 2 };
    println!("{}", p.afficher());
}
"#,
        check: Check::Stdout { expected: "(1, 2)" },
        hints: &[
            Bi { fr: "format!(\"({}, {})\", self.a, self.b)", en: "format!(\"({}, {})\", self.a, self.b)" },
        ],
        solution: r#"use std::fmt::Display;

struct Paire<T> {
    a: T,
    b: T,
}

impl<T: Display> Paire<T> {
    fn afficher(&self) -> String {
        format!("({}, {})", self.a, self.b)
    }
}

fn main() {
    let p = Paire { a: 1, b: 2 };
    println!("{}", p.afficher());
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 22,
        slug: "traits",
        title: Bi { fr: "Les traits", en: "Traits" },
        subtitle: Bi { fr: "Définir un comportement partagé", en: "Define shared behavior" },
        xp: 170,
        lesson: Bi {
            fr: r#"
## Les traits

Un **trait** décrit un **comportement** que des types peuvent implémenter.
C'est l'équivalent des « interfaces » d'autres langages.

```rust
trait Aire {
    fn aire(&self) -> u32; // une méthode, sans corps : à fournir
}
```

### Implémenter un trait pour un type

```rust
struct Carre {
    cote: u32,
}

impl Aire for Carre {
    fn aire(&self) -> u32 {
        self.cote * self.cote
    }
}
```

On lit `impl Aire for Carre` : « implémenter le trait `Aire` pour le type
`Carre` ». On peut implémenter le **même** trait pour plein de types
différents :

```rust
struct Rectangle { l: u32, h: u32 }

impl Aire for Rectangle {
    fn aire(&self) -> u32 {
        self.l * self.h
    }
}
```

Du coup, `carre.aire()` et `rectangle.aire()` s'appellent de la même façon.

> 💡 Les traits sont la pierre angulaire du polymorphisme en Rust : génériques,
> objets-traits (`dyn`), opérateurs (`+`), affichage (`Display`)… tout passe par
> des traits.
"#,
            en: r#"
## Traits

A **trait** describes a **behavior** that types can implement. It's the
equivalent of "interfaces" in other languages.

```rust
trait Aire {
    fn aire(&self) -> u32; // a method with no body: to be provided
}
```

### Implementing a trait for a type

```rust
struct Carre {
    cote: u32,
}

impl Aire for Carre {
    fn aire(&self) -> u32 {
        self.cote * self.cote
    }
}
```

Read `impl Aire for Carre` as: "implement the `Aire` trait for the `Carre`
type". You can implement the **same** trait for many different types:

```rust
struct Rectangle { l: u32, h: u32 }

impl Aire for Rectangle {
    fn aire(&self) -> u32 {
        self.l * self.h
    }
}
```

So `square.aire()` and `rectangle.aire()` are called the same way.

> 💡 Traits are the cornerstone of polymorphism in Rust: generics, trait
> objects (`dyn`), operators (`+`), printing (`Display`)… all go through traits.
"#,
        },
        task: Bi {
            fr: "Implémente le trait `Aire` pour `Carre` : la méthode `aire` renvoie côté × côté. Le programme teste un carré de côté 5 et doit afficher : 25",
            en: "Implement the `Aire` trait for `Carre`: the `aire` method returns side × side. The program tests a square of side 5 and must print: 25",
        },
        starter: r#"trait Aire {
    fn aire(&self) -> u32;
}

struct Carre {
    cote: u32,
}

impl Aire for Carre {
    // Implémente ici la méthode aire(&self) -> u32
}

fn main() {
    let c = Carre { cote: 5 };
    println!("{}", c.aire());
}
"#,
        check: Check::Stdout { expected: "25" },
        hints: &[
            Bi { fr: "fn aire(&self) -> u32 { self.cote * self.cote }", en: "fn aire(&self) -> u32 { self.cote * self.cote }" },
        ],
        solution: r#"trait Aire {
    fn aire(&self) -> u32;
}

struct Carre {
    cote: u32,
}

impl Aire for Carre {
    fn aire(&self) -> u32 {
        self.cote * self.cote
    }
}

fn main() {
    let c = Carre { cote: 5 };
    println!("{}", c.aire());
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 23,
        slug: "trait-default",
        title: Bi { fr: "Méthodes par défaut", en: "Default methods" },
        subtitle: Bi { fr: "Un trait peut fournir du code", en: "A trait can provide code" },
        xp: 170,
        lesson: Bi {
            fr: r#"
## Des méthodes par défaut dans un trait

Un trait peut fournir une **implémentation par défaut** pour certaines de ses
méthodes. Les types qui l'implémentent en **héritent** gratuitement, et peuvent
choisir de la **redéfinir**.

```rust
trait Salutation {
    fn nom(&self) -> String; // à fournir obligatoirement

    // méthode par défaut, qui utilise nom()
    fn saluer(&self) -> String {
        format!("Bonjour {}", self.nom())
    }
}
```

Un type n'a qu'à fournir `nom()` ; il obtient `saluer()` automatiquement :

```rust
struct Personne {
    prenom: String,
}

impl Salutation for Personne {
    fn nom(&self) -> String {
        self.prenom.clone()
    }
    // pas besoin de réécrire saluer() : on garde celle par défaut
}
```

Une méthode par défaut peut appeler les autres méthodes du trait (ici
`self.nom()`), même celles qui n'ont pas encore de corps : elles seront
fournies par le type qui implémente.

> 💡 C'est ainsi que la bibliothèque standard offre des dizaines de méthodes sur
> les itérateurs alors que tu n'écris qu'une seule fonction : `next`.
"#,
            en: r#"
## Default methods in a trait

A trait can provide a **default implementation** for some of its methods.
Implementing types **inherit** it for free, and may choose to **override** it.

```rust
trait Salutation {
    fn nom(&self) -> String; // must be provided

    // default method, using nom()
    fn saluer(&self) -> String {
        format!("Bonjour {}", self.nom())
    }
}
```

A type only needs to provide `nom()`; it gets `saluer()` automatically:

```rust
struct Personne {
    prenom: String,
}

impl Salutation for Personne {
    fn nom(&self) -> String {
        self.prenom.clone()
    }
    // no need to rewrite saluer(): we keep the default one
}
```

A default method can call the trait's other methods (here `self.nom()`), even
those with no body yet: they'll be provided by the implementing type.

> 💡 That's how the standard library offers dozens of methods on iterators while
> you only write a single function: `next`.
"#,
        },
        task: Bi {
            fr: "Implémente seulement `nom` pour `Personne` (renvoie le prénom). La méthode par défaut `saluer` fera le reste. Le programme teste le prénom \"Lea\" et doit afficher : Bonjour Lea",
            en: "Implement only `nom` for `Personne` (return the first name). The default `saluer` method does the rest. The program tests the name \"Lea\" and must print: Bonjour Lea",
        },
        starter: r#"trait Salutation {
    fn nom(&self) -> String;
    fn saluer(&self) -> String {
        format!("Bonjour {}", self.nom())
    }
}

struct Personne {
    prenom: String,
}

impl Salutation for Personne {
    // Implémente seulement nom(&self) -> String
}

fn main() {
    let p = Personne { prenom: String::from("Lea") };
    println!("{}", p.saluer());
}
"#,
        check: Check::Stdout { expected: "Bonjour Lea" },
        hints: &[
            Bi { fr: "fn nom(&self) -> String { self.prenom.clone() }", en: "fn nom(&self) -> String { self.prenom.clone() }" },
        ],
        solution: r#"trait Salutation {
    fn nom(&self) -> String;
    fn saluer(&self) -> String {
        format!("Bonjour {}", self.nom())
    }
}

struct Personne {
    prenom: String,
}

impl Salutation for Personne {
    fn nom(&self) -> String {
        self.prenom.clone()
    }
}

fn main() {
    let p = Personne { prenom: String::from("Lea") };
    println!("{}", p.saluer());
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 24,
        slug: "trait-bounds",
        title: Bi { fr: "Contraintes de traits", en: "Trait bounds" },
        subtitle: Bi { fr: "Exiger un comportement des génériques", en: "Require behavior from generics" },
        xp: 180,
        lesson: Bi {
            fr: r#"
## Contraindre un type générique

Une fonction générique ne peut utiliser que ce que `T` **garantit** savoir
faire. Pour autoriser une opération, on ajoute une **contrainte de trait**.

```rust
use std::fmt::Display;

fn afficher<T: Display>(x: T) {
    println!("{x}"); // possible car T: Display
}
```

### Comparer

Pour comparer avec `<`, `>`, etc., il faut le trait `PartialOrd` :

```rust
fn plus_grand<T: PartialOrd>(a: T, b: T) -> bool {
    a > b
}
```

### Plusieurs contraintes

On combine avec `+` :

```rust
fn afficher_et_comparer<T: Display + PartialOrd>(a: T, b: T) {
    println!("{a} > {b} ? {}", a > b);
}
```

### Syntaxe `where` (pour les cas longs)

```rust
fn truc<T>(x: T) -> T
where
    T: Display + Clone,
{
    x
}
```

> 💡 Les contraintes sont vérifiées **à la compilation**. Si tu appelles
> `plus_grand` avec un type non comparable, le compilateur refuse — avant même
> d'exécuter.
"#,
            en: r#"
## Constraining a generic type

A generic function can only use what `T` is **guaranteed** to do. To allow an
operation, add a **trait bound**.

```rust
use std::fmt::Display;

fn afficher<T: Display>(x: T) {
    println!("{x}"); // ok because T: Display
}
```

### Comparing

To compare with `<`, `>`, etc., you need the `PartialOrd` trait:

```rust
fn plus_grand<T: PartialOrd>(a: T, b: T) -> bool {
    a > b
}
```

### Multiple bounds

Combine with `+`:

```rust
fn show_and_compare<T: Display + PartialOrd>(a: T, b: T) {
    println!("{a} > {b} ? {}", a > b);
}
```

### `where` syntax (for long cases)

```rust
fn thing<T>(x: T) -> T
where
    T: Display + Clone,
{
    x
}
```

> 💡 Bounds are checked **at compile time**. If you call `plus_grand` with a
> non-comparable type, the compiler refuses — before running anything.
"#,
        },
        task: Bi {
            fr: "Complète `plus_grand<T: PartialOrd>` : elle renvoie true si a est strictement plus grand que b. Le programme teste (5, 3) et doit afficher : true",
            en: "Complete `plus_grand<T: PartialOrd>`: it returns true if a is strictly greater than b. The program tests (5, 3) and must print: true",
        },
        starter: r#"fn plus_grand<T: PartialOrd>(a: T, b: T) -> bool {
    // Renvoie le résultat de la comparaison a > b
}

fn main() {
    println!("{}", plus_grand(5, 3));
}
"#,
        check: Check::Stdout { expected: "true" },
        hints: &[
            Bi { fr: "Le corps tient en une expression : a > b", en: "The body is a single expression: a > b" },
        ],
        solution: r#"fn plus_grand<T: PartialOrd>(a: T, b: T) -> bool {
    a > b
}

fn main() {
    println!("{}", plus_grand(5, 3));
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 25,
        slug: "derive-debug",
        title: Bi { fr: "#[derive] : traits automatiques", en: "#[derive]: automatic traits" },
        subtitle: Bi { fr: "Debug, Clone, PartialEq gratuits", en: "Free Debug, Clone, PartialEq" },
        xp: 160,
        lesson: Bi {
            fr: r#"
## `#[derive(...)]` : implémenter des traits automatiquement

Beaucoup de traits courants peuvent être **dérivés** : le compilateur écrit
l'implémentation pour toi, via une **annotation** placée juste au-dessus du
type.

```rust
#[derive(Debug)]
struct Joueur {
    nom: String,
    score: u32,
}
```

### `Debug` : afficher pour déboguer

`Debug` active le format `{:?}` :

```rust
let j = Joueur { nom: String::from("Zoe"), score: 100 };
println!("{:?}", j);
// Joueur { nom: "Zoe", score: 100 }
```

Et `{:#?}` affiche en version « jolie », sur plusieurs lignes.

### D'autres traits dérivables

```rust
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
```

- `Clone` → active `.clone()`.
- `PartialEq` → active la comparaison `==` / `!=`.
- `Copy`, `Default`, `Hash`, `PartialOrd`… existent aussi.

> 💡 `#[derive(Debug)]` est ton meilleur ami : ajoute-le à tes structs pour
> pouvoir les inspecter avec `println!("{:?}", ...)` pendant que tu apprends.
"#,
            en: r#"
## `#[derive(...)]`: implement traits automatically

Many common traits can be **derived**: the compiler writes the implementation
for you, via an **annotation** placed right above the type.

```rust
#[derive(Debug)]
struct Joueur {
    nom: String,
    score: u32,
}
```

### `Debug`: printing for debugging

`Debug` enables the `{:?}` format:

```rust
let j = Joueur { nom: String::from("Zoe"), score: 100 };
println!("{:?}", j);
// Joueur { nom: "Zoe", score: 100 }
```

And `{:#?}` prints a "pretty", multi-line version.

### Other derivable traits

```rust
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
```

- `Clone` → enables `.clone()`.
- `PartialEq` → enables `==` / `!=` comparison.
- `Copy`, `Default`, `Hash`, `PartialOrd`… also exist.

> 💡 `#[derive(Debug)]` is your best friend: add it to your structs so you can
> inspect them with `println!("{:?}", ...)` while learning.
"#,
        },
        task: Bi {
            fr: "Ajoute l'annotation qui permet d'afficher `Joueur` avec {:?}. Le programme doit afficher : Joueur { nom: \"Zoe\", score: 100 }",
            en: "Add the annotation that allows printing `Joueur` with {:?}. The program must print: Joueur { nom: \"Zoe\", score: 100 }",
        },
        starter: r#"// Ajoute ici l'annotation #[derive(...)] qui convient
struct Joueur {
    nom: String,
    score: u32,
}

fn main() {
    let j = Joueur { nom: String::from("Zoe"), score: 100 };
    println!("{:?}", j);
}
"#,
        check: Check::Stdout { expected: "Joueur { nom: \"Zoe\", score: 100 }" },
        hints: &[
            Bi { fr: "Le format {:?} nécessite le trait Debug. Dérive-le.", en: "The {:?} format needs the Debug trait. Derive it." },
            Bi { fr: "Place #[derive(Debug)] juste au-dessus de struct Joueur.", en: "Put #[derive(Debug)] right above struct Joueur." },
        ],
        solution: r#"#[derive(Debug)]
struct Joueur {
    nom: String,
    score: u32,
}

fn main() {
    let j = Joueur { nom: String::from("Zoe"), score: 100 };
    println!("{:?}", j);
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 26,
        slug: "closures",
        title: Bi { fr: "Les closures", en: "Closures" },
        subtitle: Bi { fr: "Des fonctions anonymes qui capturent", en: "Anonymous functions that capture" },
        xp: 170,
        lesson: Bi {
            fr: r#"
## Les closures (fermetures)

Une **closure** est une fonction **anonyme** qu'on peut stocker dans une
variable ou passer à une autre fonction. Sa syntaxe utilise des barres
verticales `|...|` pour les paramètres :

```rust
let ajouter = |a, b| a + b;
println!("{}", ajouter(2, 3)); // 5

let crier = |s: &str| s.to_uppercase();
```

### Elles capturent leur environnement

Contrairement aux fonctions `fn`, une closure peut **utiliser les variables**
qui l'entourent :

```rust
let facteur = 10;
let multiplier = |x| x * facteur; // capture `facteur`
println!("{}", multiplier(5)); // 50
```

### Les passer en argument

On les reçoit via une contrainte de trait. Le trait `Fn(i32) -> i32` désigne
« une closure qui prend un `i32` et renvoie un `i32` » :

```rust
fn appliquer<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(x)
}

appliquer(|n| n + 1, 41); // 42
```

> 💡 Les closures sont partout en Rust : `map`, `filter`, `sort_by`, les
> threads… On les enchaîne avec les itérateurs au niveau suivant.
"#,
            en: r#"
## Closures

A **closure** is an **anonymous** function you can store in a variable or pass
to another function. Its syntax uses vertical bars `|...|` for parameters:

```rust
let add = |a, b| a + b;
println!("{}", add(2, 3)); // 5

let shout = |s: &str| s.to_uppercase();
```

### They capture their environment

Unlike `fn` functions, a closure can **use the variables** around it:

```rust
let factor = 10;
let multiply = |x| x * factor; // captures `factor`
println!("{}", multiply(5)); // 50
```

### Passing them as arguments

You receive them via a trait bound. The trait `Fn(i32) -> i32` means "a
closure taking an `i32` and returning an `i32`":

```rust
fn appliquer<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(x)
}

appliquer(|n| n + 1, 41); // 42
```

> 💡 Closures are everywhere in Rust: `map`, `filter`, `sort_by`, threads… We
> chain them with iterators in the next level.
"#,
        },
        task: Bi {
            fr: "Complète la closure `triple` pour qu'elle renvoie le triple de son argument. Le programme l'applique à 5 et doit afficher : 15",
            en: "Complete the `triple` closure so it returns three times its argument. The program applies it to 5 and must print: 15",
        },
        starter: r#"fn main() {
    // Complète la closure : elle doit renvoyer x * 3
    let triple = |x: i32| /* ... */;

    println!("{}", triple(5));
}
"#,
        check: Check::Stdout { expected: "15" },
        hints: &[
            Bi { fr: "Le corps d'une closure courte est juste une expression : x * 3", en: "A short closure body is just an expression: x * 3" },
        ],
        solution: r#"fn main() {
    let triple = |x: i32| x * 3;

    println!("{}", triple(5));
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 27,
        slug: "iterators-map",
        title: Bi { fr: "Itérateurs : map & filter", en: "Iterators: map & filter" },
        subtitle: Bi { fr: "Transformer des collections", en: "Transform collections" },
        xp: 180,
        lesson: Bi {
            fr: r#"
## Les itérateurs

Un **itérateur** produit une suite de valeurs, une à une. La plupart des
collections en fournissent un via `.iter()`. On enchaîne ensuite des
**adaptateurs** pour transformer le flux, de façon déclarative.

### `map` : transformer chaque élément

```rust
let v = vec![1, 2, 3];
let doubles: Vec<i32> = v.iter().map(|x| x * 2).collect();
// [2, 4, 6]
```

### `filter` : ne garder que certains éléments

```rust
let v = vec![1, 2, 3, 4, 5, 6];
let pairs: Vec<i32> = v.into_iter().filter(|x| x % 2 == 0).collect();
// [2, 4, 6]
```

### `collect` : reconstruire une collection

Les adaptateurs sont **paresseux** : ils ne font rien tant qu'on ne les
**consomme** pas. `collect()` déclenche le calcul et rassemble les résultats
(ici dans un `Vec`).

```rust
let resultat: Vec<i32> = (1..=3)
    .map(|x| x * 10)
    .collect(); // [10, 20, 30]
```

> 💡 `iter()` emprunte (`&T`), `into_iter()` consomme (`T`). Avec `iter()`, les
> closures reçoivent souvent une **référence**, d'où le `*x` ou `x % 2` selon
> les cas.
"#,
            en: r#"
## Iterators

An **iterator** yields a sequence of values, one at a time. Most collections
provide one via `.iter()`. You then chain **adaptors** to transform the
stream, declaratively.

### `map`: transform each element

```rust
let v = vec![1, 2, 3];
let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
// [2, 4, 6]
```

### `filter`: keep only some elements

```rust
let v = vec![1, 2, 3, 4, 5, 6];
let evens: Vec<i32> = v.into_iter().filter(|x| x % 2 == 0).collect();
// [2, 4, 6]
```

### `collect`: rebuild a collection

Adaptors are **lazy**: they do nothing until you **consume** them.
`collect()` triggers the computation and gathers the results (here into a
`Vec`).

```rust
let result: Vec<i32> = (1..=3)
    .map(|x| x * 10)
    .collect(); // [10, 20, 30]
```

> 💡 `iter()` borrows (`&T`), `into_iter()` consumes (`T`). With `iter()`,
> closures often receive a **reference**, hence `*x` or `x % 2` as needed.
"#,
        },
        task: Bi {
            fr: "Complète la chaîne d'itérateur : double chaque élément du vecteur avec map, puis collecte dans un Vec. Le programme teste [1, 2, 3] et doit afficher : [2, 4, 6]",
            en: "Complete the iterator chain: double each element of the vector with map, then collect into a Vec. The program tests [1, 2, 3] and must print: [2, 4, 6]",
        },
        starter: r#"fn main() {
    let v = vec![1, 2, 3];
    let doubles: Vec<i32> = v.iter()/* ajoute .map(...) ici */.collect();
    println!("{:?}", doubles);
}
"#,
        check: Check::Stdout { expected: "[2, 4, 6]" },
        hints: &[
            Bi { fr: "v.iter().map(|x| x * 2).collect()", en: "v.iter().map(|x| x * 2).collect()" },
        ],
        solution: r#"fn main() {
    let v = vec![1, 2, 3];
    let doubles: Vec<i32> = v.iter().map(|x| x * 2).collect();
    println!("{:?}", doubles);
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 28,
        slug: "iterators-sum",
        title: Bi { fr: "Itérateurs : réductions", en: "Iterators: reductions" },
        subtitle: Bi { fr: "sum, count, max, min", en: "sum, count, max, min" },
        xp: 180,
        lesson: Bi {
            fr: r#"
## Consommer un itérateur en une valeur

Certains adaptateurs ne produisent pas une collection mais **une seule
valeur** : ce sont des *consommateurs*.

```rust
let v = vec![3, 1, 4, 1, 5];

let total: i32 = v.iter().sum();       // 14
let combien = v.iter().count();        // 5
let maxi = v.iter().max();             // Some(&5)
let mini = v.iter().min();             // Some(&1)
```

`max` et `min` renvoient une **`Option`** (la collection peut être vide).

### Combiner avec map/filter

La force des itérateurs, c'est de chaîner :

```rust
// somme des carrés des nombres de 1 à 5
let s: i32 = (1..=5).map(|x| x * x).sum(); // 1+4+9+16+25 = 55
```

### Préciser le type avec le turbofish

Parfois Rust ne devine pas le type de retour de `sum`/`collect`. On le précise
avec la syntaxe **turbofish** `::<...>` :

```rust
let s = (1..=5).map(|x| x * x).sum::<i32>();
```

> 💡 `.sum()` et `.product()` existent ; `.max_by_key(|x| ...)` permet de
> maximiser selon un critère calculé.
"#,
            en: r#"
## Consuming an iterator into a value

Some adaptors produce not a collection but **a single value**: these are
*consumers*.

```rust
let v = vec![3, 1, 4, 1, 5];

let total: i32 = v.iter().sum();       // 14
let how_many = v.iter().count();       // 5
let biggest = v.iter().max();          // Some(&5)
let smallest = v.iter().min();         // Some(&1)
```

`max` and `min` return an **`Option`** (the collection could be empty).

### Combining with map/filter

The power of iterators is chaining:

```rust
// sum of squares of numbers 1 to 5
let s: i32 = (1..=5).map(|x| x * x).sum(); // 1+4+9+16+25 = 55
```

### Specifying the type with turbofish

Sometimes Rust can't infer the return type of `sum`/`collect`. Specify it with
the **turbofish** syntax `::<...>`:

```rust
let s = (1..=5).map(|x| x * x).sum::<i32>();
```

> 💡 `.sum()` and `.product()` exist; `.max_by_key(|x| ...)` maximizes by a
> computed criterion.
"#,
        },
        task: Bi {
            fr: "Calcule la somme des carrés des nombres de 1 à 5 inclus, avec map puis sum. Le programme doit afficher : 55",
            en: "Compute the sum of squares of the numbers from 1 to 5 inclusive, with map then sum. The program must print: 55",
        },
        starter: r#"fn main() {
    // Utilise (1..=5), map pour élever au carré, puis sum
    let resultat: i32 = /* ... */;
    println!("{}", resultat);
}
"#,
        check: Check::Stdout { expected: "55" },
        hints: &[
            Bi { fr: "(1..=5).map(|x| x * x).sum()", en: "(1..=5).map(|x| x * x).sum()" },
        ],
        solution: r#"fn main() {
    let resultat: i32 = (1..=5).map(|x| x * x).sum();
    println!("{}", resultat);
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 29,
        slug: "enumerate-zip",
        title: Bi { fr: "enumerate & zip", en: "enumerate & zip" },
        subtitle: Bi { fr: "Indices et itérateurs jumelés", en: "Indices and paired iterators" },
        xp: 180,
        lesson: Bi {
            fr: r#"
## `enumerate` : connaître l'index

`enumerate()` transforme chaque élément en un **tuple** `(index, valeur)` :

```rust
let fruits = ["pomme", "poire", "kiwi"];
for (i, f) in fruits.iter().enumerate() {
    println!("{i} -> {f}");
}
// 0 -> pomme
// 1 -> poire
// 2 -> kiwi
```

## `zip` : avancer deux itérateurs en parallèle

`zip` associe les éléments de **deux** itérateurs deux à deux, en s'arrêtant au
plus court :

```rust
let noms = ["Ana", "Bob"];
let ages = [30, 25];

for (nom, age) in noms.iter().zip(ages.iter()) {
    println!("{nom} a {age} ans");
}
// Ana a 30 ans
// Bob a 25 ans
```

On peut aussi `collect` un `zip` en `Vec` de tuples, ou même en `HashMap` :

```rust
use std::collections::HashMap;
let map: HashMap<_, _> = noms.iter().zip(ages.iter()).collect();
```

> 💡 `enumerate` est l'équivalent propre du « compteur manuel » : pas besoin
> d'une variable `i` que tu incrémentes à la main.
"#,
            en: r#"
## `enumerate`: knowing the index

`enumerate()` turns each element into a **tuple** `(index, value)`:

```rust
let fruits = ["apple", "pear", "kiwi"];
for (i, f) in fruits.iter().enumerate() {
    println!("{i} -> {f}");
}
// 0 -> apple
// 1 -> pear
// 2 -> kiwi
```

## `zip`: advancing two iterators in parallel

`zip` pairs elements from **two** iterators, stopping at the shorter one:

```rust
let names = ["Ana", "Bob"];
let ages = [30, 25];

for (name, age) in names.iter().zip(ages.iter()) {
    println!("{name} is {age}");
}
// Ana is 30
// Bob is 25
```

You can also `collect` a `zip` into a `Vec` of tuples, or even a `HashMap`:

```rust
use std::collections::HashMap;
let map: HashMap<_, _> = names.iter().zip(ages.iter()).collect();
```

> 💡 `enumerate` is the clean version of the "manual counter": no need for an
> `i` variable you increment by hand.
"#,
        },
        task: Bi {
            fr: "Complète la boucle : associe chaque nom à son âge avec zip et affiche une ligne \"nom a age ans\" pour chacun. Sortie attendue (deux lignes) : Ana a 30 ans / Bob a 25 ans",
            en: "Complete the loop: pair each name with its age using zip and print one line \"nom a age ans\" each. Expected output (two lines): Ana a 30 ans / Bob a 25 ans",
        },
        starter: r#"fn main() {
    let noms = ["Ana", "Bob"];
    let ages = [30, 25];

    // Parcours les deux tableaux en parallèle avec zip
    for (nom, age) in noms.iter()/* .zip(...) */ {
        println!("{nom} a {age} ans");
    }
}
"#,
        check: Check::Stdout { expected: "Ana a 30 ans\nBob a 25 ans" },
        hints: &[
            Bi { fr: "noms.iter().zip(ages.iter())", en: "noms.iter().zip(ages.iter())" },
        ],
        solution: r#"fn main() {
    let noms = ["Ana", "Bob"];
    let ages = [30, 25];

    for (nom, age) in noms.iter().zip(ages.iter()) {
        println!("{nom} a {age} ans");
    }
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 30,
        slug: "fold",
        title: Bi { fr: "fold : accumuler", en: "fold: accumulate" },
        subtitle: Bi { fr: "Replier un itérateur en une valeur", en: "Fold an iterator into one value" },
        xp: 190,
        lesson: Bi {
            fr: r#"
## `fold` : l'accumulateur universel

`fold` parcourt un itérateur en **accumulant** un résultat. C'est l'outil le
plus général : `sum`, `product`, `count`… peuvent tous s'exprimer avec `fold`.

```rust
let total = (1..=4).fold(0, |acc, x| acc + x);
// 0+1+2+3+4 = 10
```

Décortiquons `fold(0, |acc, x| acc + x)` :

- `0` : la **valeur initiale** de l'accumulateur.
- `|acc, x| ...` : une closure qui reçoit l'**accumulateur courant** (`acc`) et
  l'**élément courant** (`x`), et renvoie le **nouvel** accumulateur.

Déroulé pas à pas :

| étape | acc | x | résultat |
|---|---|---|---|
| init | 0 | — | 0 |
| 1 | 0 | 1 | 1 |
| 2 | 1 | 2 | 3 |
| 3 | 3 | 3 | 6 |
| 4 | 6 | 4 | 10 |

### Pas seulement des nombres

L'accumulateur peut être n'importe quoi — une `String`, un `Vec`… :

```rust
let mots = ["a", "b", "c"];
let concat = mots.iter().fold(String::new(), |acc, m| acc + m);
// "abc"
```

> 💡 Pour un **produit**, initialise à `1` : `fold(1, |acc, x| acc * x)`.
"#,
            en: r#"
## `fold`: the universal accumulator

`fold` walks an iterator while **accumulating** a result. It's the most
general tool: `sum`, `product`, `count`… can all be expressed with `fold`.

```rust
let total = (1..=4).fold(0, |acc, x| acc + x);
// 0+1+2+3+4 = 10
```

Breaking down `fold(0, |acc, x| acc + x)`:

- `0`: the accumulator's **initial value**.
- `|acc, x| ...`: a closure receiving the **current accumulator** (`acc`) and
  the **current element** (`x`), returning the **new** accumulator.

Step by step:

| step | acc | x | result |
|---|---|---|---|
| init | 0 | — | 0 |
| 1 | 0 | 1 | 1 |
| 2 | 1 | 2 | 3 |
| 3 | 3 | 3 | 6 |
| 4 | 6 | 4 | 10 |

### Not just numbers

The accumulator can be anything — a `String`, a `Vec`…:

```rust
let words = ["a", "b", "c"];
let concat = words.iter().fold(String::new(), |acc, m| acc + m);
// "abc"
```

> 💡 For a **product**, start at `1`: `fold(1, |acc, x| acc * x)`.
"#,
        },
        task: Bi {
            fr: "Calcule le produit des nombres de 1 à 5 (factorielle de 5) avec fold. Le programme doit afficher : 120",
            en: "Compute the product of the numbers from 1 to 5 (5 factorial) with fold. The program must print: 120",
        },
        starter: r#"fn main() {
    // Initialise l'accumulateur à 1 et multiplie chaque élément
    let produit = (1..=5).fold(1, |acc, x| /* ... */);
    println!("{}", produit);
}
"#,
        check: Check::Stdout { expected: "120" },
        hints: &[
            Bi { fr: "La closure renvoie acc * x.", en: "The closure returns acc * x." },
        ],
        solution: r#"fn main() {
    let produit = (1..=5).fold(1, |acc, x| acc * x);
    println!("{}", produit);
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 31,
        slug: "option-combinators",
        title: Bi { fr: "Combinateurs d'Option", en: "Option combinators" },
        subtitle: Bi { fr: "Transformer sans match", en: "Transform without match" },
        xp: 180,
        lesson: Bi {
            fr: r#"
## Manipuler une `Option` sans `match`

Écrire un `match` pour chaque `Option` est verbeux. Des **méthodes** permettent
de transformer le contenu de façon concise.

### `map` : transformer la valeur si elle existe

```rust
let x = Some(5);
let y = x.map(|v| v * 2); // Some(10)

let rien: Option<i32> = None;
let z = rien.map(|v| v * 2); // None (map ne fait rien)
```

### `unwrap_or` : fournir un défaut

```rust
Some(5).unwrap_or(0); // 5
None.unwrap_or(0);    // 0
```

### `and_then` : enchaîner des opérations qui renvoient une Option

```rust
fn moitie(n: i32) -> Option<i32> {
    if n % 2 == 0 { Some(n / 2) } else { None }
}

Some(8).and_then(moitie); // Some(4)
Some(5).and_then(moitie); // None
```

### `filter` : ne garder que si une condition tient

```rust
Some(4).filter(|n| n % 2 == 0); // Some(4)
Some(3).filter(|n| n % 2 == 0); // None
```

> 💡 On enchaîne ces méthodes pour décrire un calcul « qui peut échouer » sans
> jamais écrire de `if`/`match` à la main.
"#,
            en: r#"
## Working with `Option` without `match`

Writing a `match` for every `Option` is verbose. **Methods** let you transform
the contents concisely.

### `map`: transform the value if present

```rust
let x = Some(5);
let y = x.map(|v| v * 2); // Some(10)

let nothing: Option<i32> = None;
let z = nothing.map(|v| v * 2); // None (map does nothing)
```

### `unwrap_or`: provide a default

```rust
Some(5).unwrap_or(0); // 5
None.unwrap_or(0);    // 0
```

### `and_then`: chain operations that return an Option

```rust
fn half(n: i32) -> Option<i32> {
    if n % 2 == 0 { Some(n / 2) } else { None }
}

Some(8).and_then(half); // Some(4)
Some(5).and_then(half); // None
```

### `filter`: keep only if a condition holds

```rust
Some(4).filter(|n| n % 2 == 0); // Some(4)
Some(3).filter(|n| n % 2 == 0); // None
```

> 💡 Chain these methods to describe a "may fail" computation without ever
> writing a manual `if`/`match`.
"#,
        },
        task: Bi {
            fr: "Complète `longueur_nom` : elle renvoie la longueur du nom s'il existe, ou 0 sinon, en utilisant map et unwrap_or (sans match). Le programme teste Some(\"rust\") et doit afficher : 4",
            en: "Complete `longueur_nom`: it returns the name's length if present, else 0, using map and unwrap_or (no match). The program tests Some(\"rust\") and must print: 4",
        },
        starter: r#"fn longueur_nom(nom: Option<&str>) -> usize {
    // Utilise .map(...) pour obtenir la longueur, puis .unwrap_or(0)
}

fn main() {
    println!("{}", longueur_nom(Some("rust")));
}
"#,
        check: Check::Stdout { expected: "4" },
        hints: &[
            Bi { fr: "nom.map(|s| s.len()).unwrap_or(0)", en: "nom.map(|s| s.len()).unwrap_or(0)" },
        ],
        solution: r#"fn longueur_nom(nom: Option<&str>) -> usize {
    nom.map(|s| s.len()).unwrap_or(0)
}

fn main() {
    println!("{}", longueur_nom(Some("rust")));
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 32,
        slug: "result-combinators",
        title: Bi { fr: "Combinateurs de Result", en: "Result combinators" },
        subtitle: Bi { fr: "map, map_err, ok", en: "map, map_err, ok" },
        xp: 180,
        lesson: Bi {
            fr: r#"
## Manipuler un `Result` sans `match`

Comme `Option`, `Result` offre des méthodes pour transformer son contenu.

### `map` : transformer la valeur de succès (`Ok`)

```rust
let r: Result<i32, String> = Ok(21);
let double = r.map(|v| v * 2); // Ok(42)
// sur un Err, map ne change rien
```

### `map_err` : transformer l'erreur (`Err`)

```rust
let r: Result<i32, String> = Err(String::from("oups"));
let r2 = r.map_err(|e| format!("erreur: {e}"));
```

### `unwrap_or` : valeur ou défaut

```rust
Ok::<i32, String>(5).unwrap_or(0); // 5
Err::<i32, String>(String::from("x")).unwrap_or(0); // 0
```

### `ok` : convertir un `Result` en `Option`

On « oublie » l'erreur : `Ok(v)` devient `Some(v)`, `Err(_)` devient `None`.

```rust
let r: Result<i32, String> = Ok(7);
let o: Option<i32> = r.ok(); // Some(7)
```

> 💡 `map` agit sur le succès, `map_err` sur l'échec. Pratique pour adapter le
> type d'erreur quand on combine plusieurs fonctions.
"#,
            en: r#"
## Working with `Result` without `match`

Like `Option`, `Result` offers methods to transform its contents.

### `map`: transform the success value (`Ok`)

```rust
let r: Result<i32, String> = Ok(21);
let doubled = r.map(|v| v * 2); // Ok(42)
// on an Err, map changes nothing
```

### `map_err`: transform the error (`Err`)

```rust
let r: Result<i32, String> = Err(String::from("oops"));
let r2 = r.map_err(|e| format!("error: {e}"));
```

### `unwrap_or`: value or default

```rust
Ok::<i32, String>(5).unwrap_or(0); // 5
Err::<i32, String>(String::from("x")).unwrap_or(0); // 0
```

### `ok`: convert a `Result` into an `Option`

You "forget" the error: `Ok(v)` becomes `Some(v)`, `Err(_)` becomes `None`.

```rust
let r: Result<i32, String> = Ok(7);
let o: Option<i32> = r.ok(); // Some(7)
```

> 💡 `map` acts on success, `map_err` on failure. Handy to adapt the error type
> when combining several functions.
"#,
        },
        task: Bi {
            fr: "Complète `double_ok` : elle double la valeur si Ok, ou renvoie 0 si Err, avec map et unwrap_or. Le programme teste Ok(21) et doit afficher : 42",
            en: "Complete `double_ok`: it doubles the value if Ok, or returns 0 if Err, using map and unwrap_or. The program tests Ok(21) and must print: 42",
        },
        starter: r#"fn double_ok(r: Result<i32, String>) -> i32 {
    // Double la valeur de succès, sinon 0
}

fn main() {
    let r: Result<i32, String> = Ok(21);
    println!("{}", double_ok(r));
}
"#,
        check: Check::Stdout { expected: "42" },
        hints: &[
            Bi { fr: "r.map(|v| v * 2).unwrap_or(0)", en: "r.map(|v| v * 2).unwrap_or(0)" },
        ],
        solution: r#"fn double_ok(r: Result<i32, String>) -> i32 {
    r.map(|v| v * 2).unwrap_or(0)
}

fn main() {
    let r: Result<i32, String> = Ok(21);
    println!("{}", double_ok(r));
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 33,
        slug: "question-option",
        title: Bi { fr: "? avec Option", en: "? with Option" },
        subtitle: Bi { fr: "Court-circuiter sur None", en: "Short-circuit on None" },
        xp: 180,
        lesson: Bi {
            fr: r#"
## L'opérateur `?` marche aussi avec `Option`

Tu connais `?` pour `Result`. Il fonctionne **pareil** pour `Option`, dans une
fonction qui renvoie une `Option` :

- sur `Some(v)`, il extrait `v` ;
- sur `None`, il **sort immédiatement** de la fonction en renvoyant `None`.

```rust
fn premier_caractere_majuscule(s: &str) -> Option<char> {
    let c = s.chars().next()?;      // None si la chaîne est vide
    Some(c.to_ascii_uppercase())
}

premier_caractere_majuscule("rust"); // Some('R')
premier_caractere_majuscule("");     // None
```

Sans `?`, il aurait fallu écrire :

```rust
let c = match s.chars().next() {
    Some(c) => c,
    None => return None,
};
```

### Chaîner des accès qui peuvent échouer

```rust
fn deuxieme(v: &[i32]) -> Option<i32> {
    let x = v.get(1)?; // None si moins de 2 éléments
    Some(*x)
}
```

> 💡 `?` rend lisible le code qui « avance tant que tout va bien, et abandonne
> proprement dès qu'une étape renvoie `None` ».
"#,
            en: r#"
## The `?` operator also works with `Option`

You know `?` for `Result`. It works the **same** for `Option`, inside a
function returning an `Option`:

- on `Some(v)`, it extracts `v`;
- on `None`, it **returns immediately** from the function with `None`.

```rust
fn first_char_upper(s: &str) -> Option<char> {
    let c = s.chars().next()?;      // None if the string is empty
    Some(c.to_ascii_uppercase())
}

first_char_upper("rust"); // Some('R')
first_char_upper("");     // None
```

Without `?`, you'd have to write:

```rust
let c = match s.chars().next() {
    Some(c) => c,
    None => return None,
};
```

### Chaining accesses that may fail

```rust
fn second(v: &[i32]) -> Option<i32> {
    let x = v.get(1)?; // None if fewer than 2 elements
    Some(*x)
}
```

> 💡 `?` makes readable the code that "advances while all is well, and gives up
> cleanly as soon as a step returns `None`".
"#,
        },
        task: Bi {
            fr: "Complète `premier_char` : avec ?, récupère le premier caractère de la chaîne et renvoie-le en majuscule. Le programme teste \"rust\" et doit afficher : R",
            en: "Complete `premier_char`: with ?, get the first character of the string and return it uppercased. The program tests \"rust\" and must print: R",
        },
        starter: r#"fn premier_char(s: &str) -> Option<char> {
    // Récupère le premier caractère avec s.chars().next()?
    // puis renvoie Some(c.to_ascii_uppercase())
}

fn main() {
    println!("{}", premier_char("rust").unwrap());
}
"#,
        check: Check::Stdout { expected: "R" },
        hints: &[
            Bi { fr: "let c = s.chars().next()?; puis Some(c.to_ascii_uppercase())", en: "let c = s.chars().next()?; then Some(c.to_ascii_uppercase())" },
        ],
        solution: r#"fn premier_char(s: &str) -> Option<char> {
    let c = s.chars().next()?;
    Some(c.to_ascii_uppercase())
}

fn main() {
    println!("{}", premier_char("rust").unwrap());
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 34,
        slug: "match-advanced",
        title: Bi { fr: "Filtrage avancé", en: "Advanced matching" },
        subtitle: Bi { fr: "Gardes, intervalles, alternatives", en: "Guards, ranges, alternatives" },
        xp: 190,
        lesson: Bi {
            fr: r#"
## `match` : motifs avancés

Le `match` que tu connais peut faire bien plus.

### Intervalles

```rust
match note {
    0..=9 => "un chiffre",
    10..=99 => "deux chiffres",
    _ => "grand nombre",
}
```

### Alternatives avec `|`

```rust
match c {
    'a' | 'e' | 'i' | 'o' | 'u' => "voyelle",
    _ => "consonne",
}
```

### Gardes : une condition `if`

Un motif peut être affiné par une condition :

```rust
match n {
    x if x < 0 => "négatif",
    0 => "zéro",
    _ => "positif",
}
```

### Capture avec `@`

Pour à la fois tester un intervalle **et** récupérer la valeur :

```rust
match age {
    n @ 13..=19 => println!("ado de {n} ans"),
    _ => println!("autre"),
}
```

> 💡 Les gardes et intervalles rendent le `match` extrêmement expressif : on
> décrit des cas complexes de façon lisible, et le compilateur vérifie qu'on
> n'en oublie aucun.
"#,
            en: r#"
## `match`: advanced patterns

The `match` you know can do much more.

### Ranges

```rust
match score {
    0..=9 => "one digit",
    10..=99 => "two digits",
    _ => "big number",
}
```

### Alternatives with `|`

```rust
match c {
    'a' | 'e' | 'i' | 'o' | 'u' => "vowel",
    _ => "consonant",
}
```

### Guards: an `if` condition

A pattern can be refined by a condition:

```rust
match n {
    x if x < 0 => "negative",
    0 => "zero",
    _ => "positive",
}
```

### Binding with `@`

To both test a range **and** capture the value:

```rust
match age {
    n @ 13..=19 => println!("teen aged {n}"),
    _ => println!("other"),
}
```

> 💡 Guards and ranges make `match` extremely expressive: you describe complex
> cases readably, and the compiler checks you forgot none.
"#,
        },
        task: Bi {
            fr: "Complète `decrire` avec un match : 0 → \"zero\", de 1 à 9 → \"chiffre\", sinon → \"grand\". Le programme teste 5 et doit afficher : chiffre",
            en: "Complete `decrire` with a match: 0 → \"zero\", 1 to 9 → \"chiffre\", otherwise → \"grand\". The program tests 5 and must print: chiffre",
        },
        starter: r#"fn decrire(n: u32) -> &'static str {
    match n {
        // 0 => ...
        // 1..=9 => ...
        // _ => ...
    }
}

fn main() {
    println!("{}", decrire(5));
}
"#,
        check: Check::Stdout { expected: "chiffre" },
        hints: &[
            Bi { fr: "Utilise un intervalle inclusif : 1..=9 => \"chiffre\".", en: "Use an inclusive range: 1..=9 => \"chiffre\"." },
        ],
        solution: r#"fn decrire(n: u32) -> &'static str {
    match n {
        0 => "zero",
        1..=9 => "chiffre",
        _ => "grand",
    }
}

fn main() {
    println!("{}", decrire(5));
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 35,
        slug: "if-let",
        title: Bi { fr: "if let & while let", en: "if let & while let" },
        subtitle: Bi { fr: "Filtrer un seul cas, simplement", en: "Match a single case, simply" },
        xp: 180,
        lesson: Bi {
            fr: r#"
## `if let` : ne traiter qu'un seul motif

Quand tu ne t'intéresses qu'à **un** cas d'un `match`, `if let` est plus
concis :

```rust
let config: Option<i32> = Some(5);

// au lieu d'un match complet :
if let Some(v) = config {
    println!("valeur : {v}");
} else {
    println!("aucune valeur");
}
```

`if let Some(v) = config` se lit : « si `config` correspond au motif
`Some(v)`, lie `v` et exécute le bloc ».

## `while let` : boucler tant que le motif correspond

Idéal pour **vider** une structure :

```rust
let mut pile = vec![1, 2, 3];

while let Some(sommet) = pile.pop() {
    println!("{sommet}"); // 3, puis 2, puis 1
}
// la boucle s'arrête quand pop() renvoie None
```

`pile.pop()` retire et renvoie le dernier élément dans un `Some`, ou `None`
quand le vecteur est vide — ce qui termine la boucle.

> 💡 `if let` / `while let` ne sont pas exhaustifs : tu choisis volontairement
> d'ignorer les autres cas. Pratique, mais ne perds pas de vue le `match`
> complet quand tu dois tout traiter.
"#,
            en: r#"
## `if let`: handling a single pattern

When you only care about **one** case of a `match`, `if let` is more concise:

```rust
let config: Option<i32> = Some(5);

// instead of a full match:
if let Some(v) = config {
    println!("value: {v}");
} else {
    println!("no value");
}
```

`if let Some(v) = config` reads: "if `config` matches the pattern `Some(v)`,
bind `v` and run the block".

## `while let`: loop while the pattern matches

Ideal to **drain** a structure:

```rust
let mut stack = vec![1, 2, 3];

while let Some(top) = stack.pop() {
    println!("{top}"); // 3, then 2, then 1
}
// the loop stops when pop() returns None
```

`stack.pop()` removes and returns the last element wrapped in `Some`, or
`None` when the vector is empty — which ends the loop.

> 💡 `if let` / `while let` aren't exhaustive: you deliberately ignore the
> other cases. Handy, but keep the full `match` in mind when you must handle
> everything.
"#,
        },
        task: Bi {
            fr: "Complète la boucle `while let` qui dépile tous les éléments et les additionne. Le programme dépile [1, 2, 3] et doit afficher : 6",
            en: "Complete the `while let` loop that pops all elements and sums them. The program drains [1, 2, 3] and must print: 6",
        },
        starter: r#"fn main() {
    let mut pile = vec![1, 2, 3];
    let mut total = 0;

    // Dépile tant qu'il reste un élément, et ajoute-le à total
    while let Some(x) = /* ... */ {
        total += x;
    }

    println!("{}", total);
}
"#,
        check: Check::Stdout { expected: "6" },
        hints: &[
            Bi { fr: "while let Some(x) = pile.pop() { ... }", en: "while let Some(x) = pile.pop() { ... }" },
        ],
        solution: r#"fn main() {
    let mut pile = vec![1, 2, 3];
    let mut total = 0;

    while let Some(x) = pile.pop() {
        total += x;
    }

    println!("{}", total);
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 36,
        slug: "let-else",
        title: Bi { fr: "let ... else", en: "let ... else" },
        subtitle: Bi { fr: "Extraire ou sortir tôt", en: "Extract or bail early" },
        xp: 180,
        lesson: Bi {
            fr: r#"
## `let ... else` : extraire, sinon abandonner

Souvent, on veut extraire une valeur d'une `Option`/`Result`, et **quitter**
la fonction si elle n'est pas là. `let ... else` exprime exactement ça :

```rust
fn traiter(entree: Option<i32>) -> i32 {
    let Some(v) = entree else {
        return -1; // chemin d'échec : on sort
    };
    // ici, v est disponible pour la suite (chemin du succès)
    v * 10
}
```

- Si `entree` est `Some(v)`, on lie `v` et on **continue**.
- Sinon, le bloc `else` s'exécute. Il **doit** quitter le contexte (`return`,
  `break`, `continue`, `panic!`…).

L'avantage par rapport à `if let` : pas d'imbrication. La variable extraite
reste disponible **après** le `let ... else`, dans le flot principal — le code
ne « part pas en escalier » vers la droite.

```rust
// avec if let, le code utile est imbriqué :
if let Some(v) = entree {
    // tout le reste de la fonction ici, décalé...
}
```

> 💡 `let ... else` est parfait pour les **validations d'entrée** au début d'une
> fonction : « si l'argument n'est pas valide, on sort tout de suite ».
"#,
            en: r#"
## `let ... else`: extract, otherwise bail

Often you want to extract a value from an `Option`/`Result`, and **leave** the
function if it isn't there. `let ... else` expresses exactly that:

```rust
fn process(input: Option<i32>) -> i32 {
    let Some(v) = input else {
        return -1; // failure path: we leave
    };
    // here, v is available for the rest (success path)
    v * 10
}
```

- If `input` is `Some(v)`, we bind `v` and **continue**.
- Otherwise, the `else` block runs. It **must** leave the context (`return`,
  `break`, `continue`, `panic!`…).

The advantage over `if let`: no nesting. The extracted variable stays
available **after** the `let ... else`, in the main flow — the code doesn't
"stair-step" to the right.

```rust
// with if let, the useful code is nested:
if let Some(v) = input {
    // the rest of the function here, indented...
}
```

> 💡 `let ... else` is perfect for **input validation** at the start of a
> function: "if the argument is invalid, leave right away".
"#,
        },
        task: Bi {
            fr: "Complète `extraire` avec let ... else : si l'entrée est None, renvoie -1 ; sinon renvoie la valeur × 10. Le programme teste Some(4) et doit afficher : 40",
            en: "Complete `extraire` with let ... else: if the input is None, return -1; otherwise return the value × 10. The program tests Some(4) and must print: 40",
        },
        starter: r#"fn extraire(entree: Option<i32>) -> i32 {
    // let Some(v) = entree else { return -1; };
    // puis renvoie v * 10
}

fn main() {
    println!("{}", extraire(Some(4)));
}
"#,
        check: Check::Stdout { expected: "40" },
        hints: &[
            Bi { fr: "let Some(v) = entree else { return -1; }; v * 10", en: "let Some(v) = entree else { return -1; }; v * 10" },
        ],
        solution: r#"fn extraire(entree: Option<i32>) -> i32 {
    let Some(v) = entree else {
        return -1;
    };
    v * 10
}

fn main() {
    println!("{}", extraire(Some(4)));
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 37,
        slug: "modules",
        title: Bi { fr: "Les modules", en: "Modules" },
        subtitle: Bi { fr: "Organiser et encapsuler le code", en: "Organize and encapsulate code" },
        xp: 180,
        lesson: Bi {
            fr: r#"
## Organiser son code avec `mod`

Un **module** regroupe des éléments (fonctions, structs…) sous un namespace.
C'est ainsi qu'on structure un programme qui grandit.

```rust
mod geometrie {
    pub fn aire_cercle(r: f64) -> f64 {
        3.14159 * r * r
    }
}

fn main() {
    let a = geometrie::aire_cercle(2.0);
    println!("{a}");
}
```

On accède au contenu avec le chemin `module::element`.

### `pub` : la visibilité

Par défaut, **tout est privé** dans un module : invisible de l'extérieur. Le
mot-clé `pub` rend un élément **public**.

```rust
mod compte {
    pub fn solde() -> i32 { interne() }
    fn interne() -> i32 { 100 } // privé : usage interne au module
}
```

Ici `compte::solde()` est accessible, mais `compte::interne()` non.

### `use` : raccourcir les chemins

```rust
use geometrie::aire_cercle;

let a = aire_cercle(2.0); // plus besoin du préfixe
```

> 💡 L'encapsulation par défaut est volontaire : tu exposes une **API**
> minimale (`pub`) et gardes les détails internes privés, libres d'évoluer.
"#,
            en: r#"
## Organizing code with `mod`

A **module** groups items (functions, structs…) under a namespace. That's how
you structure a growing program.

```rust
mod geometrie {
    pub fn aire_cercle(r: f64) -> f64 {
        3.14159 * r * r
    }
}

fn main() {
    let a = geometrie::aire_cercle(2.0);
    println!("{a}");
}
```

You access items with the `module::item` path.

### `pub`: visibility

By default, **everything is private** in a module: invisible from outside. The
`pub` keyword makes an item **public**.

```rust
mod account {
    pub fn balance() -> i32 { internal() }
    fn internal() -> i32 { 100 } // private: module-internal use
}
```

Here `account::balance()` is accessible, but `account::internal()` isn't.

### `use`: shortening paths

```rust
use geometrie::aire_cercle;

let a = aire_cercle(2.0); // no more prefix needed
```

> 💡 Default encapsulation is intentional: you expose a minimal **API** (`pub`)
> and keep internal details private, free to change.
"#,
        },
        task: Bi {
            fr: "La fonction `carre` est privée, donc `main` ne peut pas l'appeler. Rends-la publique. Le programme calcule carre(6) et doit afficher : 36",
            en: "The `carre` function is private, so `main` can't call it. Make it public. The program computes carre(6) and must print: 36",
        },
        starter: r#"mod maths {
    // Rends cette fonction publique pour pouvoir l'appeler depuis main
    fn carre(x: i32) -> i32 {
        x * x
    }
}

fn main() {
    println!("{}", maths::carre(6));
}
"#,
        check: Check::Stdout { expected: "36" },
        hints: &[
            Bi { fr: "Ajoute le mot-clé pub devant fn carre.", en: "Add the pub keyword before fn carre." },
        ],
        solution: r#"mod maths {
    pub fn carre(x: i32) -> i32 {
        x * x
    }
}

fn main() {
    println!("{}", maths::carre(6));
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 38,
        slug: "from-into",
        title: Bi { fr: "From & Into", en: "From & Into" },
        subtitle: Bi { fr: "Convertir entre types proprement", en: "Convert between types cleanly" },
        xp: 190,
        lesson: Bi {
            fr: r#"
## Convertir avec `From` et `Into`

Le trait `From` définit comment **construire** un type à partir d'un autre.

```rust
struct Distance {
    metres: u32,
}

impl From<u32> for Distance {
    fn from(valeur: u32) -> Distance {
        Distance { metres: valeur }
    }
}
```

On l'utilise ainsi :

```rust
let d = Distance::from(100);
```

### `Into` vient gratuitement

Si tu implémentes `From<A> for B`, alors Rust te donne **automatiquement**
`Into<B> for A`. Tu peux donc écrire :

```rust
let d: Distance = 100u32.into();
```

`into()` a besoin de connaître le type cible — ici fourni par l'annotation
`: Distance`.

### Pourquoi c'est partout

Beaucoup d'API acceptent `impl Into<String>` pour être souples :

```rust
fn afficher(msg: impl Into<String>) {
    let s: String = msg.into();
    println!("{s}");
}
afficher("coucou");                 // &str -> String
afficher(String::from("coucou"));   // déjà une String
```

> 💡 Implémente toujours `From` (pas `Into`) : tu obtiens `Into` en bonus, et
> c'est la convention idiomatique.
"#,
            en: r#"
## Converting with `From` and `Into`

The `From` trait defines how to **build** a type from another.

```rust
struct Distance {
    metres: u32,
}

impl From<u32> for Distance {
    fn from(valeur: u32) -> Distance {
        Distance { metres: valeur }
    }
}
```

You use it like this:

```rust
let d = Distance::from(100);
```

### `Into` comes for free

If you implement `From<A> for B`, Rust gives you **automatically** `Into<B>
for A`. So you can write:

```rust
let d: Distance = 100u32.into();
```

`into()` needs to know the target type — here provided by the `: Distance`
annotation.

### Why it's everywhere

Many APIs accept `impl Into<String>` to be flexible:

```rust
fn afficher(msg: impl Into<String>) {
    let s: String = msg.into();
    println!("{s}");
}
afficher("hi");                 // &str -> String
afficher(String::from("hi"));   // already a String
```

> 💡 Always implement `From` (not `Into`): you get `Into` as a bonus, and it's
> the idiomatic convention.
"#,
        },
        task: Bi {
            fr: "Implémente `From<u32> for Distance` pour construire une Distance à partir d'un u32. Le programme convertit 100 et affiche le champ metres, attendu : 100",
            en: "Implement `From<u32> for Distance` to build a Distance from a u32. The program converts 100 and prints the metres field, expected: 100",
        },
        starter: r#"struct Distance {
    metres: u32,
}

impl From<u32> for Distance {
    // fn from(valeur: u32) -> Distance { ... }
}

fn main() {
    let d: Distance = 100u32.into();
    println!("{}", d.metres);
}
"#,
        check: Check::Stdout { expected: "100" },
        hints: &[
            Bi { fr: "fn from(valeur: u32) -> Distance { Distance { metres: valeur } }", en: "fn from(valeur: u32) -> Distance { Distance { metres: valeur } }" },
        ],
        solution: r#"struct Distance {
    metres: u32,
}

impl From<u32> for Distance {
    fn from(valeur: u32) -> Distance {
        Distance { metres: valeur }
    }
}

fn main() {
    let d: Distance = 100u32.into();
    println!("{}", d.metres);
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 39,
        slug: "display",
        title: Bi { fr: "Le trait Display", en: "The Display trait" },
        subtitle: Bi { fr: "Choisir comment s'affiche un type", en: "Decide how a type prints" },
        xp: 190,
        lesson: Bi {
            fr: r#"
## Afficher joliment avec `Display`

`{:?}` (Debug) affiche pour les développeurs. Pour un affichage **destiné aux
humains** avec `{}`, on implémente le trait `Display`.

```rust
use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
```

Détails :

- On importe `std::fmt`.
- La méthode s'appelle toujours `fmt`, avec cette signature exacte.
- `write!(f, ...)` écrit dans le *formateur* `f`, exactement comme `println!`
  écrit à l'écran. **Ne mets pas de `;`** : `write!` renvoie le `fmt::Result`
  attendu.

Ensuite, `{}` fonctionne :

```rust
let p = Point { x: 2, y: 3 };
println!("{p}"); // (2, 3)
```

> 💡 `Display` ne se **dérive pas** (`#[derive(Display)]` n'existe pas en
> standard) : c'est à toi de décider la représentation lisible de ton type.
"#,
            en: r#"
## Pretty-printing with `Display`

`{:?}` (Debug) prints for developers. For a **human-facing** display with
`{}`, you implement the `Display` trait.

```rust
use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
```

Details:

- Import `std::fmt`.
- The method is always called `fmt`, with this exact signature.
- `write!(f, ...)` writes into the *formatter* `f`, just like `println!` writes
  to the screen. **No `;`**: `write!` returns the expected `fmt::Result`.

Then `{}` works:

```rust
let p = Point { x: 2, y: 3 };
println!("{p}"); // (2, 3)
```

> 💡 `Display` can't be **derived** (`#[derive(Display)]` doesn't exist in std):
> it's up to you to decide your type's readable representation.
"#,
        },
        task: Bi {
            fr: "Complète la méthode `fmt` pour que Point s'affiche \"(x, y)\". Le programme affiche Point { x: 2, y: 3 } avec {} et doit donner : (2, 3)",
            en: "Complete the `fmt` method so Point prints \"(x, y)\". The program prints Point { x: 2, y: 3 } with {} and must give: (2, 3)",
        },
        starter: r#"use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Utilise write!(f, ...) pour écrire "(x, y)"
    }
}

fn main() {
    let p = Point { x: 2, y: 3 };
    println!("{p}");
}
"#,
        check: Check::Stdout { expected: "(2, 3)" },
        hints: &[
            Bi { fr: "write!(f, \"({}, {})\", self.x, self.y)", en: "write!(f, \"({}, {})\", self.x, self.y)" },
        ],
        solution: r#"use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let p = Point { x: 2, y: 3 };
    println!("{p}");
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 40,
        slug: "equality",
        title: Bi { fr: "Égalité : PartialEq", en: "Equality: PartialEq" },
        subtitle: Bi { fr: "Comparer ses propres types avec ==", en: "Compare your own types with ==" },
        xp: 180,
        lesson: Bi {
            fr: r#"
## Comparer avec `==`

Par défaut, tu ne peux pas écrire `a == b` sur tes propres structs : Rust ne
sait pas ce que « égal » signifie pour toi. Le trait **`PartialEq`** définit
cette comparaison, et il se **dérive** très simplement.

```rust
#[derive(PartialEq)]
struct Couleur {
    r: u8,
    g: u8,
    b: u8,
}

let noir = Couleur { r: 0, g: 0, b: 0 };
let autre = Couleur { r: 0, g: 0, b: 0 };

println!("{}", noir == autre); // true
```

La version dérivée compare **champ par champ** : deux valeurs sont égales si
tous leurs champs sont égaux.

### `Eq` : l'égalité totale

`Eq` est une version plus stricte, pour les types où l'égalité est « parfaite »
(ce qui exclut les flottants, à cause de `NaN != NaN`). On la dérive souvent
ensemble avec `PartialEq` quand le type le permet :

```rust
#[derive(PartialEq, Eq)]
struct Id(u32);
```

> 💡 On combine souvent `#[derive(Debug, Clone, PartialEq)]` sur les structs de
> données : afficher, copier, comparer — le trio de base très pratique.
"#,
            en: r#"
## Comparing with `==`

By default, you can't write `a == b` on your own structs: Rust doesn't know
what "equal" means for you. The **`PartialEq`** trait defines that comparison,
and it **derives** very simply.

```rust
#[derive(PartialEq)]
struct Couleur {
    r: u8,
    g: u8,
    b: u8,
}

let black = Couleur { r: 0, g: 0, b: 0 };
let other = Couleur { r: 0, g: 0, b: 0 };

println!("{}", black == other); // true
```

The derived version compares **field by field**: two values are equal if all
their fields are equal.

### `Eq`: total equality

`Eq` is a stricter version, for types where equality is "perfect" (which
excludes floats, because `NaN != NaN`). It's often derived together with
`PartialEq` when the type allows:

```rust
#[derive(PartialEq, Eq)]
struct Id(u32);
```

> 💡 People often combine `#[derive(Debug, Clone, PartialEq)]` on data structs:
> print, copy, compare — the handy base trio.
"#,
        },
        task: Bi {
            fr: "Ajoute l'annotation qui permet de comparer deux `Couleur` avec ==. Le programme compare deux couleurs identiques et doit afficher : true",
            en: "Add the annotation that lets you compare two `Couleur` values with ==. The program compares two identical colors and must print: true",
        },
        starter: r#"// Ajoute l'annotation #[derive(...)] qui active ==
struct Couleur {
    r: u8,
    g: u8,
    b: u8,
}

fn main() {
    let a = Couleur { r: 10, g: 20, b: 30 };
    let b = Couleur { r: 10, g: 20, b: 30 };
    println!("{}", a == b);
}
"#,
        check: Check::Stdout { expected: "true" },
        hints: &[
            Bi { fr: "L'opérateur == nécessite le trait PartialEq. Dérive-le.", en: "The == operator needs the PartialEq trait. Derive it." },
            Bi { fr: "#[derive(PartialEq)] au-dessus de struct Couleur.", en: "#[derive(PartialEq)] above struct Couleur." },
        ],
        solution: r#"#[derive(PartialEq)]
struct Couleur {
    r: u8,
    g: u8,
    b: u8,
}

fn main() {
    let a = Couleur { r: 10, g: 20, b: 30 };
    let b = Couleur { r: 10, g: 20, b: 30 };
    println!("{}", a == b);
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 41,
        slug: "sorting",
        title: Bi { fr: "Trier des collections", en: "Sorting collections" },
        subtitle: Bi { fr: "sort, sort_by, reverse", en: "sort, sort_by, reverse" },
        xp: 190,
        lesson: Bi {
            fr: r#"
## Trier un `Vec`

La méthode `sort()` trie **en place**, par ordre croissant. Elle exige que les
éléments soient ordonnables (trait `Ord`, que les nombres et chaînes ont déjà).

```rust
let mut v = vec![3, 1, 4, 1, 5];
v.sort();
// [1, 1, 3, 4, 5]
```

### Trier dans l'autre sens

```rust
let mut v = vec![3, 1, 4];
v.sort();
v.reverse();
// [4, 3, 1]
```

### `sort_by` : un critère sur mesure

On fournit une closure qui **compare** deux éléments et renvoie un `Ordering` :

```rust
let mut v = vec![3, 1, 4];
v.sort_by(|a, b| b.cmp(a)); // décroissant
```

`a.cmp(b)` compare et renvoie `Less`, `Equal` ou `Greater`. En inversant
(`b.cmp(a)`), on inverse le tri.

### `sort_by_key` : trier selon une valeur calculée

```rust
let mut mots = vec!["bb", "a", "ccc"];
mots.sort_by_key(|s| s.len()); // par longueur : ["a", "bb", "ccc"]
```

> 💡 `sort()` est stable et efficace. Pour de gros volumes non triables par
> `Ord`, `sort_by`/`sort_by_key` couvrent tous les besoins.
"#,
            en: r#"
## Sorting a `Vec`

The `sort()` method sorts **in place**, ascending. It requires elements to be
orderable (the `Ord` trait, which numbers and strings already have).

```rust
let mut v = vec![3, 1, 4, 1, 5];
v.sort();
// [1, 1, 3, 4, 5]
```

### Sorting the other way

```rust
let mut v = vec![3, 1, 4];
v.sort();
v.reverse();
// [4, 3, 1]
```

### `sort_by`: a custom criterion

You give a closure that **compares** two elements and returns an `Ordering`:

```rust
let mut v = vec![3, 1, 4];
v.sort_by(|a, b| b.cmp(a)); // descending
```

`a.cmp(b)` compares and returns `Less`, `Equal` or `Greater`. Reversing
(`b.cmp(a)`) reverses the sort.

### `sort_by_key`: sort by a computed value

```rust
let mut words = vec!["bb", "a", "ccc"];
words.sort_by_key(|s| s.len()); // by length: ["a", "bb", "ccc"]
```

> 💡 `sort()` is stable and efficient. For data not orderable by `Ord`,
> `sort_by`/`sort_by_key` cover every need.
"#,
        },
        task: Bi {
            fr: "Trie le vecteur par ordre DÉCROISSANT (le plus grand d'abord). Le programme part de [3, 1, 4, 1, 5] et doit afficher : [5, 4, 3, 1, 1]",
            en: "Sort the vector in DESCENDING order (largest first). The program starts from [3, 1, 4, 1, 5] and must print: [5, 4, 3, 1, 1]",
        },
        starter: r#"fn main() {
    let mut v = vec![3, 1, 4, 1, 5];
    // Trie v, puis inverse l'ordre (ou utilise sort_by)
    v.sort();
    // ... à toi de finir

    println!("{:?}", v);
}
"#,
        check: Check::Stdout { expected: "[5, 4, 3, 1, 1]" },
        hints: &[
            Bi { fr: "Après v.sort();, appelle v.reverse();", en: "After v.sort();, call v.reverse();" },
            Bi { fr: "Ou en une fois : v.sort_by(|a, b| b.cmp(a));", en: "Or in one go: v.sort_by(|a, b| b.cmp(a));" },
        ],
        solution: r#"fn main() {
    let mut v = vec![3, 1, 4, 1, 5];
    v.sort();
    v.reverse();

    println!("{:?}", v);
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 42,
        slug: "hashset",
        title: Bi { fr: "Les HashSet", en: "HashSets" },
        subtitle: Bi { fr: "Des ensembles sans doublons", en: "Sets without duplicates" },
        xp: 180,
        lesson: Bi {
            fr: r#"
## `HashSet<T>` : un ensemble

Un `HashSet` stocke des valeurs **uniques** : insérer un doublon ne fait rien.
Idéal pour « l'ensemble des choses vues ».

```rust
use std::collections::HashSet;

let mut vus = HashSet::new();
vus.insert(1);
vus.insert(2);
vus.insert(1); // ignoré : déjà présent

println!("{}", vus.len()); // 2
```

- `insert(x)` renvoie `true` si `x` était nouveau, `false` sinon.
- `contains(&x)` teste l'appartenance.

### Dédupliquer en un éclair

On peut construire un `HashSet` directement à partir d'un itérateur :

```rust
let nombres = [1, 2, 2, 3, 3, 3];
let uniques: HashSet<_> = nombres.iter().collect();
println!("{}", uniques.len()); // 3
```

### Opérations ensemblistes

```rust
let a: HashSet<i32> = [1, 2, 3].into_iter().collect();
let b: HashSet<i32> = [2, 3, 4].into_iter().collect();

a.intersection(&b); // {2, 3}
a.union(&b);        // {1, 2, 3, 4}
a.difference(&b);   // {1}
```

> 💡 Comme `HashMap`, l'ordre d'un `HashSet` est imprévisible. Pour un ensemble
> trié, il existe `BTreeSet`.
"#,
            en: r#"
## `HashSet<T>`: a set

A `HashSet` stores **unique** values: inserting a duplicate does nothing.
Perfect for "the set of things seen".

```rust
use std::collections::HashSet;

let mut seen = HashSet::new();
seen.insert(1);
seen.insert(2);
seen.insert(1); // ignored: already present

println!("{}", seen.len()); // 2
```

- `insert(x)` returns `true` if `x` was new, `false` otherwise.
- `contains(&x)` tests membership.

### Deduplicate in a flash

You can build a `HashSet` straight from an iterator:

```rust
let numbers = [1, 2, 2, 3, 3, 3];
let uniques: HashSet<_> = numbers.iter().collect();
println!("{}", uniques.len()); // 3
```

### Set operations

```rust
let a: HashSet<i32> = [1, 2, 3].into_iter().collect();
let b: HashSet<i32> = [2, 3, 4].into_iter().collect();

a.intersection(&b); // {2, 3}
a.union(&b);        // {1, 2, 3, 4}
a.difference(&b);   // {1}
```

> 💡 Like `HashMap`, a `HashSet`'s order is unpredictable. For a sorted set,
> there's `BTreeSet`.
"#,
        },
        task: Bi {
            fr: "Complète `nb_uniques` : elle renvoie le nombre de valeurs DISTINCTES dans la liste, à l'aide d'un HashSet. Le programme teste [1, 2, 2, 3, 3, 3] et doit afficher : 3",
            en: "Complete `nb_uniques`: it returns the number of DISTINCT values in the list, using a HashSet. The program tests [1, 2, 2, 3, 3, 3] and must print: 3",
        },
        starter: r#"use std::collections::HashSet;

fn nb_uniques(nombres: &[i32]) -> usize {
    // Construis un HashSet à partir des nombres, puis renvoie sa taille
}

fn main() {
    println!("{}", nb_uniques(&[1, 2, 2, 3, 3, 3]));
}
"#,
        check: Check::Stdout { expected: "3" },
        hints: &[
            Bi { fr: "let s: HashSet<_> = nombres.iter().collect(); s.len()", en: "let s: HashSet<_> = nombres.iter().collect(); s.len()" },
        ],
        solution: r#"use std::collections::HashSet;

fn nb_uniques(nombres: &[i32]) -> usize {
    let s: HashSet<_> = nombres.iter().collect();
    s.len()
}

fn main() {
    println!("{}", nb_uniques(&[1, 2, 2, 3, 3, 3]));
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 43,
        slug: "btreemap",
        title: Bi { fr: "BTreeMap : clés triées", en: "BTreeMap: sorted keys" },
        subtitle: Bi { fr: "Une map toujours ordonnée", en: "An always-ordered map" },
        xp: 190,
        lesson: Bi {
            fr: r#"
## `BTreeMap<K, V>` : comme HashMap, mais trié

`BTreeMap` associe aussi des clés à des valeurs, mais garde les clés **triées**
en permanence. Le parcours se fait donc dans un ordre **prévisible** (croissant
des clés), contrairement à `HashMap`.

```rust
use std::collections::BTreeMap;

let mut m = BTreeMap::new();
m.insert(3, "trois");
m.insert(1, "un");
m.insert(2, "deux");

for (cle, valeur) in &m {
    println!("{cle}: {valeur}");
}
// 1: un
// 2: deux
// 3: trois   <- toujours dans l'ordre des clés
```

L'API est quasi identique à `HashMap` : `insert`, `get`, `entry`, `contains_key`…

```rust
let cles: Vec<_> = m.keys().collect(); // [1, 2, 3]
```

### Quand le choisir ?

- `HashMap` : le plus rapide, ordre **indifférent**.
- `BTreeMap` : un peu plus lent, mais ordre **trié** garanti — idéal quand tu
  veux parcourir, afficher ou trouver « la plus petite/grande clé ».

> 💡 Il existe le pendant ensembliste `BTreeSet` (un `HashSet` trié).
"#,
            en: r#"
## `BTreeMap<K, V>`: like HashMap, but sorted

`BTreeMap` also maps keys to values, but keeps the keys **sorted** at all
times. Iteration thus happens in a **predictable** order (ascending keys),
unlike `HashMap`.

```rust
use std::collections::BTreeMap;

let mut m = BTreeMap::new();
m.insert(3, "three");
m.insert(1, "one");
m.insert(2, "two");

for (key, value) in &m {
    println!("{key}: {value}");
}
// 1: one
// 2: two
// 3: three   <- always in key order
```

The API is nearly identical to `HashMap`: `insert`, `get`, `entry`,
`contains_key`…

```rust
let keys: Vec<_> = m.keys().collect(); // [1, 2, 3]
```

### When to choose it?

- `HashMap`: fastest, order **doesn't matter**.
- `BTreeMap`: a bit slower, but **sorted** order guaranteed — ideal when you
  want to iterate, print, or find "the smallest/largest key".

> 💡 There's the set counterpart `BTreeSet` (a sorted `HashSet`).
"#,
        },
        task: Bi {
            fr: "Insère les clés dans le désordre, puis affiche la liste des clés (triée automatiquement). Le programme insère 3, 1, 2 et doit afficher : [1, 2, 3]",
            en: "Insert the keys out of order, then print the list of keys (auto-sorted). The program inserts 3, 1, 2 and must print: [1, 2, 3]",
        },
        starter: r#"use std::collections::BTreeMap;

fn main() {
    let mut m: BTreeMap<i32, &str> = BTreeMap::new();
    m.insert(3, "c");
    m.insert(1, "a");
    m.insert(2, "b");

    // Récupère les clés triées dans un Vec et affiche-le
    let cles: Vec<_> = /* ... */;
    println!("{:?}", cles);
}
"#,
        check: Check::Stdout { expected: "[1, 2, 3]" },
        hints: &[
            Bi { fr: "m.keys() renvoie les clés dans l'ordre. Collecte-les : m.keys().collect()", en: "m.keys() yields the keys in order. Collect them: m.keys().collect()" },
        ],
        solution: r#"use std::collections::BTreeMap;

fn main() {
    let mut m: BTreeMap<i32, &str> = BTreeMap::new();
    m.insert(3, "c");
    m.insert(1, "a");
    m.insert(2, "b");

    let cles: Vec<_> = m.keys().collect();
    println!("{:?}", cles);
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 44,
        slug: "box",
        title: Bi { fr: "Box : pointer vers le tas", en: "Box: pointing to the heap" },
        subtitle: Bi { fr: "Et permettre les types récursifs", en: "And enabling recursive types" },
        xp: 200,
        lesson: Bi {
            fr: r#"
## `Box<T>` : une valeur sur le tas

`Box<T>` est le **pointeur intelligent** le plus simple : il stocke une valeur
sur le **tas** et garde un pointeur sur la pile. Quand la `Box` est libérée, la
valeur l'est aussi.

```rust
let b = Box::new(5);
println!("{b}"); // 5 (déréférencement automatique)
```

### Le cas d'usage clé : les types récursifs

Un type ne peut pas se contenir **directement** lui-même — sa taille serait
infinie. `Box` casse la récursion : un pointeur a une taille **fixe et connue**.

```rust
enum Liste {
    // un élément, puis le reste de la liste (derrière une Box)
    Cons(i32, Box<Liste>),
    Nil, // fin de liste
}
```

### Parcourir récursivement

On combine `match` et récursion :

```rust
fn somme(liste: &Liste) -> i32 {
    match liste {
        Liste::Cons(valeur, suite) => valeur + somme(suite),
        Liste::Nil => 0,
    }
}
```

À chaque `Cons`, on additionne la valeur et la somme du **reste** ; `Nil`
arrête la récursion en renvoyant `0`.

> 💡 `Box` sert aussi à stocker un objet-trait (`Box<dyn Trait>`) ou une grosse
> valeur qu'on veut déplacer sans copier — on y revient bientôt.
"#,
            en: r#"
## `Box<T>`: a value on the heap

`Box<T>` is the simplest **smart pointer**: it stores a value on the **heap**
and keeps a pointer on the stack. When the `Box` is freed, so is the value.

```rust
let b = Box::new(5);
println!("{b}"); // 5 (automatic dereference)
```

### The key use case: recursive types

A type can't contain itself **directly** — its size would be infinite. `Box`
breaks the recursion: a pointer has a **fixed, known** size.

```rust
enum Liste {
    // one element, then the rest of the list (behind a Box)
    Cons(i32, Box<Liste>),
    Nil, // end of list
}
```

### Walking recursively

Combine `match` and recursion:

```rust
fn somme(liste: &Liste) -> i32 {
    match liste {
        Liste::Cons(value, rest) => value + somme(rest),
        Liste::Nil => 0,
    }
}
```

At each `Cons`, add the value and the sum of the **rest**; `Nil` stops the
recursion by returning `0`.

> 💡 `Box` also stores a trait object (`Box<dyn Trait>`) or a large value you
> want to move without copying — coming up soon.
"#,
        },
        task: Bi {
            fr: "Complète `somme` qui additionne tous les éléments d'une liste chaînée (Cons/Nil). Le programme somme la liste 1 -> 2 -> 3 et doit afficher : 6",
            en: "Complete `somme` which adds all elements of a linked list (Cons/Nil). The program sums the list 1 -> 2 -> 3 and must print: 6",
        },
        starter: r#"enum Liste {
    Cons(i32, Box<Liste>),
    Nil,
}

fn somme(liste: &Liste) -> i32 {
    match liste {
        // Cons(valeur, suite) => valeur + somme(suite)
        // Nil => 0
    }
}

fn main() {
    let l = Liste::Cons(1, Box::new(Liste::Cons(2, Box::new(Liste::Cons(3, Box::new(Liste::Nil))))));
    println!("{}", somme(&l));
}
"#,
        check: Check::Stdout { expected: "6" },
        hints: &[
            Bi { fr: "Liste::Cons(v, suite) => v + somme(suite), et Liste::Nil => 0", en: "Liste::Cons(v, rest) => v + somme(rest), and Liste::Nil => 0" },
        ],
        solution: r#"enum Liste {
    Cons(i32, Box<Liste>),
    Nil,
}

fn somme(liste: &Liste) -> i32 {
    match liste {
        Liste::Cons(valeur, suite) => valeur + somme(suite),
        Liste::Nil => 0,
    }
}

fn main() {
    let l = Liste::Cons(1, Box::new(Liste::Cons(2, Box::new(Liste::Cons(3, Box::new(Liste::Nil))))));
    println!("{}", somme(&l));
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 45,
        slug: "rc",
        title: Bi { fr: "Rc : propriété partagée", en: "Rc: shared ownership" },
        subtitle: Bi { fr: "Plusieurs propriétaires d'une valeur", en: "Several owners of one value" },
        xp: 200,
        lesson: Bi {
            fr: r#"
## `Rc<T>` : compter les références

La règle « un seul propriétaire » est parfois trop stricte : un nœud de graphe
peut avoir plusieurs « parents ». `Rc<T>` (*reference counted*) autorise
**plusieurs propriétaires** d'une même valeur, en comptant les références.

```rust
use std::rc::Rc;

let a = Rc::new(5);
let b = Rc::clone(&a); // ne copie PAS le 5, incrémente le compteur
let c = Rc::clone(&a);

println!("{}", Rc::strong_count(&a)); // 3
```

- `Rc::new(v)` crée la valeur, compteur = 1.
- `Rc::clone(&a)` crée un **nouveau propriétaire** : compteur +1. C'est
  **bon marché** (on ne duplique pas la donnée, juste le pointeur).
- Quand un `Rc` est libéré, le compteur décroît. À zéro, la valeur est libérée.

`Rc::strong_count(&a)` renvoie le nombre actuel de propriétaires.

### À retenir

- `Rc` est pour un usage **mono-thread** uniquement (sa version thread-safe est
  `Arc`, qu'on verra avec la concurrence).
- `Rc` donne un accès **partagé en lecture**. Pour muter la valeur partagée, on
  le combine avec `RefCell` (niveau suivant).

> 💡 On écrit `Rc::clone(&a)` plutôt que `a.clone()` : c'est un signal visuel
> qu'on duplique juste un pointeur, pas la donnée entière.
"#,
            en: r#"
## `Rc<T>`: counting references

The "single owner" rule is sometimes too strict: a graph node may have several
"parents". `Rc<T>` (*reference counted*) allows **multiple owners** of one
value, by counting references.

```rust
use std::rc::Rc;

let a = Rc::new(5);
let b = Rc::clone(&a); // does NOT copy the 5, increments the counter
let c = Rc::clone(&a);

println!("{}", Rc::strong_count(&a)); // 3
```

- `Rc::new(v)` creates the value, count = 1.
- `Rc::clone(&a)` creates a **new owner**: count +1. It's **cheap** (the data
  isn't duplicated, just the pointer).
- When an `Rc` is freed, the count drops. At zero, the value is freed.

`Rc::strong_count(&a)` returns the current number of owners.

### Remember

- `Rc` is **single-threaded** only (its thread-safe version is `Arc`, seen with
  concurrency).
- `Rc` gives **shared read** access. To mutate the shared value, combine it
  with `RefCell` (next level).

> 💡 We write `Rc::clone(&a)` rather than `a.clone()`: a visual cue that we're
> just duplicating a pointer, not the whole data.
"#,
        },
        task: Bi {
            fr: "Clone l'Rc deux fois pour qu'il y ait 3 propriétaires, puis affiche le compteur. Le programme doit afficher : 3",
            en: "Clone the Rc twice so there are 3 owners, then print the count. The program must print: 3",
        },
        starter: r#"use std::rc::Rc;

fn main() {
    let a = Rc::new(5);
    // Crée deux clones supplémentaires (b et c) avec Rc::clone
    let b = /* ... */;
    let c = /* ... */;

    // (on garde b et c en vie jusqu'ici)
    let _ = (&b, &c);
    println!("{}", Rc::strong_count(&a));
}
"#,
        check: Check::Stdout { expected: "3" },
        hints: &[
            Bi { fr: "let b = Rc::clone(&a); let c = Rc::clone(&a);", en: "let b = Rc::clone(&a); let c = Rc::clone(&a);" },
        ],
        solution: r#"use std::rc::Rc;

fn main() {
    let a = Rc::new(5);
    let b = Rc::clone(&a);
    let c = Rc::clone(&a);

    let _ = (&b, &c);
    println!("{}", Rc::strong_count(&a));
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 46,
        slug: "refcell",
        title: Bi { fr: "RefCell : mutabilité intérieure", en: "RefCell: interior mutability" },
        subtitle: Bi { fr: "Muter derrière une référence partagée", en: "Mutate behind a shared reference" },
        xp: 210,
        lesson: Bi {
            fr: r#"
## `RefCell<T>` : la mutabilité intérieure

D'habitude, pour modifier une valeur, il faut un accès `&mut`. `RefCell<T>`
permet de **muter une valeur même via une référence partagée** `&`, en
déplaçant la vérification des règles d'emprunt à **l'exécution** (au lieu de la
compilation).

```rust
use std::cell::RefCell;

let cellule = RefCell::new(0);

*cellule.borrow_mut() += 5; // emprunt mutable temporaire
*cellule.borrow_mut() += 5;

println!("{}", cellule.borrow()); // 10
```

- `borrow_mut()` donne un accès **mutable** temporaire.
- `borrow()` donne un accès **partagé** (lecture).
- `*` déréférence pour atteindre la valeur contenue.

### Attention : les règles sont vérifiées à l'exécution

`RefCell` applique toujours « plusieurs lecteurs OU un seul écrivain », mais le
viol provoque une **panique au runtime** au lieu d'une erreur de compilation :

```rust
let c = RefCell::new(1);
let a = c.borrow_mut();
let b = c.borrow_mut(); // 💥 panique : already borrowed
```

### L'usage classique : `Rc<RefCell<T>>`

Combiner `Rc` (plusieurs propriétaires) et `RefCell` (mutation) donne une
valeur **partagée et modifiable** — fréquent pour les graphes ou les arbres.

> 💡 N'utilise `RefCell` que lorsque le vérificateur d'emprunt statique est trop
> strict pour ton cas. Sinon, préfère un vrai `&mut`.
"#,
            en: r#"
## `RefCell<T>`: interior mutability

Usually, to modify a value you need `&mut` access. `RefCell<T>` lets you
**mutate a value even through a shared reference** `&`, by moving the borrow-
rule checks to **runtime** (instead of compile time).

```rust
use std::cell::RefCell;

let cell = RefCell::new(0);

*cell.borrow_mut() += 5; // temporary mutable borrow
*cell.borrow_mut() += 5;

println!("{}", cell.borrow()); // 10
```

- `borrow_mut()` gives temporary **mutable** access.
- `borrow()` gives **shared** (read) access.
- `*` dereferences to reach the contained value.

### Careful: rules are checked at runtime

`RefCell` still enforces "many readers OR one writer", but a violation causes a
**runtime panic** instead of a compile error:

```rust
let c = RefCell::new(1);
let a = c.borrow_mut();
let b = c.borrow_mut(); // 💥 panic: already borrowed
```

### The classic combo: `Rc<RefCell<T>>`

Combining `Rc` (multiple owners) and `RefCell` (mutation) gives a **shared,
mutable** value — common for graphs or trees.

> 💡 Use `RefCell` only when the static borrow checker is too strict for your
> case. Otherwise prefer a real `&mut`.
"#,
        },
        task: Bi {
            fr: "Ajoute 5 deux fois à la valeur de la RefCell via borrow_mut, puis affiche-la. Le programme part de 0 et doit afficher : 10",
            en: "Add 5 twice to the RefCell's value via borrow_mut, then print it. The program starts at 0 and must print: 10",
        },
        starter: r#"use std::cell::RefCell;

fn main() {
    let cellule = RefCell::new(0);

    // Ajoute 5, deux fois, en passant par borrow_mut()
    // *cellule.borrow_mut() += 5;

    println!("{}", cellule.borrow());
}
"#,
        check: Check::Stdout { expected: "10" },
        hints: &[
            Bi { fr: "Répète : *cellule.borrow_mut() += 5;", en: "Repeat: *cellule.borrow_mut() += 5;" },
        ],
        solution: r#"use std::cell::RefCell;

fn main() {
    let cellule = RefCell::new(0);

    *cellule.borrow_mut() += 5;
    *cellule.borrow_mut() += 5;

    println!("{}", cellule.borrow());
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 47,
        slug: "dyn-trait",
        title: Bi { fr: "Objets-traits (dyn)", en: "Trait objects (dyn)" },
        subtitle: Bi { fr: "Mélanger plusieurs types derrière un trait", en: "Mix several types behind a trait" },
        xp: 210,
        lesson: Bi {
            fr: r#"
## Les objets-traits : `Box<dyn Trait>`

Comment mettre dans **un même `Vec`** un `Chien` et un `Chat`, deux types
différents qui partagent un trait `Animal` ? Avec un **objet-trait** : on
manipule les valeurs par leur **comportement** (le trait), pas par leur type
concret.

```rust
trait Animal {
    fn cri(&self) -> String;
}

struct Chien;
struct Chat;

impl Animal for Chien {
    fn cri(&self) -> String { String::from("Wouf") }
}
impl Animal for Chat {
    fn cri(&self) -> String { String::from("Miaou") }
}
```

On les range derrière `Box<dyn Animal>` (« un Animal, peu importe lequel ») :

```rust
let animaux: Vec<Box<dyn Animal>> = vec![
    Box::new(Chien),
    Box::new(Chat),
];

for a in &animaux {
    println!("{}", a.cri());
}
// Wouf
// Miaou
```

### Statique vs dynamique

- **Générique** `<T: Animal>` : le type est connu à la compilation
  (*dispatch statique*, rapide, monomorphisé).
- **`dyn Animal`** : le type est choisi à l'exécution (*dispatch dynamique*),
  un peu plus coûteux, mais permet de mélanger des types hétérogènes.

> 💡 `dyn` t'autorise des collections **hétérogènes** : une liste d'objets de
> types variés tant qu'ils partagent le même trait.
"#,
            en: r#"
## Trait objects: `Box<dyn Trait>`

How do you put a `Chien` and a `Chat` — two different types sharing an
`Animal` trait — in **one `Vec`**? With a **trait object**: you manipulate the
values by their **behavior** (the trait), not their concrete type.

```rust
trait Animal {
    fn cri(&self) -> String;
}

struct Chien;
struct Chat;

impl Animal for Chien {
    fn cri(&self) -> String { String::from("Wouf") }
}
impl Animal for Chat {
    fn cri(&self) -> String { String::from("Miaou") }
}
```

Store them behind `Box<dyn Animal>` ("an Animal, whichever one"):

```rust
let animaux: Vec<Box<dyn Animal>> = vec![
    Box::new(Chien),
    Box::new(Chat),
];

for a in &animaux {
    println!("{}", a.cri());
}
// Wouf
// Miaou
```

### Static vs dynamic

- **Generic** `<T: Animal>`: the type is known at compile time (*static
  dispatch*, fast, monomorphized).
- **`dyn Animal`**: the type is chosen at runtime (*dynamic dispatch*), a bit
  costlier, but lets you mix heterogeneous types.

> 💡 `dyn` enables **heterogeneous** collections: a list of objects of varied
> types as long as they share the same trait.
"#,
        },
        task: Bi {
            fr: "Construis le vecteur d'animaux contenant un Chien puis un Chat, derrière Box<dyn Animal>. Sortie attendue (deux lignes) : Wouf / Miaou",
            en: "Build the vector of animals containing a Chien then a Chat, behind Box<dyn Animal>. Expected output (two lines): Wouf / Miaou",
        },
        starter: r#"trait Animal {
    fn cri(&self) -> String;
}

struct Chien;
struct Chat;

impl Animal for Chien {
    fn cri(&self) -> String { String::from("Wouf") }
}
impl Animal for Chat {
    fn cri(&self) -> String { String::from("Miaou") }
}

fn main() {
    // Déclare un Vec<Box<dyn Animal>> contenant un Chien puis un Chat
    let animaux: Vec<Box<dyn Animal>> = /* ... */;

    for a in &animaux {
        println!("{}", a.cri());
    }
}
"#,
        check: Check::Stdout { expected: "Wouf\nMiaou" },
        hints: &[
            Bi { fr: "vec![Box::new(Chien), Box::new(Chat)]", en: "vec![Box::new(Chien), Box::new(Chat)]" },
        ],
        solution: r#"trait Animal {
    fn cri(&self) -> String;
}

struct Chien;
struct Chat;

impl Animal for Chien {
    fn cri(&self) -> String { String::from("Wouf") }
}
impl Animal for Chat {
    fn cri(&self) -> String { String::from("Miaou") }
}

fn main() {
    let animaux: Vec<Box<dyn Animal>> = vec![Box::new(Chien), Box::new(Chat)];

    for a in &animaux {
        println!("{}", a.cri());
    }
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 48,
        slug: "lifetimes",
        title: Bi { fr: "Les durées de vie", en: "Lifetimes" },
        subtitle: Bi { fr: "Garantir que les références restent valides", en: "Guarantee references stay valid" },
        xp: 220,
        lesson: Bi {
            fr: r#"
## Les *lifetimes* (durées de vie)

Une référence ne doit jamais survivre à la valeur qu'elle pointe. Le plus
souvent, Rust le vérifie tout seul. Mais parfois, il a besoin que tu **relies**
explicitement les durées de vie de plusieurs références. C'est le rôle des
annotations `'a`.

### Le problème

Cette fonction ne compile pas : Rust ignore si le résultat vit aussi longtemps
que `a` ou que `b`.

```rust
fn plus_long(a: &str, b: &str) -> &str { // ❌
    if a.len() > b.len() { a } else { b }
}
```

### La solution : une annotation de durée de vie

```rust
fn plus_long<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}
```

- `<'a>` déclare un paramètre de durée de vie (un nom, comme un générique).
- `&'a str` signifie « une référence valide au moins pendant `'a` ».
- La signature promet : « le résultat vit aussi longtemps que la **plus courte**
  des durées de vie de `a` et `b` ».

Ça **ne change pas** l'exécution : c'est purement une information pour que le
compilateur garantisse la sûreté. Aucune référence pendante (*dangling*) ne peut
exister.

> 💡 Tu n'écris des lifetimes que quand le compilateur te le demande. Quand il
> le fait, pense : « quelle entrée le résultat emprunte-t-il ? » et relie-les
> avec le même `'a`.
"#,
            en: r#"
## Lifetimes

A reference must never outlive the value it points to. Most of the time, Rust
checks this on its own. But sometimes it needs you to explicitly **relate** the
lifetimes of several references. That's what `'a` annotations do.

### The problem

This function doesn't compile: Rust can't tell whether the result lives as long
as `a` or as `b`.

```rust
fn plus_long(a: &str, b: &str) -> &str { // ❌
    if a.len() > b.len() { a } else { b }
}
```

### The fix: a lifetime annotation

```rust
fn plus_long<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}
```

- `<'a>` declares a lifetime parameter (a name, like a generic).
- `&'a str` means "a reference valid for at least `'a`".
- The signature promises: "the result lives as long as the **shorter** of `a`
  and `b`'s lifetimes".

It **doesn't change** execution: it's purely information so the compiler can
guarantee safety. No dangling reference can exist.

> 💡 You only write lifetimes when the compiler asks. When it does, think:
> "which input does the result borrow from?" and relate them with the same `'a`.
"#,
        },
        task: Bi {
            fr: "Ajoute les annotations de durée de vie à `plus_long` pour qu'elle compile. Le programme teste \"court\" et \"plus long\" et doit afficher : plus long",
            en: "Add the lifetime annotations to `plus_long` so it compiles. The program tests \"court\" and \"plus long\" and must print: plus long",
        },
        starter: r#"// Ajoute le paramètre de durée de vie 'a là où il faut
fn plus_long(a: &str, b: &str) -> &str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

fn main() {
    println!("{}", plus_long("court", "plus long"));
}
"#,
        check: Check::Stdout { expected: "plus long" },
        hints: &[
            Bi { fr: "Déclare <'a> après le nom, et annote chaque &str en &'a str (paramètres ET retour).", en: "Declare <'a> after the name, and annotate each &str as &'a str (params AND return)." },
            Bi { fr: "fn plus_long<'a>(a: &'a str, b: &'a str) -> &'a str", en: "fn plus_long<'a>(a: &'a str, b: &'a str) -> &'a str" },
        ],
        solution: r#"fn plus_long<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

fn main() {
    println!("{}", plus_long("court", "plus long"));
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 49,
        slug: "lifetimes-struct",
        title: Bi { fr: "Durées de vie dans les structs", en: "Lifetimes in structs" },
        subtitle: Bi { fr: "Quand une struct contient une référence", en: "When a struct holds a reference" },
        xp: 220,
        lesson: Bi {
            fr: r#"
## Une `struct` qui contient une référence

Si une structure stocke une **référence** (et non une valeur possédée), elle a
besoin d'une annotation de durée de vie. Cela garantit que la struct ne peut pas
survivre à la donnée qu'elle référence.

```rust
struct Extrait<'a> {
    texte: &'a str,
}
```

`<'a>` après le nom du type déclare la durée de vie ; `texte: &'a str` dit « ce
champ emprunte une chaîne qui vit au moins `'a` ».

### Utilisation

```rust
fn main() {
    let phrase = String::from("Rust est genial");
    let e = Extrait { texte: &phrase };
    println!("{}", e.texte);
} // `phrase` et `e` meurent ici, dans le bon ordre
```

Le compilateur s'assure que `phrase` vit **au moins aussi longtemps** que `e`.
Si tu essayais de garder `e` après la disparition de `phrase`, il refuserait.

### Méthodes

Le bloc `impl` reprend la durée de vie :

```rust
impl<'a> Extrait<'a> {
    fn contenu(&self) -> &str {
        self.texte
    }
}
```

> 💡 Préfère stocker des valeurs **possédées** (`String`) dans tes structs quand
> tu débutes : tu n'as alors aucune lifetime à gérer. Les références dans les
> structs sont une optimisation pour cas avancés.
"#,
            en: r#"
## A `struct` holding a reference

If a struct stores a **reference** (not an owned value), it needs a lifetime
annotation. This guarantees the struct can't outlive the data it references.

```rust
struct Extrait<'a> {
    texte: &'a str,
}
```

`<'a>` after the type name declares the lifetime; `texte: &'a str` says "this
field borrows a string living at least `'a`".

### Usage

```rust
fn main() {
    let phrase = String::from("Rust est genial");
    let e = Extrait { texte: &phrase };
    println!("{}", e.texte);
} // `phrase` and `e` die here, in the right order
```

The compiler ensures `phrase` lives **at least as long** as `e`. If you tried
to keep `e` after `phrase` is gone, it would refuse.

### Methods

The `impl` block carries the lifetime:

```rust
impl<'a> Extrait<'a> {
    fn contenu(&self) -> &str {
        self.texte
    }
}
```

> 💡 As a beginner, prefer storing **owned** values (`String`) in your structs:
> then you have no lifetimes to manage. References in structs are an
> optimization for advanced cases.
"#,
        },
        task: Bi {
            fr: "Ajoute le paramètre de durée de vie à la struct `Citation` pour qu'elle puisse contenir une référence. Le programme affiche le texte et doit donner : Rust est genial",
            en: "Add the lifetime parameter to the `Citation` struct so it can hold a reference. The program prints the text and must give: Rust est genial",
        },
        starter: r#"// Ajoute <'a> et annote le champ texte
struct Citation {
    texte: &str,
}

fn main() {
    let phrase = String::from("Rust est genial");
    let c = Citation { texte: &phrase };
    println!("{}", c.texte);
}
"#,
        check: Check::Stdout { expected: "Rust est genial" },
        hints: &[
            Bi { fr: "struct Citation<'a> { texte: &'a str }", en: "struct Citation<'a> { texte: &'a str }" },
        ],
        solution: r#"struct Citation<'a> {
    texte: &'a str,
}

fn main() {
    let phrase = String::from("Rust est genial");
    let c = Citation { texte: &phrase };
    println!("{}", c.texte);
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 50,
        slug: "custom-iterator",
        title: Bi { fr: "Créer un itérateur", en: "Build an iterator" },
        subtitle: Bi { fr: "Implémenter le trait Iterator", en: "Implement the Iterator trait" },
        xp: 220,
        lesson: Bi {
            fr: r#"
## Implémenter `Iterator`

Tu as beaucoup **utilisé** les itérateurs ; voyons comment en **créer** un. Il
suffit d'implémenter le trait `Iterator`, qui ne demande qu'**une** méthode :
`next`.

```rust
struct Compteur {
    actuel: u32,
}

impl Compteur {
    fn new() -> Compteur {
        Compteur { actuel: 0 }
    }
}

impl Iterator for Compteur {
    type Item = u32; // le type des valeurs produites

    fn next(&mut self) -> Option<u32> {
        if self.actuel < 3 {
            self.actuel += 1;
            Some(self.actuel) // produit la valeur suivante
        } else {
            None // None = fin de l'itération
        }
    }
}
```

Points clés :

- `type Item = u32;` est un **type associé** : ce que l'itérateur produit.
- `next` renvoie `Some(valeur)` à chaque étape, puis `None` pour s'arrêter.

### Le bonus : tous les adaptateurs gratuits

Dès que `next` est défini, tu hérites de **toute** la boîte à outils
(`map`, `filter`, `sum`, `collect`…) via les méthodes par défaut du trait :

```rust
let v: Vec<u32> = Compteur::new().collect(); // [1, 2, 3]
let total: u32 = Compteur::new().sum();       // 6
```

> 💡 C'est l'exemple parfait des **méthodes par défaut** : tu écris `next`, et la
> bibliothèque standard te donne des dizaines de méthodes par-dessus.
"#,
            en: r#"
## Implementing `Iterator`

You've **used** iterators a lot; let's **create** one. Just implement the
`Iterator` trait, which requires only **one** method: `next`.

```rust
struct Compteur {
    actuel: u32,
}

impl Compteur {
    fn new() -> Compteur {
        Compteur { actuel: 0 }
    }
}

impl Iterator for Compteur {
    type Item = u32; // the type of produced values

    fn next(&mut self) -> Option<u32> {
        if self.actuel < 3 {
            self.actuel += 1;
            Some(self.actuel) // yield the next value
        } else {
            None // None = end of iteration
        }
    }
}
```

Key points:

- `type Item = u32;` is an **associated type**: what the iterator produces.
- `next` returns `Some(value)` each step, then `None` to stop.

### The bonus: all adaptors for free

Once `next` is defined, you inherit the **whole** toolbox (`map`, `filter`,
`sum`, `collect`…) via the trait's default methods:

```rust
let v: Vec<u32> = Compteur::new().collect(); // [1, 2, 3]
let total: u32 = Compteur::new().sum();       // 6
```

> 💡 It's the perfect example of **default methods**: you write `next`, and the
> standard library gives you dozens of methods on top.
"#,
        },
        task: Bi {
            fr: "Complète la méthode `next` : le compteur produit 1, 2, 3 puis s'arrête. Le programme collecte les valeurs et doit afficher : [1, 2, 3]",
            en: "Complete the `next` method: the counter yields 1, 2, 3 then stops. The program collects the values and must print: [1, 2, 3]",
        },
        starter: r#"struct Compteur {
    actuel: u32,
}

impl Compteur {
    fn new() -> Compteur {
        Compteur { actuel: 0 }
    }
}

impl Iterator for Compteur {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        // Tant que actuel < 3 : incrémente et renvoie Some(actuel)
        // Sinon : renvoie None
    }
}

fn main() {
    let v: Vec<u32> = Compteur::new().collect();
    println!("{:?}", v);
}
"#,
        check: Check::Stdout { expected: "[1, 2, 3]" },
        hints: &[
            Bi { fr: "if self.actuel < 3 { self.actuel += 1; Some(self.actuel) } else { None }", en: "if self.actuel < 3 { self.actuel += 1; Some(self.actuel) } else { None }" },
        ],
        solution: r#"struct Compteur {
    actuel: u32,
}

impl Compteur {
    fn new() -> Compteur {
        Compteur { actuel: 0 }
    }
}

impl Iterator for Compteur {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.actuel < 3 {
            self.actuel += 1;
            Some(self.actuel)
        } else {
            None
        }
    }
}

fn main() {
    let v: Vec<u32> = Compteur::new().collect();
    println!("{:?}", v);
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 51,
        slug: "threads",
        title: Bi { fr: "Les threads", en: "Threads" },
        subtitle: Bi { fr: "Exécuter du code en parallèle", en: "Run code in parallel" },
        xp: 220,
        lesson: Bi {
            fr: r#"
## Lancer un thread

Un **thread** exécute du code « en même temps » que le reste du programme. On
en crée un avec `thread::spawn`, en lui passant une closure :

```rust
use std::thread;

let handle = thread::spawn(|| {
    // ce bloc s'exécute dans un nouveau thread
    let mut total = 0;
    for i in 1..=100 {
        total += i;
    }
    total // valeur renvoyée par le thread
});

let resultat = handle.join().unwrap(); // attend la fin et récupère la valeur
println!("{resultat}"); // 5050
```

- `spawn` renvoie un **`JoinHandle`**.
- `handle.join()` **attend** que le thread se termine et renvoie un `Result`
  contenant sa valeur de retour.

### `move` pour capturer des données

Si la closure utilise des variables de l'extérieur, il faut souvent `move` pour
en **transférer la propriété** au thread :

```rust
let donnees = vec![1, 2, 3];
let h = thread::spawn(move || donnees.len());
println!("{}", h.join().unwrap()); // 3
```

> 💡 Le système d'ownership de Rust empêche, **à la compilation**, des bugs
> classiques de concurrence (data races). On exploite ça au niveau suivant avec
> `Arc` et `Mutex`.
"#,
            en: r#"
## Spawning a thread

A **thread** runs code "at the same time" as the rest of the program. You
create one with `thread::spawn`, passing it a closure:

```rust
use std::thread;

let handle = thread::spawn(|| {
    // this block runs in a new thread
    let mut total = 0;
    for i in 1..=100 {
        total += i;
    }
    total // value returned by the thread
});

let result = handle.join().unwrap(); // wait for the end and get the value
println!("{result}"); // 5050
```

- `spawn` returns a **`JoinHandle`**.
- `handle.join()` **waits** for the thread to finish and returns a `Result`
  holding its return value.

### `move` to capture data

If the closure uses outside variables, you often need `move` to **transfer
ownership** to the thread:

```rust
let data = vec![1, 2, 3];
let h = thread::spawn(move || data.len());
println!("{}", h.join().unwrap()); // 3
```

> 💡 Rust's ownership system prevents, **at compile time**, classic concurrency
> bugs (data races). We leverage that next with `Arc` and `Mutex`.
"#,
        },
        task: Bi {
            fr: "Lance un thread qui calcule la somme de 1 à 100, puis attends-le et affiche le résultat. Attendu : 5050",
            en: "Spawn a thread computing the sum from 1 to 100, then join it and print the result. Expected: 5050",
        },
        starter: r#"use std::thread;

fn main() {
    // Crée un thread qui renvoie la somme de 1..=100
    let handle = thread::spawn(|| {
        // calcule et renvoie la somme
    });

    let resultat = handle.join().unwrap();
    println!("{}", resultat);
}
"#,
        check: Check::Stdout { expected: "5050" },
        hints: &[
            Bi { fr: "Dans la closure : (1..=100).sum::<i32>()", en: "In the closure: (1..=100).sum::<i32>()" },
        ],
        solution: r#"use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        (1..=100).sum::<i32>()
    });

    let resultat = handle.join().unwrap();
    println!("{}", resultat);
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 52,
        slug: "arc-mutex",
        title: Bi { fr: "Arc & Mutex", en: "Arc & Mutex" },
        subtitle: Bi { fr: "Partager un état entre threads", en: "Share state across threads" },
        xp: 240,
        lesson: Bi {
            fr: r#"
## Partager des données mutables entre threads

Pour qu'un état soit **partagé** par plusieurs threads et **modifiable** en
sûreté, Rust combine deux outils :

- **`Arc<T>`** (*Atomic Rc*) : comme `Rc`, mais utilisable entre threads. Il
  permet **plusieurs propriétaires** de la même valeur.
- **`Mutex<T>`** : un verrou. Pour accéder à la valeur, un thread doit
  **verrouiller** le mutex ; un seul thread à la fois y a accès.

```rust
use std::sync::{Arc, Mutex};
use std::thread;

let compteur = Arc::new(Mutex::new(0));
let mut handles = vec![];

for _ in 0..10 {
    let c = Arc::clone(&compteur); // un propriétaire de plus pour ce thread
    let h = thread::spawn(move || {
        let mut n = c.lock().unwrap(); // verrouille
        *n += 1;                       // modifie en exclusivité
    }); // le verrou est relâché ici, à la fin du bloc
    handles.push(h);
}

for h in handles {
    h.join().unwrap();
}

println!("{}", *compteur.lock().unwrap()); // 10
```

- `c.lock().unwrap()` renvoie un **garde** qui donne accès `&mut` à la valeur.
- Le verrou se **libère automatiquement** quand le garde sort de portée.

> 💡 `Arc<Mutex<T>>` est le motif standard pour « plusieurs threads qui
> modifient le même état ». `Arc` pour le partage, `Mutex` pour l'exclusion.
"#,
            en: r#"
## Sharing mutable data across threads

For state to be **shared** by several threads and safely **mutable**, Rust
combines two tools:

- **`Arc<T>`** (*Atomic Rc*): like `Rc`, but usable across threads. It allows
  **multiple owners** of the same value.
- **`Mutex<T>`**: a lock. To access the value, a thread must **lock** the
  mutex; only one thread at a time has access.

```rust
use std::sync::{Arc, Mutex};
use std::thread;

let compteur = Arc::new(Mutex::new(0));
let mut handles = vec![];

for _ in 0..10 {
    let c = Arc::clone(&compteur); // one more owner for this thread
    let h = thread::spawn(move || {
        let mut n = c.lock().unwrap(); // lock
        *n += 1;                       // mutate exclusively
    }); // the lock is released here, at the end of the block
    handles.push(h);
}

for h in handles {
    h.join().unwrap();
}

println!("{}", *compteur.lock().unwrap()); // 10
```

- `c.lock().unwrap()` returns a **guard** giving `&mut` access to the value.
- The lock is **released automatically** when the guard goes out of scope.

> 💡 `Arc<Mutex<T>>` is the standard pattern for "several threads mutating the
> same state". `Arc` for sharing, `Mutex` for exclusion.
"#,
        },
        task: Bi {
            fr: "Dans chaque thread, verrouille le mutex et ajoute 1 au compteur. 10 threads s'exécutent ; le programme doit afficher : 10",
            en: "In each thread, lock the mutex and add 1 to the counter. 10 threads run; the program must print: 10",
        },
        starter: r#"use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let compteur = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let c = Arc::clone(&compteur);
        let h = thread::spawn(move || {
            // Verrouille c et ajoute 1 à la valeur
        });
        handles.push(h);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("{}", *compteur.lock().unwrap());
}
"#,
        check: Check::Stdout { expected: "10" },
        hints: &[
            Bi { fr: "let mut n = c.lock().unwrap(); *n += 1;", en: "let mut n = c.lock().unwrap(); *n += 1;" },
        ],
        solution: r#"use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let compteur = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let c = Arc::clone(&compteur);
        let h = thread::spawn(move || {
            let mut n = c.lock().unwrap();
            *n += 1;
        });
        handles.push(h);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("{}", *compteur.lock().unwrap());
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 53,
        slug: "channels",
        title: Bi { fr: "Les canaux (channels)", en: "Channels" },
        subtitle: Bi { fr: "Communiquer entre threads par messages", en: "Communicate between threads via messages" },
        xp: 230,
        lesson: Bi {
            fr: r#"
## Communiquer par messages

Plutôt que de partager de la mémoire, Rust encourage souvent à **communiquer
par messages** via un **canal** (*channel*). Un canal a deux extrémités :

- `tx` (*transmitter*) : pour **envoyer**.
- `rx` (*receiver*) : pour **recevoir**.

```rust
use std::sync::mpsc;
use std::thread;

let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    for i in 1..=3 {
        tx.send(i).unwrap(); // envoie 1, 2, 3
    }
    // tx est libéré ici : le canal se ferme
});

let total: i32 = rx.iter().sum(); // reçoit jusqu'à la fermeture
println!("{total}"); // 6
```

- `mpsc` = *multi-producer, single-consumer* : plusieurs émetteurs possibles,
  un seul récepteur.
- `tx.send(v)` envoie une valeur.
- `rx.iter()` produit les valeurs reçues, et **s'arrête** quand tous les `tx`
  ont été libérés (le canal est fermé).
- `rx.recv()` reçoit une seule valeur (et **bloque** en attendant).

> 💡 « Ne communiquez pas en partageant la mémoire ; partagez la mémoire en
> communiquant. » Les canaux évitent bien des problèmes de verrous.
"#,
            en: r#"
## Communicating via messages

Rather than sharing memory, Rust often encourages **communicating by
messages** through a **channel**. A channel has two ends:

- `tx` (*transmitter*): to **send**.
- `rx` (*receiver*): to **receive**.

```rust
use std::sync::mpsc;
use std::thread;

let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    for i in 1..=3 {
        tx.send(i).unwrap(); // sends 1, 2, 3
    }
    // tx is dropped here: the channel closes
});

let total: i32 = rx.iter().sum(); // receives until closed
println!("{total}"); // 6
```

- `mpsc` = *multi-producer, single-consumer*: several senders possible, one
  receiver.
- `tx.send(v)` sends a value.
- `rx.iter()` yields received values, and **stops** when all `tx` have been
  dropped (the channel is closed).
- `rx.recv()` receives a single value (and **blocks** while waiting).

> 💡 "Do not communicate by sharing memory; share memory by communicating."
> Channels avoid many lock-related problems.
"#,
        },
        task: Bi {
            fr: "Dans le thread, envoie les valeurs 1, 2 et 3 dans le canal. Le thread principal les additionne ; le programme doit afficher : 6",
            en: "In the thread, send the values 1, 2 and 3 into the channel. The main thread sums them; the program must print: 6",
        },
        starter: r#"use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        // Envoie 1, 2 puis 3 avec tx.send(...)
        for i in 1..=3 {
            // ...
        }
    });

    let total: i32 = rx.iter().sum();
    println!("{}", total);
}
"#,
        check: Check::Stdout { expected: "6" },
        hints: &[
            Bi { fr: "Dans la boucle : tx.send(i).unwrap();", en: "In the loop: tx.send(i).unwrap();" },
        ],
        solution: r#"use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        for i in 1..=3 {
            tx.send(i).unwrap();
        }
    });

    let total: i32 = rx.iter().sum();
    println!("{}", total);
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 54,
        slug: "parse",
        title: Bi { fr: "Convertir du texte en nombre", en: "Parsing text into numbers" },
        subtitle: Bi { fr: "La méthode parse", en: "The parse method" },
        xp: 200,
        lesson: Bi {
            fr: r#"
## `parse` : du texte vers un nombre

Pour transformer une chaîne en nombre, on utilise `parse`. Comme ça peut
échouer (« abc » n'est pas un nombre), `parse` renvoie un **`Result`**.

```rust
let n: i32 = "42".parse().unwrap();      // 42
let f: f64 = "3.14".parse().unwrap();    // 3.14
let echec = "abc".parse::<i32>();        // Err(...)
```

### Préciser le type cible

Rust doit savoir vers quel type convertir. Deux façons :

```rust
let n: i32 = "42".parse().unwrap();    // via l'annotation de la variable
let n = "42".parse::<i32>().unwrap();  // via le turbofish ::<i32>
```

### Gérer l'échec proprement

Plutôt que `unwrap` (qui panique), on traite l'`Err` :

```rust
fn vers_nombre(s: &str) -> i32 {
    s.trim().parse().unwrap_or(0) // 0 si ce n'est pas un nombre
}

vers_nombre("  42 "); // 42
vers_nombre("oups");  // 0
```

`trim()` retire les espaces autour — utile car `" 42 ".parse()` échouerait à
cause des espaces.

> 💡 `parse` s'appuie sur le trait `FromStr`. Tu peux l'implémenter pour tes
> propres types afin de les construire depuis une chaîne.
"#,
            en: r#"
## `parse`: text to number

To turn a string into a number, use `parse`. Since it can fail ("abc" is not a
number), `parse` returns a **`Result`**.

```rust
let n: i32 = "42".parse().unwrap();      // 42
let f: f64 = "3.14".parse().unwrap();    // 3.14
let fail = "abc".parse::<i32>();         // Err(...)
```

### Specifying the target type

Rust must know which type to convert to. Two ways:

```rust
let n: i32 = "42".parse().unwrap();    // via the variable annotation
let n = "42".parse::<i32>().unwrap();  // via the turbofish ::<i32>
```

### Handling failure cleanly

Rather than `unwrap` (which panics), handle the `Err`:

```rust
fn vers_nombre(s: &str) -> i32 {
    s.trim().parse().unwrap_or(0) // 0 if it isn't a number
}

vers_nombre("  42 "); // 42
vers_nombre("oops");  // 0
```

`trim()` removes surrounding whitespace — useful because `" 42 ".parse()` would
fail due to the spaces.

> 💡 `parse` relies on the `FromStr` trait. You can implement it for your own
> types to build them from a string.
"#,
        },
        task: Bi {
            fr: "Complète `vers_nombre` : convertit la chaîne (espaces compris) en i32, ou 0 si ce n'est pas un nombre. Le programme teste \"  42 \" et doit afficher : 42",
            en: "Complete `vers_nombre`: convert the string (spaces included) into an i32, or 0 if it isn't a number. The program tests \"  42 \" and must print: 42",
        },
        starter: r#"fn vers_nombre(s: &str) -> i32 {
    // Retire les espaces avec trim(), parse() en i32, défaut 0
}

fn main() {
    println!("{}", vers_nombre("  42 "));
}
"#,
        check: Check::Stdout { expected: "42" },
        hints: &[
            Bi { fr: "s.trim().parse().unwrap_or(0)", en: "s.trim().parse().unwrap_or(0)" },
        ],
        solution: r#"fn vers_nombre(s: &str) -> i32 {
    s.trim().parse().unwrap_or(0)
}

fn main() {
    println!("{}", vers_nombre("  42 "));
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 55,
        slug: "newtype",
        title: Bi { fr: "Le patron newtype", en: "The newtype pattern" },
        subtitle: Bi { fr: "Emballer un type pour plus de sûreté", en: "Wrap a type for more safety" },
        xp: 200,
        lesson: Bi {
            fr: r#"
## Le patron *newtype*

Le *newtype* consiste à **emballer** un type existant dans une struct tuple à un
seul champ. Ça crée un type **distinct**, avec sa propre sémantique.

```rust
struct Euros(u32);
struct Metres(u32);
```

Même si tous deux contiennent un `u32`, ce sont des **types différents** : tu ne
peux pas additionner des `Euros` et des `Metres` par erreur. Le compilateur
attrape la confusion.

### Accéder à la valeur emballée

Avec l'index `.0` (c'est une struct tuple) :

```rust
let prix = Euros(20);
println!("{}", prix.0); // 20
```

### Lui donner des méthodes

```rust
impl Euros {
    fn afficher(&self) -> String {
        format!("{} €", self.0)
    }
}

Euros(20).afficher(); // "20 €"
```

### Pourquoi c'est utile

- **Sûreté** : impossible de mélanger des unités (euros vs mètres).
- **Lisibilité** : `fn payer(montant: Euros)` est plus parlant que `u32`.
- **Extension** : tu peux implémenter des traits sur **ton** type, même autour
  d'un type standard.

> 💡 Des bibliothèques entières reposent sur les newtypes pour modéliser des
> identifiants, des unités, des quantités… sans aucun coût à l'exécution.
"#,
            en: r#"
## The *newtype* pattern

The *newtype* pattern **wraps** an existing type in a single-field tuple
struct. It creates a **distinct** type with its own semantics.

```rust
struct Euros(u32);
struct Metres(u32);
```

Even though both hold a `u32`, they are **different types**: you can't add
`Euros` and `Metres` by mistake. The compiler catches the confusion.

### Accessing the wrapped value

With the `.0` index (it's a tuple struct):

```rust
let price = Euros(20);
println!("{}", price.0); // 20
```

### Giving it methods

```rust
impl Euros {
    fn afficher(&self) -> String {
        format!("{} €", self.0)
    }
}

Euros(20).afficher(); // "20 €"
```

### Why it's useful

- **Safety**: you can't mix units (euros vs meters).
- **Readability**: `fn pay(amount: Euros)` is clearer than `u32`.
- **Extension**: you can implement traits on **your** type, even around a
  standard type.

> 💡 Whole libraries rely on newtypes to model identifiers, units, quantities…
> with zero runtime cost.
"#,
        },
        task: Bi {
            fr: "Complète la méthode `afficher` du newtype `Euros` : elle renvoie le montant suivi de \" €\". Le programme teste Euros(20) et doit afficher : 20 €",
            en: "Complete the `afficher` method of the `Euros` newtype: it returns the amount followed by \" €\". The program tests Euros(20) and must print: 20 €",
        },
        starter: r#"struct Euros(u32);

impl Euros {
    fn afficher(&self) -> String {
        // Renvoie une String de la forme "20 €" (accède au montant via self.0)
    }
}

fn main() {
    println!("{}", Euros(20).afficher());
}
"#,
        check: Check::Stdout { expected: "20 €" },
        hints: &[
            Bi { fr: "format!(\"{} €\", self.0)", en: "format!(\"{} €\", self.0)" },
        ],
        solution: r#"struct Euros(u32);

impl Euros {
    fn afficher(&self) -> String {
        format!("{} €", self.0)
    }
}

fn main() {
    println!("{}", Euros(20).afficher());
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 56,
        slug: "impl-trait-return",
        title: Bi { fr: "Renvoyer impl Trait", en: "Returning impl Trait" },
        subtitle: Bi { fr: "Renvoyer « quelque chose qui sait faire »", en: "Return \"something that can\"" },
        xp: 210,
        lesson: Bi {
            fr: r#"
## Renvoyer `impl Trait`

Parfois, une fonction veut renvoyer une valeur dont le type est compliqué à
écrire — typiquement un **itérateur** ou une **closure**. La syntaxe `impl
Trait` en position de retour dit : « je renvoie **un** type qui implémente ce
trait, sans préciser lequel ».

```rust
fn compteur_depuis(n: i32) -> impl Iterator<Item = i32> {
    (n..).take(3)
}

let v: Vec<i32> = compteur_depuis(5).collect();
// [5, 6, 7]
```

Le type réel renvoyé par `(n..).take(3)` a un nom très long et interne. `impl
Iterator<Item = i32>` te dispense de l'écrire : seul le **contrat** (« c'est un
itérateur de `i32` ») compte pour l'appelant.

### Renvoyer une closure

```rust
fn multiplicateur(facteur: i32) -> impl Fn(i32) -> i32 {
    move |x| x * facteur
}

let double = multiplicateur(2);
println!("{}", double(21)); // 42
```

Le `move` transfère `facteur` dans la closure, qui peut ainsi survivre à la
fonction.

### À noter

`impl Trait` en retour ne désigne **qu'un seul** type concret (choisi à la
compilation). Pour renvoyer des types **différents** selon un `if`, il faut un
objet-trait `Box<dyn Trait>`.

> 💡 `impl Trait` rend les signatures lisibles tout en gardant le *dispatch
> statique* (rapide, sans `Box`).
"#,
            en: r#"
## Returning `impl Trait`

Sometimes a function wants to return a value whose type is awkward to write —
typically an **iterator** or a **closure**. The `impl Trait` syntax in return
position says: "I return **a** type implementing this trait, without naming
which one".

```rust
fn compteur_depuis(n: i32) -> impl Iterator<Item = i32> {
    (n..).take(3)
}

let v: Vec<i32> = compteur_depuis(5).collect();
// [5, 6, 7]
```

The real type returned by `(n..).take(3)` has a long internal name. `impl
Iterator<Item = i32>` spares you from writing it: only the **contract** ("it's
an iterator of `i32`") matters to the caller.

### Returning a closure

```rust
fn multiplier(factor: i32) -> impl Fn(i32) -> i32 {
    move |x| x * factor
}

let double = multiplier(2);
println!("{}", double(21)); // 42
```

The `move` transfers `factor` into the closure, so it can outlive the function.

### Note

`impl Trait` in return position names **only one** concrete type (chosen at
compile time). To return **different** types depending on an `if`, you need a
trait object `Box<dyn Trait>`.

> 💡 `impl Trait` keeps signatures readable while keeping *static dispatch*
> (fast, no `Box`).
"#,
        },
        task: Bi {
            fr: "Complète `compteur_depuis` pour qu'elle renvoie un itérateur des 3 entiers à partir de n. Le programme collecte compteur_depuis(5) et doit afficher : [5, 6, 7]",
            en: "Complete `compteur_depuis` so it returns an iterator of the 3 integers starting from n. The program collects compteur_depuis(5) and must print: [5, 6, 7]",
        },
        starter: r#"fn compteur_depuis(n: i32) -> impl Iterator<Item = i32> {
    // Renvoie un itérateur des 3 premiers entiers à partir de n
    // Astuce : (n..).take(3)
}

fn main() {
    let v: Vec<i32> = compteur_depuis(5).collect();
    println!("{:?}", v);
}
"#,
        check: Check::Stdout { expected: "[5, 6, 7]" },
        hints: &[
            Bi { fr: "(n..).take(3)", en: "(n..).take(3)" },
        ],
        solution: r#"fn compteur_depuis(n: i32) -> impl Iterator<Item = i32> {
    (n..).take(3)
}

fn main() {
    let v: Vec<i32> = compteur_depuis(5).collect();
    println!("{:?}", v);
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 57,
        slug: "default-trait",
        title: Bi { fr: "Le trait Default", en: "The Default trait" },
        subtitle: Bi { fr: "Des valeurs par défaut élégantes", en: "Elegant default values" },
        xp: 200,
        lesson: Bi {
            fr: r#"
## `Default` : une valeur « par défaut »

Le trait `Default` fournit une valeur par défaut pour un type, via
`Default::default()`. Les types standards l'ont déjà (`0` pour les entiers,
`false` pour `bool`, `""` pour `String`…).

On le **dérive** pour ses structs : chaque champ prend sa propre valeur par
défaut.

```rust
#[derive(Default, Debug)]
struct Config {
    volume: u32,        // 0 par défaut
    plein_ecran: bool,  // false par défaut
}

let c = Config::default();
// Config { volume: 0, plein_ecran: false }
```

### La syntaxe de mise à jour `..Default::default()`

Très pratique : on ne précise que les champs voulus, le reste prend la valeur
par défaut.

```rust
let c = Config {
    volume: 50,
    ..Default::default() // remplit les autres champs (ici plein_ecran: false)
};
// Config { volume: 50, plein_ecran: false }
```

Les `..` à la fin disent « complète le reste à partir de cette valeur ».

> 💡 `..autre_valeur` marche avec n'importe quelle struct, pas seulement avec
> `Default` : `Config { volume: 50, ..base }` reprend les champs de `base`.
"#,
            en: r#"
## `Default`: a "default" value

The `Default` trait provides a default value for a type, via
`Default::default()`. Standard types already have one (`0` for integers,
`false` for `bool`, `""` for `String`…).

You **derive** it for your structs: each field takes its own default value.

```rust
#[derive(Default, Debug)]
struct Config {
    volume: u32,        // 0 by default
    plein_ecran: bool,  // false by default
}

let c = Config::default();
// Config { volume: 0, plein_ecran: false }
```

### The update syntax `..Default::default()`

Very handy: specify only the fields you want, the rest take their default.

```rust
let c = Config {
    volume: 50,
    ..Default::default() // fills the other fields (here plein_ecran: false)
};
// Config { volume: 50, plein_ecran: false }
```

The trailing `..` says "fill the rest from this value".

> 💡 `..other_value` works with any struct, not only with `Default`:
> `Config { volume: 50, ..base }` reuses `base`'s fields.
"#,
        },
        task: Bi {
            fr: "Dérive Default sur Config, puis construis une config avec volume = 50 et le reste par défaut. Le programme affiche \"volume plein_ecran\", attendu : 50 false",
            en: "Derive Default on Config, then build a config with volume = 50 and the rest by default. The program prints \"volume plein_ecran\", expected: 50 false",
        },
        starter: r#"// Ajoute le derive qui convient
struct Config {
    volume: u32,
    plein_ecran: bool,
}

fn main() {
    // Construis une Config avec volume: 50 et le reste par défaut
    let c = Config {
        volume: 50,
        // ...
    };
    println!("{} {}", c.volume, c.plein_ecran);
}
"#,
        check: Check::Stdout { expected: "50 false" },
        hints: &[
            Bi { fr: "Dérive Default, puis complète avec ..Default::default()", en: "Derive Default, then complete with ..Default::default()" },
        ],
        solution: r#"#[derive(Default)]
struct Config {
    volume: u32,
    plein_ecran: bool,
}

fn main() {
    let c = Config {
        volume: 50,
        ..Default::default()
    };
    println!("{} {}", c.volume, c.plein_ecran);
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 58,
        slug: "operator-overload",
        title: Bi { fr: "Surcharger un opérateur", en: "Overloading an operator" },
        subtitle: Bi { fr: "Donner un sens à + sur ses types", en: "Give + meaning on your types" },
        xp: 220,
        lesson: Bi {
            fr: r#"
## Donner un sens à `+`, `-`, `*`…

En Rust, les opérateurs sont définis par des **traits** de `std::ops`. Pour
autoriser `a + b` sur ton type, tu implémentes le trait `Add`.

```rust
use std::ops::Add;

#[derive(Debug)]
struct Vecteur2D {
    x: i32,
    y: i32,
}

impl Add for Vecteur2D {
    type Output = Vecteur2D; // le type du résultat de l'addition

    fn add(self, autre: Vecteur2D) -> Vecteur2D {
        Vecteur2D {
            x: self.x + autre.x,
            y: self.y + autre.y,
        }
    }
}
```

Maintenant `+` fonctionne :

```rust
let a = Vecteur2D { x: 1, y: 2 };
let b = Vecteur2D { x: 3, y: 4 };
let c = a + b; // appelle add(a, b)
// Vecteur2D { x: 4, y: 6 }
```

- `type Output` est un **type associé** : ce que renvoie l'addition.
- `add(self, autre)` reçoit les deux opérandes **par valeur** (ils sont
  consommés).

D'autres traits du même genre : `Sub` (`-`), `Mul` (`*`), `Div` (`/`),
`Neg` (`-x`), `Index` (`x[i]`)…

> 💡 La surcharge d'opérateurs rend les types mathématiques (vecteurs, matrices,
> nombres complexes…) naturels à manipuler, tout en restant explicite : c'est
> juste un appel de méthode déguisé.
"#,
            en: r#"
## Giving meaning to `+`, `-`, `*`…

In Rust, operators are defined by **traits** in `std::ops`. To allow `a + b` on
your type, you implement the `Add` trait.

```rust
use std::ops::Add;

#[derive(Debug)]
struct Vecteur2D {
    x: i32,
    y: i32,
}

impl Add for Vecteur2D {
    type Output = Vecteur2D; // the type of the addition's result

    fn add(self, autre: Vecteur2D) -> Vecteur2D {
        Vecteur2D {
            x: self.x + autre.x,
            y: self.y + autre.y,
        }
    }
}
```

Now `+` works:

```rust
let a = Vecteur2D { x: 1, y: 2 };
let b = Vecteur2D { x: 3, y: 4 };
let c = a + b; // calls add(a, b)
// Vecteur2D { x: 4, y: 6 }
```

- `type Output` is an **associated type**: what the addition returns.
- `add(self, autre)` receives both operands **by value** (they're consumed).

Other traits of the same kind: `Sub` (`-`), `Mul` (`*`), `Div` (`/`),
`Neg` (`-x`), `Index` (`x[i]`)…

> 💡 Operator overloading makes mathematical types (vectors, matrices, complex
> numbers…) natural to use, while staying explicit: it's just a disguised
> method call.
"#,
        },
        task: Bi {
            fr: "Implémente `Add` pour `Vecteur2D` : additionne les composantes x et y. Le programme additionne {1,2} et {3,4} et doit afficher : Vecteur2D { x: 4, y: 6 }",
            en: "Implement `Add` for `Vecteur2D`: add the x and y components. The program adds {1,2} and {3,4} and must print: Vecteur2D { x: 4, y: 6 }",
        },
        starter: r#"use std::ops::Add;

#[derive(Debug)]
struct Vecteur2D {
    x: i32,
    y: i32,
}

impl Add for Vecteur2D {
    type Output = Vecteur2D;

    fn add(self, autre: Vecteur2D) -> Vecteur2D {
        // Renvoie un Vecteur2D dont x et y sont les sommes des composantes
    }
}

fn main() {
    let a = Vecteur2D { x: 1, y: 2 };
    let b = Vecteur2D { x: 3, y: 4 };
    println!("{:?}", a + b);
}
"#,
        check: Check::Stdout { expected: "Vecteur2D { x: 4, y: 6 }" },
        hints: &[
            Bi { fr: "Vecteur2D { x: self.x + autre.x, y: self.y + autre.y }", en: "Vecteur2D { x: self.x + autre.x, y: self.y + autre.y }" },
        ],
        solution: r#"use std::ops::Add;

#[derive(Debug)]
struct Vecteur2D {
    x: i32,
    y: i32,
}

impl Add for Vecteur2D {
    type Output = Vecteur2D;

    fn add(self, autre: Vecteur2D) -> Vecteur2D {
        Vecteur2D {
            x: self.x + autre.x,
            y: self.y + autre.y,
        }
    }
}

fn main() {
    let a = Vecteur2D { x: 1, y: 2 };
    let b = Vecteur2D { x: 3, y: 4 };
    println!("{:?}", a + b);
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 59,
        slug: "recursion",
        title: Bi { fr: "La récursivité", en: "Recursion" },
        subtitle: Bi { fr: "Une fonction qui s'appelle elle-même", en: "A function calling itself" },
        xp: 210,
        lesson: Bi {
            fr: r#"
## Les fonctions récursives

Une fonction **récursive** s'appelle elle-même pour résoudre un problème en le
réduisant à un cas plus petit. Toute récursion a besoin de **deux** ingrédients :

1. un **cas de base** qui arrête la descente ;
2. un **cas récursif** qui se rapproche du cas de base.

### La factorielle

`n! = n × (n-1) × … × 1` :

```rust
fn factorielle(n: u64) -> u64 {
    if n <= 1 {
        1               // cas de base
    } else {
        n * factorielle(n - 1) // cas récursif
    }
}

factorielle(5); // 5 * 4 * 3 * 2 * 1 = 120
```

Déroulé : `factorielle(3)` = `3 * factorielle(2)` = `3 * 2 * factorielle(1)` =
`3 * 2 * 1` = `6`.

### Attention au cas de base

Sans cas de base (ou s'il n'est jamais atteint), la fonction s'appelle à
l'infini et **déborde la pile** (*stack overflow*). C'est le pendant de la
boucle infinie.

```rust
fn boucle(n: u64) -> u64 {
    n + boucle(n + 1) // 💥 jamais de fin
}
```

> 💡 Beaucoup de problèmes « en arbre » (parcours de fichiers, structures
> imbriquées) s'expriment naturellement de façon récursive. Pour des calculs
> linéaires simples, une boucle est souvent plus efficace.
"#,
            en: r#"
## Recursive functions

A **recursive** function calls itself to solve a problem by reducing it to a
smaller case. Every recursion needs **two** ingredients:

1. a **base case** that stops the descent;
2. a **recursive case** that moves toward the base case.

### Factorial

`n! = n × (n-1) × … × 1`:

```rust
fn factorielle(n: u64) -> u64 {
    if n <= 1 {
        1               // base case
    } else {
        n * factorielle(n - 1) // recursive case
    }
}

factorielle(5); // 5 * 4 * 3 * 2 * 1 = 120
```

Unrolling: `factorielle(3)` = `3 * factorielle(2)` = `3 * 2 * factorielle(1)` =
`3 * 2 * 1` = `6`.

### Mind the base case

Without a base case (or if it's never reached), the function calls itself
forever and **overflows the stack**. It's the recursion equivalent of an
infinite loop.

```rust
fn loop_forever(n: u64) -> u64 {
    n + loop_forever(n + 1) // 💥 never ends
}
```

> 💡 Many "tree-like" problems (file traversal, nested structures) express
> naturally with recursion. For simple linear computations, a loop is often
> more efficient.
"#,
        },
        task: Bi {
            fr: "Complète `factorielle` (récursive) : cas de base n <= 1 renvoie 1, sinon n × factorielle(n-1). Le programme teste 5 et doit afficher : 120",
            en: "Complete `factorielle` (recursive): base case n <= 1 returns 1, otherwise n × factorielle(n-1). The program tests 5 and must print: 120",
        },
        starter: r#"fn factorielle(n: u64) -> u64 {
    if n <= 1 {
        1
    } else {
        // Cas récursif : n * factorielle(n - 1)
    }
}

fn main() {
    println!("{}", factorielle(5));
}
"#,
        check: Check::Stdout { expected: "120" },
        hints: &[
            Bi { fr: "n * factorielle(n - 1)", en: "n * factorielle(n - 1)" },
        ],
        solution: r#"fn factorielle(n: u64) -> u64 {
    if n <= 1 {
        1
    } else {
        n * factorielle(n - 1)
    }
}

fn main() {
    println!("{}", factorielle(5));
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 60,
        slug: "slices",
        title: Bi { fr: "Les slices", en: "Slices" },
        subtitle: Bi { fr: "Emprunter une portion d'une collection", en: "Borrow a portion of a collection" },
        xp: 210,
        lesson: Bi {
            fr: r#"
## Les slices : une vue sur une portion

Une **slice** `&[T]` est une **référence** vers une portion contiguë d'un
tableau ou d'un vecteur. Elle ne possède rien : elle **emprunte** une fenêtre.

```rust
let donnees = [10, 20, 30, 40, 50];

let tout = &donnees[..];      // [10, 20, 30, 40, 50]
let debut = &donnees[..2];    // [10, 20]
let milieu = &donnees[1..4];  // [20, 30, 40]
let fin = &donnees[3..];      // [40, 50]
```

Les bornes suivent la logique des intervalles : `[1..4]` prend les indices 1, 2,
3 (le 4 est **exclu**).

### Des fonctions génériques sur les slices

Prendre `&[T]` en paramètre rend une fonction utilisable avec un **tableau** ou
un **vecteur**, sans distinction :

```rust
fn somme(tranche: &[i32]) -> i32 {
    tranche.iter().sum()
}

let tableau = [1, 2, 3];
let vecteur = vec![4, 5, 6];
somme(&tableau);     // ok
somme(&vecteur);     // ok aussi
somme(&vecteur[1..]); // sur une sous-partie
```

### Le cas des chaînes

`&str` est exactement une slice de chaîne : `&"bonjour"[0..3]` vaut `"bon"`.

> 💡 Préfère `&[T]` à `&Vec<T>` dans les signatures de fonctions : c'est plus
> général et tout aussi efficace.
"#,
            en: r#"
## Slices: a view over a portion

A **slice** `&[T]` is a **reference** to a contiguous portion of an array or a
vector. It owns nothing: it **borrows** a window.

```rust
let donnees = [10, 20, 30, 40, 50];

let all = &donnees[..];       // [10, 20, 30, 40, 50]
let start = &donnees[..2];    // [10, 20]
let middle = &donnees[1..4];  // [20, 30, 40]
let end = &donnees[3..];      // [40, 50]
```

The bounds follow range logic: `[1..4]` takes indices 1, 2, 3 (4 is
**excluded**).

### Generic functions over slices

Taking `&[T]` as a parameter makes a function usable with an **array** or a
**vector**, indistinctly:

```rust
fn somme(slice: &[i32]) -> i32 {
    slice.iter().sum()
}

let array = [1, 2, 3];
let vector = vec![4, 5, 6];
somme(&array);     // ok
somme(&vector);    // ok too
somme(&vector[1..]); // on a sub-part
```

### The string case

`&str` is exactly a string slice: `&"hello"[0..3]` is `"hel"`.

> 💡 Prefer `&[T]` over `&Vec<T>` in function signatures: it's more general and
> just as efficient.
"#,
        },
        task: Bi {
            fr: "Appelle `somme` avec la TRANCHE des éléments d'indices 1 à 3 (inclus) du tableau. Sur [10, 20, 30, 40, 50], cela vaut 20+30+40, attendu : 90",
            en: "Call `somme` with the SLICE of elements at indices 1 to 3 (inclusive) of the array. On [10, 20, 30, 40, 50] that's 20+30+40, expected: 90",
        },
        starter: r#"fn somme(tranche: &[i32]) -> i32 {
    tranche.iter().sum()
}

fn main() {
    let donnees = [10, 20, 30, 40, 50];
    // Passe la tranche des indices 1, 2 et 3 (donc &donnees[1..4])
    let resultat = somme(/* ... */);
    println!("{}", resultat);
}
"#,
        check: Check::Stdout { expected: "90" },
        hints: &[
            Bi { fr: "La tranche des indices 1 à 3 inclus s'écrit &donnees[1..4].", en: "The slice of indices 1 to 3 inclusive is &donnees[1..4]." },
        ],
        solution: r#"fn somme(tranche: &[i32]) -> i32 {
    tranche.iter().sum()
}

fn main() {
    let donnees = [10, 20, 30, 40, 50];
    let resultat = somme(&donnees[1..4]);
    println!("{}", resultat);
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 61,
        slug: "collect-result",
        title: Bi { fr: "collect vers un Result", en: "collect into a Result" },
        subtitle: Bi { fr: "Tout réussit, ou la première erreur", en: "All succeed, or the first error" },
        xp: 230,
        lesson: Bi {
            fr: r#"
## `collect` peut produire un `Result`

Un des tours les plus élégants de Rust : si tu as un itérateur de
`Result<T, E>`, tu peux le `collect` directement en **un seul**
`Result<Vec<T>, E>`.

- Si **tous** les éléments sont `Ok`, tu obtiens `Ok(vec![...])`.
- Dès qu'**un** élément est `Err`, le `collect` **s'arrête** et renvoie cette
  première `Err`.

```rust
let entrees = ["1", "2", "3"];

let resultat: Result<Vec<i32>, _> =
    entrees.iter().map(|s| s.parse::<i32>()).collect();

println!("{:?}", resultat.unwrap()); // [1, 2, 3]
```

Avec une entrée invalide, on récupère l'erreur au lieu d'un vecteur partiel :

```rust
let entrees = ["1", "oups", "3"];
let r: Result<Vec<i32>, _> =
    entrees.iter().map(|s| s.parse::<i32>()).collect();
// r est Err(...) — la conversion de "oups" a échoué
```

C'est le pattern « tout ou rien » : on valide une liste entière, et la moindre
erreur fait échouer l'ensemble proprement.

> 💡 Le même mécanisme marche avec `Option` : un itérateur d'`Option<T>` se
> `collect` en `Option<Vec<T>>` (None si un seul élément est None).
"#,
            en: r#"
## `collect` can produce a `Result`

One of Rust's most elegant tricks: if you have an iterator of `Result<T, E>`,
you can `collect` it directly into **a single** `Result<Vec<T>, E>`.

- If **all** elements are `Ok`, you get `Ok(vec![...])`.
- As soon as **one** element is `Err`, the `collect` **stops** and returns that
  first `Err`.

```rust
let inputs = ["1", "2", "3"];

let result: Result<Vec<i32>, _> =
    inputs.iter().map(|s| s.parse::<i32>()).collect();

println!("{:?}", result.unwrap()); // [1, 2, 3]
```

With an invalid input, you get the error instead of a partial vector:

```rust
let inputs = ["1", "oops", "3"];
let r: Result<Vec<i32>, _> =
    inputs.iter().map(|s| s.parse::<i32>()).collect();
// r is Err(...) — parsing "oops" failed
```

It's the "all or nothing" pattern: you validate a whole list, and the slightest
error makes the whole thing fail cleanly.

> 💡 The same mechanism works with `Option`: an iterator of `Option<T>`
> collects into `Option<Vec<T>>` (None if a single element is None).
"#,
        },
        task: Bi {
            fr: "Annote le type de `resultat` pour collecter les conversions en un Result<Vec<i32>, _>. Le programme convertit [\"1\", \"2\", \"3\"] et affiche le Vec, attendu : [1, 2, 3]",
            en: "Annotate the type of `resultat` to collect the conversions into a Result<Vec<i32>, _>. The program converts [\"1\", \"2\", \"3\"] and prints the Vec, expected: [1, 2, 3]",
        },
        starter: r#"fn main() {
    let entrees = ["1", "2", "3"];

    // Donne le bon type à `resultat` pour que collect produise un Result
    let resultat: /* ... */ =
        entrees.iter().map(|s| s.parse::<i32>()).collect();

    println!("{:?}", resultat.unwrap());
}
"#,
        check: Check::Stdout { expected: "[1, 2, 3]" },
        hints: &[
            Bi { fr: "Le type cible est Result<Vec<i32>, _> (l'underscore laisse Rust déduire l'erreur).", en: "The target type is Result<Vec<i32>, _> (the underscore lets Rust infer the error)." },
        ],
        solution: r#"fn main() {
    let entrees = ["1", "2", "3"];

    let resultat: Result<Vec<i32>, _> =
        entrees.iter().map(|s| s.parse::<i32>()).collect();

    println!("{:?}", resultat.unwrap());
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 62,
        slug: "capstone",
        title: Bi { fr: "Projet final : la calculatrice", en: "Capstone: the calculator" },
        subtitle: Bi { fr: "Enum, match et itérateurs réunis", en: "Enum, match and iterators together" },
        xp: 300,
        lesson: Bi {
            fr: r#"
## Bravo, te voilà au bout du parcours ! 🎉

Ce dernier niveau **rassemble** ce que tu as appris : une `enum` qui modélise
des opérations, un `match` pour les exécuter, et un `fold` pour les enchaîner.

### L'idée : une petite calculatrice à états

On part d'un état (un nombre), et on lui applique une suite d'**opérations**.
Chaque opération est une variante d'enum, qui peut transporter une donnée :

```rust
enum Operation {
    Ajouter(i32),
    Soustraire(i32),
    Reset, // remet l'état à 0
}
```

### Appliquer une opération

Une fonction prend l'état courant et une opération, et renvoie le **nouvel**
état :

```rust
fn appliquer(etat: i32, op: Operation) -> i32 {
    match op {
        Operation::Ajouter(n) => etat + n,
        Operation::Soustraire(n) => etat - n,
        Operation::Reset => 0,
    }
}
```

### Enchaîner avec `fold`

On part de `0` et on replie la liste d'opérations :

```rust
let operations = vec![
    Operation::Ajouter(10),
    Operation::Ajouter(5),
    Operation::Soustraire(3),
];

let resultat = operations
    .into_iter()
    .fold(0, |etat, op| appliquer(etat, op));
// 0 +10 +5 -3 = 12
```

Tu viens de combiner enums à données, filtrage exhaustif, et programmation par
itérateurs : c'est tout l'esprit de Rust réuni. 🦀

> 💡 Et maintenant ? Lance-toi dans un vrai projet : un jeu en terminal, un
> petit serveur web (comme RustQuest !), un outil en ligne de commande…
"#,
            en: r#"
## Congrats, you've reached the end! 🎉

This final level **combines** what you've learned: an `enum` modeling
operations, a `match` to execute them, and a `fold` to chain them.

### The idea: a small stateful calculator

You start from a state (a number) and apply a series of **operations**. Each
operation is an enum variant, possibly carrying data:

```rust
enum Operation {
    Ajouter(i32),
    Soustraire(i32),
    Reset, // resets the state to 0
}
```

### Applying an operation

A function takes the current state and an operation, and returns the **new**
state:

```rust
fn appliquer(etat: i32, op: Operation) -> i32 {
    match op {
        Operation::Ajouter(n) => etat + n,
        Operation::Soustraire(n) => etat - n,
        Operation::Reset => 0,
    }
}
```

### Chaining with `fold`

Start from `0` and fold the list of operations:

```rust
let operations = vec![
    Operation::Ajouter(10),
    Operation::Ajouter(5),
    Operation::Soustraire(3),
];

let result = operations
    .into_iter()
    .fold(0, |etat, op| appliquer(etat, op));
// 0 +10 +5 -3 = 12
```

You've just combined data-carrying enums, exhaustive matching, and iterator
programming: the whole spirit of Rust in one place. 🦀

> 💡 What now? Dive into a real project: a terminal game, a small web server
> (like RustQuest!), a command-line tool…
"#,
        },
        task: Bi {
            fr: "Complète le `match` de `appliquer` pour les trois opérations (Ajouter, Soustraire, Reset). Le programme applique +10, +5, -3 à partir de 0 et doit afficher : 12",
            en: "Complete the `match` in `appliquer` for the three operations (Ajouter, Soustraire, Reset). The program applies +10, +5, -3 starting from 0 and must print: 12",
        },
        starter: r#"enum Operation {
    Ajouter(i32),
    Soustraire(i32),
    Reset,
}

fn appliquer(etat: i32, op: Operation) -> i32 {
    match op {
        // Ajouter(n) => etat + n
        // Soustraire(n) => etat - n
        // Reset => 0
    }
}

fn main() {
    let operations = vec![
        Operation::Ajouter(10),
        Operation::Ajouter(5),
        Operation::Soustraire(3),
    ];

    let resultat = operations
        .into_iter()
        .fold(0, |etat, op| appliquer(etat, op));

    println!("{}", resultat);
}
"#,
        check: Check::Stdout { expected: "12" },
        hints: &[
            Bi { fr: "Traite les trois variantes : Operation::Ajouter(n) => etat + n, etc.", en: "Handle the three variants: Operation::Ajouter(n) => etat + n, etc." },
        ],
        solution: r#"enum Operation {
    Ajouter(i32),
    Soustraire(i32),
    Reset,
}

fn appliquer(etat: i32, op: Operation) -> i32 {
    match op {
        Operation::Ajouter(n) => etat + n,
        Operation::Soustraire(n) => etat - n,
        Operation::Reset => 0,
    }
}

fn main() {
    let operations = vec![
        Operation::Ajouter(10),
        Operation::Ajouter(5),
        Operation::Soustraire(3),
    ];

    let resultat = operations
        .into_iter()
        .fold(0, |etat, op| appliquer(etat, op));

    println!("{}", resultat);
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 63,
        slug: "iter-find",
        title: Bi { fr: "Itérateurs : find", en: "Iterators: find" },
        subtitle: Bi { fr: "Trouver le premier qui convient", en: "Find the first match" },
        xp: 200,
        lesson: Bi {
            fr: r#"
## `find` : le premier élément qui satisfait une condition

`find` parcourt l'itérateur et renvoie le **premier** élément pour lequel la
closure est `true`, sous forme d'`Option`. Il **s'arrête** dès qu'il l'a trouvé.

```rust
let v = [3, 7, 4, 9];
let premier_pair = v.iter().find(|&&x| x % 2 == 0);
// Some(&4)
```

Note le double `&&` : `iter()` produit des `&i32`, et `find` passe encore une
référence à la closure, d'où `|&&x|` pour récupérer directement la valeur.

### `position` : l'index plutôt que la valeur

```rust
let i = v.iter().position(|&x| x == 4); // Some(2)
```

### Les transformer

`find` renvoie `Option<&T>`. Pour une `Option<T>` (valeur copiée) :

```rust
let n: Option<i32> = v.iter().find(|&&x| x % 2 == 0).copied();
```

> 💡 `find` est paresseux : il ne lit que jusqu'au premier succès, contrairement
> à `filter().collect()` qui parcourt tout.
"#,
            en: r#"
## `find`: the first element satisfying a condition

`find` walks the iterator and returns the **first** element for which the
closure is `true`, as an `Option`. It **stops** as soon as it finds one.

```rust
let v = [3, 7, 4, 9];
let first_even = v.iter().find(|&&x| x % 2 == 0);
// Some(&4)
```

Note the double `&&`: `iter()` yields `&i32`, and `find` passes another
reference to the closure, hence `|&&x|` to get the value directly.

### `position`: the index instead of the value

```rust
let i = v.iter().position(|&x| x == 4); // Some(2)
```

### Transforming them

`find` returns `Option<&T>`. For an `Option<T>` (copied value):

```rust
let n: Option<i32> = v.iter().find(|&&x| x % 2 == 0).copied();
```

> 💡 `find` is lazy: it only reads up to the first success, unlike
> `filter().collect()` which goes through everything.
"#,
        },
        task: Bi {
            fr: "Complète `premier_pair` avec find : elle renvoie le premier nombre pair (copié). Le programme teste [3, 7, 4, 9] et doit afficher : 4",
            en: "Complete `premier_pair` with find: it returns the first even number (copied). The program tests [3, 7, 4, 9] and must print: 4",
        },
        starter: r#"fn premier_pair(v: &[i32]) -> Option<i32> {
    // Utilise v.iter().find(...) puis .copied()
}

fn main() {
    println!("{}", premier_pair(&[3, 7, 4, 9]).unwrap());
}
"#,
        check: Check::Stdout { expected: "4" },
        hints: &[
            Bi { fr: "v.iter().find(|&&x| x % 2 == 0).copied()", en: "v.iter().find(|&&x| x % 2 == 0).copied()" },
        ],
        solution: r#"fn premier_pair(v: &[i32]) -> Option<i32> {
    v.iter().find(|&&x| x % 2 == 0).copied()
}

fn main() {
    println!("{}", premier_pair(&[3, 7, 4, 9]).unwrap());
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 64,
        slug: "iter-all-any",
        title: Bi { fr: "Itérateurs : all & any", en: "Iterators: all & any" },
        subtitle: Bi { fr: "Vérifier une propriété globale", en: "Check a global property" },
        xp: 200,
        lesson: Bi {
            fr: r#"
## `all` et `any` : des tests sur tout l'itérateur

- `all(closure)` renvoie `true` si **tous** les éléments satisfont la condition.
- `any(closure)` renvoie `true` si **au moins un** la satisfait.

```rust
let v = [2, 4, 6];

v.iter().all(|&x| x % 2 == 0); // true (tous pairs)
v.iter().any(|&x| x > 5);      // true (6 > 5)
v.iter().all(|&x| x > 5);      // false
```

Les deux sont **paresseux** et court-circuitent :

- `all` s'arrête au premier élément qui **échoue** ;
- `any` s'arrête au premier qui **réussit**.

```rust
let vide: [i32; 0] = [];
vide.iter().all(|&x| x > 0); // true  (vacuément vrai)
vide.iter().any(|&x| x > 0); // false (aucun élément)
```

> 💡 Sur un itérateur vide, `all` vaut `true` (« tous » de rien) et `any` vaut
> `false` — une convention logique qui surprend parfois.
"#,
            en: r#"
## `all` and `any`: tests over the whole iterator

- `all(closure)` returns `true` if **all** elements satisfy the condition.
- `any(closure)` returns `true` if **at least one** does.

```rust
let v = [2, 4, 6];

v.iter().all(|&x| x % 2 == 0); // true (all even)
v.iter().any(|&x| x > 5);      // true (6 > 5)
v.iter().all(|&x| x > 5);      // false
```

Both are **lazy** and short-circuit:

- `all` stops at the first element that **fails**;
- `any` stops at the first that **succeeds**.

```rust
let empty: [i32; 0] = [];
empty.iter().all(|&x| x > 0); // true  (vacuously true)
empty.iter().any(|&x| x > 0); // false (no element)
```

> 💡 On an empty iterator, `all` is `true` ("all" of nothing) and `any` is
> `false` — a logical convention that sometimes surprises.
"#,
        },
        task: Bi {
            fr: "Complète `tous_positifs` qui renvoie true si tous les éléments sont strictement positifs. Le programme teste [2, 4, 6] et doit afficher : true",
            en: "Complete `tous_positifs` returning true if all elements are strictly positive. The program tests [2, 4, 6] and must print: true",
        },
        starter: r#"fn tous_positifs(v: &[i32]) -> bool {
    // Utilise v.iter().all(...)
}

fn main() {
    println!("{}", tous_positifs(&[2, 4, 6]));
}
"#,
        check: Check::Stdout { expected: "true" },
        hints: &[
            Bi { fr: "v.iter().all(|&x| x > 0)", en: "v.iter().all(|&x| x > 0)" },
        ],
        solution: r#"fn tous_positifs(v: &[i32]) -> bool {
    v.iter().all(|&x| x > 0)
}

fn main() {
    println!("{}", tous_positifs(&[2, 4, 6]));
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 65,
        slug: "filter-map",
        title: Bi { fr: "filter_map", en: "filter_map" },
        subtitle: Bi { fr: "Filtrer et transformer d'un coup", en: "Filter and transform at once" },
        xp: 210,
        lesson: Bi {
            fr: r#"
## `filter_map` : filtrer et transformer en une étape

`filter_map` applique une closure qui renvoie une `Option`. Les `Some(v)` sont
**gardés** (et déballés en `v`) ; les `None` sont **éliminés**.

C'est idéal quand une transformation peut **échouer** et qu'on veut juste
ignorer les échecs.

```rust
let entrees = ["1", "x", "3", "oups", "5"];

let nombres: Vec<i32> = entrees
    .iter()
    .filter_map(|s| s.parse::<i32>().ok())
    .collect();
// [1, 3, 5]  (les non-nombres sont ignorés)
```

`s.parse::<i32>()` renvoie un `Result` ; `.ok()` le convertit en `Option`
(`Ok(v)` → `Some(v)`, `Err` → `None`). `filter_map` ne garde que les `Some`.

### L'équivalent verbeux

```rust
.iter()
.map(|s| s.parse::<i32>())
.filter(|r| r.is_ok())
.map(|r| r.unwrap())
```

`filter_map` fait tout ça proprement, sans `unwrap`.

> 💡 « map qui peut dire non » : chaque fois que tu enchaînes un `filter` après
> un `map` (ou inversement) à cause d'une `Option`/`Result`, pense `filter_map`.
"#,
            en: r#"
## `filter_map`: filter and transform in one step

`filter_map` applies a closure returning an `Option`. The `Some(v)` are
**kept** (and unwrapped to `v`); the `None` are **dropped**.

It's ideal when a transformation can **fail** and you just want to skip
failures.

```rust
let inputs = ["1", "x", "3", "oops", "5"];

let numbers: Vec<i32> = inputs
    .iter()
    .filter_map(|s| s.parse::<i32>().ok())
    .collect();
// [1, 3, 5]  (non-numbers are ignored)
```

`s.parse::<i32>()` returns a `Result`; `.ok()` converts it to an `Option`
(`Ok(v)` → `Some(v)`, `Err` → `None`). `filter_map` keeps only the `Some`.

### The verbose equivalent

```rust
.iter()
.map(|s| s.parse::<i32>())
.filter(|r| r.is_ok())
.map(|r| r.unwrap())
```

`filter_map` does all of that cleanly, with no `unwrap`.

> 💡 "map that can say no": whenever you chain a `filter` after a `map` (or vice
> versa) because of an `Option`/`Result`, reach for `filter_map`.
"#,
        },
        task: Bi {
            fr: "Complète `somme_valides` : elle additionne uniquement les chaînes convertibles en nombre, avec filter_map. Le programme teste [\"1\", \"x\", \"3\"] et doit afficher : 4",
            en: "Complete `somme_valides`: it sums only the strings convertible to a number, using filter_map. The program tests [\"1\", \"x\", \"3\"] and must print: 4",
        },
        starter: r#"fn somme_valides(entrees: &[&str]) -> i32 {
    // filter_map avec s.parse::<i32>().ok(), puis sum
}

fn main() {
    println!("{}", somme_valides(&["1", "x", "3"]));
}
"#,
        check: Check::Stdout { expected: "4" },
        hints: &[
            Bi { fr: "entrees.iter().filter_map(|s| s.parse::<i32>().ok()).sum()", en: "entrees.iter().filter_map(|s| s.parse::<i32>().ok()).sum()" },
        ],
        solution: r#"fn somme_valides(entrees: &[&str]) -> i32 {
    entrees.iter().filter_map(|s| s.parse::<i32>().ok()).sum()
}

fn main() {
    println!("{}", somme_valides(&["1", "x", "3"]));
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 66,
        slug: "flat-map",
        title: Bi { fr: "flat_map & flatten", en: "flat_map & flatten" },
        subtitle: Bi { fr: "Aplatir des collections imbriquées", en: "Flatten nested collections" },
        xp: 210,
        lesson: Bi {
            fr: r#"
## Aplatir : `flatten` et `flat_map`

### `flatten` : enlever un niveau d'imbrication

```rust
let imbrique = [[1, 2], [3, 4], [5, 6]];
let plat: Vec<i32> = imbrique.iter().flatten().copied().collect();
// [1, 2, 3, 4, 5, 6]
```

### `flat_map` : map **puis** flatten

`flat_map` applique une closure qui renvoie un itérateur (ou une collection),
puis aplatit le tout en un seul flux.

```rust
let mots = ["bonjour", "salut"];
let lettres: Vec<char> = mots.iter().flat_map(|m| m.chars()).collect();
// tous les caractères des deux mots, à la suite
```

C'est `map(...).flatten()` en une seule opération :

```rust
let listes = [vec![1, 2], vec![3, 4]];
let somme: i32 = listes.iter().flat_map(|v| v.iter()).sum();
// 1 + 2 + 3 + 4 = 10
```

### Cas pratique

`flat_map` brille quand chaque élément produit **zéro, un ou plusieurs**
résultats : générer des paires, dérouler des plages, etc.

> 💡 Astuce mnémotechnique : `flat_map(f)` ≡ `map(f).flatten()`. Quand ta closure
> renvoie « plusieurs choses », c'est `flat_map`.
"#,
            en: r#"
## Flattening: `flatten` and `flat_map`

### `flatten`: remove one level of nesting

```rust
let nested = [[1, 2], [3, 4], [5, 6]];
let flat: Vec<i32> = nested.iter().flatten().copied().collect();
// [1, 2, 3, 4, 5, 6]
```

### `flat_map`: map **then** flatten

`flat_map` applies a closure returning an iterator (or collection), then
flattens it all into one stream.

```rust
let words = ["hello", "hi"];
let letters: Vec<char> = words.iter().flat_map(|m| m.chars()).collect();
// all characters of both words, in sequence
```

It's `map(...).flatten()` in one operation:

```rust
let lists = [vec![1, 2], vec![3, 4]];
let sum: i32 = lists.iter().flat_map(|v| v.iter()).sum();
// 1 + 2 + 3 + 4 = 10
```

### Practical case

`flat_map` shines when each element yields **zero, one or many** results:
generating pairs, expanding ranges, etc.

> 💡 Mnemonic: `flat_map(f)` ≡ `map(f).flatten()`. When your closure returns
> "several things", it's `flat_map`.
"#,
        },
        task: Bi {
            fr: "Complète `aplatir_somme` : additionne tous les nombres de toutes les sous-listes avec flat_map. Le programme teste [[1, 2], [3, 4]] et doit afficher : 10",
            en: "Complete `aplatir_somme`: sum all numbers of all sub-lists using flat_map. The program tests [[1, 2], [3, 4]] and must print: 10",
        },
        starter: r#"fn aplatir_somme(listes: &[Vec<i32>]) -> i32 {
    // listes.iter().flat_map(|v| v.iter()).sum()
}

fn main() {
    let donnees = vec![vec![1, 2], vec![3, 4]];
    println!("{}", aplatir_somme(&donnees));
}
"#,
        check: Check::Stdout { expected: "10" },
        hints: &[
            Bi { fr: "listes.iter().flat_map(|v| v.iter()).sum()", en: "listes.iter().flat_map(|v| v.iter()).sum()" },
        ],
        solution: r#"fn aplatir_somme(listes: &[Vec<i32>]) -> i32 {
    listes.iter().flat_map(|v| v.iter()).sum()
}

fn main() {
    let donnees = vec![vec![1, 2], vec![3, 4]];
    println!("{}", aplatir_somme(&donnees));
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 67,
        slug: "chain-rev",
        title: Bi { fr: "chain & rev", en: "chain & rev" },
        subtitle: Bi { fr: "Concaténer et inverser des itérateurs", en: "Concatenate and reverse iterators" },
        xp: 200,
        lesson: Bi {
            fr: r#"
## `chain` : mettre deux itérateurs bout à bout

```rust
let a = [1, 2];
let b = [5, 6];
let ensemble: Vec<i32> = a.iter().chain(b.iter()).copied().collect();
// [1, 2, 5, 6]
```

`chain` produit d'abord tous les éléments du premier itérateur, puis ceux du
second.

## `rev` : parcourir à l'envers

```rust
let v: Vec<i32> = (1..=3).rev().collect();
// [3, 2, 1]
```

`rev` ne fonctionne que sur les itérateurs **réversibles** (ceux qui savent
avancer par les deux bouts : plages, slices, vecteurs…).

### Les combiner

```rust
let r: Vec<i32> = (1..=3).chain(10..=11).rev().collect();
// d'abord [1, 2, 3, 10, 11], puis inversé : [11, 10, 3, 2, 1]
```

> 💡 `chain` et `rev` restent **paresseux** : aucun nouveau vecteur n'est créé
> tant que tu n'appelles pas `collect`/`sum`/etc.
"#,
            en: r#"
## `chain`: put two iterators end to end

```rust
let a = [1, 2];
let b = [5, 6];
let both: Vec<i32> = a.iter().chain(b.iter()).copied().collect();
// [1, 2, 5, 6]
```

`chain` yields all elements of the first iterator, then those of the second.

## `rev`: iterate backwards

```rust
let v: Vec<i32> = (1..=3).rev().collect();
// [3, 2, 1]
```

`rev` only works on **reversible** iterators (those that can advance from both
ends: ranges, slices, vectors…).

### Combining them

```rust
let r: Vec<i32> = (1..=3).chain(10..=11).rev().collect();
// first [1, 2, 3, 10, 11], then reversed: [11, 10, 3, 2, 1]
```

> 💡 `chain` and `rev` stay **lazy**: no new vector is created until you call
> `collect`/`sum`/etc.
"#,
        },
        task: Bi {
            fr: "Concatène l'intervalle 1..=2 puis 5..=6 avec chain et collecte dans un Vec. Le programme doit afficher : [1, 2, 5, 6]",
            en: "Concatenate the range 1..=2 then 5..=6 with chain and collect into a Vec. The program must print: [1, 2, 5, 6]",
        },
        starter: r#"fn main() {
    // Utilise (1..=2).chain(5..=6) puis collect
    let v: Vec<i32> = /* ... */;
    println!("{:?}", v);
}
"#,
        check: Check::Stdout { expected: "[1, 2, 5, 6]" },
        hints: &[
            Bi { fr: "(1..=2).chain(5..=6).collect()", en: "(1..=2).chain(5..=6).collect()" },
        ],
        solution: r#"fn main() {
    let v: Vec<i32> = (1..=2).chain(5..=6).collect();
    println!("{:?}", v);
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 68,
        slug: "take-while",
        title: Bi { fr: "take_while & skip_while", en: "take_while & skip_while" },
        subtitle: Bi { fr: "Couper un flux selon une condition", en: "Cut a stream by a condition" },
        xp: 210,
        lesson: Bi {
            fr: r#"
## Découper un itérateur dynamiquement

### `take_while` : prendre tant que c'est vrai

Garde les éléments **depuis le début**, et **s'arrête** au premier qui échoue.

```rust
let v = [1, 2, 3, 8, 2, 1];
let debut: Vec<i32> = v.iter().take_while(|&&x| x < 5).copied().collect();
// [1, 2, 3]   (s'arrête à 8, et ignore tout le reste)
```

Attention : dès que la condition est fausse **une fois**, ça s'arrête — même si
des éléments suivants la satisferaient à nouveau (ici le `2` et le `1` finaux
sont ignorés).

### `skip_while` : ignorer tant que c'est vrai

L'inverse : saute le début, puis garde **tout** à partir du premier échec.

```rust
let reste: Vec<i32> = v.iter().skip_while(|&&x| x < 5).copied().collect();
// [8, 2, 1]
```

### Cousins à index fixe

- `take(n)` : prendre les `n` premiers.
- `skip(n)` : sauter les `n` premiers.

```rust
(1..).take(3);     // 1, 2, 3 (sur une plage infinie !)
[10, 20, 30].iter().skip(1); // 20, 30
```

> 💡 `take(n)` sur une plage **infinie** `(1..)` est un idiome courant et sûr :
> la paresse garantit qu'on n'évalue que ce qu'on consomme.
"#,
            en: r#"
## Cutting an iterator dynamically

### `take_while`: take while it's true

Keeps elements **from the start**, and **stops** at the first that fails.

```rust
let v = [1, 2, 3, 8, 2, 1];
let start: Vec<i32> = v.iter().take_while(|&&x| x < 5).copied().collect();
// [1, 2, 3]   (stops at 8, and ignores the rest)
```

Careful: once the condition is false **once**, it stops — even if later
elements would satisfy it again (here the final `2` and `1` are ignored).

### `skip_while`: skip while it's true

The opposite: skip the start, then keep **everything** from the first failure.

```rust
let rest: Vec<i32> = v.iter().skip_while(|&&x| x < 5).copied().collect();
// [8, 2, 1]
```

### Fixed-index cousins

- `take(n)`: take the first `n`.
- `skip(n)`: skip the first `n`.

```rust
(1..).take(3);     // 1, 2, 3 (on an infinite range!)
[10, 20, 30].iter().skip(1); // 20, 30
```

> 💡 `take(n)` on an **infinite** range `(1..)` is a common, safe idiom:
> laziness guarantees we only evaluate what we consume.
"#,
        },
        task: Bi {
            fr: "Garde les éléments du début tant qu'ils sont < 5, avec take_while. Le programme teste [1, 2, 3, 8, 2] et doit afficher : [1, 2, 3]",
            en: "Keep the leading elements while they are < 5, using take_while. The program tests [1, 2, 3, 8, 2] and must print: [1, 2, 3]",
        },
        starter: r#"fn main() {
    let v = [1, 2, 3, 8, 2];
    // take_while tant que x < 5, puis collect
    let debut: Vec<i32> = v.iter()/* ... */.copied().collect();
    println!("{:?}", debut);
}
"#,
        check: Check::Stdout { expected: "[1, 2, 3]" },
        hints: &[
            Bi { fr: "v.iter().take_while(|&&x| x < 5).copied().collect()", en: "v.iter().take_while(|&&x| x < 5).copied().collect()" },
        ],
        solution: r#"fn main() {
    let v = [1, 2, 3, 8, 2];
    let debut: Vec<i32> = v.iter().take_while(|&&x| x < 5).copied().collect();
    println!("{:?}", debut);
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 69,
        slug: "windows",
        title: Bi { fr: "windows : fenêtres glissantes", en: "windows: sliding windows" },
        subtitle: Bi { fr: "Regarder les éléments par groupes", en: "Look at elements in groups" },
        xp: 210,
        lesson: Bi {
            fr: r#"
## `windows` : des fenêtres qui se chevauchent

Sur une slice, `windows(n)` produit toutes les **sous-tranches consécutives**
de taille `n`, en glissant d'un cran à chaque fois.

```rust
let v = [1, 2, 3, 4];
for f in v.windows(2) {
    println!("{:?}", f);
}
// [1, 2]
// [2, 3]
// [3, 4]
```

Chaque fenêtre est une slice `&[T]`. Parfait pour comparer des **éléments
adjacents** (différences, tendances, doublons consécutifs…).

```rust
// sommes des paires adjacentes
let sommes: Vec<i32> = v.windows(2).map(|f| f[0] + f[1]).collect();
// [3, 5, 7]
```

### Détecter une suite croissante

```rust
let croissant = v.windows(2).all(|f| f[0] <= f[1]); // true
```

> 💡 `windows` **chevauche** (les fenêtres partagent des éléments). Pour des
> groupes **disjoints**, c'est `chunks` (niveau suivant).
"#,
            en: r#"
## `windows`: overlapping windows

On a slice, `windows(n)` yields all **consecutive sub-slices** of size `n`,
sliding by one each time.

```rust
let v = [1, 2, 3, 4];
for f in v.windows(2) {
    println!("{:?}", f);
}
// [1, 2]
// [2, 3]
// [3, 4]
```

Each window is a slice `&[T]`. Perfect for comparing **adjacent** elements
(differences, trends, consecutive duplicates…).

```rust
// sums of adjacent pairs
let sums: Vec<i32> = v.windows(2).map(|f| f[0] + f[1]).collect();
// [3, 5, 7]
```

### Detect an increasing sequence

```rust
let increasing = v.windows(2).all(|f| f[0] <= f[1]); // true
```

> 💡 `windows` **overlaps** (windows share elements). For **disjoint** groups,
> use `chunks` (next level).
"#,
        },
        task: Bi {
            fr: "Calcule la somme de chaque paire d'éléments adjacents avec windows(2). Le programme teste [1, 2, 3, 4] et doit afficher : [3, 5, 7]",
            en: "Compute the sum of each pair of adjacent elements with windows(2). The program tests [1, 2, 3, 4] and must print: [3, 5, 7]",
        },
        starter: r#"fn main() {
    let v = [1, 2, 3, 4];
    // windows(2), puis map sur f[0] + f[1]
    let sommes: Vec<i32> = v.windows(2)/* ... */.collect();
    println!("{:?}", sommes);
}
"#,
        check: Check::Stdout { expected: "[3, 5, 7]" },
        hints: &[
            Bi { fr: "v.windows(2).map(|f| f[0] + f[1]).collect()", en: "v.windows(2).map(|f| f[0] + f[1]).collect()" },
        ],
        solution: r#"fn main() {
    let v = [1, 2, 3, 4];
    let sommes: Vec<i32> = v.windows(2).map(|f| f[0] + f[1]).collect();
    println!("{:?}", sommes);
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 70,
        slug: "chunks",
        title: Bi { fr: "chunks : groupes disjoints", en: "chunks: disjoint groups" },
        subtitle: Bi { fr: "Découper en paquets", en: "Split into packets" },
        xp: 210,
        lesson: Bi {
            fr: r#"
## `chunks` : découper en paquets

`chunks(n)` découpe une slice en morceaux **disjoints** de taille `n` (le
dernier peut être plus petit s'il ne reste pas assez d'éléments).

```rust
let v = [1, 2, 3, 4, 5];
for c in v.chunks(2) {
    println!("{:?}", c);
}
// [1, 2]
// [3, 4]
// [5]      <- dernier paquet, incomplet
```

Contrairement à `windows`, les paquets **ne se chevauchent pas**.

### Cas pratique : traiter par lots

```rust
// somme de chaque paquet
let sommes: Vec<i32> = v.chunks(2).map(|c| c.iter().sum()).collect();
// [3, 7, 5]
```

`c` est une slice `&[T]`, donc on peut lui appliquer `iter().sum()`, `len()`,
etc.

> 💡 `chunks` est très utile pour paginer, afficher des grilles, ou traiter des
> données par blocs (par exemple 3 valeurs RVB à la fois).
"#,
            en: r#"
## `chunks`: split into packets

`chunks(n)` splits a slice into **disjoint** pieces of size `n` (the last one
may be smaller if there aren't enough elements left).

```rust
let v = [1, 2, 3, 4, 5];
for c in v.chunks(2) {
    println!("{:?}", c);
}
// [1, 2]
// [3, 4]
// [5]      <- last packet, incomplete
```

Unlike `windows`, packets **don't overlap**.

### Practical case: process in batches

```rust
// sum of each packet
let sums: Vec<i32> = v.chunks(2).map(|c| c.iter().sum()).collect();
// [3, 7, 5]
```

`c` is a slice `&[T]`, so you can apply `iter().sum()`, `len()`, etc.

> 💡 `chunks` is great for paging, rendering grids, or processing data in blocks
> (e.g. 3 RGB values at a time).
"#,
        },
        task: Bi {
            fr: "Calcule la somme de chaque paquet de 2 éléments avec chunks(2). Le programme teste [1, 2, 3, 4, 5] et doit afficher : [3, 7, 5]",
            en: "Compute the sum of each packet of 2 elements with chunks(2). The program tests [1, 2, 3, 4, 5] and must print: [3, 7, 5]",
        },
        starter: r#"fn main() {
    let v = [1, 2, 3, 4, 5];
    // chunks(2), puis map sur c.iter().sum()
    let sommes: Vec<i32> = v.chunks(2)/* ... */.collect();
    println!("{:?}", sommes);
}
"#,
        check: Check::Stdout { expected: "[3, 7, 5]" },
        hints: &[
            Bi { fr: "v.chunks(2).map(|c| c.iter().sum()).collect()", en: "v.chunks(2).map(|c| c.iter().sum()).collect()" },
        ],
        solution: r#"fn main() {
    let v = [1, 2, 3, 4, 5];
    let sommes: Vec<i32> = v.chunks(2).map(|c| c.iter().sum()).collect();
    println!("{:?}", sommes);
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 71,
        slug: "split-join",
        title: Bi { fr: "Découper et joindre du texte", en: "Splitting and joining text" },
        subtitle: Bi { fr: "split, collect, join", en: "split, collect, join" },
        xp: 200,
        lesson: Bi {
            fr: r#"
## Manipuler des chaînes : `split` et `join`

### `split` : découper selon un séparateur

`split` renvoie un **itérateur** de sous-chaînes (`&str`).

```rust
let csv = "a,b,c";
let parts: Vec<&str> = csv.split(',').collect();
// ["a", "b", "c"]
```

Variantes utiles :

```rust
"un deux  trois".split_whitespace(); // gère les espaces multiples
"a=b=c".splitn(2, '=');              // au plus 2 morceaux : ["a", "b=c"]
"ligne1\nligne2".lines();            // découpe par lignes
```

### `join` : recoller avec un séparateur

```rust
let mots = ["a", "b", "c"];
let phrase = mots.join("-"); // "a-b-c"
```

### Les enchaîner

```rust
let inverse = "a,b,c"
    .split(',')
    .collect::<Vec<_>>()
    .join("-"); // "a-b-c"
```

### Autres méthodes pratiques

```rust
"  espace  ".trim();        // "espace"
"Rust".replace("R", "Cr");  // "Crust"
"abc".chars().rev().collect::<String>(); // "cba"
```

> 💡 `split` est paresseux et `join` attend une slice de choses affichables.
> Ensemble, ils couvrent l'essentiel du traitement de texte simple.
"#,
            en: r#"
## Working with strings: `split` and `join`

### `split`: cut by a separator

`split` returns an **iterator** of substrings (`&str`).

```rust
let csv = "a,b,c";
let parts: Vec<&str> = csv.split(',').collect();
// ["a", "b", "c"]
```

Useful variants:

```rust
"one two  three".split_whitespace(); // handles multiple spaces
"a=b=c".splitn(2, '=');              // at most 2 pieces: ["a", "b=c"]
"line1\nline2".lines();              // split by lines
```

### `join`: glue back with a separator

```rust
let words = ["a", "b", "c"];
let sentence = words.join("-"); // "a-b-c"
```

### Chaining them

```rust
let rejoined = "a,b,c"
    .split(',')
    .collect::<Vec<_>>()
    .join("-"); // "a-b-c"
```

### Other handy methods

```rust
"  space  ".trim();         // "space"
"Rust".replace("R", "Cr");  // "Crust"
"abc".chars().rev().collect::<String>(); // "cba"
```

> 💡 `split` is lazy and `join` expects a slice of printable things. Together
> they cover most simple text processing.
"#,
        },
        task: Bi {
            fr: "Découpe \"a,b,c\" sur les virgules, puis recolle avec des tirets. Le programme doit afficher : a-b-c",
            en: "Split \"a,b,c\" on commas, then glue back with dashes. The program must print: a-b-c",
        },
        starter: r#"fn main() {
    let csv = "a,b,c";
    // Découpe sur ',', collecte dans un Vec, puis join avec "-"
    let resultat = csv.split(',')/* ... */;
    println!("{}", resultat);
}
"#,
        check: Check::Stdout { expected: "a-b-c" },
        hints: &[
            Bi { fr: "csv.split(',').collect::<Vec<_>>().join(\"-\")", en: "csv.split(',').collect::<Vec<_>>().join(\"-\")" },
        ],
        solution: r#"fn main() {
    let csv = "a,b,c";
    let resultat = csv.split(',').collect::<Vec<_>>().join("-");
    println!("{}", resultat);
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 72,
        slug: "chars",
        title: Bi { fr: "Parcourir les caractères", en: "Iterating characters" },
        subtitle: Bi { fr: "chars, bytes et compagnie", en: "chars, bytes and friends" },
        xp: 200,
        lesson: Bi {
            fr: r#"
## Texte et caractères

Une chaîne Rust est de l'**UTF-8**. Pour la parcourir, deux vues principales :

- `chars()` : les **caractères** Unicode (un `char` peut faire plusieurs
  octets, comme `é` ou `🦀`).
- `bytes()` : les **octets** bruts (`u8`).

```rust
let mot = "café";
mot.chars().count(); // 4 caractères
mot.len();           // 5 octets (le 'é' en fait 2)
```

⚠️ `len()` compte les **octets**, pas les caractères ! Pour compter les
caractères, utilise `chars().count()`.

### Filtrer / transformer des caractères

```rust
// compter les voyelles
let n = "rust".chars().filter(|c| "aeiou".contains(*c)).count(); // 1

// mettre en majuscules manuellement
let crie: String = "rust".chars().map(|c| c.to_ascii_uppercase()).collect();
// "RUST"
```

### Utilitaires sur `char`

```rust
'a'.is_alphabetic();   // true
'5'.is_numeric();      // true
'5'.to_digit(10);      // Some(5)
'a'.to_ascii_uppercase(); // 'A'
```

> 💡 Comme on ne peut pas indexer une chaîne par position (`s[0]` est interdit,
> à cause de l'UTF-8), on passe presque toujours par `chars()` ou des slices
> d'octets.
"#,
            en: r#"
## Text and characters

A Rust string is **UTF-8**. To traverse it, two main views:

- `chars()`: the Unicode **characters** (a `char` can span several bytes, like
  `é` or `🦀`).
- `bytes()`: the raw **bytes** (`u8`).

```rust
let word = "café";
word.chars().count(); // 4 characters
word.len();           // 5 bytes (the 'é' takes 2)
```

⚠️ `len()` counts **bytes**, not characters! To count characters, use
`chars().count()`.

### Filtering / transforming characters

```rust
// count vowels
let n = "rust".chars().filter(|c| "aeiou".contains(*c)).count(); // 1

// uppercase manually
let shout: String = "rust".chars().map(|c| c.to_ascii_uppercase()).collect();
// "RUST"
```

### `char` utilities

```rust
'a'.is_alphabetic();   // true
'5'.is_numeric();      // true
'5'.to_digit(10);      // Some(5)
'a'.to_ascii_uppercase(); // 'A'
```

> 💡 Since you can't index a string by position (`s[0]` is forbidden, due to
> UTF-8), you almost always go through `chars()` or byte slices.
"#,
        },
        task: Bi {
            fr: "Complète `voyelles` qui compte les voyelles (a, e, i, o, u) d'une chaîne. Le programme teste \"rust est genial\" et doit afficher : 5",
            en: "Complete `voyelles` which counts the vowels (a, e, i, o, u) of a string. The program tests \"rust est genial\" and must print: 5",
        },
        starter: r#"fn voyelles(s: &str) -> usize {
    // s.chars().filter(|c| "aeiou".contains(*c)).count()
}

fn main() {
    println!("{}", voyelles("rust est genial"));
}
"#,
        check: Check::Stdout { expected: "5" },
        hints: &[
            Bi { fr: "s.chars().filter(|c| \"aeiou\".contains(*c)).count()", en: "s.chars().filter(|c| \"aeiou\".contains(*c)).count()" },
        ],
        solution: r#"fn voyelles(s: &str) -> usize {
    s.chars().filter(|c| "aeiou".contains(*c)).count()
}

fn main() {
    println!("{}", voyelles("rust est genial"));
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 73,
        slug: "checked-arithmetic",
        title: Bi { fr: "Arithmétique sûre", en: "Safe arithmetic" },
        subtitle: Bi { fr: "Gérer les débordements", en: "Handling overflow" },
        xp: 210,
        lesson: Bi {
            fr: r#"
## Quand les nombres débordent

Chaque type entier a des **bornes**. Un `u8` va de 0 à 255. Que se passe-t-il
si on dépasse ? En mode debug, Rust **panique** ; en mode release, ça
« reboucle » silencieusement. Pour contrôler ça, il existe des méthodes
explicites.

### `checked_*` : renvoie une `Option`

`None` en cas de débordement, `Some(résultat)` sinon.

```rust
200u8.checked_add(100); // None (300 > 255)
200u8.checked_add(50);  // Some(250)
```

### `saturating_*` : reste à la borne

Plafonne (ou plancher) au lieu de déborder.

```rust
200u8.saturating_add(100); // 255 (plafonné)
10u8.saturating_sub(20);   // 0   (planché)
```

### `wrapping_*` : reboucle volontairement

```rust
255u8.wrapping_add(1); // 0 (repart à zéro, comme un compteur kilométrique)
```

### `overflowing_*` : valeur + indicateur

```rust
255u8.overflowing_add(1); // (0, true)  -> le `true` signale le débordement
```

> 💡 `checked_*` est le plus sûr : il te force à décider quoi faire en cas de
> débordement, comme `Option` te force à gérer l'absence.
"#,
            en: r#"
## When numbers overflow

Each integer type has **bounds**. A `u8` goes from 0 to 255. What happens if
you exceed it? In debug mode, Rust **panics**; in release mode, it silently
"wraps". To control this, explicit methods exist.

### `checked_*`: returns an `Option`

`None` on overflow, `Some(result)` otherwise.

```rust
200u8.checked_add(100); // None (300 > 255)
200u8.checked_add(50);  // Some(250)
```

### `saturating_*`: stays at the bound

Caps (or floors) instead of overflowing.

```rust
200u8.saturating_add(100); // 255 (capped)
10u8.saturating_sub(20);   // 0   (floored)
```

### `wrapping_*`: wraps on purpose

```rust
255u8.wrapping_add(1); // 0 (back to zero, like a car odometer)
```

### `overflowing_*`: value + flag

```rust
255u8.overflowing_add(1); // (0, true)  -> the `true` signals overflow
```

> 💡 `checked_*` is the safest: it forces you to decide what to do on overflow,
> just as `Option` forces you to handle absence.
"#,
        },
        task: Bi {
            fr: "Complète `somme_sure` qui renvoie l'addition de deux u8, ou None si ça déborde. Le programme teste 200 + 100 (débordement) et doit afficher : None",
            en: "Complete `somme_sure` returning the sum of two u8, or None if it overflows. The program tests 200 + 100 (overflow) and must print: None",
        },
        starter: r#"fn somme_sure(a: u8, b: u8) -> Option<u8> {
    // Utilise a.checked_add(b)
}

fn main() {
    println!("{:?}", somme_sure(200, 100));
}
"#,
        check: Check::Stdout { expected: "None" },
        hints: &[
            Bi { fr: "Le corps est simplement a.checked_add(b)", en: "The body is just a.checked_add(b)" },
        ],
        solution: r#"fn somme_sure(a: u8, b: u8) -> Option<u8> {
    a.checked_add(b)
}

fn main() {
    println!("{:?}", somme_sure(200, 100));
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 74,
        slug: "match-tuples",
        title: Bi { fr: "match sur les tuples", en: "Matching on tuples" },
        subtitle: Bi { fr: "Filtrer plusieurs valeurs à la fois", en: "Match several values at once" },
        xp: 210,
        lesson: Bi {
            fr: r#"
## Filtrer plusieurs valeurs ensemble

`match` peut décomposer un **tuple**, ce qui permet de tester plusieurs valeurs
d'un seul coup. Très pratique pour les machines à états ou la géométrie.

```rust
fn situer(point: (i32, i32)) -> &'static str {
    match point {
        (0, 0) => "origine",
        (_, 0) => "sur l'axe X",
        (0, _) => "sur l'axe Y",
        (x, y) if x == y => "sur la diagonale",
        _ => "ailleurs",
    }
}
```

Décomposons les motifs :

- `(0, 0)` : les deux valeurs valent exactement 0.
- `(_, 0)` : la seconde vaut 0, la première **peu importe** (`_`).
- `(x, y) if x == y` : on capture les deux **et** on ajoute une garde.
- `_` : tout le reste.

### Exemple : pierre-feuille-ciseaux

```rust
match (moi, toi) {
    ("pierre", "ciseaux") => "gagné",
    ("ciseaux", "papier") => "gagné",
    ("papier", "pierre") => "gagné",
    (a, b) if a == b => "égalité",
    _ => "perdu",
}
```

> 💡 Faire correspondre des tuples remplace avantageusement des cascades de `if`
> imbriqués : tout l'espace des cas est listé clairement, et le compilateur
> vérifie l'exhaustivité.
"#,
            en: r#"
## Matching several values together

`match` can decompose a **tuple**, letting you test several values at once.
Great for state machines or geometry.

```rust
fn situer(point: (i32, i32)) -> &'static str {
    match point {
        (0, 0) => "origine",
        (_, 0) => "sur l'axe X",
        (0, _) => "sur l'axe Y",
        (x, y) if x == y => "sur la diagonale",
        _ => "ailleurs",
    }
}
```

Breaking down the patterns:

- `(0, 0)`: both values are exactly 0.
- `(_, 0)`: the second is 0, the first **doesn't matter** (`_`).
- `(x, y) if x == y`: capture both **and** add a guard.
- `_`: everything else.

### Example: rock-paper-scissors

```rust
match (me, you) {
    ("rock", "scissors") => "win",
    ("scissors", "paper") => "win",
    ("paper", "rock") => "win",
    (a, b) if a == b => "draw",
    _ => "lose",
}
```

> 💡 Matching tuples beats cascades of nested `if`s: the whole case space is
> listed clearly, and the compiler checks exhaustiveness.
"#,
        },
        task: Bi {
            fr: "Complète `situer` : (0,0) → \"origine\", (_,0) → \"axe X\", (0,_) → \"axe Y\", sinon \"ailleurs\". Le programme teste (0, 0) et doit afficher : origine",
            en: "Complete `situer`: (0,0) → \"origine\", (_,0) → \"axe X\", (0,_) → \"axe Y\", otherwise \"ailleurs\". The program tests (0, 0) and must print: origine",
        },
        starter: r#"fn situer(point: (i32, i32)) -> &'static str {
    match point {
        // (0, 0) => "origine",
        // (_, 0) => "axe X",
        // (0, _) => "axe Y",
        // _ => "ailleurs",
    }
}

fn main() {
    println!("{}", situer((0, 0)));
}
"#,
        check: Check::Stdout { expected: "origine" },
        hints: &[
            Bi { fr: "L'ordre compte : place (0, 0) avant (_, 0) et (0, _).", en: "Order matters: put (0, 0) before (_, 0) and (0, _)." },
        ],
        solution: r#"fn situer(point: (i32, i32)) -> &'static str {
    match point {
        (0, 0) => "origine",
        (_, 0) => "axe X",
        (0, _) => "axe Y",
        _ => "ailleurs",
    }
}

fn main() {
    println!("{}", situer((0, 0)));
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 75,
        slug: "fnmut",
        title: Bi { fr: "Closures qui modifient (FnMut)", en: "Mutating closures (FnMut)" },
        subtitle: Bi { fr: "Capturer un état mutable", en: "Capture mutable state" },
        xp: 220,
        lesson: Bi {
            fr: r#"
## Les trois familles de closures

Selon ce qu'une closure fait de son environnement, elle implémente :

- **`Fn`** : elle **lit** seulement (emprunt partagé). On peut l'appeler
  plusieurs fois.
- **`FnMut`** : elle **modifie** une variable capturée (emprunt mutable).
- **`FnOnce`** : elle **consomme** une variable capturée (elle ne peut être
  appelée qu'une fois).

Le compilateur choisit la bonne catégorie tout seul. Mais pour modifier un état
capturé, la closure (et la variable qui la contient) doivent être `mut`.

```rust
let mut total = 0;
let mut ajouter = |x: i32| total += x; // capture total en mutable
ajouter(10);
ajouter(5);
// total vaut 15
```

⚠️ Tant que la closure `ajouter` existe, elle « tient » `total` en emprunt
mutable : tu ne peux pas lire `total` en même temps. On entoure donc souvent la
closure d'un **bloc** pour libérer l'emprunt :

```rust
let mut total = 0;
{
    let mut ajouter = |x: i32| total += x;
    ajouter(10);
    ajouter(5);
} // la closure meurt ici, total est de nouveau libre
println!("{total}"); // 15
```

> 💡 Une `FnMut` est un petit objet à état : pratique pour un compteur, un
> accumulateur, ou un générateur de valeurs successives.
"#,
            en: r#"
## The three families of closures

Depending on what a closure does with its environment, it implements:

- **`Fn`**: it only **reads** (shared borrow). Callable many times.
- **`FnMut`**: it **modifies** a captured variable (mutable borrow).
- **`FnOnce`**: it **consumes** a captured variable (callable only once).

The compiler picks the right category on its own. But to modify captured state,
the closure (and the variable holding it) must be `mut`.

```rust
let mut total = 0;
let mut ajouter = |x: i32| total += x; // captures total mutably
ajouter(10);
ajouter(5);
// total is 15
```

⚠️ As long as the `ajouter` closure exists, it "holds" `total` in a mutable
borrow: you can't read `total` at the same time. So you often wrap the closure
in a **block** to release the borrow:

```rust
let mut total = 0;
{
    let mut ajouter = |x: i32| total += x;
    ajouter(10);
    ajouter(5);
} // the closure dies here, total is free again
println!("{total}"); // 15
```

> 💡 An `FnMut` is a tiny stateful object: handy for a counter, an accumulator,
> or a generator of successive values.
"#,
        },
        task: Bi {
            fr: "Complète la closure `ajouter` (FnMut) pour qu'elle ajoute son argument à `total`. Le programme ajoute 10 puis 5 et doit afficher : 15",
            en: "Complete the `ajouter` closure (FnMut) so it adds its argument to `total`. The program adds 10 then 5 and must print: 15",
        },
        starter: r#"fn main() {
    let mut total = 0;
    {
        // La closure doit ajouter x à total
        let mut ajouter = |x: i32| /* ... */;
        ajouter(10);
        ajouter(5);
    }
    println!("{}", total);
}
"#,
        check: Check::Stdout { expected: "15" },
        hints: &[
            Bi { fr: "Le corps modifie la variable capturée : total += x", en: "The body mutates the captured variable: total += x" },
        ],
        solution: r#"fn main() {
    let mut total = 0;
    {
        let mut ajouter = |x: i32| total += x;
        ajouter(10);
        ajouter(5);
    }
    println!("{}", total);
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 76,
        slug: "custom-error",
        title: Bi { fr: "Erreurs personnalisées", en: "Custom errors" },
        subtitle: Bi { fr: "Un type d'erreur à toi", en: "Your own error type" },
        xp: 220,
        lesson: Bi {
            fr: r#"
## Définir son propre type d'erreur

Plutôt qu'une `String`, une **`enum`** rend les erreurs explicites et faciles à
filtrer. On lui ajoute `Display` pour un message lisible.

```rust
use std::fmt;

enum MonErreur {
    Vide,
    Negatif,
}

impl fmt::Display for MonErreur {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MonErreur::Vide => write!(f, "valeur vide"),
            MonErreur::Negatif => write!(f, "valeur negative"),
        }
    }
}
```

On l'utilise dans un `Result` :

```rust
fn verifier(n: i32) -> Result<i32, MonErreur> {
    if n == 0 {
        Err(MonErreur::Vide)
    } else if n < 0 {
        Err(MonErreur::Negatif)
    } else {
        Ok(n)
    }
}
```

L'appelant peut **filtrer par variante** (`match e { MonErreur::Vide => ... }`)
ou simplement **afficher** le message (`println!("{e}")`).

> 💡 Pour une vraie bibliothèque, on implémente aussi `std::error::Error` et le
> trait `Debug` — mais une enum + `Display` est déjà une excellente base.
"#,
            en: r#"
## Defining your own error type

Rather than a `String`, an **`enum`** makes errors explicit and easy to match.
You add `Display` for a readable message.

```rust
use std::fmt;

enum MonErreur {
    Vide,
    Negatif,
}

impl fmt::Display for MonErreur {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MonErreur::Vide => write!(f, "valeur vide"),
            MonErreur::Negatif => write!(f, "valeur negative"),
        }
    }
}
```

You use it in a `Result`:

```rust
fn verifier(n: i32) -> Result<i32, MonErreur> {
    if n == 0 {
        Err(MonErreur::Vide)
    } else if n < 0 {
        Err(MonErreur::Negatif)
    } else {
        Ok(n)
    }
}
```

The caller can **match by variant** (`match e { MonErreur::Vide => ... }`) or
simply **print** the message (`println!("{e}")`).

> 💡 For a real library, you'd also implement `std::error::Error` and `Debug` —
> but an enum + `Display` is already a great foundation.
"#,
        },
        task: Bi {
            fr: "Complète l'implémentation de `Display` pour `MonErreur`. Le programme appelle verifier(0) et affiche l'erreur, attendu : valeur vide",
            en: "Complete the `Display` implementation for `MonErreur`. The program calls verifier(0) and prints the error, expected: valeur vide",
        },
        starter: r#"use std::fmt;

enum MonErreur {
    Vide,
    Negatif,
}

impl fmt::Display for MonErreur {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // MonErreur::Vide => write!(f, "valeur vide"),
            // MonErreur::Negatif => write!(f, "valeur negative"),
        }
    }
}

fn verifier(n: i32) -> Result<i32, MonErreur> {
    if n == 0 {
        Err(MonErreur::Vide)
    } else if n < 0 {
        Err(MonErreur::Negatif)
    } else {
        Ok(n)
    }
}

fn main() {
    match verifier(0) {
        Ok(v) => println!("{v}"),
        Err(e) => println!("{e}"),
    }
}
"#,
        check: Check::Stdout { expected: "valeur vide" },
        hints: &[
            Bi { fr: "Chaque variante écrit son message : MonErreur::Vide => write!(f, \"valeur vide\"),", en: "Each variant writes its message: MonErreur::Vide => write!(f, \"valeur vide\")," },
        ],
        solution: r#"use std::fmt;

enum MonErreur {
    Vide,
    Negatif,
}

impl fmt::Display for MonErreur {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MonErreur::Vide => write!(f, "valeur vide"),
            MonErreur::Negatif => write!(f, "valeur negative"),
        }
    }
}

fn verifier(n: i32) -> Result<i32, MonErreur> {
    if n == 0 {
        Err(MonErreur::Vide)
    } else if n < 0 {
        Err(MonErreur::Negatif)
    } else {
        Ok(n)
    }
}

fn main() {
    match verifier(0) {
        Ok(v) => println!("{v}"),
        Err(e) => println!("{e}"),
    }
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 77,
        slug: "box-dyn-error",
        title: Bi { fr: "Box<dyn Error>", en: "Box<dyn Error>" },
        subtitle: Bi { fr: "Une erreur « attrape-tout »", en: "A catch-all error" },
        xp: 230,
        lesson: Bi {
            fr: r#"
## `Box<dyn Error>` : le type d'erreur universel

Quand une fonction peut échouer de **plusieurs façons** (parsing, I/O…), définir
une enum pour chaque cas est lourd. Le type `Box<dyn std::error::Error>` accepte
**n'importe quelle** erreur standard.

```rust
use std::error::Error;

fn doubler_texte(s: &str) -> Result<i32, Box<dyn Error>> {
    let n: i32 = s.parse()?; // l'erreur de parse devient Box<dyn Error>
    Ok(n * 2)
}
```

La magie est dans l'opérateur **`?`** : il convertit automatiquement l'erreur
concrète (ici `ParseIntError`) en `Box<dyn Error>`, grâce à une implémentation
de `From`. Tu peux donc mélanger des erreurs de types différents dans la même
fonction.

```rust
fn main() {
    match doubler_texte("42") {
        Ok(v) => println!("{v}"),     // 84
        Err(e) => println!("erreur : {e}"),
    }
}
```

### Quand l'utiliser ?

- Dans les **applications** et les prototypes : simple et flexible.
- Dans une **bibliothèque** publique, on préfère souvent un type d'erreur
  précis (enum) pour que les utilisateurs puissent filtrer les cas.

> 💡 `Box<dyn Error>` est le couteau suisse de la gestion d'erreurs. Combiné à
> `?`, il rend le « chemin heureux » très lisible.
"#,
            en: r#"
## `Box<dyn Error>`: the universal error type

When a function can fail in **several ways** (parsing, I/O…), defining an enum
for each case is heavy. The type `Box<dyn std::error::Error>` accepts **any**
standard error.

```rust
use std::error::Error;

fn doubler_texte(s: &str) -> Result<i32, Box<dyn Error>> {
    let n: i32 = s.parse()?; // the parse error becomes Box<dyn Error>
    Ok(n * 2)
}
```

The magic is in the **`?`** operator: it automatically converts the concrete
error (here `ParseIntError`) into `Box<dyn Error>`, thanks to a `From`
implementation. So you can mix errors of different types in the same function.

```rust
fn main() {
    match doubler_texte("42") {
        Ok(v) => println!("{v}"),     // 84
        Err(e) => println!("error: {e}"),
    }
}
```

### When to use it?

- In **applications** and prototypes: simple and flexible.
- In a public **library**, a precise error type (enum) is often preferred so
  users can match the cases.

> 💡 `Box<dyn Error>` is the Swiss-army knife of error handling. Combined with
> `?`, it keeps the "happy path" very readable.
"#,
        },
        task: Bi {
            fr: "Complète `doubler_texte` : convertis la chaîne en i32 avec ? puis renvoie le double. Le programme teste \"42\" et doit afficher : 84",
            en: "Complete `doubler_texte`: convert the string to i32 with ? then return the double. The program tests \"42\" and must print: 84",
        },
        starter: r#"use std::error::Error;

fn doubler_texte(s: &str) -> Result<i32, Box<dyn Error>> {
    // let n: i32 = s.parse()?;  puis Ok(n * 2)
}

fn main() {
    println!("{}", doubler_texte("42").unwrap());
}
"#,
        check: Check::Stdout { expected: "84" },
        hints: &[
            Bi { fr: "let n: i32 = s.parse()?; Ok(n * 2)", en: "let n: i32 = s.parse()?; Ok(n * 2)" },
        ],
        solution: r#"use std::error::Error;

fn doubler_texte(s: &str) -> Result<i32, Box<dyn Error>> {
    let n: i32 = s.parse()?;
    Ok(n * 2)
}

fn main() {
    println!("{}", doubler_texte("42").unwrap());
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 78,
        slug: "drop",
        title: Bi { fr: "Le trait Drop", en: "The Drop trait" },
        subtitle: Bi { fr: "Du code à la libération d'une valeur", en: "Code when a value is freed" },
        xp: 220,
        lesson: Bi {
            fr: r#"
## `Drop` : agir quand une valeur disparaît

Le trait `Drop` permet d'exécuter du code **automatiquement** quand une valeur
sort de portée (est libérée). C'est l'équivalent d'un « destructeur ».

```rust
struct Garde;

impl Drop for Garde {
    fn drop(&mut self) {
        println!("nettoyage");
    }
}

fn main() {
    let _g = Garde;
    println!("travail");
} // ici _g est libéré -> "nettoyage" s'affiche
```

Sortie :

```text
travail
nettoyage
```

`drop` est appelé **tout seul** à la fin de la portée, dans l'ordre **inverse**
de la création.

### À quoi ça sert vraiment ?

À libérer des ressources sans rien oublier : fermer un fichier, relâcher un
verrou (`MutexGuard` est un `Drop` !), couper une connexion réseau… C'est ce qui
permet à Rust de garantir le nettoyage **sans ramasse-miettes**.

### Libérer plus tôt

```rust
let g = Garde;
drop(g); // force la libération immédiate (fonction std::mem::drop)
```

> 💡 Tu n'appelles jamais `g.drop()` directement (le compilateur l'interdit) :
> tu utilises la fonction libre `drop(g)` si besoin de libérer avant la fin.
"#,
            en: r#"
## `Drop`: act when a value disappears

The `Drop` trait runs code **automatically** when a value goes out of scope (is
freed). It's the equivalent of a "destructor".

```rust
struct Garde;

impl Drop for Garde {
    fn drop(&mut self) {
        println!("nettoyage");
    }
}

fn main() {
    let _g = Garde;
    println!("travail");
} // here _g is freed -> "nettoyage" prints
```

Output:

```text
travail
nettoyage
```

`drop` is called **on its own** at the end of scope, in the **reverse** order of
creation.

### What is it really for?

Releasing resources without forgetting anything: closing a file, releasing a
lock (`MutexGuard` is a `Drop`!), closing a network connection… This is what
lets Rust guarantee cleanup **without a garbage collector**.

### Freeing earlier

```rust
let g = Garde;
drop(g); // force immediate release (the std::mem::drop function)
```

> 💡 You never call `g.drop()` directly (the compiler forbids it): you use the
> free function `drop(g)` if you need to release early.
"#,
        },
        task: Bi {
            fr: "Implémente `drop` pour `Garde` afin qu'il affiche \"nettoyage\". Sortie attendue (deux lignes) : travail / nettoyage",
            en: "Implement `drop` for `Garde` so it prints \"nettoyage\". Expected output (two lines): travail / nettoyage",
        },
        starter: r#"struct Garde;

impl Drop for Garde {
    fn drop(&mut self) {
        // Affiche "nettoyage"
    }
}

fn main() {
    let _g = Garde;
    println!("travail");
}
"#,
        check: Check::Stdout { expected: "travail\nnettoyage" },
        hints: &[
            Bi { fr: "println!(\"nettoyage\");", en: "println!(\"nettoyage\");" },
        ],
        solution: r#"struct Garde;

impl Drop for Garde {
    fn drop(&mut self) {
        println!("nettoyage");
    }
}

fn main() {
    let _g = Garde;
    println!("travail");
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 79,
        slug: "deref",
        title: Bi { fr: "Le trait Deref", en: "The Deref trait" },
        subtitle: Bi { fr: "Créer son propre pointeur intelligent", en: "Build your own smart pointer" },
        xp: 230,
        lesson: Bi {
            fr: r#"
## `Deref` : se comporter comme une référence

Le trait `Deref` définit ce que fait l'opérateur de déréférencement `*` sur ton
type. C'est lui qui permet à `Box<T>`, `Rc<T>`… de se comporter comme la valeur
qu'ils contiennent.

```rust
use std::ops::Deref;

struct MaBox(i32);

impl Deref for MaBox {
    type Target = i32;

    fn deref(&self) -> &i32 {
        &self.0
    }
}

fn main() {
    let b = MaBox(5);
    println!("{}", *b); // 5 — *b appelle b.deref()
}
```

- `type Target` : le type pointé.
- `deref(&self) -> &Target` : renvoie une **référence** vers la valeur interne.
- `*b` est en fait du sucre pour `*(b.deref())`.

### La coercition de déréférencement (*deref coercion*)

Rust insère automatiquement des `deref` quand c'est nécessaire. C'est pour ça
qu'on peut passer un `&String` là où un `&str` est attendu : `String`
implémente `Deref<Target = str>`.

```rust
fn bonjour(nom: &str) { println!("Bonjour {nom}"); }
let s = String::from("Val");
bonjour(&s); // &String -> &str automatiquement
```

> 💡 `Deref` est ce qui rend les pointeurs intelligents « transparents » : tu
> les utilises presque comme la valeur sous-jacente.
"#,
            en: r#"
## `Deref`: behave like a reference

The `Deref` trait defines what the dereference operator `*` does on your type.
It's what lets `Box<T>`, `Rc<T>`… behave like the value they contain.

```rust
use std::ops::Deref;

struct MaBox(i32);

impl Deref for MaBox {
    type Target = i32;

    fn deref(&self) -> &i32 {
        &self.0
    }
}

fn main() {
    let b = MaBox(5);
    println!("{}", *b); // 5 — *b calls b.deref()
}
```

- `type Target`: the pointed-to type.
- `deref(&self) -> &Target`: returns a **reference** to the inner value.
- `*b` is actually sugar for `*(b.deref())`.

### Deref coercion

Rust inserts `deref` calls automatically when needed. That's why you can pass a
`&String` where a `&str` is expected: `String` implements `Deref<Target = str>`.

```rust
fn bonjour(nom: &str) { println!("Bonjour {nom}"); }
let s = String::from("Val");
bonjour(&s); // &String -> &str automatically
```

> 💡 `Deref` is what makes smart pointers "transparent": you use them almost like
> the underlying value.
"#,
        },
        task: Bi {
            fr: "Complète la méthode `deref` de `MaBox` pour qu'elle renvoie une référence vers la valeur interne. Le programme affiche *b, attendu : 5",
            en: "Complete the `deref` method of `MaBox` so it returns a reference to the inner value. The program prints *b, expected: 5",
        },
        starter: r#"use std::ops::Deref;

struct MaBox(i32);

impl Deref for MaBox {
    type Target = i32;

    fn deref(&self) -> &i32 {
        // Renvoie une référence vers la valeur contenue (self.0)
    }
}

fn main() {
    let b = MaBox(5);
    println!("{}", *b);
}
"#,
        check: Check::Stdout { expected: "5" },
        hints: &[
            Bi { fr: "&self.0", en: "&self.0" },
        ],
        solution: r#"use std::ops::Deref;

struct MaBox(i32);

impl Deref for MaBox {
    type Target = i32;

    fn deref(&self) -> &i32 {
        &self.0
    }
}

fn main() {
    let b = MaBox(5);
    println!("{}", *b);
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 80,
        slug: "binary-heap",
        title: Bi { fr: "BinaryHeap : file de priorité", en: "BinaryHeap: priority queue" },
        subtitle: Bi { fr: "Toujours le plus grand d'abord", en: "Always the largest first" },
        xp: 220,
        lesson: Bi {
            fr: r#"
## `BinaryHeap<T>` : un tas de priorité

Un `BinaryHeap` est une file où l'on retire toujours l'élément **le plus
grand** en premier (c'est un *max-heap*), quel que soit l'ordre d'insertion.

```rust
use std::collections::BinaryHeap;

let mut tas = BinaryHeap::new();
tas.push(3);
tas.push(1);
tas.push(4);
tas.push(1);
tas.push(5);

tas.pop(); // Some(5) — toujours le maximum
tas.pop(); // Some(4)
tas.peek(); // Some(&3) — regarde sans retirer
```

- `push(x)` : insère.
- `pop()` : retire et renvoie le **max** (dans une `Option`).
- `peek()` : regarde le max sans le retirer.

### Et pour un *min-heap* ?

Enveloppe les valeurs dans `std::cmp::Reverse` pour inverser l'ordre :

```rust
use std::cmp::Reverse;
let mut tas = BinaryHeap::new();
tas.push(Reverse(3));
tas.push(Reverse(1));
tas.pop(); // Some(Reverse(1)) — le plus petit !
```

> 💡 Le `BinaryHeap` est l'outil idéal pour les algorithmes qui traitent
> toujours « l'élément le plus prioritaire » : Dijkstra, ordonnancement de
> tâches, fusion de flux triés…
"#,
            en: r#"
## `BinaryHeap<T>`: a priority heap

A `BinaryHeap` is a queue where you always remove the **largest** element first
(it's a *max-heap*), regardless of insertion order.

```rust
use std::collections::BinaryHeap;

let mut tas = BinaryHeap::new();
tas.push(3);
tas.push(1);
tas.push(4);
tas.push(1);
tas.push(5);

tas.pop(); // Some(5) — always the maximum
tas.pop(); // Some(4)
tas.peek(); // Some(&3) — look without removing
```

- `push(x)`: insert.
- `pop()`: remove and return the **max** (in an `Option`).
- `peek()`: look at the max without removing it.

### What about a *min-heap*?

Wrap values in `std::cmp::Reverse` to invert the order:

```rust
use std::cmp::Reverse;
let mut tas = BinaryHeap::new();
tas.push(Reverse(3));
tas.push(Reverse(1));
tas.pop(); // Some(Reverse(1)) — the smallest!
```

> 💡 The `BinaryHeap` is ideal for algorithms that always process "the highest
> priority element": Dijkstra, task scheduling, merging sorted streams…
"#,
        },
        task: Bi {
            fr: "Empile chaque valeur dans le tas, puis affiche le maximum retiré avec pop. Le programme empile [3, 1, 4, 1, 5] et doit afficher : 5",
            en: "Push each value into the heap, then print the maximum removed with pop. The program pushes [3, 1, 4, 1, 5] and must print: 5",
        },
        starter: r#"use std::collections::BinaryHeap;

fn main() {
    let mut tas = BinaryHeap::new();
    for x in [3, 1, 4, 1, 5] {
        // Empile x
    }
    println!("{}", tas.pop().unwrap());
}
"#,
        check: Check::Stdout { expected: "5" },
        hints: &[
            Bi { fr: "Dans la boucle : tas.push(x);", en: "In the loop: tas.push(x);" },
        ],
        solution: r#"use std::collections::BinaryHeap;

fn main() {
    let mut tas = BinaryHeap::new();
    for x in [3, 1, 4, 1, 5] {
        tas.push(x);
    }
    println!("{}", tas.pop().unwrap());
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 81,
        slug: "vecdeque",
        title: Bi { fr: "VecDeque : file à deux bouts", en: "VecDeque: double-ended queue" },
        subtitle: Bi { fr: "Ajouter et retirer aux deux extrémités", en: "Add and remove at both ends" },
        xp: 210,
        lesson: Bi {
            fr: r#"
## `VecDeque<T>` : une file efficace

Un `Vec` est rapide à la fin, mais retirer **au début** est coûteux (il faut
tout décaler). `VecDeque` (*double-ended queue*) est efficace aux **deux**
extrémités.

```rust
use std::collections::VecDeque;

let mut file = VecDeque::new();
file.push_back(1);  // ajoute à la fin
file.push_back(2);
file.push_front(0); // ajoute au début
// file : [0, 1, 2]

file.pop_front(); // Some(0) — retire au début
file.pop_back();  // Some(2) — retire à la fin
```

### Une file d'attente (FIFO)

C'est la structure parfaite pour une file « premier entré, premier sorti » :

```rust
let mut file = VecDeque::new();
file.push_back("a"); // arrivée
file.push_back("b");
file.pop_front();    // Some("a") — départ, dans l'ordre d'arrivée
```

- `push_back` / `pop_front` → comportement **FIFO** (file d'attente).
- `push_back` / `pop_back` → comportement **LIFO** (pile).

> 💡 `VecDeque` est le choix par défaut pour les parcours en largeur (BFS), les
> tampons circulaires, ou toute file d'attente de tâches.
"#,
            en: r#"
## `VecDeque<T>`: an efficient queue

A `Vec` is fast at the end, but removing **at the start** is costly (everything
must shift). `VecDeque` (*double-ended queue*) is efficient at **both** ends.

```rust
use std::collections::VecDeque;

let mut file = VecDeque::new();
file.push_back(1);  // add at the end
file.push_back(2);
file.push_front(0); // add at the start
// file: [0, 1, 2]

file.pop_front(); // Some(0) — remove from the front
file.pop_back();  // Some(2) — remove from the back
```

### A FIFO queue

It's the perfect structure for a "first in, first out" queue:

```rust
let mut file = VecDeque::new();
file.push_back("a"); // arrival
file.push_back("b");
file.pop_front();    // Some("a") — departure, in arrival order
```

- `push_back` / `pop_front` → **FIFO** behavior (queue).
- `push_back` / `pop_back` → **LIFO** behavior (stack).

> 💡 `VecDeque` is the default choice for breadth-first searches (BFS), circular
> buffers, or any task queue.
"#,
        },
        task: Bi {
            fr: "Retire le premier élément de la file (comportement FIFO) avec pop_front. Le programme ajoute 1, 2, 3 par l'arrière et doit afficher : 1",
            en: "Remove the first element of the queue (FIFO behavior) with pop_front. The program adds 1, 2, 3 at the back and must print: 1",
        },
        starter: r#"use std::collections::VecDeque;

fn main() {
    let mut file = VecDeque::new();
    file.push_back(1);
    file.push_back(2);
    file.push_back(3);

    // Retire et affiche le premier arrivé
    println!("{}", file./* ... */.unwrap());
}
"#,
        check: Check::Stdout { expected: "1" },
        hints: &[
            Bi { fr: "file.pop_front()", en: "file.pop_front()" },
        ],
        solution: r#"use std::collections::VecDeque;

fn main() {
    let mut file = VecDeque::new();
    file.push_back(1);
    file.push_back(2);
    file.push_back(3);

    println!("{}", file.pop_front().unwrap());
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 82,
        slug: "sort-by-key",
        title: Bi { fr: "Trier selon un critère", en: "Sorting by a criterion" },
        subtitle: Bi { fr: "sort_by sur des structures", en: "sort_by on structures" },
        xp: 220,
        lesson: Bi {
            fr: r#"
## Trier des données complexes

Tu sais trier des nombres. Pour trier des **tuples** ou des **structs**, on
indique le **critère** avec `sort_by_key` ou `sort_by`.

### `sort_by_key` : trier selon une valeur extraite

```rust
let mut scores = vec![("Ana", 30), ("Bob", 50), ("Cleo", 10)];
scores.sort_by_key(|p| p.1); // trie par le 2e champ (le nombre)
// [("Cleo", 10), ("Ana", 30), ("Bob", 50)]
```

La closure renvoie la **clé** sur laquelle comparer ; le tri est croissant.

### `sort_by` : une comparaison sur mesure

Pour un ordre **décroissant**, on compare à l'envers :

```rust
scores.sort_by(|a, b| b.1.cmp(&a.1)); // décroissant par le nombre
// [("Bob", 50), ("Ana", 30), ("Cleo", 10)]
```

`a.cmp(&b)` renvoie un `Ordering` (`Less`/`Equal`/`Greater`). En écrivant
`b.cmp(&a)`, on inverse le sens.

### Pour les flottants

`f64` n'a pas d'ordre total (à cause de `NaN`), donc `sort` ne marche pas
directement. On utilise :

```rust
let mut v = vec![3.1, 1.2, 2.3];
v.sort_by(|a, b| a.partial_cmp(b).unwrap());
```

> 💡 `sort_by_key` est le plus lisible quand un simple champ suffit. `sort_by`
> couvre les cas complexes (ordre inversé, critères multiples…).
"#,
            en: r#"
## Sorting complex data

You can sort numbers. To sort **tuples** or **structs**, you specify the
**criterion** with `sort_by_key` or `sort_by`.

### `sort_by_key`: sort by an extracted value

```rust
let mut scores = vec![("Ana", 30), ("Bob", 50), ("Cleo", 10)];
scores.sort_by_key(|p| p.1); // sort by the 2nd field (the number)
// [("Cleo", 10), ("Ana", 30), ("Bob", 50)]
```

The closure returns the **key** to compare on; the sort is ascending.

### `sort_by`: a custom comparison

For **descending** order, compare the other way:

```rust
scores.sort_by(|a, b| b.1.cmp(&a.1)); // descending by the number
// [("Bob", 50), ("Ana", 30), ("Cleo", 10)]
```

`a.cmp(&b)` returns an `Ordering` (`Less`/`Equal`/`Greater`). Writing
`b.cmp(&a)` reverses the direction.

### For floats

`f64` has no total order (because of `NaN`), so `sort` doesn't work directly.
Use:

```rust
let mut v = vec![3.1, 1.2, 2.3];
v.sort_by(|a, b| a.partial_cmp(b).unwrap());
```

> 💡 `sort_by_key` is most readable when a single field is enough. `sort_by`
> covers complex cases (reversed order, multiple criteria…).
"#,
        },
        task: Bi {
            fr: "Trie les scores par nombre DÉCROISSANT, puis affiche le nom du premier. Sur ces données, le plus haut score est Bob, attendu : Bob",
            en: "Sort the scores by number DESCENDING, then print the first name. On this data, the highest score is Bob, expected: Bob",
        },
        starter: r#"fn main() {
    let mut scores = vec![("Ana", 30), ("Bob", 50), ("Cleo", 10)];
    // Trie par le 2e champ (.1) en ordre décroissant avec sort_by
    scores.sort_by(|a, b| /* ... */);
    println!("{}", scores[0].0);
}
"#,
        check: Check::Stdout { expected: "Bob" },
        hints: &[
            Bi { fr: "b.1.cmp(&a.1) compare dans l'ordre décroissant.", en: "b.1.cmp(&a.1) compares in descending order." },
        ],
        solution: r#"fn main() {
    let mut scores = vec![("Ana", 30), ("Bob", 50), ("Cleo", 10)];
    scores.sort_by(|a, b| b.1.cmp(&a.1));
    println!("{}", scores[0].0);
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 83,
        slug: "macros",
        title: Bi { fr: "Les macros (macro_rules!)", en: "Macros (macro_rules!)" },
        subtitle: Bi { fr: "Écrire du code qui écrit du code", en: "Write code that writes code" },
        xp: 230,
        lesson: Bi {
            fr: r#"
## Écrire ses propres macros

Tu utilises des macros depuis le début : `println!`, `vec!`, `format!`… (le `!`
les distingue des fonctions). On peut en **créer** avec `macro_rules!`.

Une macro fonctionne par **motif → expansion** : elle reçoit des bouts de code
et produit du code, **à la compilation**.

```rust
macro_rules! carre {
    ($x:expr) => {
        $x * $x
    };
}

fn main() {
    println!("{}", carre!(5)); // 25
}
```

- `($x:expr)` : le motif. `$x` capture une **expression**.
- `=> { ... }` : ce que la macro produit, en réinjectant `$x`.

### Plusieurs règles, et la répétition

Une macro peut avoir plusieurs branches et gérer un **nombre variable**
d'arguments avec `$(...),*` :

```rust
macro_rules! somme {
    ($($x:expr),*) => {
        0 $(+ $x)*
    };
}
somme!(1, 2, 3); // 0 + 1 + 2 + 3 = 6
```

### Macro vs fonction

Une macro agit **sur la syntaxe** (avant la vérification de types) : elle peut
accepter un nombre variable d'arguments, générer des structs, etc. — des choses
impossibles pour une fonction.

> 💡 Les macros sont puissantes mais plus difficiles à lire et déboguer.
> Privilégie une fonction quand elle suffit ; réserve les macros aux cas où la
> syntaxe doit varier.
"#,
            en: r#"
## Writing your own macros

You've used macros from the start: `println!`, `vec!`, `format!`… (the `!`
distinguishes them from functions). You can **create** some with `macro_rules!`.

A macro works by **pattern → expansion**: it receives bits of code and produces
code, **at compile time**.

```rust
macro_rules! carre {
    ($x:expr) => {
        $x * $x
    };
}

fn main() {
    println!("{}", carre!(5)); // 25
}
```

- `($x:expr)`: the pattern. `$x` captures an **expression**.
- `=> { ... }`: what the macro produces, re-injecting `$x`.

### Multiple rules, and repetition

A macro can have several branches and handle a **variable number** of arguments
with `$(...),*`:

```rust
macro_rules! somme {
    ($($x:expr),*) => {
        0 $(+ $x)*
    };
}
somme!(1, 2, 3); // 0 + 1 + 2 + 3 = 6
```

### Macro vs function

A macro acts **on syntax** (before type checking): it can accept a variable
number of arguments, generate structs, etc. — things impossible for a function.

> 💡 Macros are powerful but harder to read and debug. Prefer a function when it
> suffices; reserve macros for cases where the syntax must vary.
"#,
        },
        task: Bi {
            fr: "Complète le corps de la macro `carre!` pour qu'elle produise le carré de son argument. Le programme teste carre!(5) et doit afficher : 25",
            en: "Complete the body of the `carre!` macro so it produces the square of its argument. The program tests carre!(5) and must print: 25",
        },
        starter: r#"macro_rules! carre {
    ($x:expr) => {
        // Produit $x multiplié par lui-même
    };
}

fn main() {
    println!("{}", carre!(5));
}
"#,
        check: Check::Stdout { expected: "25" },
        hints: &[
            Bi { fr: "$x * $x", en: "$x * $x" },
        ],
        solution: r#"macro_rules! carre {
    ($x:expr) => {
        $x * $x
    };
}

fn main() {
    println!("{}", carre!(5));
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 84,
        slug: "impl-trait-arg",
        title: Bi { fr: "impl Trait en argument", en: "impl Trait as argument" },
        subtitle: Bi { fr: "Accepter « n'importe quoi qui sait faire »", en: "Accept \"anything that can\"" },
        xp: 220,
        lesson: Bi {
            fr: r#"
## `impl Trait` en position d'argument

On a vu `impl Trait` en **retour** de fonction. On peut aussi l'utiliser pour
un **paramètre** : « accepte n'importe quel type qui implémente ce trait ».

```rust
fn somme(items: impl Iterator<Item = i32>) -> i32 {
    items.sum()
}

somme(vec![1, 2, 3].into_iter()); // 6
somme(1..=4);                     // 10  (une plage est un itérateur)
```

`impl Iterator<Item = i32>` est un raccourci pour un générique :

```rust
// strictement équivalent :
fn somme<T: Iterator<Item = i32>>(items: T) -> i32 {
    items.sum()
}
```

### Très courant avec Display

```rust
fn afficher(x: impl std::fmt::Display) {
    println!("-> {x}");
}
afficher(42);
afficher("coucou");
```

### `impl Trait` vs `dyn Trait`

- `impl Trait` en argument : **un seul** type concret par appel, choisi à la
  compilation (rapide, *statique*).
- `&dyn Trait` : le type peut varier à l'exécution (dynamique). Utile pour
  stocker des types hétérogènes (`Vec<Box<dyn Trait>>`).

> 💡 `impl Trait` en argument rend les signatures concises et lisibles tout en
> gardant la performance du *dispatch statique*.
"#,
            en: r#"
## `impl Trait` in argument position

We saw `impl Trait` in a function's **return**. You can also use it for a
**parameter**: "accept any type implementing this trait".

```rust
fn somme(items: impl Iterator<Item = i32>) -> i32 {
    items.sum()
}

somme(vec![1, 2, 3].into_iter()); // 6
somme(1..=4);                     // 10  (a range is an iterator)
```

`impl Iterator<Item = i32>` is shorthand for a generic:

```rust
// strictly equivalent:
fn somme<T: Iterator<Item = i32>>(items: T) -> i32 {
    items.sum()
}
```

### Very common with Display

```rust
fn afficher(x: impl std::fmt::Display) {
    println!("-> {x}");
}
afficher(42);
afficher("hi");
```

### `impl Trait` vs `dyn Trait`

- `impl Trait` in argument: **one** concrete type per call, chosen at compile
  time (fast, *static*).
- `&dyn Trait`: the type can vary at runtime (dynamic). Useful to store
  heterogeneous types (`Vec<Box<dyn Trait>>`).

> 💡 `impl Trait` in argument keeps signatures concise and readable while
> keeping *static dispatch* performance.
"#,
        },
        task: Bi {
            fr: "Complète `somme` qui additionne tous les éléments d'un itérateur d'i32. Le programme l'appelle avec vec![1, 2, 3] et doit afficher : 6",
            en: "Complete `somme` which adds all elements of an iterator of i32. The program calls it with vec![1, 2, 3] and must print: 6",
        },
        starter: r#"fn somme(items: impl Iterator<Item = i32>) -> i32 {
    // Additionne tous les éléments
}

fn main() {
    println!("{}", somme(vec![1, 2, 3].into_iter()));
}
"#,
        check: Check::Stdout { expected: "6" },
        hints: &[
            Bi { fr: "items.sum()", en: "items.sum()" },
        ],
        solution: r#"fn somme(items: impl Iterator<Item = i32>) -> i32 {
    items.sum()
}

fn main() {
    println!("{}", somme(vec![1, 2, 3].into_iter()));
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 85,
        slug: "supertraits",
        title: Bi { fr: "Les supertraits", en: "Supertraits" },
        subtitle: Bi { fr: "Un trait qui en exige un autre", en: "A trait requiring another" },
        xp: 230,
        lesson: Bi {
            fr: r#"
## Les supertraits : bâtir sur un autre trait

Un trait peut **exiger** qu'un autre soit implémenté. On dit que le second est
un **supertrait** du premier. Cela permet à un trait d'**utiliser** les méthodes
de son supertrait.

```rust
trait Nomme {
    fn nom(&self) -> String;
}

// Salue exige Nomme : on l'écrit "Salue: Nomme"
trait Salue: Nomme {
    fn salue(&self) -> String {
        // on peut appeler nom(), garanti par le supertrait
        format!("Salut {}", self.nom())
    }
}
```

Pour qu'un type soit `Salue`, il doit **aussi** être `Nomme` :

```rust
struct Personne {
    prenom: String,
}

impl Nomme for Personne {
    fn nom(&self) -> String {
        self.prenom.clone()
    }
}

impl Salue for Personne {} // salue() est fourni par défaut

let p = Personne { prenom: String::from("Bob") };
p.salue(); // "Salut Bob"
```

> 💡 Les supertraits modélisent une hiérarchie de capacités : « pour saluer, il
> faut d'abord savoir donner son nom ». C'est l'équivalent Rust de l'héritage
> d'interfaces.
"#,
            en: r#"
## Supertraits: building on another trait

A trait can **require** another to be implemented. The second is called a
**supertrait** of the first. This lets a trait **use** its supertrait's methods.

```rust
trait Nomme {
    fn nom(&self) -> String;
}

// Salue requires Nomme: written "Salue: Nomme"
trait Salue: Nomme {
    fn salue(&self) -> String {
        // we can call nom(), guaranteed by the supertrait
        format!("Salut {}", self.nom())
    }
}
```

For a type to be `Salue`, it must **also** be `Nomme`:

```rust
struct Personne {
    prenom: String,
}

impl Nomme for Personne {
    fn nom(&self) -> String {
        self.prenom.clone()
    }
}

impl Salue for Personne {} // salue() is provided by default

let p = Personne { prenom: String::from("Bob") };
p.salue(); // "Salut Bob"
```

> 💡 Supertraits model a hierarchy of capabilities: "to greet, you must first be
> able to give your name". It's Rust's equivalent of interface inheritance.
"#,
        },
        task: Bi {
            fr: "Implémente `Nomme` pour `Personne` (renvoie le prénom). La méthode `salue` du trait `Salue` fera le reste. Le programme teste \"Bob\" et doit afficher : Salut Bob",
            en: "Implement `Nomme` for `Personne` (return the first name). The `salue` method of the `Salue` trait does the rest. The program tests \"Bob\" and must print: Salut Bob",
        },
        starter: r#"trait Nomme {
    fn nom(&self) -> String;
}

trait Salue: Nomme {
    fn salue(&self) -> String {
        format!("Salut {}", self.nom())
    }
}

struct Personne {
    prenom: String,
}

impl Nomme for Personne {
    // Implémente nom(&self) -> String
}

impl Salue for Personne {}

fn main() {
    let p = Personne { prenom: String::from("Bob") };
    println!("{}", p.salue());
}
"#,
        check: Check::Stdout { expected: "Salut Bob" },
        hints: &[
            Bi { fr: "fn nom(&self) -> String { self.prenom.clone() }", en: "fn nom(&self) -> String { self.prenom.clone() }" },
        ],
        solution: r#"trait Nomme {
    fn nom(&self) -> String;
}

trait Salue: Nomme {
    fn salue(&self) -> String {
        format!("Salut {}", self.nom())
    }
}

struct Personne {
    prenom: String,
}

impl Nomme for Personne {
    fn nom(&self) -> String {
        self.prenom.clone()
    }
}

impl Salue for Personne {}

fn main() {
    let p = Personne { prenom: String::from("Bob") };
    println!("{}", p.salue());
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 86,
        slug: "assoc-const",
        title: Bi { fr: "Constantes associées", en: "Associated constants" },
        subtitle: Bi { fr: "Des valeurs attachées à un type", en: "Values attached to a type" },
        xp: 210,
        lesson: Bi {
            fr: r#"
## Les constantes associées

Un trait (ou un bloc `impl`) peut déclarer des **constantes** liées au type, pas
seulement des méthodes.

```rust
trait Forme {
    const COTES: u32; // chaque forme devra fixer sa valeur

    fn decrire(&self) -> String {
        format!("J'ai {} côtés", Self::COTES)
    }
}

struct Triangle;
struct Carre;

impl Forme for Triangle {
    const COTES: u32 = 3;
}

impl Forme for Carre {
    const COTES: u32 = 4;
}
```

On accède à la constante avec `Type::CONST` :

```rust
println!("{}", Triangle::COTES); // 3
println!("{}", Carre::COTES);    // 4
```

Dans une méthode du trait, on utilise `Self::COTES` pour désigner la constante
du type concret.

### Constantes dans un simple `impl`

Pas besoin de trait pour ça :

```rust
struct Cercle;
impl Cercle {
    const PI: f64 = 3.14159;
}
```

> 💡 Les constantes associées sont parfaites pour des paramètres intrinsèques à
> un type (nombre de côtés, capacité max, version…), connus à la compilation.
"#,
            en: r#"
## Associated constants

A trait (or an `impl` block) can declare **constants** tied to the type, not
just methods.

```rust
trait Forme {
    const COTES: u32; // each shape will set its value

    fn decrire(&self) -> String {
        format!("J'ai {} côtés", Self::COTES)
    }
}

struct Triangle;
struct Carre;

impl Forme for Triangle {
    const COTES: u32 = 3;
}

impl Forme for Carre {
    const COTES: u32 = 4;
}
```

You access the constant with `Type::CONST`:

```rust
println!("{}", Triangle::COTES); // 3
println!("{}", Carre::COTES);    // 4
```

Inside a trait method, use `Self::COTES` to refer to the concrete type's
constant.

### Constants in a plain `impl`

No trait needed for that:

```rust
struct Cercle;
impl Cercle {
    const PI: f64 = 3.14159;
}
```

> 💡 Associated constants are perfect for parameters intrinsic to a type (number
> of sides, max capacity, version…), known at compile time.
"#,
        },
        task: Bi {
            fr: "Donne à la constante `COTES` du `Triangle` la bonne valeur. Le programme affiche Triangle::COTES, attendu : 3",
            en: "Give the `Triangle`'s `COTES` constant the right value. The program prints Triangle::COTES, expected: 3",
        },
        starter: r#"trait Forme {
    const COTES: u32;
}

struct Triangle;

impl Forme for Triangle {
    // Définis la constante COTES
}

fn main() {
    println!("{}", Triangle::COTES);
}
"#,
        check: Check::Stdout { expected: "3" },
        hints: &[
            Bi { fr: "const COTES: u32 = 3;", en: "const COTES: u32 = 3;" },
        ],
        solution: r#"trait Forme {
    const COTES: u32;
}

struct Triangle;

impl Forme for Triangle {
    const COTES: u32 = 3;
}

fn main() {
    println!("{}", Triangle::COTES);
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 87,
        slug: "tests",
        title: Bi { fr: "Écrire des tests", en: "Writing tests" },
        subtitle: Bi { fr: "assert_eq! et compagnie", en: "assert_eq! and friends" },
        xp: 220,
        lesson: Bi {
            fr: r#"
## Tester son code

Rust intègre les tests dans le langage. Les macros d'assertion vérifient qu'une
condition est vraie, et **paniquent** sinon.

```rust
assert!(2 + 2 == 4);            // vrai : ok
assert_eq!(2 + 2, 4);          // compare deux valeurs égales
assert_ne!(2 + 2, 5);          // vérifie qu'elles diffèrent
```

`assert_eq!` est le plus utile : en cas d'échec, il **affiche les deux valeurs**
(`left` et `right`), ce qui rend le diagnostic immédiat.

### Les vraies fonctions de test

Dans un projet, on marque les fonctions de test avec `#[test]`. La commande
`cargo test` les exécute toutes :

```rust
fn addition(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn test_addition() {
    assert_eq!(addition(2, 3), 5);
    assert_eq!(addition(-1, 1), 0);
}
```

Chaque `#[test]` réussit si aucune assertion ne panique. C'est la base du
développement fiable : on décrit le comportement attendu, et le compilateur +
les tests le garantissent.

> 💡 Ici, comme on exécute un seul `main`, on met les `assert_eq!` directement
> dedans : s'ils passent tous, le programme atteint le `println!` final.
"#,
            en: r#"
## Testing your code

Rust builds testing into the language. Assertion macros check a condition is
true, and **panic** otherwise.

```rust
assert!(2 + 2 == 4);            // true: ok
assert_eq!(2 + 2, 4);          // compares two equal values
assert_ne!(2 + 2, 5);          // checks they differ
```

`assert_eq!` is the most useful: on failure it **prints both values** (`left`
and `right`), making diagnosis immediate.

### Real test functions

In a project, you mark test functions with `#[test]`. The `cargo test` command
runs them all:

```rust
fn addition(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn test_addition() {
    assert_eq!(addition(2, 3), 5);
    assert_eq!(addition(-1, 1), 0);
}
```

Each `#[test]` passes if no assertion panics. It's the foundation of reliable
development: you describe the expected behavior, and the compiler + tests
guarantee it.

> 💡 Here, since we run a single `main`, we put the `assert_eq!` directly in it:
> if they all pass, the program reaches the final `println!`.
"#,
        },
        task: Bi {
            fr: "Complète l'assertion : addition(2, 3) doit valoir 5. Si tous les tests passent, le programme affiche : tests ok",
            en: "Complete the assertion: addition(2, 3) must equal 5. If all tests pass, the program prints: tests ok",
        },
        starter: r#"fn addition(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    // Complète la valeur attendue
    assert_eq!(addition(2, 3), /* ... */);
    assert_eq!(addition(10, 10), 20);
    println!("tests ok");
}
"#,
        check: Check::Stdout { expected: "tests ok" },
        hints: &[
            Bi { fr: "2 + 3 vaut 5.", en: "2 + 3 is 5." },
        ],
        solution: r#"fn addition(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    assert_eq!(addition(2, 3), 5);
    assert_eq!(addition(10, 10), 20);
    println!("tests ok");
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 88,
        slug: "projet-tokenizer",
        title: Bi { fr: "Projet calculatrice 1/3 : tokeniser", en: "Calculator project 1/3: tokenize" },
        subtitle: Bi { fr: "Découper une expression en jetons", en: "Split an expression into tokens" },
        xp: 250,
        lesson: Bi {
            fr: r#"
## 🛠️ Mini-projet : une calculatrice

On va construire, en trois niveaux, une **calculatrice** qui évalue des
expressions en **notation polonaise inverse** (RPN). En RPN, l'opérateur vient
**après** ses opérandes : `3 4 +` signifie `3 + 4`. Pas de parenthèses, pas de
priorités : parfait pour s'entraîner !

### Étape 1 : le *tokeniser*

D'abord, transformer le texte `"3 4 +"` en une liste de **jetons** (*tokens*).
On modélise un jeton avec une `enum` :

```rust
#[derive(Debug)]
enum Token {
    Nombre(i32),
    Plus,
    Fois,
}
```

Le tokeniser découpe l'expression sur les espaces et classe chaque morceau :

```rust
fn tokeniser(expr: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    for partie in expr.split_whitespace() {
        let token = match partie {
            "+" => Token::Plus,
            "*" => Token::Fois,
            n => Token::Nombre(n.parse().unwrap()),
        };
        tokens.push(token);
    }
    tokens
}
```

Le dernier bras du `match`, `n => ...`, capture tout le reste (un nombre) et le
convertit avec `parse`.

> 💡 Séparer « lire le texte » (tokeniser) de « calculer » (évaluer) est un
> principe clé : chaque étape est simple et testable isolément.
"#,
            en: r#"
## 🛠️ Mini-project: a calculator

Over three levels, we'll build a **calculator** that evaluates expressions in
**Reverse Polish Notation** (RPN). In RPN, the operator comes **after** its
operands: `3 4 +` means `3 + 4`. No parentheses, no precedence: perfect for
practice!

### Step 1: the tokenizer

First, turn the text `"3 4 +"` into a list of **tokens**. We model a token with
an `enum`:

```rust
#[derive(Debug)]
enum Token {
    Nombre(i32),
    Plus,
    Fois,
}
```

The tokenizer splits the expression on spaces and classifies each piece:

```rust
fn tokeniser(expr: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    for partie in expr.split_whitespace() {
        let token = match partie {
            "+" => Token::Plus,
            "*" => Token::Fois,
            n => Token::Nombre(n.parse().unwrap()),
        };
        tokens.push(token);
    }
    tokens
}
```

The last `match` arm, `n => ...`, captures everything else (a number) and
converts it with `parse`.

> 💡 Separating "reading the text" (tokenize) from "computing" (evaluate) is a
> key principle: each step is simple and testable in isolation.
"#,
        },
        task: Bi {
            fr: "Complète le `match` du tokeniser : \"+\" → Plus, \"*\" → Fois, sinon un Nombre. Le programme tokenise \"3 4 +\" et doit afficher : [Nombre(3), Nombre(4), Plus]",
            en: "Complete the tokenizer's `match`: \"+\" → Plus, \"*\" → Fois, otherwise a Nombre. The program tokenizes \"3 4 +\" and must print: [Nombre(3), Nombre(4), Plus]",
        },
        starter: r#"#[derive(Debug)]
enum Token {
    Nombre(i32),
    Plus,
    Fois,
}

fn tokeniser(expr: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    for partie in expr.split_whitespace() {
        let token = match partie {
            // "+" => Token::Plus,
            // "*" => Token::Fois,
            // n => Token::Nombre(n.parse().unwrap()),
        };
        tokens.push(token);
    }
    tokens
}

fn main() {
    println!("{:?}", tokeniser("3 4 +"));
}
"#,
        check: Check::Stdout { expected: "[Nombre(3), Nombre(4), Plus]" },
        hints: &[
            Bi { fr: "Le dernier bras attrape un nombre : n => Token::Nombre(n.parse().unwrap()),", en: "The last arm catches a number: n => Token::Nombre(n.parse().unwrap())," },
        ],
        solution: r#"#[derive(Debug)]
enum Token {
    Nombre(i32),
    Plus,
    Fois,
}

fn tokeniser(expr: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    for partie in expr.split_whitespace() {
        let token = match partie {
            "+" => Token::Plus,
            "*" => Token::Fois,
            n => Token::Nombre(n.parse().unwrap()),
        };
        tokens.push(token);
    }
    tokens
}

fn main() {
    println!("{:?}", tokeniser("3 4 +"));
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 89,
        slug: "projet-eval",
        title: Bi { fr: "Projet calculatrice 2/3 : évaluer", en: "Calculator project 2/3: evaluate" },
        subtitle: Bi { fr: "Une pile pour calculer", en: "A stack to compute" },
        xp: 270,
        lesson: Bi {
            fr: r#"
## Étape 2 : évaluer avec une pile

La notation RPN s'évalue avec une **pile** (un `Vec` qu'on utilise par le
sommet). L'algorithme est élégant :

- un **nombre** → on l'**empile** ;
- un **opérateur** → on **dépile deux** valeurs, on calcule, on **rempile** le
  résultat.

À la fin, la pile contient une seule valeur : le résultat.

```rust
fn evaluer(tokens: &[Token]) -> i32 {
    let mut pile: Vec<i32> = Vec::new();
    for token in tokens {
        match token {
            Token::Nombre(n) => pile.push(*n),
            Token::Plus => {
                let b = pile.pop().unwrap();
                let a = pile.pop().unwrap();
                pile.push(a + b);
            }
            Token::Fois => {
                let b = pile.pop().unwrap();
                let a = pile.pop().unwrap();
                pile.push(a * b);
            }
        }
    }
    pile.pop().unwrap()
}
```

Déroulé pour `3 4 +` :

| jeton | action | pile |
|---|---|---|
| 3 | empile | [3] |
| 4 | empile | [3, 4] |
| + | dépile 4 et 3, empile 7 | [7] |

⚠️ L'**ordre** compte pour la soustraction/division : on dépile `b` puis `a`,
et on calcule `a OP b` (pas `b OP a`).

> 💡 Cette « machine à pile » est exactement le principe des vraies machines
> virtuelles (JVM, WebAssembly…). Tu construis un mini-interpréteur !
"#,
            en: r#"
## Step 2: evaluate with a stack

RPN evaluates with a **stack** (a `Vec` used by its top). The algorithm is
elegant:

- a **number** → **push** it;
- an **operator** → **pop two** values, compute, **push** the result back.

At the end, the stack holds a single value: the result.

```rust
fn evaluer(tokens: &[Token]) -> i32 {
    let mut pile: Vec<i32> = Vec::new();
    for token in tokens {
        match token {
            Token::Nombre(n) => pile.push(*n),
            Token::Plus => {
                let b = pile.pop().unwrap();
                let a = pile.pop().unwrap();
                pile.push(a + b);
            }
            Token::Fois => {
                let b = pile.pop().unwrap();
                let a = pile.pop().unwrap();
                pile.push(a * b);
            }
        }
    }
    pile.pop().unwrap()
}
```

Trace for `3 4 +`:

| token | action | stack |
|---|---|---|
| 3 | push | [3] |
| 4 | push | [3, 4] |
| + | pop 4 and 3, push 7 | [7] |

⚠️ **Order** matters for subtraction/division: pop `b` then `a`, and compute
`a OP b` (not `b OP a`).

> 💡 This "stack machine" is exactly how real virtual machines work (JVM,
> WebAssembly…). You're building a mini-interpreter!
"#,
        },
        task: Bi {
            fr: "Complète `evaluer` : empile les nombres, et pour Plus / Fois dépile deux valeurs et empile le résultat. Le programme évalue \"3 4 +\" et doit afficher : 7",
            en: "Complete `evaluer`: push numbers, and for Plus / Fois pop two values and push the result. The program evaluates \"3 4 +\" and must print: 7",
        },
        starter: r#"#[derive(Debug)]
enum Token {
    Nombre(i32),
    Plus,
    Fois,
}

fn tokeniser(expr: &str) -> Vec<Token> {
    expr.split_whitespace()
        .map(|partie| match partie {
            "+" => Token::Plus,
            "*" => Token::Fois,
            n => Token::Nombre(n.parse().unwrap()),
        })
        .collect()
}

fn evaluer(tokens: &[Token]) -> i32 {
    let mut pile: Vec<i32> = Vec::new();
    for token in tokens {
        match token {
            Token::Nombre(n) => pile.push(*n),
            Token::Plus => {
                // dépile b puis a, empile a + b
            }
            Token::Fois => {
                // dépile b puis a, empile a * b
            }
        }
    }
    pile.pop().unwrap()
}

fn main() {
    let tokens = tokeniser("3 4 +");
    println!("{}", evaluer(&tokens));
}
"#,
        check: Check::Stdout { expected: "7" },
        hints: &[
            Bi { fr: "let b = pile.pop().unwrap(); let a = pile.pop().unwrap(); pile.push(a + b);", en: "let b = pile.pop().unwrap(); let a = pile.pop().unwrap(); pile.push(a + b);" },
        ],
        solution: r#"#[derive(Debug)]
enum Token {
    Nombre(i32),
    Plus,
    Fois,
}

fn tokeniser(expr: &str) -> Vec<Token> {
    expr.split_whitespace()
        .map(|partie| match partie {
            "+" => Token::Plus,
            "*" => Token::Fois,
            n => Token::Nombre(n.parse().unwrap()),
        })
        .collect()
}

fn evaluer(tokens: &[Token]) -> i32 {
    let mut pile: Vec<i32> = Vec::new();
    for token in tokens {
        match token {
            Token::Nombre(n) => pile.push(*n),
            Token::Plus => {
                let b = pile.pop().unwrap();
                let a = pile.pop().unwrap();
                pile.push(a + b);
            }
            Token::Fois => {
                let b = pile.pop().unwrap();
                let a = pile.pop().unwrap();
                pile.push(a * b);
            }
        }
    }
    pile.pop().unwrap()
}

fn main() {
    let tokens = tokeniser("3 4 +");
    println!("{}", evaluer(&tokens));
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 90,
        slug: "projet-pipeline",
        title: Bi { fr: "Projet calculatrice 3/3 : le tout", en: "Calculator project 3/3: all together" },
        subtitle: Bi { fr: "Assembler tokeniser + évaluer", en: "Assemble tokenize + evaluate" },
        xp: 280,
        lesson: Bi {
            fr: r#"
## Étape 3 : tout assembler

Le tokeniser et l'évaluateur sont prêts (et gèrent maintenant les **quatre**
opérations `+ - * /`). Il ne reste qu'à les **enchaîner** : texte → jetons →
résultat.

```rust
let resultat = evaluer(&tokeniser(expr));
```

C'est la beauté des petites fonctions composables : chacune fait une chose, et
on les branche l'une dans l'autre.

### Une expression plus costaude

Testons `"5 1 2 + 4 * + 3 -"`, qui correspond, en notation classique, à :

```text
5 + ((1 + 2) * 4) - 3
= 5 + (3 * 4) - 3
= 5 + 12 - 3
= 14
```

Déroulé de la pile :

| jeton | pile |
|---|---|
| 5 1 2 | [5, 1, 2] |
| + | [5, 3] |
| 4 | [5, 3, 4] |
| * | [5, 12] |
| + | [17] |
| 3 | [17, 3] |
| - | [14] |

Et voilà : ta calculatrice évalue des expressions arbitraires ! 🎉

> 💡 Félicitations, tu as écrit un mini-interpréteur complet. Pour aller plus
> loin : gérer les erreurs (pile vide → `Result`), lire l'entrée au clavier,
> ajouter la notation infixe avec un *parseur*…
"#,
            en: r#"
## Step 3: put it all together

The tokenizer and evaluator are ready (and now handle the **four** operations
`+ - * /`). All that's left is to **chain** them: text → tokens → result.

```rust
let resultat = evaluer(&tokeniser(expr));
```

That's the beauty of small composable functions: each does one thing, and you
plug them into each other.

### A heftier expression

Let's test `"5 1 2 + 4 * + 3 -"`, which in classic notation is:

```text
5 + ((1 + 2) * 4) - 3
= 5 + (3 * 4) - 3
= 5 + 12 - 3
= 14
```

Stack trace:

| token | stack |
|---|---|
| 5 1 2 | [5, 1, 2] |
| + | [5, 3] |
| 4 | [5, 3, 4] |
| * | [5, 12] |
| + | [17] |
| 3 | [17, 3] |
| - | [14] |

There you go: your calculator evaluates arbitrary expressions! 🎉

> 💡 Congrats, you wrote a full mini-interpreter. To go further: handle errors
> (empty stack → `Result`), read keyboard input, add infix notation with a
> *parser*…
"#,
        },
        task: Bi {
            fr: "Complète `main` : tokenise puis évalue l'expression. Le programme évalue \"5 1 2 + 4 * + 3 -\" et doit afficher : 14",
            en: "Complete `main`: tokenize then evaluate the expression. The program evaluates \"5 1 2 + 4 * + 3 -\" and must print: 14",
        },
        starter: r#"enum Token {
    Nombre(i32),
    Plus,
    Moins,
    Fois,
    Divise,
}

fn tokeniser(expr: &str) -> Vec<Token> {
    expr.split_whitespace()
        .map(|partie| match partie {
            "+" => Token::Plus,
            "-" => Token::Moins,
            "*" => Token::Fois,
            "/" => Token::Divise,
            n => Token::Nombre(n.parse().unwrap()),
        })
        .collect()
}

fn evaluer(tokens: &[Token]) -> i32 {
    let mut pile: Vec<i32> = Vec::new();
    for token in tokens {
        match token {
            Token::Nombre(n) => pile.push(*n),
            Token::Plus => {
                let b = pile.pop().unwrap();
                let a = pile.pop().unwrap();
                pile.push(a + b);
            }
            Token::Moins => {
                let b = pile.pop().unwrap();
                let a = pile.pop().unwrap();
                pile.push(a - b);
            }
            Token::Fois => {
                let b = pile.pop().unwrap();
                let a = pile.pop().unwrap();
                pile.push(a * b);
            }
            Token::Divise => {
                let b = pile.pop().unwrap();
                let a = pile.pop().unwrap();
                pile.push(a / b);
            }
        }
    }
    pile.pop().unwrap()
}

fn main() {
    let expr = "5 1 2 + 4 * + 3 -";
    // Tokenise puis évalue expr
    let resultat = /* ... */;
    println!("{}", resultat);
}
"#,
        check: Check::Stdout { expected: "14" },
        hints: &[
            Bi { fr: "evaluer(&tokeniser(expr))", en: "evaluer(&tokeniser(expr))" },
        ],
        solution: r#"enum Token {
    Nombre(i32),
    Plus,
    Moins,
    Fois,
    Divise,
}

fn tokeniser(expr: &str) -> Vec<Token> {
    expr.split_whitespace()
        .map(|partie| match partie {
            "+" => Token::Plus,
            "-" => Token::Moins,
            "*" => Token::Fois,
            "/" => Token::Divise,
            n => Token::Nombre(n.parse().unwrap()),
        })
        .collect()
}

fn evaluer(tokens: &[Token]) -> i32 {
    let mut pile: Vec<i32> = Vec::new();
    for token in tokens {
        match token {
            Token::Nombre(n) => pile.push(*n),
            Token::Plus => {
                let b = pile.pop().unwrap();
                let a = pile.pop().unwrap();
                pile.push(a + b);
            }
            Token::Moins => {
                let b = pile.pop().unwrap();
                let a = pile.pop().unwrap();
                pile.push(a - b);
            }
            Token::Fois => {
                let b = pile.pop().unwrap();
                let a = pile.pop().unwrap();
                pile.push(a * b);
            }
            Token::Divise => {
                let b = pile.pop().unwrap();
                let a = pile.pop().unwrap();
                pile.push(a / b);
            }
        }
    }
    pile.pop().unwrap()
}

fn main() {
    let expr = "5 1 2 + 4 * + 3 -";
    let resultat = evaluer(&tokeniser(expr));
    println!("{}", resultat);
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 91,
        slug: "vie-afficher",
        title: Bi { fr: "Jeu de la Vie 1/5 : afficher", en: "Game of Life 1/5: render" },
        subtitle: Bi { fr: "Dessiner une grille en ASCII", en: "Draw a grid in ASCII" },
        xp: 260,
        lesson: Bi {
            fr: r#"
## 🌱 Mini-projet : le Jeu de la Vie

Le **Jeu de la Vie** de Conway est un automate cellulaire célèbre : une grille
de cellules **vivantes** ou **mortes** évolue, génération après génération,
selon des règles simples. On va le construire en 5 niveaux.

### La grille

On représente la grille par un `Vec<Vec<bool>>` : `true` = vivante, `false` =
morte.

```rust
let grille = vec![
    vec![true,  false, false],
    vec![false, true,  false],
    vec![false, false, true ],
];
```

### L'afficher

On la transforme en texte : `#` pour une cellule vivante, `.` pour une morte, et
un saut de ligne après chaque rangée.

```rust
fn afficher(grille: &[Vec<bool>]) -> String {
    let mut sortie = String::new();
    for ligne in grille {
        for &cellule in ligne {
            sortie.push(if cellule { '#' } else { '.' });
        }
        sortie.push('\n');
    }
    sortie
}
```

La grille ci-dessus donnera :

```text
#..
.#.
..#
```

> 💡 On commence toujours par **visualiser** : pouvoir afficher l'état rend tout
> le reste (compter, faire évoluer) bien plus facile à déboguer.
"#,
            en: r#"
## 🌱 Mini-project: the Game of Life

Conway's **Game of Life** is a famous cellular automaton: a grid of **alive** or
**dead** cells evolves, generation after generation, following simple rules.
We'll build it over 5 levels.

### The grid

We represent the grid with a `Vec<Vec<bool>>`: `true` = alive, `false` = dead.

```rust
let grille = vec![
    vec![true,  false, false],
    vec![false, true,  false],
    vec![false, false, true ],
];
```

### Rendering it

We turn it into text: `#` for an alive cell, `.` for a dead one, and a newline
after each row.

```rust
fn afficher(grille: &[Vec<bool>]) -> String {
    let mut sortie = String::new();
    for ligne in grille {
        for &cellule in ligne {
            sortie.push(if cellule { '#' } else { '.' });
        }
        sortie.push('\n');
    }
    sortie
}
```

The grid above gives:

```text
#..
.#.
..#
```

> 💡 Always start by **visualizing**: being able to display the state makes
> everything else (counting, evolving) much easier to debug.
"#,
        },
        task: Bi {
            fr: "Complète `afficher` : ajoute '#' si la cellule est vivante, '.' sinon. Le programme affiche une diagonale et doit donner : #.. / .#. / ..#",
            en: "Complete `afficher`: push '#' if the cell is alive, '.' otherwise. The program renders a diagonal and must give: #.. / .#. / ..#",
        },
        starter: r#"fn afficher(grille: &[Vec<bool>]) -> String {
    let mut sortie = String::new();
    for ligne in grille {
        for &cellule in ligne {
            // Ajoute '#' si cellule est vivante, sinon '.'
        }
        sortie.push('\n');
    }
    sortie
}

fn main() {
    let grille = vec![
        vec![true, false, false],
        vec![false, true, false],
        vec![false, false, true],
    ];
    print!("{}", afficher(&grille));
}
"#,
        check: Check::Stdout { expected: "#..\n.#.\n..#" },
        hints: &[
            Bi { fr: "sortie.push(if cellule { '#' } else { '.' });", en: "sortie.push(if cellule { '#' } else { '.' });" },
        ],
        solution: r#"fn afficher(grille: &[Vec<bool>]) -> String {
    let mut sortie = String::new();
    for ligne in grille {
        for &cellule in ligne {
            sortie.push(if cellule { '#' } else { '.' });
        }
        sortie.push('\n');
    }
    sortie
}

fn main() {
    let grille = vec![
        vec![true, false, false],
        vec![false, true, false],
        vec![false, false, true],
    ];
    print!("{}", afficher(&grille));
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 92,
        slug: "vie-voisins",
        title: Bi { fr: "Jeu de la Vie 2/5 : les voisins", en: "Game of Life 2/5: neighbors" },
        subtitle: Bi { fr: "Compter les cellules vivantes autour", en: "Count alive cells around" },
        xp: 270,
        lesson: Bi {
            fr: r#"
## Étape 2 : compter les voisins vivants

Chaque cellule a jusqu'à **8 voisines** (horizontales, verticales, diagonales).
La règle d'évolution dépend de combien sont vivantes. Il faut donc savoir les
compter — en faisant attention aux **bords** de la grille.

```rust
fn voisins_vivants(grille: &[Vec<bool>], x: usize, y: usize) -> usize {
    let mut compte = 0;
    let lignes = grille.len() as i32;
    let colonnes = grille[0].len() as i32;

    for dx in [-1i32, 0, 1] {
        for dy in [-1i32, 0, 1] {
            if dx == 0 && dy == 0 {
                continue; // la cellule elle-même n'est pas sa voisine
            }
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0 && nx < lignes && ny >= 0 && ny < colonnes && grille[nx as usize][ny as usize] {
                compte += 1;
            }
        }
    }
    compte
}
```

Décortiquons :

- Les deux boucles `dx`, `dy ∈ {-1, 0, 1}` balaient les 9 cases du carré 3×3
  centré sur `(x, y)`.
- On **saute** `(0, 0)` (la cellule centrale).
- On calcule la position voisine en `i32` (pour autoriser `-1`), puis on vérifie
  qu'elle est **dans la grille** avant d'y accéder.
- Si la voisine existe **et** est vivante, on incrémente.

> 💡 Le test des bornes `nx >= 0 && nx < lignes && ...` est crucial : sans lui,
> accéder à `grille[-1]` ou `grille[3]` ferait paniquer le programme.
"#,
            en: r#"
## Step 2: count alive neighbors

Each cell has up to **8 neighbors** (horizontal, vertical, diagonal). The
evolution rule depends on how many are alive. So we must count them — minding
the grid **edges**.

```rust
fn voisins_vivants(grille: &[Vec<bool>], x: usize, y: usize) -> usize {
    let mut compte = 0;
    let lignes = grille.len() as i32;
    let colonnes = grille[0].len() as i32;

    for dx in [-1i32, 0, 1] {
        for dy in [-1i32, 0, 1] {
            if dx == 0 && dy == 0 {
                continue; // a cell is not its own neighbor
            }
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0 && nx < lignes && ny >= 0 && ny < colonnes && grille[nx as usize][ny as usize] {
                compte += 1;
            }
        }
    }
    compte
}
```

Breaking it down:

- The two loops `dx`, `dy ∈ {-1, 0, 1}` sweep the 9 cells of the 3×3 square
  centered on `(x, y)`.
- We **skip** `(0, 0)` (the central cell).
- We compute the neighbor position in `i32` (to allow `-1`), then check it's
  **inside the grid** before accessing it.
- If the neighbor exists **and** is alive, we increment.

> 💡 The bounds check `nx >= 0 && nx < lignes && ...` is crucial: without it,
> accessing `grille[-1]` or `grille[3]` would panic.
"#,
        },
        task: Bi {
            fr: "Complète la condition qui incrémente `compte` : la voisine doit être dans la grille ET vivante. Le programme compte les voisins du centre d'un « blinker » vertical et doit afficher : 2",
            en: "Complete the condition that increments `compte`: the neighbor must be inside the grid AND alive. The program counts the neighbors of a vertical \"blinker\" center and must print: 2",
        },
        starter: r#"fn voisins_vivants(grille: &[Vec<bool>], x: usize, y: usize) -> usize {
    let mut compte = 0;
    let lignes = grille.len() as i32;
    let colonnes = grille[0].len() as i32;

    for dx in [-1i32, 0, 1] {
        for dy in [-1i32, 0, 1] {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            // Si (nx, ny) est dans la grille ET que la cellule est vivante : compte += 1
        }
    }
    compte
}

fn main() {
    let grille = vec![
        vec![false, true, false],
        vec![false, true, false],
        vec![false, true, false],
    ];
    println!("{}", voisins_vivants(&grille, 1, 1));
}
"#,
        check: Check::Stdout { expected: "2" },
        hints: &[
            Bi { fr: "if nx >= 0 && nx < lignes && ny >= 0 && ny < colonnes && grille[nx as usize][ny as usize] { compte += 1; }", en: "if nx >= 0 && nx < lignes && ny >= 0 && ny < colonnes && grille[nx as usize][ny as usize] { compte += 1; }" },
        ],
        solution: r#"fn voisins_vivants(grille: &[Vec<bool>], x: usize, y: usize) -> usize {
    let mut compte = 0;
    let lignes = grille.len() as i32;
    let colonnes = grille[0].len() as i32;

    for dx in [-1i32, 0, 1] {
        for dy in [-1i32, 0, 1] {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0 && nx < lignes && ny >= 0 && ny < colonnes && grille[nx as usize][ny as usize] {
                compte += 1;
            }
        }
    }
    compte
}

fn main() {
    let grille = vec![
        vec![false, true, false],
        vec![false, true, false],
        vec![false, true, false],
    ];
    println!("{}", voisins_vivants(&grille, 1, 1));
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 93,
        slug: "vie-regle",
        title: Bi { fr: "Jeu de la Vie 3/5 : la règle", en: "Game of Life 3/5: the rule" },
        subtitle: Bi { fr: "Vivre, mourir ou naître", en: "Live, die or be born" },
        xp: 260,
        lesson: Bi {
            fr: r#"
## Étape 3 : la règle de Conway

Tout le jeu tient en **trois règles**, selon l'état d'une cellule et son nombre
de voisines vivantes :

1. Une cellule **vivante** avec **2 ou 3** voisines **survit**.
2. Une cellule **morte** avec **exactement 3** voisines **naît**.
3. Dans tous les autres cas, la cellule est **morte** à la génération suivante
   (sous-population ou surpopulation).

Un `match` sur le **tuple** `(vivante, voisins)` exprime ça parfaitement :

```rust
fn prochaine(vivante: bool, voisins: usize) -> bool {
    match (vivante, voisins) {
        (true, 2) | (true, 3) => true,  // survie
        (false, 3) => true,             // naissance
        _ => false,                     // mort / reste morte
    }
}
```

- `(true, 2) | (true, 3)` : vivante avec 2 **ou** 3 voisines → reste vivante.
- `(false, 3)` : morte avec 3 voisines → devient vivante.
- `_` : tout le reste → morte.

> 💡 Ces règles ultra-simples produisent une complexité étonnante : oscillateurs,
> vaisseaux qui se déplacent, et même des structures capables de calculer !
"#,
            en: r#"
## Step 3: Conway's rule

The whole game fits in **three rules**, based on a cell's state and its number
of alive neighbors:

1. An **alive** cell with **2 or 3** neighbors **survives**.
2. A **dead** cell with **exactly 3** neighbors is **born**.
3. In all other cases, the cell is **dead** next generation (under- or
   overpopulation).

A `match` on the **tuple** `(alive, neighbors)` expresses this perfectly:

```rust
fn prochaine(vivante: bool, voisins: usize) -> bool {
    match (vivante, voisins) {
        (true, 2) | (true, 3) => true,  // survival
        (false, 3) => true,             // birth
        _ => false,                     // death / stays dead
    }
}
```

- `(true, 2) | (true, 3)`: alive with 2 **or** 3 neighbors → stays alive.
- `(false, 3)`: dead with 3 neighbors → becomes alive.
- `_`: everything else → dead.

> 💡 These ultra-simple rules produce astonishing complexity: oscillators,
> spaceships that move, and even structures capable of computing!
"#,
        },
        task: Bi {
            fr: "Complète le `match` de `prochaine` avec les règles de Conway. Le programme teste une cellule vivante avec 2 voisines (elle survit) et doit afficher : true",
            en: "Complete the `match` in `prochaine` with Conway's rules. The program tests an alive cell with 2 neighbors (it survives) and must print: true",
        },
        starter: r#"fn prochaine(vivante: bool, voisins: usize) -> bool {
    match (vivante, voisins) {
        // (true, 2) | (true, 3) => true,   // survie
        // (false, 3) => true,              // naissance
        // _ => false,                      // mort
    }
}

fn main() {
    println!("{}", prochaine(true, 2));
}
"#,
        check: Check::Stdout { expected: "true" },
        hints: &[
            Bi { fr: "Survie : (true, 2) | (true, 3) => true. Naissance : (false, 3) => true. Sinon false.", en: "Survival: (true, 2) | (true, 3) => true. Birth: (false, 3) => true. Otherwise false." },
        ],
        solution: r#"fn prochaine(vivante: bool, voisins: usize) -> bool {
    match (vivante, voisins) {
        (true, 2) | (true, 3) => true,
        (false, 3) => true,
        _ => false,
    }
}

fn main() {
    println!("{}", prochaine(true, 2));
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 94,
        slug: "vie-etape",
        title: Bi { fr: "Jeu de la Vie 4/5 : une génération", en: "Game of Life 4/5: one generation" },
        subtitle: Bi { fr: "Calculer la grille suivante", en: "Compute the next grid" },
        xp: 280,
        lesson: Bi {
            fr: r#"
## Étape 4 : faire évoluer toute la grille

On a tout : afficher, compter les voisins, la règle. On assemble pour calculer
la grille de la **génération suivante**.

Point crucial : on construit une **nouvelle** grille, on ne modifie pas
l'ancienne pendant le calcul. Sinon, changer une cellule fausserait le compte
des voisines des cellules suivantes ! Toutes les naissances/morts doivent être
calculées à partir du **même** état de départ.

```rust
fn etape(grille: &[Vec<bool>]) -> Vec<Vec<bool>> {
    let mut nouvelle = grille.to_vec(); // copie de la même taille
    for x in 0..grille.len() {
        for y in 0..grille[0].len() {
            let v = voisins_vivants(grille, x, y);
            nouvelle[x][y] = prochaine(grille[x][y], v);
        }
    }
    nouvelle
}
```

- `grille.to_vec()` crée une copie (bonne taille, contenu peu importe : on
  écrase chaque cellule).
- Pour chaque cellule, on lit les voisines **dans l'ancienne grille** et on écrit
  le résultat **dans la nouvelle**.

### Le « blinker »

Un blinker vertical (3 cellules en colonne) devient horizontal en une étape :

```text
.#.        ...
.#.   ->   ###
.#.        ...
```

> 💡 « Lire l'ancien, écrire le nouveau » est un schéma classique des automates
> et simulations : ne jamais modifier l'état qu'on est en train de lire.
"#,
            en: r#"
## Step 4: evolve the whole grid

We have everything: render, count neighbors, the rule. We assemble them to
compute the **next generation** grid.

Crucial point: we build a **new** grid; we don't modify the old one during the
computation. Otherwise, changing one cell would corrupt the neighbor counts of
later cells! All births/deaths must be computed from the **same** starting
state.

```rust
fn etape(grille: &[Vec<bool>]) -> Vec<Vec<bool>> {
    let mut nouvelle = grille.to_vec(); // copy of the same size
    for x in 0..grille.len() {
        for y in 0..grille[0].len() {
            let v = voisins_vivants(grille, x, y);
            nouvelle[x][y] = prochaine(grille[x][y], v);
        }
    }
    nouvelle
}
```

- `grille.to_vec()` creates a copy (right size, content irrelevant: we overwrite
  each cell).
- For each cell, we read neighbors **in the old grid** and write the result
  **into the new one**.

### The "blinker"

A vertical blinker (3 cells in a column) becomes horizontal in one step:

```text
.#.        ...
.#.   ->   ###
.#.        ...
```

> 💡 "Read the old, write the new" is a classic pattern in automata and
> simulations: never modify the state you're currently reading.
"#,
        },
        task: Bi {
            fr: "Complète `etape` : pour chaque cellule, calcule ses voisins puis son état suivant. Le programme fait évoluer un blinker vertical d'une étape et doit afficher : ... / ### / ...",
            en: "Complete `etape`: for each cell, compute its neighbors then its next state. The program evolves a vertical blinker by one step and must print: ... / ### / ...",
        },
        starter: r#"fn voisins_vivants(grille: &[Vec<bool>], x: usize, y: usize) -> usize {
    let mut compte = 0;
    let lignes = grille.len() as i32;
    let colonnes = grille[0].len() as i32;
    for dx in [-1i32, 0, 1] {
        for dy in [-1i32, 0, 1] {
            if dx == 0 && dy == 0 { continue; }
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0 && nx < lignes && ny >= 0 && ny < colonnes && grille[nx as usize][ny as usize] {
                compte += 1;
            }
        }
    }
    compte
}

fn prochaine(vivante: bool, voisins: usize) -> bool {
    matches!((vivante, voisins), (true, 2) | (true, 3) | (false, 3))
}

fn afficher(grille: &[Vec<bool>]) -> String {
    let mut s = String::new();
    for ligne in grille {
        for &c in ligne { s.push(if c { '#' } else { '.' }); }
        s.push('\n');
    }
    s
}

fn etape(grille: &[Vec<bool>]) -> Vec<Vec<bool>> {
    let mut nouvelle = grille.to_vec();
    for x in 0..grille.len() {
        for y in 0..grille[0].len() {
            // Calcule les voisins de (x, y), puis nouvelle[x][y] = prochaine(...)
        }
    }
    nouvelle
}

fn main() {
    let grille = vec![
        vec![false, true, false],
        vec![false, true, false],
        vec![false, true, false],
    ];
    print!("{}", afficher(&etape(&grille)));
}
"#,
        check: Check::Stdout { expected: "...\n###\n..." },
        hints: &[
            Bi { fr: "let v = voisins_vivants(grille, x, y); nouvelle[x][y] = prochaine(grille[x][y], v);", en: "let v = voisins_vivants(grille, x, y); nouvelle[x][y] = prochaine(grille[x][y], v);" },
        ],
        solution: r#"fn voisins_vivants(grille: &[Vec<bool>], x: usize, y: usize) -> usize {
    let mut compte = 0;
    let lignes = grille.len() as i32;
    let colonnes = grille[0].len() as i32;
    for dx in [-1i32, 0, 1] {
        for dy in [-1i32, 0, 1] {
            if dx == 0 && dy == 0 { continue; }
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0 && nx < lignes && ny >= 0 && ny < colonnes && grille[nx as usize][ny as usize] {
                compte += 1;
            }
        }
    }
    compte
}

fn prochaine(vivante: bool, voisins: usize) -> bool {
    matches!((vivante, voisins), (true, 2) | (true, 3) | (false, 3))
}

fn afficher(grille: &[Vec<bool>]) -> String {
    let mut s = String::new();
    for ligne in grille {
        for &c in ligne { s.push(if c { '#' } else { '.' }); }
        s.push('\n');
    }
    s
}

fn etape(grille: &[Vec<bool>]) -> Vec<Vec<bool>> {
    let mut nouvelle = grille.to_vec();
    for x in 0..grille.len() {
        for y in 0..grille[0].len() {
            let v = voisins_vivants(grille, x, y);
            nouvelle[x][y] = prochaine(grille[x][y], v);
        }
    }
    nouvelle
}

fn main() {
    let grille = vec![
        vec![false, true, false],
        vec![false, true, false],
        vec![false, true, false],
    ];
    print!("{}", afficher(&etape(&grille)));
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 95,
        slug: "vie-simulation",
        title: Bi { fr: "Jeu de la Vie 5/5 : la simulation", en: "Game of Life 5/5: the simulation" },
        subtitle: Bi { fr: "Enchaîner les générations", en: "Chain the generations" },
        xp: 300,
        lesson: Bi {
            fr: r#"
## Étape 5 : lancer la simulation 🎉

Dernière pièce : appliquer `etape` **plusieurs fois** pour faire défiler les
générations. Une simple boucle suffit, en remplaçant la grille par sa version
suivante à chaque tour :

```rust
let mut grille = grille_initiale;
for _ in 0..n {
    grille = etape(&grille);
}
```

### Le blinker oscille

Le blinker a une **période de 2** : vertical → horizontal → vertical… Donc après
**2** générations, il revient à son état de départ :

```text
.#.   étape 1   ...   étape 2   .#.
.#.   ------>   ###   ------>   .#.
.#.             ...             .#.
```

C'est ton premier **oscillateur** ! En partant d'un blinker vertical et en
appliquant 2 étapes, on retrouve un blinker vertical.

Tu as construit un automate cellulaire complet : modélisation d'une grille,
calcul de voisinage, règles d'évolution, et simulation dans le temps. Bravo ! 🦀

> 💡 Pour aller plus loin : essaie un *planeur* (glider) qui se déplace en
> diagonale, agrandis la grille, ou affiche chaque génération pour voir
> l'animation dans le terminal.
"#,
            en: r#"
## Step 5: run the simulation 🎉

Last piece: apply `etape` **several times** to scroll through generations. A
simple loop does it, replacing the grid with its next version each turn:

```rust
let mut grille = grille_initiale;
for _ in 0..n {
    grille = etape(&grille);
}
```

### The blinker oscillates

The blinker has a **period of 2**: vertical → horizontal → vertical… So after
**2** generations, it returns to its starting state:

```text
.#.   step 1    ...   step 2    .#.
.#.   ------>   ###   ------>   .#.
.#.             ...             .#.
```

It's your first **oscillator**! Starting from a vertical blinker and applying 2
steps, you get a vertical blinker back.

You've built a full cellular automaton: grid modeling, neighborhood counting,
evolution rules, and simulation over time. Well done! 🦀

> 💡 To go further: try a *glider* that moves diagonally, enlarge the grid, or
> print each generation to see the animation in the terminal.
"#,
        },
        task: Bi {
            fr: "Complète la boucle de simulation : applique `etape` 2 fois. Le blinker (période 2) revient à la verticale ; le programme doit afficher : .#. / .#. / .#.",
            en: "Complete the simulation loop: apply `etape` 2 times. The blinker (period 2) returns to vertical; the program must print: .#. / .#. / .#.",
        },
        starter: r#"fn voisins_vivants(grille: &[Vec<bool>], x: usize, y: usize) -> usize {
    let mut compte = 0;
    let lignes = grille.len() as i32;
    let colonnes = grille[0].len() as i32;
    for dx in [-1i32, 0, 1] {
        for dy in [-1i32, 0, 1] {
            if dx == 0 && dy == 0 { continue; }
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0 && nx < lignes && ny >= 0 && ny < colonnes && grille[nx as usize][ny as usize] {
                compte += 1;
            }
        }
    }
    compte
}

fn prochaine(vivante: bool, voisins: usize) -> bool {
    matches!((vivante, voisins), (true, 2) | (true, 3) | (false, 3))
}

fn afficher(grille: &[Vec<bool>]) -> String {
    let mut s = String::new();
    for ligne in grille {
        for &c in ligne { s.push(if c { '#' } else { '.' }); }
        s.push('\n');
    }
    s
}

fn etape(grille: &[Vec<bool>]) -> Vec<Vec<bool>> {
    let mut nouvelle = grille.to_vec();
    for x in 0..grille.len() {
        for y in 0..grille[0].len() {
            let v = voisins_vivants(grille, x, y);
            nouvelle[x][y] = prochaine(grille[x][y], v);
        }
    }
    nouvelle
}

fn main() {
    let mut grille = vec![
        vec![false, true, false],
        vec![false, true, false],
        vec![false, true, false],
    ];

    // Applique etape 2 fois (le blinker revient à son état initial)
    for _ in 0..2 {
        // grille = ...
    }

    print!("{}", afficher(&grille));
}
"#,
        check: Check::Stdout { expected: ".#.\n.#.\n.#." },
        hints: &[
            Bi { fr: "Dans la boucle : grille = etape(&grille);", en: "In the loop: grille = etape(&grille);" },
        ],
        solution: r#"fn voisins_vivants(grille: &[Vec<bool>], x: usize, y: usize) -> usize {
    let mut compte = 0;
    let lignes = grille.len() as i32;
    let colonnes = grille[0].len() as i32;
    for dx in [-1i32, 0, 1] {
        for dy in [-1i32, 0, 1] {
            if dx == 0 && dy == 0 { continue; }
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0 && nx < lignes && ny >= 0 && ny < colonnes && grille[nx as usize][ny as usize] {
                compte += 1;
            }
        }
    }
    compte
}

fn prochaine(vivante: bool, voisins: usize) -> bool {
    matches!((vivante, voisins), (true, 2) | (true, 3) | (false, 3))
}

fn afficher(grille: &[Vec<bool>]) -> String {
    let mut s = String::new();
    for ligne in grille {
        for &c in ligne { s.push(if c { '#' } else { '.' }); }
        s.push('\n');
    }
    s
}

fn etape(grille: &[Vec<bool>]) -> Vec<Vec<bool>> {
    let mut nouvelle = grille.to_vec();
    for x in 0..grille.len() {
        for y in 0..grille[0].len() {
            let v = voisins_vivants(grille, x, y);
            nouvelle[x][y] = prochaine(grille[x][y], v);
        }
    }
    nouvelle
}

fn main() {
    let mut grille = vec![
        vec![false, true, false],
        vec![false, true, false],
        vec![false, true, false],
    ];

    for _ in 0..2 {
        grille = etape(&grille);
    }

    print!("{}", afficher(&grille));
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 96,
        slug: "fizzbuzz",
        title: Bi { fr: "Défi : FizzBuzz", en: "Challenge: FizzBuzz" },
        subtitle: Bi { fr: "Le classique des entretiens", en: "The interview classic" },
        xp: 220,
        lesson: Bi {
            fr: r#"
## FizzBuzz, le grand classique

Le défi le plus célèbre de la programmation. Pour les nombres de 1 à 15 :

- multiple de **3 et 5** (donc de 15) → afficher `FizzBuzz` ;
- multiple de **3** → `Fizz` ;
- multiple de **5** → `Buzz` ;
- sinon → le nombre.

L'astuce est l'**ordre des tests** : il faut vérifier le cas « 3 **et** 5 »
**en premier**, sinon on ne l'atteindrait jamais (un multiple de 15 est aussi
multiple de 3).

```rust
for n in 1..=15 {
    if n % 15 == 0 {
        println!("FizzBuzz");
    } else if n % 3 == 0 {
        println!("Fizz");
    } else if n % 5 == 0 {
        println!("Buzz");
    } else {
        println!("{n}");
    }
}
```

> 💡 Derrière sa simplicité, FizzBuzz teste deux choses : savoir utiliser le
> modulo `%`, et bien **ordonner** ses conditions du plus spécifique au plus
> général.
"#,
            en: r#"
## FizzBuzz, the great classic

Programming's most famous challenge. For numbers 1 to 15:

- multiple of **3 and 5** (i.e. 15) → print `FizzBuzz`;
- multiple of **3** → `Fizz`;
- multiple of **5** → `Buzz`;
- otherwise → the number.

The trick is the **order of tests**: check the "3 **and** 5" case **first**,
otherwise you'd never reach it (a multiple of 15 is also a multiple of 3).

```rust
for n in 1..=15 {
    if n % 15 == 0 {
        println!("FizzBuzz");
    } else if n % 3 == 0 {
        println!("Fizz");
    } else if n % 5 == 0 {
        println!("Buzz");
    } else {
        println!("{n}");
    }
}
```

> 💡 Behind its simplicity, FizzBuzz tests two things: using the modulo `%`, and
> properly **ordering** conditions from most specific to most general.
"#,
        },
        task: Bi {
            fr: "Complète les conditions FizzBuzz pour les nombres de 1 à 15. La sortie attendue va de 1, 2, Fizz, ... jusqu'à FizzBuzz.",
            en: "Complete the FizzBuzz conditions for numbers 1 to 15. The expected output goes 1, 2, Fizz, ... up to FizzBuzz.",
        },
        starter: r#"fn main() {
    for n in 1..=15 {
        if n % 15 == 0 {
            println!("FizzBuzz");
        } else if /* multiple de 3 */ {
            println!("Fizz");
        } else if /* multiple de 5 */ {
            println!("Buzz");
        } else {
            println!("{n}");
        }
    }
}
"#,
        check: Check::Stdout {
            expected: "1\n2\nFizz\n4\nBuzz\nFizz\n7\n8\nFizz\nBuzz\n11\nFizz\n13\n14\nFizzBuzz",
        },
        hints: &[
            Bi { fr: "Multiple de 3 : n % 3 == 0. Multiple de 5 : n % 5 == 0.", en: "Multiple of 3: n % 3 == 0. Multiple of 5: n % 5 == 0." },
        ],
        solution: r#"fn main() {
    for n in 1..=15 {
        if n % 15 == 0 {
            println!("FizzBuzz");
        } else if n % 3 == 0 {
            println!("Fizz");
        } else if n % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{n}");
        }
    }
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 97,
        slug: "cesar",
        title: Bi { fr: "Défi : chiffre de César", en: "Challenge: Caesar cipher" },
        subtitle: Bi { fr: "Décaler les lettres de l'alphabet", en: "Shift letters of the alphabet" },
        xp: 250,
        lesson: Bi {
            fr: r#"
## Le chiffrement de César

Un des plus vieux codes secrets : on **décale** chaque lettre d'un cran dans
l'alphabet. Avec un décalage de 1, `a` devient `b`, `b` devient `c`… et `z`
revient à `a` (on « boucle »).

### L'idée en Rust

On travaille sur les **codes ASCII**. Pour une lettre minuscule :

1. on se ramène à un nombre de 0 à 25 : `c - 'a'` ;
2. on ajoute le décalage, modulo 26 (pour boucler) ;
3. on revient à une lettre : `+ 'a'`.

```rust
fn cesar(texte: &str, decalage: u8) -> String {
    texte
        .chars()
        .map(|c| {
            if c.is_ascii_lowercase() {
                let base = b'a';
                (((c as u8 - base + decalage) % 26) + base) as char
            } else {
                c // on laisse les autres caractères tels quels
            }
        })
        .collect()
}
```

- `b'a'` est l'octet de `'a'` (97).
- `c as u8` convertit le `char` en son code.
- `% 26` assure que `z + 1` revienne à `a`.
- `as char` reconvertit le code en caractère.

Avec `cesar("abc", 1)` → `"bcd"`.

> 💡 C'est ta première manipulation d'encodage de caractères. Le même principe
> (passer en nombre, calculer, revenir en lettre) sert dans plein d'algorithmes
> de texte.
"#,
            en: r#"
## The Caesar cipher

One of the oldest secret codes: **shift** each letter by one in the alphabet.
With a shift of 1, `a` becomes `b`, `b` becomes `c`… and `z` wraps back to `a`.

### The idea in Rust

We work on **ASCII codes**. For a lowercase letter:

1. bring it to a number from 0 to 25: `c - 'a'`;
2. add the shift, modulo 26 (to wrap around);
3. go back to a letter: `+ 'a'`.

```rust
fn cesar(texte: &str, decalage: u8) -> String {
    texte
        .chars()
        .map(|c| {
            if c.is_ascii_lowercase() {
                let base = b'a';
                (((c as u8 - base + decalage) % 26) + base) as char
            } else {
                c // leave other characters unchanged
            }
        })
        .collect()
}
```

- `b'a'` is the byte of `'a'` (97).
- `c as u8` converts the `char` to its code.
- `% 26` ensures `z + 1` wraps back to `a`.
- `as char` converts the code back to a character.

With `cesar("abc", 1)` → `"bcd"`.

> 💡 It's your first character-encoding manipulation. The same principle (go to a
> number, compute, go back to a letter) is used in many text algorithms.
"#,
        },
        task: Bi {
            fr: "Complète la formule de décalage pour une lettre minuscule. Le programme chiffre \"abc\" avec un décalage de 1 et doit afficher : bcd",
            en: "Complete the shift formula for a lowercase letter. The program enciphers \"abc\" with a shift of 1 and must print: bcd",
        },
        starter: r#"fn cesar(texte: &str, decalage: u8) -> String {
    texte
        .chars()
        .map(|c| {
            if c.is_ascii_lowercase() {
                let base = b'a';
                // Renvoie la lettre décalée : (((c - base + decalage) % 26) + base) en char
            } else {
                c
            }
        })
        .collect()
}

fn main() {
    println!("{}", cesar("abc", 1));
}
"#,
        check: Check::Stdout { expected: "bcd" },
        hints: &[
            Bi { fr: "(((c as u8 - base + decalage) % 26) + base) as char", en: "(((c as u8 - base + decalage) % 26) + base) as char" },
        ],
        solution: r#"fn cesar(texte: &str, decalage: u8) -> String {
    texte
        .chars()
        .map(|c| {
            if c.is_ascii_lowercase() {
                let base = b'a';
                (((c as u8 - base + decalage) % 26) + base) as char
            } else {
                c
            }
        })
        .collect()
}

fn main() {
    println!("{}", cesar("abc", 1));
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 98,
        slug: "palindrome",
        title: Bi { fr: "Défi : palindrome", en: "Challenge: palindrome" },
        subtitle: Bi { fr: "Se lit pareil dans les deux sens", en: "Reads the same both ways" },
        xp: 220,
        lesson: Bi {
            fr: r#"
## Détecter un palindrome

Un **palindrome** se lit identiquement de gauche à droite et de droite à gauche :
`kayak`, `radar`, `ressasser`…

L'approche la plus simple : comparer la suite des caractères à la suite
**inversée**.

```rust
fn est_palindrome(s: &str) -> bool {
    let avant: Vec<char> = s.chars().collect();
    let arriere: Vec<char> = s.chars().rev().collect();
    avant == arriere
}
```

- `s.chars()` produit les caractères.
- `.rev()` inverse l'itérateur.
- On collecte les deux en `Vec<char>` et on les **compare** (`==` fonctionne sur
  les `Vec` si les éléments sont comparables).

```rust
est_palindrome("kayak"); // true
est_palindrome("rust");  // false
```

### Variante plus fine

Pour ignorer la casse et les espaces, on filtrerait d'abord :

```rust
let nettoye: String = s.chars().filter(|c| c.is_alphanumeric())
                               .collect::<String>().to_lowercase();
```

> 💡 Comparer une séquence à son inverse est un patron utile bien au-delà des
> palindromes (détection de symétries, vérification de cohérence…).
"#,
            en: r#"
## Detecting a palindrome

A **palindrome** reads identically left to right and right to left: `kayak`,
`radar`, `level`…

The simplest approach: compare the sequence of characters to the **reversed**
one.

```rust
fn est_palindrome(s: &str) -> bool {
    let avant: Vec<char> = s.chars().collect();
    let arriere: Vec<char> = s.chars().rev().collect();
    avant == arriere
}
```

- `s.chars()` yields the characters.
- `.rev()` reverses the iterator.
- We collect both into `Vec<char>` and **compare** them (`==` works on `Vec`s if
  the elements are comparable).

```rust
est_palindrome("kayak"); // true
est_palindrome("rust");  // false
```

### Finer variant

To ignore case and spaces, you'd filter first:

```rust
let cleaned: String = s.chars().filter(|c| c.is_alphanumeric())
                               .collect::<String>().to_lowercase();
```

> 💡 Comparing a sequence to its reverse is a useful pattern well beyond
> palindromes (symmetry detection, consistency checks…).
"#,
        },
        task: Bi {
            fr: "Complète `est_palindrome` : compare les caractères à leur version inversée. Le programme teste \"kayak\" et doit afficher : true",
            en: "Complete `est_palindrome`: compare the characters to their reversed version. The program tests \"kayak\" and must print: true",
        },
        starter: r#"fn est_palindrome(s: &str) -> bool {
    let avant: Vec<char> = s.chars().collect();
    // Construis la version inversée et compare
}

fn main() {
    println!("{}", est_palindrome("kayak"));
}
"#,
        check: Check::Stdout { expected: "true" },
        hints: &[
            Bi { fr: "let arriere: Vec<char> = s.chars().rev().collect(); avant == arriere", en: "let arriere: Vec<char> = s.chars().rev().collect(); avant == arriere" },
        ],
        solution: r#"fn est_palindrome(s: &str) -> bool {
    let avant: Vec<char> = s.chars().collect();
    let arriere: Vec<char> = s.chars().rev().collect();
    avant == arriere
}

fn main() {
    println!("{}", est_palindrome("kayak"));
}
"#,
    },
    // -------------------------------------------------------------------
    Level {
        id: 99,
        slug: "histogramme",
        title: Bi { fr: "Défi : histogramme ASCII", en: "Challenge: ASCII histogram" },
        subtitle: Bi { fr: "Dessiner des barres avec du texte", en: "Draw bars with text" },
        xp: 230,
        lesson: Bi {
            fr: r##"
## Un petit histogramme en texte

Visualisons des données avec des barres de `#`. Pour chaque valeur, on dessine
une ligne contenant autant de `#` que sa valeur.

La méthode `repeat` d'une chaîne est parfaite :

```rust
"#".repeat(3); // donne trois dieses
"ab".repeat(2); // donne abab
```

On transforme chaque nombre en sa barre, puis on **joint** les lignes avec des
sauts de ligne :

```rust
fn histogramme(valeurs: &[usize]) -> String {
    valeurs
        .iter()
        .map(|&v| "#".repeat(v))
        .collect::<Vec<_>>()
        .join("\n")
}
```

Avec `[3, 1, 4]` :

```text
###
#
####
```

- `map(|&v| "#".repeat(v))` : chaque valeur devient une ligne de `#`.
- `collect::<Vec<_>>()` : on rassemble les lignes.
- `join("\n")` : on les colle avec des retours à la ligne.

> 💡 Tu combines ici plusieurs acquis : itérateurs, `map`, `collect`, `join` et
> manipulation de chaînes. Avec quelques lignes, tu produis un graphique !
"##,
            en: r##"
## A small text histogram

Let's visualize data with bars of `#`. For each value, we draw a line with as
many `#` as its value.

A string's `repeat` method is perfect:

```rust
"#".repeat(3); // donne trois dieses
"ab".repeat(2); // donne abab
```

We turn each number into its bar, then **join** the lines with newlines:

```rust
fn histogramme(valeurs: &[usize]) -> String {
    valeurs
        .iter()
        .map(|&v| "#".repeat(v))
        .collect::<Vec<_>>()
        .join("\n")
}
```

With `[3, 1, 4]`:

```text
###
#
####
```

- `map(|&v| "#".repeat(v))`: each value becomes a line of `#`.
- `collect::<Vec<_>>()`: gather the lines.
- `join("\n")`: glue them with newlines.

> 💡 Here you combine several skills: iterators, `map`, `collect`, `join`, and
> string manipulation. In a few lines, you produce a chart!
"##,
        },
        task: Bi {
            fr: "Complète `histogramme` : chaque valeur devient une ligne de '#', jointes par des sauts de ligne. Le programme teste [3, 1, 4] et doit afficher : ### / # / ####",
            en: "Complete `histogramme`: each value becomes a line of '#', joined by newlines. The program tests [3, 1, 4] and must print: ### / # / ####",
        },
        starter: r##"fn histogramme(valeurs: &[usize]) -> String {
    valeurs
        .iter()
        // map chaque valeur v vers "#".repeat(v), collect en Vec, puis join("\n")
}

fn main() {
    print!("{}", histogramme(&[3, 1, 4]));
}
"##,
        check: Check::Stdout { expected: "###\n#\n####" },
        hints: &[
            Bi { fr: ".map(|&v| \"#\".repeat(v)).collect::<Vec<_>>().join(\"\\n\")", en: ".map(|&v| \"#\".repeat(v)).collect::<Vec<_>>().join(\"\\n\")" },
        ],
        solution: r##"fn histogramme(valeurs: &[usize]) -> String {
    valeurs
        .iter()
        .map(|&v| "#".repeat(v))
        .collect::<Vec<_>>()
        .join("\n")
}

fn main() {
    print!("{}", histogramme(&[3, 1, 4]));
}
"##,
    },
    // -------------------------------------------------------------------
    Level {
        id: 100,
        slug: "finale",
        title: Bi { fr: "Niveau 100 : la consécration 🏆", en: "Level 100: the crowning 🏆" },
        subtitle: Bi { fr: "Une pyramide pour célébrer", en: "A pyramid to celebrate" },
        xp: 500,
        lesson: Bi {
            fr: r#"
## 🏆 Le centième niveau !

Tu y es. Cent niveaux de Rust, des premières variables aux automates cellulaires
et aux mini-interpréteurs. Pour la dernière épreuve, un peu d'art ASCII : une
**pyramide** d'étoiles.

### Construire une pyramide

Pour une hauteur `h`, la ligne numéro `i` (de 1 à `h`) contient :

- `h - i` **espaces** (pour centrer) ;
- `2 * i - 1` **étoiles**.

```rust
let hauteur = 4;
for i in 1..=hauteur {
    let espaces = " ".repeat(hauteur - i);
    let etoiles = "*".repeat(2 * i - 1);
    println!("{espaces}{etoiles}");
}
```

Ce qui dessine :

```text
   *
  ***
 *****
*******
```

- ligne 1 : 3 espaces, 1 étoile ;
- ligne 2 : 2 espaces, 3 étoiles ;
- … et ainsi de suite, chaque ligne gagne 2 étoiles et perd 1 espace.

### Et après ?

Tu maîtrises maintenant les fondations **et** beaucoup de sujets avancés de Rust.
La suite, c'est **construire** : un vrai projet à toi. Un jeu, un outil en ligne
de commande, une API web (axum, comme RustQuest !)… Le meilleur moyen
d'apprendre, c'est de créer.

Merci d'avoir joué. **Bravo, et bon code !** 🦀

> 💡 Astuce : `" ".repeat(0)` renvoie une chaîne vide — donc la dernière ligne
> n'a aucun espace devant, et la pyramide est bien alignée à gauche.
"#,
            en: r#"
## 🏆 The hundredth level!

You made it. A hundred levels of Rust, from your first variables to cellular
automata and mini-interpreters. For the final test, a bit of ASCII art: a
**pyramid** of stars.

### Building a pyramid

For a height `h`, line number `i` (from 1 to `h`) contains:

- `h - i` **spaces** (to center it);
- `2 * i - 1` **stars**.

```rust
let hauteur = 4;
for i in 1..=hauteur {
    let espaces = " ".repeat(hauteur - i);
    let etoiles = "*".repeat(2 * i - 1);
    println!("{espaces}{etoiles}");
}
```

Which draws:

```text
   *
  ***
 *****
*******
```

- line 1: 3 spaces, 1 star;
- line 2: 2 spaces, 3 stars;
- … and so on, each line gains 2 stars and loses 1 space.

### What's next?

You now master the foundations **and** many advanced Rust topics. What follows
is **building**: a real project of your own. A game, a command-line tool, a web
API (axum, like RustQuest!)… The best way to learn is to create.

Thanks for playing. **Congratulations, and happy coding!** 🦀

> 💡 Tip: `" ".repeat(0)` returns an empty string — so the last line has no
> leading space, and the pyramid is properly left-aligned.
"#,
        },
        task: Bi {
            fr: "Complète la boucle pour dessiner une pyramide de hauteur 4 : (hauteur - i) espaces puis (2*i - 1) étoiles. La sortie doit former une pyramide, suivie du message de bravo.",
            en: "Complete the loop to draw a pyramid of height 4: (hauteur - i) spaces then (2*i - 1) stars. The output must form a pyramid, followed by the congrats message.",
        },
        starter: r#"fn main() {
    let hauteur = 4;
    for i in 1..=hauteur {
        // Construis la ligne : (hauteur - i) espaces, puis (2 * i - 1) étoiles
        let espaces = " ".repeat(/* ... */);
        let etoiles = "*".repeat(/* ... */);
        println!("{espaces}{etoiles}");
    }
    println!("Bravo, 100 niveaux termines !");
}
"#,
        check: Check::Stdout {
            expected: "   *\n  ***\n *****\n*******\nBravo, 100 niveaux termines !",
        },
        hints: &[
            Bi { fr: "espaces = \" \".repeat(hauteur - i) et etoiles = \"*\".repeat(2 * i - 1)", en: "espaces = \" \".repeat(hauteur - i) and etoiles = \"*\".repeat(2 * i - 1)" },
        ],
        solution: r#"fn main() {
    let hauteur = 4;
    for i in 1..=hauteur {
        let espaces = " ".repeat(hauteur - i);
        let etoiles = "*".repeat(2 * i - 1);
        println!("{espaces}{etoiles}");
    }
    println!("Bravo, 100 niveaux termines !");
}
"#,
    },
    // END_LEVELS
];

// ===========================================================================
// LES PALIERS — un récapitulatif tous les 10 niveaux
// ===========================================================================

/// Un palier de révision, débloqué après avoir terminé un bloc de 10 niveaux.
pub struct Palier {
    /// Le palier s'affiche après ce niveau (10, 20, … 100).
    pub after_level: u32,
    pub title: Bi,
    /// Récapitulatif au format Markdown.
    pub recap: Bi,
}

#[derive(Serialize)]
pub struct PalierView {
    pub after_level: u32,
    pub title: &'static str,
    pub recap_html: String,
    pub unlocked: bool,
}

impl Palier {
    pub fn view(&self, lang: &str, unlocked: bool) -> PalierView {
        PalierView {
            after_level: self.after_level,
            title: self.title.pick(lang),
            recap_html: md_to_html(self.recap.pick(lang)),
            unlocked,
        }
    }
}

pub static PALIERS: &[Palier] = &[
    Palier {
        after_level: 10,
        title: Bi { fr: "Palier 1 — Les fondations", en: "Checkpoint 1 — Foundations" },
        recap: Bi {
            fr: r#"
## 🎉 Bloc 1 terminé : les fondations (niveaux 1 à 10)

Tu maîtrises maintenant la base de tout programme Rust :

- **`println!` et `fn main`** — afficher du texte, point d'entrée du programme.
- **Variables** — `let`, immuabilité par défaut, `mut` pour modifier.
- **Types scalaires** — `i32`, `f64`, `bool`, `char`.
- **Fonctions** — paramètres typés, valeur de retour, expression finale sans `;`.
- **Conditions** — `if` / `else`, et `if` comme expression.
- **Boucles** — `loop`, `while`, `for` et les intervalles `..` / `..=`.
- **Ownership** — un seul propriétaire, le *move*.
- **Références & emprunt** — `&` (lecture) et `&mut` (écriture).
- **Structures** — `struct` + `impl` (méthodes avec `&self`).
- **Énumérations & `match`** — modéliser des choix, filtrage exhaustif.

➡️ La suite : `Option`, `Vec`, et la gestion des erreurs. En route !
"#,
            en: r#"
## 🎉 Block 1 done: the foundations (levels 1 to 10)

You now master the basics of any Rust program:

- **`println!` and `fn main`** — printing text, the program's entry point.
- **Variables** — `let`, immutable by default, `mut` to change.
- **Scalar types** — `i32`, `f64`, `bool`, `char`.
- **Functions** — typed parameters, return value, trailing expression with no `;`.
- **Conditions** — `if` / `else`, and `if` as an expression.
- **Loops** — `loop`, `while`, `for` and the ranges `..` / `..=`.
- **Ownership** — a single owner, the *move*.
- **References & borrowing** — `&` (read) and `&mut` (write).
- **Structs** — `struct` + `impl` (methods with `&self`).
- **Enums & `match`** — model choices, exhaustive matching.

➡️ Next: `Option`, `Vec`, and error handling. Let's go!
"#,
        },
    },
    Palier {
        after_level: 20,
        title: Bi { fr: "Palier 2 — Données & erreurs", en: "Checkpoint 2 — Data & errors" },
        recap: Bi {
            fr: r#"
## 🎉 Bloc 2 terminé (niveaux 11 à 20)

- **`Option`** — l'absence rendue explicite, sans `null`.
- **`Vec<T>`** — la liste dynamique.
- **Tuples** — regrouper des valeurs de types différents.
- **Tableaux `[T; N]`** — taille fixe.
- **`String` vs `&str`** — texte possédé vs emprunté.
- **`HashMap`** — associer des clés à des valeurs (`entry`).
- **`Result`** — succès `Ok` ou échec `Err`, sans exceptions.
- **Opérateur `?`** — propager une erreur élégamment.
- **Paniques & accès sûrs** — `unwrap`, `expect`, `get`.
- **Génériques de fonction** — écrire `<T>` une seule fois.

➡️ La suite : traits, itérateurs et closures — le cœur idiomatique de Rust.
"#,
            en: r#"
## 🎉 Block 2 done (levels 11 to 20)

- **`Option`** — absence made explicit, no `null`.
- **`Vec<T>`** — the dynamic list.
- **Tuples** — group values of different types.
- **Arrays `[T; N]`** — fixed size.
- **`String` vs `&str`** — owned vs borrowed text.
- **`HashMap`** — map keys to values (`entry`).
- **`Result`** — `Ok` success or `Err` failure, no exceptions.
- **The `?` operator** — propagate an error elegantly.
- **Panics & safe access** — `unwrap`, `expect`, `get`.
- **Generic functions** — write `<T>` once.

➡️ Next: traits, iterators and closures — Rust's idiomatic heart.
"#,
        },
    },
    Palier {
        after_level: 30,
        title: Bi { fr: "Palier 3 — Traits & itérateurs", en: "Checkpoint 3 — Traits & iterators" },
        recap: Bi {
            fr: r#"
## 🎉 Bloc 3 terminé (niveaux 21 à 30)

- **Structs génériques** — `Paire<T>` et `impl<T>`.
- **Traits** — définir un comportement, l'implémenter pour un type.
- **Méthodes par défaut** — du code fourni directement par le trait.
- **Trait bounds** — `<T: Trait>` pour contraindre un générique.
- **`#[derive(...)]`** — `Debug`, `Clone`, `PartialEq` gratuits.
- **Closures** — `|x| ...`, qui capturent leur environnement.
- **Itérateurs : `map` / `filter` / `collect`** — transformer des collections.
- **Réductions** — `sum`, `count`, `max`, `min`.
- **`enumerate` / `zip`** — indices et itérateurs jumelés.
- **`fold`** — l'accumulateur universel.

➡️ La suite : combinateurs, organisation du code, et affichage.
"#,
            en: r#"
## 🎉 Block 3 done (levels 21 to 30)

- **Generic structs** — `Paire<T>` and `impl<T>`.
- **Traits** — define behavior, implement it for a type.
- **Default methods** — code provided by the trait itself.
- **Trait bounds** — `<T: Trait>` to constrain a generic.
- **`#[derive(...)]`** — free `Debug`, `Clone`, `PartialEq`.
- **Closures** — `|x| ...`, capturing their environment.
- **Iterators: `map` / `filter` / `collect`** — transform collections.
- **Reductions** — `sum`, `count`, `max`, `min`.
- **`enumerate` / `zip`** — indices and paired iterators.
- **`fold`** — the universal accumulator.

➡️ Next: combinators, code organization, and display.
"#,
        },
    },
    Palier {
        after_level: 40,
        title: Bi { fr: "Palier 4 — Combinateurs & organisation", en: "Checkpoint 4 — Combinators & organization" },
        recap: Bi {
            fr: r#"
## 🎉 Bloc 4 terminé (niveaux 31 à 40)

- **Combinateurs d'`Option`** — `map`, `unwrap_or`, `and_then`, `filter`.
- **Combinateurs de `Result`** — `map`, `map_err`, `ok`.
- **`?` avec `Option`** — court-circuiter sur `None`.
- **`match` avancé** — gardes, intervalles, alternatives `|`.
- **`if let` / `while let`** — filtrer un seul cas.
- **`let ... else`** — extraire ou sortir tôt.
- **Modules** — `mod`, `pub`, `use`, encapsulation.
- **`From` / `Into`** — conversions propres entre types.
- **`Display`** — afficher joliment avec `{}`.
- **`PartialEq`** — comparer ses types avec `==`.

➡️ La suite : tri, collections spécialisées et pointeurs intelligents.
"#,
            en: r#"
## 🎉 Block 4 done (levels 31 to 40)

- **`Option` combinators** — `map`, `unwrap_or`, `and_then`, `filter`.
- **`Result` combinators** — `map`, `map_err`, `ok`.
- **`?` with `Option`** — short-circuit on `None`.
- **Advanced `match`** — guards, ranges, `|` alternatives.
- **`if let` / `while let`** — match a single case.
- **`let ... else`** — extract or bail early.
- **Modules** — `mod`, `pub`, `use`, encapsulation.
- **`From` / `Into`** — clean conversions between types.
- **`Display`** — pretty-print with `{}`.
- **`PartialEq`** — compare your types with `==`.

➡️ Next: sorting, specialized collections and smart pointers.
"#,
        },
    },
    Palier {
        after_level: 50,
        title: Bi { fr: "Palier 5 — Collections & pointeurs", en: "Checkpoint 5 — Collections & pointers" },
        recap: Bi {
            fr: r#"
## 🎉 Bloc 5 terminé — mi-parcours ! (niveaux 41 à 50)

- **Tri** — `sort`, `sort_by`, `reverse`.
- **`HashSet`** — un ensemble sans doublons.
- **`BTreeMap`** — une map aux clés triées.
- **`Box<T>`** — une valeur sur le tas, et les types récursifs.
- **`Rc<T>`** — la propriété partagée (comptage de références).
- **`RefCell<T>`** — la mutabilité intérieure.
- **Objets-traits `dyn`** — mélanger des types derrière un trait.
- **Durées de vie** — `'a` sur les fonctions…
- **…et sur les structs** — quand on stocke une référence.
- **Itérateur personnalisé** — implémenter `Iterator` (méthode `next`).

🏁 Tu es à **mi-chemin des 100 niveaux**. Bravo, la moitié est faite !
"#,
            en: r#"
## 🎉 Block 5 done — halfway! (levels 41 to 50)

- **Sorting** — `sort`, `sort_by`, `reverse`.
- **`HashSet`** — a set without duplicates.
- **`BTreeMap`** — a map with sorted keys.
- **`Box<T>`** — a value on the heap, and recursive types.
- **`Rc<T>`** — shared ownership (reference counting).
- **`RefCell<T>`** — interior mutability.
- **Trait objects `dyn`** — mix types behind a trait.
- **Lifetimes** — `'a` on functions…
- **…and on structs** — when storing a reference.
- **Custom iterator** — implement `Iterator` (the `next` method).

🏁 You're **halfway through the 100 levels**. Well done — half done!
"#,
        },
    },
    Palier {
        after_level: 60,
        title: Bi { fr: "Palier 6 — Concurrence & avancé", en: "Checkpoint 6 — Concurrency & advanced" },
        recap: Bi {
            fr: r#"
## 🎉 Bloc 6 terminé (niveaux 51 à 60)

- **Threads** — `thread::spawn` et `join`.
- **`Arc<Mutex<T>>`** — partager un état mutable entre threads.
- **Channels `mpsc`** — communiquer par messages.
- **`parse`** — convertir du texte en nombre.
- **Newtype** — emballer un type pour plus de sûreté.
- **`impl Trait` en retour** — renvoyer un itérateur/une closure.
- **`Default`** — valeurs par défaut et `..Default::default()`.
- **Surcharge d'opérateurs** — implémenter `Add` (`+`).
- **Récursivité** — cas de base + cas récursif.
- **Slices** — `&[T]`, emprunter une portion.

➡️ La suite : les itérateurs experts et la manipulation de texte.
"#,
            en: r#"
## 🎉 Block 6 done (levels 51 to 60)

- **Threads** — `thread::spawn` and `join`.
- **`Arc<Mutex<T>>`** — share mutable state across threads.
- **Channels `mpsc`** — communicate via messages.
- **`parse`** — convert text into a number.
- **Newtype** — wrap a type for more safety.
- **`impl Trait` in return** — return an iterator/closure.
- **`Default`** — default values and `..Default::default()`.
- **Operator overloading** — implement `Add` (`+`).
- **Recursion** — base case + recursive case.
- **Slices** — `&[T]`, borrow a portion.

➡️ Next: expert iterators and text manipulation.
"#,
        },
    },
    Palier {
        after_level: 70,
        title: Bi { fr: "Palier 7 — Itérateurs experts", en: "Checkpoint 7 — Expert iterators" },
        recap: Bi {
            fr: r#"
## 🎉 Bloc 7 terminé (niveaux 61 à 70)

- **`collect` vers `Result`** — tout réussit, ou la première erreur.
- **Capstone** — une calculatrice à états (enum + `match` + `fold`).
- **`find` / `position`** — le premier élément qui convient.
- **`all` / `any`** — vérifier une propriété globale.
- **`filter_map`** — filtrer et transformer d'un coup.
- **`flat_map` / `flatten`** — aplatir des collections imbriquées.
- **`chain` / `rev`** — concaténer et inverser des itérateurs.
- **`take_while` / `skip_while`** — couper un flux selon une condition.
- **`windows`** — des fenêtres glissantes.
- **`chunks`** — des groupes disjoints.

➡️ La suite : texte, nombres et traits avancés.
"#,
            en: r#"
## 🎉 Block 7 done (levels 61 to 70)

- **`collect` into `Result`** — all succeed, or the first error.
- **Capstone** — a stateful calculator (enum + `match` + `fold`).
- **`find` / `position`** — the first matching element.
- **`all` / `any`** — check a global property.
- **`filter_map`** — filter and transform at once.
- **`flat_map` / `flatten`** — flatten nested collections.
- **`chain` / `rev`** — concatenate and reverse iterators.
- **`take_while` / `skip_while`** — cut a stream by a condition.
- **`windows`** — sliding windows.
- **`chunks`** — disjoint groups.

➡️ Next: text, numbers and advanced traits.
"#,
        },
    },
    Palier {
        after_level: 80,
        title: Bi { fr: "Palier 8 — Texte, nombres & traits avancés", en: "Checkpoint 8 — Text, numbers & advanced traits" },
        recap: Bi {
            fr: r#"
## 🎉 Bloc 8 terminé (niveaux 71 à 80)

- **`split` / `join`** — découper et recoller du texte.
- **`chars` / `bytes`** — parcourir les caractères.
- **Arithmétique sûre** — `checked_*`, `saturating_*`, `wrapping_*`.
- **`match` sur tuples** — filtrer plusieurs valeurs à la fois.
- **Closures `FnMut`** — capturer un état mutable.
- **Erreurs personnalisées** — une `enum` + `Display`.
- **`Box<dyn Error>`** — l'erreur attrape-tout avec `?`.
- **`Drop`** — du code à la libération d'une valeur.
- **`Deref`** — créer son propre pointeur intelligent.
- **`BinaryHeap`** — une file de priorité.

➡️ La suite : derniers outils, puis place aux **projets** !
"#,
            en: r#"
## 🎉 Block 8 done (levels 71 to 80)

- **`split` / `join`** — splitting and joining text.
- **`chars` / `bytes`** — iterating characters.
- **Safe arithmetic** — `checked_*`, `saturating_*`, `wrapping_*`.
- **`match` on tuples** — match several values at once.
- **`FnMut` closures** — capture mutable state.
- **Custom errors** — an `enum` + `Display`.
- **`Box<dyn Error>`** — the catch-all error with `?`.
- **`Drop`** — code when a value is freed.
- **`Deref`** — build your own smart pointer.
- **`BinaryHeap`** — a priority queue.

➡️ Next: a few last tools, then on to the **projects**!
"#,
        },
    },
    Palier {
        after_level: 90,
        title: Bi { fr: "Palier 9 — Outils & début des projets", en: "Checkpoint 9 — Tools & first project" },
        recap: Bi {
            fr: r#"
## 🎉 Bloc 9 terminé (niveaux 81 à 90)

- **`VecDeque`** — une file à deux bouts (FIFO/LIFO).
- **`sort_by_key` / `sort_by`** — trier des données complexes.
- **`macro_rules!`** — écrire du code qui écrit du code.
- **`impl Trait` en argument** — accepter « ce qui sait faire ».
- **Supertraits** — un trait qui en exige un autre.
- **Constantes associées** — des valeurs attachées à un type.
- **Tests** — `assert_eq!` et `#[test]`.
- **Projet RPN 1/3** — le *tokeniser*.
- **Projet RPN 2/3** — l'évaluateur à pile.
- **Projet RPN 3/3** — le pipeline complet.

➡️ Dernière ligne droite : le **Jeu de la Vie** et les défis finaux !
"#,
            en: r#"
## 🎉 Block 9 done (levels 81 to 90)

- **`VecDeque`** — a double-ended queue (FIFO/LIFO).
- **`sort_by_key` / `sort_by`** — sort complex data.
- **`macro_rules!`** — write code that writes code.
- **`impl Trait` as argument** — accept "anything that can".
- **Supertraits** — a trait requiring another.
- **Associated constants** — values attached to a type.
- **Tests** — `assert_eq!` and `#[test]`.
- **RPN project 1/3** — the tokenizer.
- **RPN project 2/3** — the stack evaluator.
- **RPN project 3/3** — the full pipeline.

➡️ Home stretch: the **Game of Life** and the final challenges!
"#,
        },
    },
    Palier {
        after_level: 100,
        title: Bi { fr: "Palier 10 — La consécration 🏆", en: "Checkpoint 10 — The crowning 🏆" },
        recap: Bi {
            fr: r#"
## 🏆 Bloc 10 terminé — TU AS FINI LES 100 NIVEAUX !

- **Jeu de la Vie** — afficher la grille, compter les voisins, la règle de
  Conway, une génération, la simulation complète.
- **FizzBuzz** — le classique des entretiens.
- **Chiffre de César** — manipuler l'encodage des caractères.
- **Palindrome** — comparer une séquence à son inverse.
- **Histogramme ASCII** — visualiser des données avec du texte.
- **La pyramide finale** — ton 100ᵉ niveau ! 🎉

Tu es parti·e de `println!` et tu as construit des automates cellulaires, un
mini-interpréteur, et touché à la concurrence et aux pointeurs intelligents.
**Bravo, sincèrement.**

➡️ Et maintenant ? **Construis.** Un jeu, un outil en ligne de commande, une API
web (axum, comme RustQuest !). Le meilleur moyen d'apprendre, c'est de créer. 🦀
"#,
            en: r#"
## 🏆 Block 10 done — YOU FINISHED ALL 100 LEVELS!

- **Game of Life** — render the grid, count neighbors, Conway's rule, one
  generation, the full simulation.
- **FizzBuzz** — the interview classic.
- **Caesar cipher** — manipulate character encoding.
- **Palindrome** — compare a sequence to its reverse.
- **ASCII histogram** — visualize data with text.
- **The final pyramid** — your 100th level! 🎉

You started from `println!` and built cellular automata, a mini-interpreter,
and touched concurrency and smart pointers. **Truly well done.**

➡️ What now? **Build.** A game, a command-line tool, a web API (axum, like
RustQuest!). The best way to learn is to create. 🦀
"#,
        },
    },
];
