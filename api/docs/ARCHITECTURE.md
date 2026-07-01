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
6. If `WEB_DIST` env var is set, mount a `ServeDir` fallback serving the pre-built web UI as a SPA (with `index.html` fallback for client-side routes)
7. `axum::serve` with graceful shutdown (tokio signal)

## Web UI Serving (Docker Deployment)

In production (Docker), the API also serves the pre-built frontend as static files. This keeps the deployment to a single container with one process — no nginx or separate static file server needed.

### How it works

1. The Docker multi-stage build compiles the React/Vite frontend into `web/dist/`.
2. The final runtime image includes both the API binary and the pre-built frontend at `/app/web-dist/`.
3. When the container starts, the `WEB_DIST=/app/web-dist` environment variable tells the API to mount a `ServeDir` fallback service.
4. All requests that don't match an API route (`/api/*`, `/docs`, `/api-docs/*`) fall through to the static file server, which:
   - Serves actual files (JS, CSS, fonts, favicon) directly
   - Serves `index.html` for any unrecognized path (SPA fallback for client-side routing)

### Request routing (Docker)

```
  Request
     │
     ▼
  Axum Router
     │
     ├── /api/*          →  API handler (profiles, health, WS, etc.)
     ├── /docs           →  Swagger UI
     ├── /api-docs/*     →  OpenAPI spec
     └── /* (fallback)   →  ServeDir(/app/web-dist)
                              ├── /assets/*        →  static files (JS, CSS, fonts)
                              ├── /favicon.svg     →  favicon
                              └── /* (not found)   →  index.html (SPA)
```

### Development vs Production

| Aspect | Development | Production (Docker) |
|--------|------------|-------------------|
| Backend | `cargo run` on `:8000` | Same binary in container |
| Frontend | `pnpm web:dev` on `:5173` with HMR, proxying `/api` → `:8000` | Pre-built static files served by the API binary |
| Architecture | Two processes, separate ports | Single process, single port |
| Hot reload | Yes (Vite HMR + `cargo watch`) | No (re-build image) |

During development, the frontend and backend run **completely independently** — just as they did before Docker was introduced. The `WEB_DIST` env var is only set in the container, so the static file serving logic is a no-op in local dev.

### Config

The `WEB_DIST` environment variable controls the feature:

```rust
// In config.rs
pub struct Config {
    // ...
    pub web_dist: Option<String>,  // None in dev, Some("/app/web-dist") in Docker
}
```

```rust
// In main.rs
if let Some(web_dist) = &config.web_dist {
    app = app.fallback_service(
        ServeDir::new(web_dist)
            .precompressed_gzip()
            .append_index_html_on_directories(true)
            .fallback(ServeFile::new(web_path.join("index.html"))),
    );
}
```

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
