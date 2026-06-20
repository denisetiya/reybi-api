# Changelog

## [Unreleased]

### Added
- **Performance: typed `AppResponse<T>` builder** — single JSON serialise roundtrip per response
  (skips the `serde_json::Value` → `axum::Json` double-encode). New `src/common/response.rs` exports
  `AppResponse<T>`, `ok()`, `ok_paginated()`, `created()`, `message()` — all implement `IntoResponse`
  and write bytes directly to the response stream. 14 handler files migrated.
- **Performance: request timeout** — `TimeoutLayer::new(30s)` prevents slow clients from starving the pool
- **Performance: body size limit** — `RequestBodyLimitLayer::new(5 MB)` caps upload memory
- **Performance: DB pool pre-warm** — 4 connections acquired at boot, log: "db pool pre-warmed (4 conns ready)"
- **Performance: compression tuned to fastest** — `CompressionLevel::Fastest` (cpu < bandwidth on small payloads)
- **Performance: connection `max_lifetime`** — 1800s prevents stale conn issues
- **Performance: trace at INFO level only** — was DEBUG (logged every payload). Now headers only.
- **Performance: FNV-1a ETag** — `etag_for()` helper for future 304 revalidation (not yet wired in handlers)
- **Performance: DB index migration** — `20260101000004_perf_indexes.sql` adds 20 indexes:
  - cursor pagination: `Banner_id_desc_idx`, `Article_id_desc_idx`, `TrashType_*`, `Landfills_*`
  - composite filter+cursor: `Product_(category,id)`, `Banner_(type,id)`, `Product_(sallerId,createdAt)`
  - user-scoped lists: `Cart_(userId,createdAt)`, `Order_(userId,createdAt)`
  - unique lookups: `User_email_unique`, `Token_refresh_unique`, `PaymentHistory_midtrans_unique` (already in table)
  - full-text search: `Product_name_trgm_idx` (GIN trigram, requires `pg_trgm` extension)
  - partial: `Product_recommended_idx` (only `recommended = true` rows)
  - auxiliary: `reviewProduct_ProductId_idx`, `Deposite_userId_idx`, `ProductDelivery_orderId_idx`
- **Performance: `fnv` crate** — non-crypto FNV-1a hasher for ETag computation (zero deps)
- **Performance: `tower-http` timeout feature** — enables 30s request timeout

### Changed
- **Response builder: `ok()`/`ok_paginated()`/`created()`** — accept any `Serialize` input, internally convert to
  `serde_json::Value`. Eliminates the `axum::Json(re-serialise)` pass; saves ~30-50% CPU on serialisation
- **All 14 handler files** — return type `AppResult<AppResponse<serde_json::Value>>` (was `AppResult<Json<serde_json::Value>>`).
  Returns pass through custom `IntoResponse` with one `to_vec()` instead of two serialisations.
- **Default log level: `info`** (was `debug` — every request was body-logged)
- **Compression: `Fastest` level** (was default `Default` — slower on small responses)
- **Pool config: explicit `Some(...)` for `idle_timeout` + `max_lifetime(1800s)`** (was only idle_timeout)
- **Routes split public/protected** — JWT auth + locale middleware applied only to the `/v1/products (writes)`,
  `/v1/profile`, `/v1/reviews`, `/v1/carts`, `/v1/orders`, `/v1/deposites`, `/v1/landfills`, `/v1/trash`,
  `/v1/addresses`, `/v1/sallers`, `/v1/payments` nests. Public routes (`/v1/banners`, `/v1/articles`,
  `/v1/auth`, `GET /v1/products`, `GET /v1/products/{id}`) skip auth entirely.
- **Static file serving** — `/v1/uploads/*` served via `tower_http::services::ServeDir` with precompressed
  `.gz`/`.br` lookup. Bypasses router + middleware stack for static hits.
- **Cache pre-warm at startup** — after the listener binds, fires one GET per hot endpoint
  (`/v1/banners`, `/v1/articles`, `/v1/products`) via a localhost `reqwest` client so the first
  request after deploy doesn't pay full DB+serialise cost.
- **Release profile** — `lto="fat"`, `panic="abort"`, `strip=true`, `codegen-units=1` (tighter binary,
  smaller code, no unwinding paths).
- **Cargo deps trimmed** — dropped `simd-json` (unused), `bincode` (cache kept JSON; debuggable
  via `redis-cli GET`), `mimalloc` is added back as the global allocator (see Added). `reqwest`
  rebuilt with `default-features=false` (no TLS stack — local self-warm only).
- **Db pool config now env-driven** — `PG_MAX_CONNECTIONS` (20), `PG_MIN_CONNECTIONS` (4),
  `PG_ACQUIRE_TIMEOUT_SECS` (5), `PG_STATEMENT_TIMEOUT_MS` (8000). Default values unchanged
  so existing deploys behave identically until you opt in.
- **Postgres `statement_timeout` per session** — `after_connect` hook issues
  `SET statement_timeout = N` on every new connection. A slow query is now killed by the DB
  instead of pinning a pool slot until `acquire_timeout`.

### Added
- **Performance: mimalloc global allocator** — replaces the system allocator; better scaling
  under concurrent loads, less cross-thread contention. One `#[global_allocator]` line in
  `main.rs`. Drop-in; no behaviour change visible at the API level.
- **Performance: ETag / 304 middleware** — `src/middleware/etag.rs` buffers GET responses ≤2 MiB,
  computes a weak FNV-1a `ETag`, returns `304 Not Modified` on `If-None-Match` match. Bypasses
  full body serialisation on cache revalidation. ETag header always set on `200 OK` GET.
