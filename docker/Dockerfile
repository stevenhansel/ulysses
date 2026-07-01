# =============================================================================
# Ulysses — Multi-stage Dockerfile
#
# Builds both the Rust API backend and the React/Vite frontend, then packages
# them into a single runtime image. The API binary serves the pre-built web UI
# as static files (SPA with index.html fallback).
#
# Layer caching strategy:
#   - Frontend: dependency manifests copied first, then source — npm install
#     is cached unless package.json / lockfile changes.
#   - Backend:  Cargo manifests + dummy src → dependencies cached; real source
#     copied afterward so only the workspace crate is recompiled.
# =============================================================================

# ---- Stage 1: Frontend builder -------------------------------------------------
FROM node:22-slim AS frontend-builder

# Enable pnpm via corepack
RUN corepack enable && corepack prepare pnpm@latest --activate

WORKDIR /app

# Copy dependency manifests first (layer caching)
COPY package.json pnpm-workspace.yaml pnpm-lock.yaml ./
COPY web/package.json web/

# Install dependencies (frozen lockfile ensures reproducible builds)
RUN --mount=type=cache,target=/root/.local/share/pnpm/store \
    pnpm install --frozen-lockfile

# Copy frontend source and build
COPY web/ web/
RUN pnpm --filter ulysses-web build

# The built artifacts are at web/dist/

# ---- Stage 2: Backend builder ---------------------------------------------------
FROM rust:1.95-slim-bookworm AS backend-builder

# Install system dependencies needed by cargo / SQLite
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        pkg-config libssl-dev curl && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app/api

# Copy dependency manifests first
COPY api/Cargo.toml api/Cargo.lock ./

# Copy all source code, migrations, and env example upfront
COPY api/migrations ./migrations/
COPY api/src ./src/
COPY api/.env.example ./.env.example

# Build the release binary
# The --mount=type=cache persists the compiled dependency cache across rebuilds
RUN --mount=type=cache,target=/app/api/target \
    cargo build --release --bin ulysses-api && \
    cp /app/api/target/release/ulysses-api /app/ulysses-api

# ---- Stage 3: Runtime image (single container, both API + frontend) -------------
FROM debian:bookworm-slim

# Install runtime dependencies: ca-certificates, curl (for healthcheck), tzdata
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        ca-certificates \
        curl \
        tzdata && \
    rm -rf /var/lib/apt/lists/*

# Create a non-root user for security
RUN groupadd -r ulysses && useradd -r -g ulysses -d /app -s /sbin/nologin ulysses

WORKDIR /app

# Copy the compiled API binary from the backend builder
COPY --from=backend-builder /app/ulysses-api /app/ulysses-api

# Copy the pre-built frontend from the frontend builder
COPY --from=frontend-builder /app/web/dist /app/web-dist

# Create a volume for persistent SQLite data
VOLUME /app/data

# Ensure the data directory is writable by the non-root user
RUN mkdir -p /app/data && chown -R ulysses:ulysses /app/data /app/web-dist

# Environment variables (sensible defaults for Docker)
ENV HOST=0.0.0.0
ENV PORT=8000
ENV DATABASE_URL=sqlite:/app/data/ulysses.db?mode=rwc
ENV WEB_DIST=/app/web-dist
ENV RUST_LOG=ulysses_api=info

# Expose the API port
EXPOSE 8000

# Switch to non-root user
USER ulysses

# Health check: verify the API is responding
HEALTHCHECK --interval=30s --timeout=3s --start-period=10s --retries=3 \
    CMD curl -sf http://localhost:8000/api/proxy/health || exit 1

ENTRYPOINT ["/app/ulysses-api"]
