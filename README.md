# Ulysses

> A lightweight proxy and model manager for self-hosted LLMs. Designed for individuals with limited VRAM who need to serve multiple models without manual swapping.

Ulysses sits between your LLM client (e.g., SillyTavern, Open WebUI, a custom script) and one or more inference backends (e.g., llama.cpp, vLLM, anything exposing an OpenAI-compatible API). It routes requests to the right model, hot-swaps models on demand, and gives you real-time visibility into both hardware and inference performance.

## Features

### Model Management (Profiles)

Configure your inference backends and models in one place. Each **profile** defines:

- A backend endpoint (e.g., a llama.cpp server, vLLM instance, or any OpenAI-compatible API)
- A model identifier (e.g., a GGUF file path, a Hugging Face model name, or a model tag)
- Optional overrides like context length, GPU layers, or inference parameters

Ulysses is **backend-agnostic** — you link your own inference tooling. It ships with sensible defaults for llama.cpp but works with any server that speaks the OpenAI Chat Completions API.

### Hot Swapping

Ulysses acts as a smart gateway. When a request arrives:
- **Model is loaded** → the request is processed immediately.
- **Model is not loaded & no active requests** → the model is swapped in immediately, then the request is processed.
- **Model is not loaded & a request is in progress** → the request is queued. As soon as the active request finishes, Ulysses hot-swaps the model and processes the queued request automatically.

### Hardware Monitoring

Real-time dashboards for your system's hardware:

- **CPU** — utilization, temperature, frequency
- **GPUs** — utilization, VRAM usage, temperature, power draw (NVIDIA and AMD)
- **RAM** — total, used, available, swap
- Per-process metrics where applicable

### LLM Performance Monitoring

Track live inference metrics per-request and over time:

- **Tokens per second** (tok/s) — generation speed
- **Prompt processing speed** — tokens per second during the prefill / prompt evaluation phase
- **Time to first token** (TTFT)
- **Request latency** — queue wait time + inference time

## Development

### Prerequisites

- **Rust** 1.97.0 (via `rustup`)
- **cargo-leptos** (for the development server)
- **wasm32-unknown-unknown target** (for WASM hydration builds)

```bash
rustup target add wasm32-unknown-unknown
cargo install cargo-leptos
```

### Getting started

The project is a single Rust crate with both the API server and the Leptos frontend in `src/`.

```bash
# Start the development server (with hot-reload)
cargo leptos serve
```

Open [http://localhost:8000](http://localhost:8000) in your browser.

`cargo leptos serve` handles everything — the Rust server, WASM hydration builds, and Tailwind CSS compilation — all with automatic hot-reload on changes.

> **Note:** No Node.js installation is required. `cargo-leptos` handles CSS compilation via Lightning CSS.

### Available commands

```bash
# Check SSR compilation (fast)
cargo check

# Check WASM compilation
cargo check --target wasm32-unknown-unknown --features hydrate --no-default-features

# Run all tests (23 tests across 6 suites)
cargo test

# Format
cargo fmt

# Lint
cargo clippy --all-targets --all-features -- -D warnings
```

### Build for production

```bash
# Production build (SSR binary + WASM/JS/CSS assets)
cargo leptos build --release
```

The SSR binary is at `target/release/ulysses` (approximately 20 MB).
The WASM/JS/CSS assets are at `site/pkg/`.

## Project structure

```
ulysses/
├── src/
│   ├── main.rs              # Leptos + Axum integrated entrypoint
│   ├── main_wasm.rs         # WASM hydration entry point
│   ├── lib.rs               # Module re-exports (feature-gated)
│   ├── config.rs            # Config from env (HOST, PORT, DATABASE_URL)
│   ├── context.rs           # DI container (AppState, Context)
│   ├── error.rs             # AppError → IntoResponse
│   ├── types.rs             # Shared types (serde + conditional utoipa)
│   ├── api/                 # REST API handlers + OpenAPI
│   │   ├── mod.rs           # All routers + aggregated OpenAPI spec
│   │   └── proxy/           # Proxy module
│   │       ├── mod.rs       # Router + health check
│   │       ├── controller_http.rs
│   │       ├── controller_ws.rs
│   │       ├── service.rs
│   │       ├── repository.rs
│   │       └── tests/       # Integration, service, repository, controller tests
│   ├── web/                 # Leptos frontend
│   │   ├── mod.rs           # UlyssesShell, App, NotFoundPage, router
│   │   ├── server_fns.rs    # #[server] function declarations
│   │   ├── routes/          # Page components (home, dashboard, settings)
│   │   ├── components/      # UI primitives + layout components
│   │   └── domain/          # Domain-specific feature components
│   └── models/              # Database models
│
├── style/
│   └── main.css             # Tailwind v4 + CSS theme variables
├── site/pkg/                 # Compiled WASM/JS/CSS assets (build output)
├── migrations/               # SQLite database migrations
├── docs/
│   └── ARCHITECTURE.md       # Architecture documentation
├── docker/
│   ├── Dockerfile            # Multi-stage build (Rust 1.97 → distroless)
│   └── docker-compose.yml    # Docker Compose configuration
├── .github/workflows/
│   └── ci.yml               # CI/CD: lint, test, docker build & push
├── tests/                    # Integration tests
│   ├── web_routes.rs         # SSR rendering tests (14)
│   └── web_server_fns.rs     # Server function tests (3)
├── Cargo.toml
├── rust-toolchain.toml       # Pinned to 1.97.0
└── .cargo/
    └── config.toml           # WASM runner config
```

## API documentation

Once the server is running, the Swagger UI is available at [http://localhost:8000/docs](http://localhost:8000/docs).

## Testing

```bash
# Run all tests
cargo test

# Run specific test suites
cargo test --test web_routes       # SSR rendering tests
cargo test --test web_server_fns   # Server function tests

# Run with output
cargo test -- --nocapture
```

### Test suites (23 tests)

| Suite | Location | Tests | What it covers |
|---|---|---|---|
| SSR rendering | `tests/web_routes.rs` | 14 | Leptos page components render correct HTML |
| Server functions | `tests/web_server_fns.rs` | 3 | `#[server]` fns with in-memory SQLite |
| Integration | `src/api/proxy/tests/integration.rs` | 2 | Full HTTP stack with axum-test |
| Service | `src/api/proxy/tests/service.rs` | 1 | Service layer with in-memory DB |
| Repository | `src/api/proxy/tests/repository.rs` | 1 | Repository layer with in-memory DB |
| Controller HTTP | `src/api/proxy/tests/controller_http.rs` | 2 | HTTP controller level |

## Docker

```bash
# Build the Docker image
docker build -f docker/Dockerfile -t ulysses .

# Run (with persistent data volume)
docker run -d \
    --name ulysses \
    -p 8000:8000 \
    -v ulysses_data:/app/data \
    ulysses

# Or using docker compose
docker compose -f docker/docker-compose.yml up -d
```

### Configuration

| Variable | Default | Description |
|----------|---------|-------------|
| `DATABASE_URL` | `sqlite:/app/data/ulysses.db?mode=rwc` | SQLite database path |
| `RUST_LOG` | `ulysses=info` | Logging level |
| `LEPTOS_SITE_ADDR` | `0.0.0.0:8000` | Server bind address |
| `LEPTOS_SITE_ROOT` | `/app/site` | Site root directory |
| `LEPTOS_SITE_PKG_DIR` | `/app/site/pkg` | WASM assets directory |
| `LEPTOS_OUTPUT_NAME` | `ulysses` | Output binary name |
