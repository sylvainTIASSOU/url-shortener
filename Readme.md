<div align="center">

# 🔗 URL Shortener — Rust & Axum

### API REST asynchrone • Architecture modulaire • OpenAPI / Swagger

[![Rust](https://img.shields.io/badge/Rust-2024-000000?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Axum](https://img.shields.io/badge/Axum-0.8.9-295BA7)](https://docs.rs/axum/)
[![Tokio](https://img.shields.io/badge/Tokio-1.54.1-4C9BE8)](https://tokio.rs/)
[![OpenAPI](https://img.shields.io/badge/OpenAPI-Utoipa-6DA544)](https://docs.rs/utoipa/)
[![Status](https://img.shields.io/badge/Statut-prototype%20p%C3%A9dagogique-F0AD4E)](#limites-actuelles)

**Projet Rust de conception d’une API de raccourcissement d’URL, centré sur les fondamentaux du développement backend et les bonnes pratiques d’architecture logicielle.**

[Code source](https://github.com/sylvainTIASSOU/url-shortener) · [Architecture](#architecture) · [Référence API](#documentation-de-lapi) · [Compétences démontrées](#compétences-techniques-acquises-et-démontrées)

</div>

---

## Présentation

Ce dépôt met en pratique la conception d’un service HTTP avec **Rust**, **Axum** et **Tokio**. Il illustre le découpage d’une application backend en modules, l’utilisation d’extracteurs et de réponses typées, la sérialisation JSON avec Serde, les middleware Tower, les logs et la génération d’un point d’accès Swagger UI.

L’objectif est d’établir des fondations solides pour un futur raccourcisseur d’URL persistant. L’API est aujourd’hui **un prototype pédagogique**, pas un service public prêt à recevoir du trafic réel.

> **État du projet : prototype en cours de développement.** Les routes principales existent, mais les données sont simulées. La génération de codes uniques, le stockage en base de données, la redirection HTTP et le comptage réel des clics restent à implémenter.

## Sommaire

- [Compétences techniques acquises et démontrées](#compétences-techniques-acquises-et-démontrées)
- [Fonctionnalités](#fonctionnalités)
- [Stack technique](#stack-technique)
- [Prérequis](#prérequis)
- [Installation et démarrage](#installation-et-démarrage)
- [Configuration](#configuration)
- [Documentation de l’API](#documentation-de-lapi)
- [Swagger et OpenAPI](#swagger-et-openapi)
- [Architecture](#architecture)
- [Flux d’une requête](#flux-dune-requête)
- [Gestion des erreurs](#gestion-des-erreurs)
- [Limites actuelles](#limites-actuelles)
- [Middleware et observabilité](#middleware-et-observabilité)
- [Développement](#développement)
- [Sécurité : points d’attention](#sécurité--points-dattention)
- [Dépannage](#dépannage)
- [Feuille de route](#feuille-de-route)
- [Contribution](#contribution)
- [Auteur](#auteur)
- [Licence](#licence)

## Compétences techniques acquises et démontrées

Cette section met en valeur **les compétences directement illustrées par l’implémentation**, sans présenter les futures fonctionnalités comme terminées.

| Domaine | Compétence mise en pratique | Preuve technique |
| --- | --- | --- |
| **Rust backend** | Structuration en `struct`, types, modules et organisation d'une crate bibliothèque + binaire. | `src/lib.rs`, `src/bin/main.rs`, `src/models/links.rs` |
| **Axum & API REST** | Routage HTTP, handlers asynchrones, extracteurs JSON, `Path` et `Query`, statuts de réponse. | `src/app.rs`, `src/controllers/link.rs` |
| **Programmation asynchrone** | Lancement d'un service HTTP sur le runtime Tokio et gestion de handlers `async`. | `src/bin/main.rs`, `src/app.rs` |
| **Sérialisation & DTO** | Modèles de requête et de réponse avec Serde ; décomposition des données entrantes. | `src/models/links.rs` |
| **Architecture modulaire** | Séparation des responsabilités : configuration, contrôleurs, modèles, erreurs, middleware. | Arborescence `src/` |
| **Gestion d'erreurs** | Définition d'erreurs typées avec `thiserror` et conversion en codes HTTP via `IntoResponse`. | `src/error/error.rs` |
| **Middleware & sécurité HTTP** | Contrôle d'en-tête `x-api-key` sur une route, configuration CORS et composition de couches Tower. | `src/middlewares/trace.rs`, `src/app.rs` |
| **Observabilité** | Initialisation de `tracing`, journalisation HTTP et génération d'identifiants de requête. | `src/middlewares/trace.rs`, `src/app.rs` |
| **Documentation d'API** | Intégration des bibliothèques Utoipa et Swagger UI ; premières annotations OpenAPI. | `src/app.rs`, handler `health` |
| **Configuration** | Lecture d'un `.env`, parsing du port et configuration des logs. | `src/config/config.rs`, `src/.env.example` |
| **Outillage & qualité** | Utilisation de Cargo, rustfmt et Clippy comme outils de vérification. | `Cargo.toml` ; commandes de développement |

### Ce que ce projet m’a permis de pratiquer

> Concevoir une base d’API REST en Rust avec Axum et Tokio, structurer un backend en modules cohérents, exposer des endpoints typés, manipuler des données JSON avec Serde, centraliser des erreurs HTTP et expérimenter les middleware, les logs et la documentation Swagger/OpenAPI.

### Limites de la démonstration

Ce projet **ne prouve pas encore** une maîtrise mise en œuvre de PostgreSQL/SQLx, de migrations, de redirections HTTP, de génération de codes uniques, de statistiques persistantes, de CI/CD ou de déploiement de production. Ces sujets figurent dans la feuille de route.

## Fonctionnalités

| Fonctionnalité | État actuel |
| --- | --- |
| Vérification de disponibilité du serveur | `GET /health` |
| Création d’un lien | `POST /links` renvoie un objet JSON simulé |
| Protection de la création par clé API | En-tête `x-api-key`, valeur de démonstration codée en dur |
| Liste avec paramètres de pagination | Renvoie un message texte, sans collection de liens |
| Recherche par code | Renvoie un objet simulé contenant le code demandé |
| Gestion centralisée des erreurs applicatives | Type `AppError` et réponses JSON |
| Configuration par variables d’environnement | Chargement facultatif d’un fichier `.env` |
| Logs et identifiant de requête | Middleware présent sur les routes publiques de l’API |
| Interface Swagger UI | Montée ; déclaration OpenAPI des routes à compléter |
| Persistance, codes uniques et redirections | Non implémentés |
| Statistiques de clics | Valeur fixe de démonstration : `32` |

## Stack technique

Les versions ci-dessous correspondent aux déclarations de `Cargo.toml`. `Cargo.lock` contient les versions résolues.

| Composant | Technologie | Version déclarée |
| --- | --- | --- |
| Langage | Rust | Édition 2024 |
| Framework HTTP | Axum | `0.8.9` |
| Runtime asynchrone | Tokio | `1.54.1` |
| Sérialisation | Serde / Serde JSON | `1.0.229` / `1.0.154` |
| Middleware | Tower / Tower HTTP | `0.5.3` / `0.7.1` |
| Gestion d’erreurs | thiserror | `2.0.21` |
| Variables d’environnement | dotenvy | `0.15.7` |
| Journalisation | tracing / tracing-subscriber | `0.1.44` / `0.3.23` |
| OpenAPI | utoipa | `6.0.0` |
| Swagger UI | utoipa-swagger-ui | `10.0.2` |

PostgreSQL et SQLx ne sont pas encore intégrés. Aucun serveur de base de données n’est nécessaire pour le comportement actuel.

## Prérequis

- Une toolchain Rust stable récente, compatible avec l’édition 2024 et les dépendances du projet, avec Cargo.
- Git pour récupérer les sources.
- `curl` pour essayer les endpoints.

Rust peut être installé avec [rustup](https://rustup.rs/). Le dépôt ne fixe pas de version minimale de Rust avec `rust-version` et ne contient pas de fichier `rust-toolchain.toml`.

Vérifier l’installation :

```bash
rustc --version
cargo --version
```

## Installation et démarrage

### 1. Cloner le dépôt

```bash
git clone https://github.com/sylvainTIASSOU/url-shortener.git
cd url-shortener
```

### 2. Préparer la configuration

Depuis la racine du projet :

```bash
cp src/.env.example .env
```

Le fichier d’exemple contient :

```dotenv
DATABASE_URL=postgres://postgres:password@localhost:5432/urlshortener
PORT=3000
LOG_LEVEL=debug
```

`DATABASE_URL` est actuellement lue mais n’est utilisée par aucune connexion. Les identifiants ci-dessus sont des valeurs d’exemple.

### 3. Démarrer le serveur

```bash
cargo run --locked
```

Avec le fichier `.env` d’exemple, l’API est accessible sur **http://localhost:3000**. Sans variable `PORT`, le serveur utilise **3030**.

Le serveur écoute sur `0.0.0.0`, donc sur toutes les interfaces réseau disponibles.

### 4. Vérifier le fonctionnement

```bash
curl -i http://localhost:3000/health
```

Réponse attendue d’après le handler : statut `200 OK`, avec le corps texte :

```text
Server is alive.
```

### Compilation en mode release

```bash
cargo build --release --locked
./target/release/url-shortener
```

Lancer le binaire depuis la racine du projet permet de charger le fichier `.env` qui s’y trouve. Les variables peuvent aussi être fournies directement par l’environnement d’exécution.

## Configuration

Le chargement de `.env` utilise `dotenvy::dotenv()`. L’absence de ce fichier n’empêche pas le démarrage et les variables déjà présentes dans l’environnement sont prioritaires.

| Variable | Valeur sans configuration | Rôle actuel |
| --- | --- | --- |
| `PORT` | `3030` | Port HTTP, converti en `u16` |
| `DATABASE_URL` | Chaîne vide | Lue dans `Config`, sans connexion à une base |
| `LOG_LEVEL` | `debug` pour le logger | Filtre de logs transmis à `tracing_subscriber::EnvFilter` |

**Particularité de `LOG_LEVEL` :** `Config::from_env()` définit un défaut à `info`, mais `init_tracing()` relit directement la variable avec un défaut à `debug`. Le champ `Config.log_level` ne pilote donc pas le logger dans l’implémentation actuelle.

Exemple de surcharge pour un shell compatible POSIX :

```bash
PORT=8080 LOG_LEVEL=info cargo run --locked
```

La clé API n’est pas configurable par variable d’environnement : sa valeur actuelle est `secret`, dans `src/middlewares/trace.rs`.

## Documentation de l’API

Les exemples suivants supposent `PORT=3000`.

### Vue d’ensemble

| Méthode | Route | Clé API | Réponse actuelle |
| --- | --- | --- | --- |
| `GET` | `/health` | Non | Texte de disponibilité |
| `POST` | `/links` | Oui | Objet `Link`, statut `201` |
| `GET` | `/links` | Non | Message texte de pagination |
| `GET` | `/links/{code}` | Non | Objet `Link` simulé |
| `GET` | `/swagger-ui/` | Non | Interface Swagger |
| `GET` | `/api-docs/openapi.json` | Non | Document OpenAPI |

### Vérifier la disponibilité

```bash
curl -i http://localhost:3000/health
```

Retourne `200 OK` et `Server is alive.`. Cet endpoint ne vérifie aucune dépendance externe.

### Créer un lien

```bash
curl -i -X POST http://localhost:3000/links \
  -H 'Content-Type: application/json' \
  -H 'x-api-key: secret' \
  -d '{"url":"https://www.rust-lang.org/"}'
```

Corps requis :

| Champ | Type | Description |
| --- | --- | --- |
| `url` | `string` | URL à transmettre au handler |

Réponse attendue, statut `201 Created` :

```json
{
  "code": "abc123",
  "url": "https://www.rust-lang.org/",
  "clicks": 32
}
```

Le handler recopie `url`, retourne toujours `abc123` et initialise `clicks` à `32`. Il ne valide pas la syntaxe de l’URL et n’enregistre rien. Cette réponse ne constitue donc pas encore un lien court utilisable.

Une clé absente ou différente de `secret` produit un statut `401 Unauthorized`, sans corps JSON personnalisé.

### Lister les liens

```bash
curl -i 'http://localhost:3000/links?page=2&limit=5'
```

Réponse attendue, statut `200 OK` :

```text
liste of links page=2 limit=5
```

| Paramètre | Type | Défaut | Description |
| --- | --- | --- | --- |
| `page` | `u32`, facultatif | `1` | Numéro de page affiché dans la réponse |
| `limit` | `u32`, facultatif | `10` | Taille de page affichée dans la réponse |

La route n’interroge pas de stockage et ne retourne pas de tableau JSON. Les valeurs `0` sont acceptées ; aucune borne métier supplémentaire n’est appliquée. Les paramètres non convertibles en `u32` sont rejetés par l’extracteur Axum.

### Consulter un code

```bash
curl -i http://localhost:3000/links/abc123
```

Réponse attendue, statut `200 OK` :

```json
{
  "code": "abc123",
  "url": "",
  "clicks": 32
}
```

Tout code non vide correspondant à cette route produit actuellement un objet simulé. La route ne retrouve pas l’URL envoyée lors d’un précédent `POST`, ne signale pas les codes inconnus et n’effectue aucune redirection `301`, `302` ou `307`.

### Modèle `Link`

| Champ | Type Rust | Description actuelle |
| --- | --- | --- |
| `code` | `String` | Code fixe à la création, ou code demandé à la consultation |
| `url` | `String` | Valeur reçue à la création, chaîne vide à la consultation |
| `clicks` | `u64` | Valeur simulée : `32` |

## Swagger et OpenAPI

Après démarrage avec `PORT=3000` :

- Interface : [http://localhost:3000/swagger-ui/](http://localhost:3000/swagger-ui/)
- Schéma JSON : [http://localhost:3000/api-docs/openapi.json](http://localhost:3000/api-docs/openapi.json)

**État actuel :** `health` possède une annotation `#[utoipa::path]`, mais `ApiDoc` ne déclare aucun `paths(...)`. Les endpoints ne sont donc pas enregistrés dans le document OpenAPI généré. Les modèles et le schéma de sécurité de la clé API restent également à documenter.

Le tableau des routes et les exemples de ce README décrivent les handlers présents dans le code.

## Architecture

| Fichier | Responsabilité |
| --- | --- |
| `Cargo.toml` | Métadonnées, dépendances et cibles du projet |
| `Cargo.lock` | Versions résolues des dépendances |
| `src/bin/main.rs` | Point d’entrée Tokio et appel à `App::run()` |
| `src/lib.rs` | Déclaration des modules et réexport de `App` |
| `src/app.rs` | Construction des routeurs, middleware, Swagger et démarrage HTTP |
| `src/config/config.rs` | Lecture de la configuration |
| `src/controllers/link.rs` | Handlers HTTP et recherche simulée |
| `src/models/links.rs` | Structures `Link`, `CreateLink` et `Pagination` |
| `src/error/error.rs` | Erreurs applicatives et conversion en réponses HTTP |
| `src/middlewares/trace.rs` | Initialisation des logs et vérification de la clé API |
| `src/context.rs` | Fichier actuellement vide |
| `src/.env.example` | Exemple de configuration à copier à la racine |

Le binaire s’appelle `url-shortener` et la bibliothèque Rust `url_shortener`. Aucun état partagé ni pool de connexions n’est encore injecté dans les handlers.

## Flux d’une requête

```mermaid
flowchart LR
    A[Client HTTP] --> B[Routeur Axum]
    B --> C{Route protégée ?}
    C -->|POST /links| D[Middleware x-api-key]
    C -->|GET| E[Handler]
    D -->|Clé acceptée| E
    D -->|Clé refusée| F[401 Unauthorized]
    E --> G[DTO et réponse HTTP]
    G --> A
```

**Important :** les handlers retournent actuellement des valeurs de démonstration. Il n’y a pas de base de données ni de service de redirection derrière ce flux.

### Choix de conception

- Le **routeur** déclare les endpoints et compose les middleware.
- Les **contrôleurs** traitent les requêtes et produisent les réponses.
- Les **modèles** centralisent les structures de données exposées à l’API.
- La **configuration** centralise la lecture des variables d’environnement.
- Les **erreurs applicatives** sont typées et converties en réponses HTTP.
- Le fichier `context.rs` laisse une place à un futur état partagé, sans en fournir pour le moment.

## Gestion des erreurs

`AppError` implémente `IntoResponse` pour convertir les erreurs applicatives en réponses JSON :

| Variante | Statut | Valeur du champ `error` |
| --- | --- | --- |
| `NotFound` | `404 Not Found` | `not found` |
| `InvalidInput(message)` | `400 Bad Request` | Message fourni |
| `Database(message)` | `500 Internal Server Error` | Message fourni |
| `Internal` | `500 Internal Server Error` | `Internal error` |

Exemple de format applicatif :

```json
{
  "error": "code cannot be empty"
}
```

Toutes ces variantes ne sont pas utilisées par les handlers actuels. En particulier, la recherche simulée ne retourne pas `NotFound` pour un code inconnu. Le contrôle du code vide existe dans les fonctions, mais un segment vide ne correspond normalement pas à `/links/{code}`.

Les rejets des extracteurs Axum — JSON mal formé, champ requis absent ou paramètre de requête invalide — ainsi que le `401` du middleware ne passent pas par `AppError`. Toutes les erreurs HTTP ne suivent donc pas encore le même format JSON.

## Limites actuelles

La version documentée ne doit pas être confondue avec un raccourcisseur d’URL fonctionnel en production :

1. `POST /links` renvoie un code fixe (`abc123`) sans l’enregistrer et sans vérifier l’URL.
2. `GET /links` renvoie un message texte ; la pagination ne s’applique pas à une collection réelle.
3. `GET /links/{code}` construit un objet simulé, y compris pour un code inconnu.
4. Le champ `clicks` est fixé à `32` et ne mesure aucune visite.
5. Il n’existe pas encore de route de redirection ni de génération de codes uniques.
6. La valeur de la clé API est codée en dur ; CORS et la portée des middleware sont à ajuster.
7. Les endpoints ne sont pas encore enregistrés dans le schéma OpenAPI malgré l’accès à Swagger UI.
8. Aucun test dédié, schéma SQL, migration, Dockerfile ou pipeline CI n’a été identifié dans la version étudiée.

Ces limites sont volontairement documentées pour permettre à un lecteur ou à un recruteur d’évaluer le code tel qu’il existe.

## Middleware et observabilité

### Clé API

Seule la création via `POST /links` est protégée par `require_api_key`. La valeur `secret` est une clé de démonstration codée en dur, à remplacer par une configuration adaptée avant un usage public.

### Logs et identifiant de requête

`TraceLayer::new_for_http()` fournit la journalisation des requêtes. `SetRequestIdLayer` ajoute un UUID dans l’en-tête de requête `x-request-id` lorsqu’il est absent.

`PropagateRequestIdLayer` n’est pas installé : le serveur ne garantit pas le renvoi de cet identifiant dans les en-têtes de réponse.

### CORS et portée des couches

La configuration CORS déclare toutes les origines et tous les en-têtes autorisés, avec uniquement `GET` comme méthode autorisée.

Dans `src/app.rs`, les couches de trace, d’identifiant et de CORS sont appliquées au routeur public **avant** la fusion du routeur protégé et de Swagger. Elles ne couvrent donc pas uniformément toutes les routes, notamment `POST /links`. Un appel depuis un navigateur sur une autre origine nécessite de revoir cette configuration, la portée des couches et la prise en charge de `POST` et du précontrôle CORS.

## Développement

Commandes usuelles à exécuter depuis la racine :

```bash
# Vérifier la compilation
cargo check --locked

# Vérifier le formatage
cargo fmt --all -- --check

# Appliquer le formatage
cargo fmt --all

# Exécuter l’analyse statique
cargo clippy --locked --all-targets --all-features

# Exécuter les tests disponibles
cargo test --locked
```

Les composants `rustfmt` et `clippy` peuvent être ajoutés avec :

```bash
rustup component add rustfmt clippy
```

Le dépôt analysé ne contient pas de tests unitaires ou d’intégration dédiés, de pipeline CI, de Dockerfile ni de migrations. Les commandes ci-dessus sont des commandes de vérification à lancer localement ; elles ne constituent pas une attestation de réussite des contrôles.

## Sécurité : points d’attention

- La valeur `x-api-key: secret` est **uniquement destinée aux essais locaux**. Ne l’utilisez pas pour protéger une API accessible sur Internet.
- Externaliser les secrets dans l’environnement et ne jamais committer de véritables identifiants dans `.env`.
- Valider les URL fournies par les clients (`http`/`https`, taille maximale, format, politiques éventuelles contre les liens dangereux).
- Prévoir des contrôles d’abus (limitation du débit, journalisation appropriée) avant toute ouverture publique.
- Réduire les risques d’exposition d’informations internes dans les erreurs HTTP et les logs.
- Appliquer les politiques CORS et la propagation de `x-request-id` de manière cohérente à toutes les routes concernées.

## Dépannage

| Problème | Vérification ou action |
| --- | --- |
| Connexion refusée sur le port `3000` | Sans `.env` ni `PORT`, le port utilisé est `3030`. Vérifier également que le serveur a démarré. |
| `.env` semble ignoré | Copier `src/.env.example` vers `.env` à la racine et lancer Cargo depuis cette racine. Vérifier les variables déjà exportées. |
| `401 Unauthorized` sur la création | Fournir exactement `x-api-key: secret`. |
| `Configuration error: PORT must be a number` | Utiliser une valeur convertible en `u16`, comprise entre `0` et `65535`. Pour un port fixe, choisir une valeur non nulle disponible. |
| Échec lors de l’ouverture du port | Vérifier qu’il n’est pas occupé et que le processus a les permissions nécessaires. |
| Une URL créée n’est pas retrouvée | Aucune persistance n’est implémentée ; les réponses sont simulées. |
| Swagger n’affiche pas les opérations | Ajouter les handlers dans `paths(...)` de `ApiDoc` et compléter leurs annotations. |
| La requête fonctionne avec `curl`, mais pas depuis un navigateur | Revoir CORS et l’application des couches au routeur protégé. |
| Logs plus détaillés que prévu | Sans `LOG_LEVEL`, `init_tracing()` utilise `debug`. Définir explicitement `LOG_LEVEL=info` si nécessaire. |

## Feuille de route

Les éléments suivants sont des évolutions envisagées, pas des fonctionnalités disponibles :

- [ ] Ajouter PostgreSQL, SQLx et les migrations.
- [ ] Introduire un état applicatif partagé et un pool de connexions.
- [ ] Générer des codes courts uniques avec gestion des collisions.
- [ ] Valider les URL et les schémas autorisés.
- [ ] Enregistrer les liens et implémenter une pagination réelle.
- [ ] Retourner `404` pour les codes inconnus.
- [ ] Ajouter une route de redirection HTTP.
- [ ] Incrémenter les clics lors des redirections.
- [ ] Externaliser la clé API et renforcer la protection contre les abus.
- [ ] Harmoniser les erreurs applicatives, les rejets Axum et l’authentification.
- [ ] Appliquer les middleware aux routes prévues et finaliser CORS.
- [ ] Propager les identifiants de requête vers les réponses.
- [ ] Compléter OpenAPI : routes, modèles, réponses et sécurité.
- [ ] Ajouter des tests unitaires et d’intégration.
- [ ] Gérer l’arrêt gracieux du serveur.
- [ ] Ajouter Docker et une chaîne CI.

## Contribution

Les propositions peuvent être soumises via les [issues](https://github.com/sylvainTIASSOU/url-shortener/issues) ou une pull request.

1. Créer un fork du dépôt et une branche dédiée.
2. Implémenter une modification ciblée.
3. Ajouter les tests pertinents et actualiser la documentation si le comportement change.
4. Exécuter les commandes de formatage, de compilation, d’analyse statique et de test.
5. Ouvrir une pull request décrivant le problème résolu, le comportement obtenu et les vérifications effectuées.

## Auteur

**Sylvain TIASSOU** — [Profil GitHub](https://github.com/sylvainTIASSOU)

Dépôt : [sylvainTIASSOU/url-shortener](https://github.com/sylvainTIASSOU/url-shortener).

## Licence

Aucun fichier `LICENSE` n’est présent dans la version analysée et aucune licence n’est déclarée dans `Cargo.toml`. Les conditions de réutilisation restent à préciser par l’auteur.

---

Documentation basée sur l’analyse disponible du commit [`2812363`](https://github.com/sylvainTIASSOU/url-shortener/commit/28123636b6f1dc488518aedf14e168499c684a98). Le dépôt distant n’a pas pu être relu en direct lors de cette révision ; les changements plus récents éventuels ne sont donc pas pris en compte. Les exemples de réponses sont déduits des handlers et non de tests d’exécution.
