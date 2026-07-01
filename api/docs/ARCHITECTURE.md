# Architecture

## Tech Stack

| Component          | Choice         | Rationale                                |
|--------------------|----------------|------------------------------------------|
| Language           | Rust           | Performance, safety, concurrency         |
| Web Framework      | Axum           | Async, ergonomic, Tower ecosystem        |
| Middleware         | tower-http     | CORS, compression, tracing, etc.         |
| Runtime            | Tokio          | Industry-standard async runtime          |
| Database           | SQLite         | Embedded, zero-fuss, right for single-host |
| DB Interface       | sqlx           | Compile-time checked queries, async      |
| Config             | dotenvy + serde| Environment files + typed deserialization |
| Logging            | tracing        | Structured, async-aware, ecosystem std   |
| Testing            | axum_test      | First-class Axum integration testing     |
| Error Handling     | AppError enum  | Centralized IntoResponse impl            |
| API Documentation  | utoipa         | OpenAPI 3.1 code-first via derive macros |
| UI for API Docs    | utoipa-swagger-ui | Serves Swagger UI at `/docs`          |
| Type Sharing       | ts-rs          | Auto-export Rust types to TypeScript     |

## Project Structure

```
api/
├── Cargo.toml
├── .env
├── docs/
│   └── ARCHITECTURE.md        ← this file
├── migrations/                # SQLx migrations
└── src/
    ├── main.rs                # Entrypoint: init, serve
    ├── lib.rs                 # Re-exports
    ├── config.rs              # Config struct (serde + dotenvy)
    ├── context.rs             # DI container (Context)
    ├── error.rs               # AppError enum → IntoResponse
    ├── models/                # Database models
    │   ├── mod.rs
    │   └── *.rs
    └── modules/               # Feature modules
        ├── mod.rs             # Merges all sub-routers
        └── <module_name>/
            ├── mod.rs         # Module router (HTTP + WS routes)
            ├── controller_http.rs   # REST endpoints
            ├── controller_ws.rs     # WebSocket handler(s)
            ├── service.rs           # Business logic
            └── repository.rs        # Data access (sqlx)
```

## Layered Architecture

Every module follows a strict **Controller → Service → Repository** dependency chain:

```
  Client
     │
     ▼
  [Axum Router]
     │
     ├── HTTP → controller_http.rs
     └── WS   → controller_ws.rs
     │
     ▼
  service.rs  (business logic, orchestration)
     │
     ▼
  repository.rs  (data access via sqlx)
     │
     ▼
  SQLite
```

Rules:
- **Controllers** handle only transport concerns: parsing requests, extracting params, calling service, formatting responses.
- **Services** contain business logic, validation, orchestration across repositories.
- **Repositories** are the only layer that touches the database.
- **No layer skips** — controllers never call repositories directly.

## Context (DI Container)

Defined in `src/context.rs`, the `Context` struct is initialized once at startup and passed to every layer. It holds all singleton dependencies:

```rust
pub struct Context {
    pub config: Config,
    pub db: SqlitePool,
    // module-level singletons (queues, handles, etc.) are added here
}
```

Axum uses `State<Arc<Context>>` to inject the context into handlers.

## Error Handling

A single `AppError` enum implements `IntoResponse`, so any handler can return `Result<_, AppError>`:

```rust
pub enum AppError {
    NotFound,
    BadRequest(String),
    Internal(String),
    Database(sqlx::Error),
    WebSocket(String),
}

impl IntoResponse for AppError { ... }
```

## WebSocket

WebSocket is treated as an alternate **interface/transport**, not a separate concern. A module that supports WS simply defines `controller_ws.rs` alongside `controller_http.rs`. The module's `mod.rs` merges both into its sub-router.

## API Documentation (OpenAPI / Swagger UI)

Documentation is generated **code-first** using `utoipa`:

1. Each handler is annotated with `#[utoipa::path(...)]` — this defines the HTTP method, path, request/response types, tags, and error codes.
2. A central `ApiDoc` struct in `src/modules/mod.rs` collects all handlers via `#[openapi(paths(...), components(schemas(...)))]`.
3. `ApiDoc::openapi()` returns the complete `utoipa::openapi::OpenApi` spec.
4. The spec is mounted at `/api-docs/openapi.json` and Swagger UI at `/docs/` using `utoipa-swagger-ui`.

### Usage

```rust
// 1. Annotate the handler
#[utoipa::path(
    get,
    path = "/api/proxy/profiles",
    tag = "proxy",
    responses(
        (status = 200, description = "OK", body = Vec<ProfileResponse>),
    ),
)]
pub async fn list_profiles(State(ctx): State<Arc<Context>>) -> ... { ... }

// 2. Register in ApiDoc (src/modules/mod.rs)
#[derive(OpenApi)]
#[openapi(
    info(title = "Ulysses API", version = "0.1.0"),
    paths(
        crate::modules::proxy::controller_http::list_profiles,
        // ... more handlers from other modules
    ),
    components(schemas(
        crate::modules::proxy::controller_http::ProfileResponse,
        // ... more types from other modules
    )),
)]
pub struct ApiDoc;
```

**Important**: `paths(...)` must use **direct crate-absolute function references** (e.g. `crate::modules::proxy::controller_http::list_profiles`), not re-exports. This is because utoipa generates a `__path_<fn>` submodule alongside each annotated function.

### Endpoints

| URL | Description |
|---|---|
| `/docs/` | Swagger UI (interactive) |
| `/api-docs/openapi.json` | Raw OpenAPI 3.1 spec |

## Startup Sequence

1. Load `.env` → parse `Config`
2. Initialize `SqlitePool`, run pending migrations
3. Build `Context { config, db, ... }`
4. Build merged router from all modules
5. Mount Swagger UI with aggregated OpenAPI spec
6. `axum::serve` with graceful shutdown (tokio signal)

## Module Registration

Each module's `mod.rs` exposes a function like:

```rust
pub fn router(state: Arc<Context>) -> Router<Arc<Context>>;
```

`modules/mod.rs` aggregates them:

```rust
pub fn all_routers(state: Arc<Context>) -> (Router<Arc<Context>>, utoipa::openapi::OpenApi) {
    let proxy = proxy::router(state.clone());

    let router = Router::new().merge(proxy);

    let api = ApiDoc::openapi();

    (router, api)
}
```

### Adding a new module

1. Create `src/modules/<name>/` with `mod.rs`, `controller_http.rs`, `service.rs`, `repository.rs` (and `controller_ws.rs` if needed).
2. Register the module's router in `modules/mod.rs`:
   - Add `mod <name>;`
   - Merge `<name>::router(state.clone())` into the router
3. Register the module's handlers and schemas in the `ApiDoc` struct:
   - Add handlers to `paths(...)`
   - Add types to `components(schemas(...))`
4. Create tests in `<name>/tests/`.
