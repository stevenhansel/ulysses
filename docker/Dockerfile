# =============================================================================
# Ulysses — Dockerfile (Leptos SSR + WASM)
#
# Based on the official Leptos deployment guide:
#   https://book.leptos.dev/deployment/ssr.html
# =============================================================================

# ---- Stage 1: Builder --------------------------------------------------------
FROM rust:1.97-slim-bookworm AS builder

RUN apt-get update -y && \
    apt-get install -y --no-install-recommends \
        pkg-config libssl-dev curl perl ca-certificates && \
    apt-get autoremove -y && \
    apt-get clean -y && \
    rm -rf /var/lib/apt/lists/*

# Install cargo-leptos and WASM target
RUN OPENSSL_NO_VENDOR=1 cargo install cargo-leptos && \
    rustup target add wasm32-unknown-unknown

WORKDIR /app

COPY . .

RUN cargo leptos build --release

# ---- Stage 2: Runtime --------------------------------------------------------
FROM debian:bookworm-slim AS runtime

WORKDIR /app

RUN apt-get update -y && \
    apt-get install -y --no-install-recommends ca-certificates && \
    apt-get autoremove -y && \
    apt-get clean -y && \
    rm -rf /var/lib/apt/lists/*

# Copy the SSR binary
COPY --from=builder /app/target/release/ulysses /app/ulysses

# Copy WASM/JS/CSS assets
COPY --from=builder /app/site /app/site

VOLUME /app/data

ENV DATABASE_URL=sqlite:/app/data/ulysses.db?mode=rwc
ENV RUST_LOG=ulysses=info
ENV LEPTOS_SITE_ADDR=0.0.0.0:8000
ENV LEPTOS_SITE_ROOT=/app/site
ENV LEPTOS_SITE_PKG_DIR=/app/site/pkg
ENV LEPTOS_OUTPUT_NAME=ulysses

EXPOSE 8000

CMD ["/app/ulysses"]
