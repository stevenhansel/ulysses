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

This means you never need to manually unload/reload models. Ulysses handles the switching so you can treat your VRAM-constrained setup like a multi-model server.

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

All metrics are exposed via both a web dashboard and a JSON API for integration with external monitoring tools.

## Development

### Prerequisites

- **Rust** 1.88+ (MSRV for ts-rs)
- **SQLite** (bundled automatically via `libsqlite3-sys`)
- **Node.js** 22+ and **pnpm** 11+

### Getting started

#### Backend (API)

```bash
# Navigate to the API project
cd api

# Copy the example environment file and adjust as needed
cp .env.example .env

# Run the application (this also applies pending migrations)
cargo run
```

The server starts on `http://localhost:8000` by default (configurable via `HOST` and `PORT` in `.env`).

#### Frontend (Web UI)

```bash
# Install dependencies from the monorepo root
pnpm install

# Start the Vite dev server (proxies /api to localhost:8000)
pnpm web:dev
```

The web UI is served at `http://localhost:5173` with hot module replacement. API requests to `/api/*` are automatically proxied to the Rust backend at `http://localhost:8000` in development.

### Available commands

#### Backend (`api/`)

```bash
# Check compilation (fast, skips test compilation)
cargo check

# Run all tests
cargo test

# Run tests for a specific module
cargo test -- proxy

# Run with verbose logging
RUST_LOG=ulysses_api=debug cargo run

# Watch mode (requires cargo-watch)
cargo watch -x run

# Format
cargo fmt

# Lint
cargo clippy -- -D warnings
```

#### Frontend (`web/`)

From the monorepo root, use the workspace scripts:

```bash
pnpm web:dev       # Start Vite dev server (HMR at localhost:5173)
pnpm web:build     # Type-check + production build
pnpm web:test      # Run all tests once
pnpm web:lint      # Run oxlint
```

Or from the `web/` directory directly:

```bash
pnpm dev           # Vite dev server
pnpm build         # tsc -b && vite build
pnpm test          # Vitest single run
pnpm test:watch    # Vitest watch mode
pnpm lint          # oxlint
pnpm preview       # Preview production build
```

## Docker

Ulysses can be run as a single Docker container that serves both the API and the web UI. The Docker image uses a multi-stage build with separate frontend and backend stages for optimal layer caching — if only the frontend changes, only the frontend stage is rebuilt.

### Pre-built image

Pre-built images are published to GitHub Container Registry:

```bash
docker pull ghcr.io/stevenhansel/ulysses:latest
```

### Running with Docker

```bash
# Pull and run the pre-built image
docker run -d \
  --name ulysses \
  -p 8000:8000 \
  -v ulysses_data:/app/data \
  ghcr.io/stevenhansel/ulysses:latest
```

### Running with Docker Compose (recommended)

```bash
# Using the pre-built image from GHCR
docker compose up -d

# Or force a local rebuild
docker compose build --no-cache
```

Open [http://localhost:8000](http://localhost:8000) to access the web UI. The API is available at `http://localhost:8000/api/...` and the Swagger documentation at `http://localhost:8000/docs`.

### Building locally

```bash
# Build the image from source
docker build -f docker/Dockerfile -t ulysses .

# Run the locally-built image
docker run -d \
  --name ulysses \
  -p 8000:8000 \
  -v ulysses_data:/app/data \
  ulysses
```

### Configuration

The container accepts the same environment variables as the API:

| Variable | Default | Description |
|----------|---------|-------------|
| `HOST` | `0.0.0.0` | Bind address |
| `PORT` | `8000` | HTTP port |
| `DATABASE_URL` | `sqlite:/app/data/ulysses.db?mode=rwc` | SQLite database path |
| `RUST_LOG` | `ulysses_api=info` | Logging level |

Persistent data (SQLite database) is stored in the `/app/data` volume.

### Project structure

```
ulysses/
├── api/             # Rust API backend (axum, SQLite)
│   ├── src/         # Layered architecture (Controller → Service → Repository)
│   ├── migrations/  # SQLite database migrations
│   └── docs/        # Architecture & design docs
│
├── web/             # React frontend (Vite, TanStack Router)
│   ├── src/         # Routes, features, components, hooks, lib
│   ├── tests/       # Test infrastructure (MSW, Vitest setup)
│   └── docs/        # Architecture & design docs
│
├── package.json     # Root workspace scripts
└── pnpm-workspace.yaml
```

The API backend follows a **layered architecture** (Controller → Service → Repository) inside feature modules. See [`api/docs/ARCHITECTURE.md`](api/docs/ARCHITECTURE.md) for the full breakdown.

The web frontend uses a **feature-based layered architecture** (Routes → Features → Infrastructure → Shared). See [`web/docs/ARCHITECTURE.md`](web/docs/ARCHITECTURE.md) for the full breakdown.

