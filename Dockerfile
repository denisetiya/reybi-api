# ============================================================================
#  Reybi API (Rust) – production image
#  - multi-stage build (chef → build → runtime)
#  - cargo-chef for fast incremental builds
#  - non-root user
#  - release binary, distroless-style runtime
# ============================================================================

ARG RUST_VERSION=1.85

# ---------- Stage 1: planner (dependency cache) ----------
FROM rust:${RUST_VERSION}-bookworm AS chef
RUN cargo install cargo-chef --locked
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# ---------- Stage 2: build ----------
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies (cached layer)
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin reybi-api

# ---------- Stage 3: runtime (tiny) ----------
FROM debian:bookworm-slim AS runtime
RUN apt-get update \
 && apt-get install -y --no-install-recommends \
      ca-certificates curl tini \
 && rm -rf /var/lib/apt/lists/*

# create non-root user
RUN groupadd -r -g 1001 reybi \
 && useradd  -r -u 1001 -g reybi -d /app -s /sbin/nologin reybi

WORKDIR /app
RUN mkdir -p /app/uploads /app/migrations \
 && chown -R reybi:reybi /app

COPY --from=builder --chown=reybi:reybi /app/target/release/reybi-api /usr/local/bin/reybi-api
COPY --from=builder --chown=reybi:reybi /app/migrations              /app/migrations

USER reybi
EXPOSE 3000
ENV PORT=3000 \
    RUST_LOG=info

HEALTHCHECK --interval=30s --timeout=5s --start-period=20s --retries=3 \
  CMD curl -fsS http://127.0.0.1:${PORT}/ || exit 1

# tini = proper PID 1, forwards signals, reaps zombies
ENTRYPOINT ["/usr/bin/tini", "--"]
CMD ["/usr/local/bin/reybi-api"]