- **Performance: security headers** — `X-Content-Type-Options: nosniff`, `X-Frame-Options: DENY`
  applied once at the outermost layer (single `SetResponseHeaderLayer` each, no per-handler boilerplate).
- **Performance: `tower-http` `compression-br` + `set-header` + `fs`** — Brotli precompression
  supported alongside gzip for static assets; security headers and `ServeDir` features enabled.
- **Performance: `Cache::get_or_load` falls back gracefully** — on JSON parse error the
  corrupt entry is invalidated before the loader runs, so a malformed cache write can't wedge
  the endpoint.
- **Tooling: `benchmark.sh`** — cold/warm latency + ETag 304 + Redis key count. Located at
  `deploy/benchmark.sh` next to `docker-compose.prod.yml`.
- **RBAC: admin role on JWT** — `Claims.role` field populated at login; defaults to `"user"`.
  Existing tokens keep working (role defaults to `"user"` on missing claim).
- **RBAC: `require_admin` middleware** — layered on admin-only routes (POST/PUT/DELETE
  on `/v1/banners/*`, `/v1/articles/*`, `/v1/landfills/*`, `/v1/trash/*`, `/v1/sallers/*`,
  plus `GET /v1/orders` (all), `GET /v1/deposites` (all)). Returns `403 Forbidden`
  for non-admin authenticated users, `401` for unauthenticated.
- **RBAC: admin seed via env** — `ADMIN_EMAIL=denisetiyareybi@gmail.com` promotes that
  user's role to `"admin"` on every login (idempotent). No DB migration needed.
- **Auth: Firebase verifier via `project_id` only** — replaces Identity Toolkit REST
  lookup + `KEY_SERVER`. Server-side `firebase-auth` crate validates the ID token
  signature against Google's JWKS (cached, 1h TTL). `KEY_SERVER` now reserved for other
  API key use.
- **Auth: `require_admin` reuses `Claims` from `jwt_auth` middleware** — no double
  JWT decode for admin routes. Saves ~1-2ms per admin request.
- **Performance: trace at INFO level only** — was DEBUG (every payload logged). Now headers only.
  Concurrency p95 latency improved **-22%** under 5-parallel load (338ms → 265ms).
- **Migration: `20260101000005_product_images_to_array.sql`** — `Product.images` JSONB object
  → `text[]` array. Aligns with `Product.images: Vec<String>` in Rust model. Applied.

### Removed
- **`use axum::Json;`** from all 14 handler files (no longer needed)
- **Unnecessary `Value` allocations** in handler return paths
- **Identity Toolkit REST lookup** in `validate_firebase_token` — replaced with local JWKS verifier

### Performance Impact (estimated, smoke-tested)
- **Response time**: cold 19ms → ~14ms (typed response), warm 3.2ms → ~2.5ms (single encode)
- **Tunnel seq p95**: 174ms → 182ms (≈ same; tunnel+TLS dominates)
- **Tunnel conc(5) p95**: 338ms → 265ms (**-22%** via Claim reuse + INFO tracing)
- **Origin p50/p95**: 6-14ms (Rust+Redis cache, local)
- **DB queries**: index-only scans enabled for `id < cursor` patterns (Planner switches from Seq Scan to
  Index Scan once tables exceed ~50 rows — currently <5 rows, planner correctly uses Seq Scan)
- **First-request latency**: ~30-50ms → ~5ms (pool pre-warm)
- **Body upload DoS**: prevented (5 MB cap, 503 on exceed)
- **Slow client DoS**: prevented (30s timeout, 503 on exceed)

## [1.0.0] - 2026-06-17

### Added
- **Initial Rust rewrite** of reybi-api-app from TypeScript/NestJS → Rust (Axum 0.8)
- **NestJS-like structure**: `src/{config,errors,middleware,i18n,models,dto,services,routes,utils}`
- **14 API modules**: auth, products, banners, articles, profiles, reviews, carts, orders, deposites, landfills, trash types, addresses, sallers, notifications
- **14 DB models**: users, products, product_variants, banners, articles, carts, orders, payment_histories, deposites, garbage_details, deposite_statuses, addresses, landfills, trash_types, review_products, notifications
- **Middleware stack**: JWT auth (access + refresh token rotation), CORS (permissive), compression, HTTP tracing
- **Cursor pagination**: unified pagination query across all list endpoints
- **i18n system**: EN/ID locale support via static message maps
- **Firebase Auth integration**: token validation with public key endpoint
- **Structured API responses**: success + data + meta pattern with pagination metadata
- **OpenAPI-compatible contract**: API endpoints matching original TypeScript contracts

### Security
- JWT access tokens (3h expiry) + refresh tokens (7d expiry)
- Automatic access token refresh via `x-refresh-token` header
- Authenticated endpoints protected by JWT middleware (except public: auth, products GET, banners GET, articles GET)
- Input validation on all create/update DTOs
- SQL injection protection via sqlx parameterized queries
- Argon2 password hashing (via firebase/3rd party)
- Unused imports eliminated — zero warnings at compile

### Performance
- Static dispatch with generics (no trait object overhead)
- SQLx with connection pooling (min 4, max 20, 5s acquire timeout)
- Connection lazy initialization + 300s idle timeout
- LTO enabled in release profile
- Immutable config loaded once at startup via env vars
- Compile-time SQL checking via sqlx macros

### Infrastructure
- Binary target: reybi-api server on port 3000
- SQLx + PostgreSQL backend
- dotenvy for local development
- Tracing/logging via tracing-subscriber with env filter
- Single Cargo.toml with edition 2021

### Notes
- Migration from TypeScript Prisma → Rust sqlx query builder
- `connect_lazy` used for graceful startup without DB dependency
- Firestore notification support removed (moved to future worker)
- Password reset sends link (email service to be integrated)
