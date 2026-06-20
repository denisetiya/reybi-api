# syntax=docker/dockerfile:1.7
# ============================================================================
#  Reybi API (Rust/Axum + sqlx) – production image
#  - cargo-chef for cached dependency layer
#  - multi-stage: chef → planner → builder → runtime
#  - distroless-like runtime (debian-slim, non-root)
# ============================================================================

# ---------- Stage 0: toolchain + planner ----------
FROM rust:1-slim-bookworm AS planner
WORKDIR /app
RUN cargo install cargo-chef --locked
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# ---------- Stage 1: dependency build (cache hit on Cargo.toml/lock) ----------
FROM rust:1-slim-bookworm AS chef
WORKDIR /app
# pkg-config + libssl-dev required by openssl-sys (transitive: reqwest + firebase-auth)
RUN apt-get update && apt-get install -y --no-install-recommends \
      pkg-config libssl-dev ca-certificates \
    && rm -rf /var/lib/apt/lists/*
RUN cargo install cargo-chef --locked
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# ---------- Stage 2: source build ----------
FROM rust:1-slim-bookworm AS builder
WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends \
      pkg-config libssl-dev ca-certificates \
    && rm -rf /var/lib/apt/lists/*
COPY --from=chef /app/target/release/deps /app/target/release/deps
COPY . .
# SQLX_OFFLINE=true avoids needing a live DB at compile time.
ENV SQLX_OFFLINE=true
RUN cargo build --release --bin reybi-api \
 && strip target/release/reybi-api

# ---------- Stage 3: runtime (tiny) ----------
FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y --no-install-recommends \
      ca-certificates curl tini \
    && rm -rf /var/lib/apt/lists/*

# non-root user
RUN groupadd -r -g 1001 reybi \
 && useradd  -r -u 1001 -g reybi -d /app -s /sbin/nologin reybi

WORKDIR /app
RUN mkdir -p /app/uploads /app/migrations \
 && chown -R reybi:reybi /app

# copy binary + migrations (binary built in stage 2)
COPY --from=builder --chown=reybi:reybi /app/target/release/reybi-api /usr/local/bin/reybi-api
COPY --from=builder --chown=reybi:reybi /app/migrations              /app/migrations

USER reybi
ENV RUST_LOG=reybi_api=info,tower_http=info,sqlx=warn \
    HOST=0.0.0.0 \
    PORT=3000
EXPOSE 3000

HEALTHCHECK --interval=30s --timeout=5s --start-period=20s --retries=3 \
  CMD curl -fsS http://127.0.0.1:${PORT}/ || exit 1

# tini = proper PID 1, reaps zombies, forwards signals
ENTRYPOINT ["/usr/bin/tini", "--"]
CMD ["reybi-api"]