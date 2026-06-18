# Reybi API

**E-commerce & Waste Management Platform** — Rust rewrite from TypeScript/NestJS.

[![Rust](https://img.shields.io/badge/Rust-1.82+-orange?logo=rust)](https://www.rust-lang.org/)
[![Axum](https://img.shields.io/badge/Axum-0.8-blue?logo=rust)](https://github.com/tokio-rs/axum)
[![PostgreSQL](https://img.shields.io/badge/PostgreSQL-16+-blue?logo=postgresql)](https://www.postgresql.org/)
[![License](https://img.shields.io/badge/License-MIT-green)](LICENSE)
[![CI](https://img.shields.io/badge/build-0%20errors%200%20warnings-success)](https://github.com/denisetiya/reybi-api)

---

## Overview

High-performance REST API for the Reybi e-commerce ecosystem — product catalog, cart, orders, payments, waste deposite management, landfill locations, and seller operations. 42 endpoints across 14 modules.

**Key decisions:**
- Rust + Axum 0.8 for 100K+ req/s throughput
- Cursor pagination on every list endpoint (no offset pagination)
- Firebase JWT auth with auto-refresh token rotation
- Structured API responses: `{ success, data, meta }`
- i18n: English (EN) + Indonesian (ID)
- Apinox-generated docs: OpenAPI, Postman, Insomnia, Scalar, Hurl, curl

---

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Language | Rust (edition 2021) |
| Framework | Axum 0.8 + Tokio |
| Database | PostgreSQL + sqlx 0.8 (pooled) |
| Auth | JWT (jsonwebtoken 9) + Argon2 |
| Validation | validator 0.20 (derive) |
| Serialization | serde + serde_json |
| HTTP | tower-http (CORS, compression, tracing) |
| Compression | Gzip + Brotli + Zstd |
| Docs | Apinox (OpenAPI, Postman, Scalar, Insomnia, Hurl) |

---

## Quick Start

### Prerequisites

- Rust 1.82+
- PostgreSQL 16+
- Docker (optional, for containerized build)

### Environment

```bash
cp .env.example .env
# Edit .env with your PostgreSQL URL, JWT secret, Firebase project ID
```

Required variables:
```env
DATABASE_URL=postgres://user:pass@localhost:5432/reybi
JWT_SECRET=your-jwt-secret-at-least-32-chars
FIREBASE_PROJECT_ID=your-firebase-project
HOST=0.0.0.0
PORT=3000
```

### Build & Run

```bash
# Compile (release mode with LTO)
cargo build --release

# Run
./target/release/reybi-api

# Or via Docker
docker build -f Dockerfile.build -t reybi-api .
docker run -p 3000:3000 --env-file .env reybi-api
```

Server starts on `http://localhost:3000/v1`.

---

## Module Structure

```
src/
├── main.rs              # Server entry point, route composition
├── lib.rs               # Module declarations
├── config/
│   └── mod.rs           # AppConfig + AppState
├── errors/
│   └── mod.rs           # AppError enum (13 error codes)
├── middleware/
│   └── mod.rs           # JWT auth, refresh token rotation
├── i18n/
│   ├── mod.rs           # Locale middleware
│   ├── en.rs            # English messages
│   └── id.rs            # Indonesian messages
├── models/              # Database structs (14 Prisma tables → Rust)
├── dto/                 # Shared request/response DTOs
├── utils/               # Helper functions
├── common/
│   ├── mod.rs
│   ├── response.rs      # ok(), ok_paginated(), message() builders
│   └── pagination.rs    # PaginationQuery + paginate() + HasCursor trait
└── modules/             # Domain modules (NestJS-style)
    ├── auth/            # handler, service, dto, routes, mod
    ├── product/         # handler, service, dto, routes, mod
    ├── banner/          # handler, service, dto, routes, mod
    ├── article/         # handler, service, dto, routes, mod
    ├── profile/         # handler, service, dto, routes, mod
    ├── review/          # handler, service, dto, routes, mod
    ├── cart/            # handler, service, dto, routes, mod
    ├── order/           # handler, service, dto, routes, mod
    ├── deposite/        # handler, service, dto, routes, mod
    ├── landfill/        # handler, service, dto, routes, mod
    ├── trash/           # handler, service, dto, routes, mod
    ├── address/         # handler, service, dto, routes, mod
    └── saller/          # handler, service, dto, routes, mod
```

Each module follows the pattern: `routes → handler → service → models`.

---

## API Overview

Base URL: `http://localhost:3000/v1`

| Module | Endpoints | Auth | Description |
|--------|-----------|------|-------------|
| Auth | 3 | Mixed | Register, login (Firebase → JWT), password reset |
| Products | 6 | Mixed | CRUD + public list with search/filter |
| Banners | 3 | Mixed | Banner CRUD + public list by type |
| Articles | 5 | Mixed | Article CRUD + public list |
| Profile | 2 | Required | Get/update user profile |
| Reviews | 2 | Required | Create/update product reviews |
| Carts | 3 | Required | Add/get/remove cart items |
| Orders | 4 | Required | Create/cancel/list orders |
| Deposites | 3 | Required | Waste deposite pickup requests |
| Landfills | 4 | Admin | Landfill location CRUD |
| Trash | 4 | Admin | Trash type CRUD |
| Addresses | 2 | Required | Create/update addresses |
| Sallers | 1 | Public | List seller products |

Full API docs: [OpenAPI](./docs/apinox/openapi/reybi-api.openapi.yaml) | [Scalar](./docs/apinox/docs/reybi-api.scalar.html) | [Postman](./docs/apinox/postman/reybi-api.postman_collection.json)

---

## Cursor Pagination

All list endpoints use **cursor-based pagination** — no offsets. This ensures stable pagination even when rows are inserted or deleted between requests.

### How It Works

```
GET /v1/products?cursor=eyJpZCI6NTB9&limit=25
```

1. **First page** — omit `cursor`, the API returns the first N items
2. **Next pages** — use the `cursor` from `meta.pagination` in the previous response
3. **Last page** — when `has_more` is `false`, there are no more items

### Response Format

```json
{
  "success": true,
  "data": [ ... ],
  "meta": {
    "locale": "en",
    "pagination": {
      "cursor": "eyJpZCI6MjV9",
      "has_more": true,
      "count": 25
    }
  }
}
```

| Field | Type | Description |
|-------|------|-------------|
| `data` | array | Page items (up to `limit`) |
| `meta.pagination.cursor` | string\|null | Opaque token for next page. `null` when `has_more` is `false` |
| `meta.pagination.has_more` | bool | `true` if more items exist after this page |
| `meta.pagination.count` | int | Number of items in this page |

### Query Parameters

| Param | Type | Default | Max | Description |
|-------|------|---------|-----|-------------|
| `cursor` | `string` | none | — | Opaque token from previous page (omit for first page) |
| `limit` | `integer` | `25` | `100` | Items per page |

### Example Flow (3 Pages)

```bash
BASE="http://localhost:3000/v1"

# Page 1 — first 25 products
curl "$BASE/products?limit=25" | jq '.meta.pagination'
# {
#   "cursor": "eyJpZCI6MjV9",
#   "has_more": true,
#   "count": 25
# }

# Page 2 — next 25 products
curl "$BASE/products?cursor=eyJpZCI6MjV9&limit=25" | jq '.meta.pagination'
# {
#   "cursor": "eyJpZCI6NTB9",
#   "has_more": true,
#   "count": 25
# }

# Page 3 — last page (only 12 items remain)
curl "$BASE/products?cursor=eyJpZCI6NTB9&limit=25" | jq '.meta.pagination'
# {
#   "cursor": null,
#   "has_more": false,
#   "count": 12
# }
```

### Endpoints with Cursor Pagination

| Endpoint | Filter/Search | Module |
|----------|--------------|--------|
| `GET /v1/products` | `category`, `search` | product |
| `GET /v1/banners` | — | banner |
| `GET /v1/banners/type/{type}` | — | banner |
| `GET /v1/articles` | — | article |
| `GET /v1/carts/user/{user_id}` | — | cart |
| `GET /v1/orders` | — | order |
| `GET /v1/orders/user/{user_id}` | — | order |
| `GET /v1/deposites` | — | deposite |
| `GET /v1/deposites/user/{id}` | — | deposite |
| `GET /v1/landfills` | — | landfill |
| `GET /v1/trash` | — | trash |
| `GET /v1/sallers/{id}/products` | — | saller |

### Important Notes

- **Cursor is opaque** — never parse or construct it client-side. The format may change.
- **`has_more: false`** means end of dataset — `cursor` will be `null`.
- **`count`** reflects items in the *current* page, not total.
- **Order stability** — items ordered by `created_at DESC, id DESC`.
- **Default limit is 25** — always specify `limit` for predictable page sizes.

---

## Auth Flow

```
┌─────────┐     Firebase ID Token      ┌──────────┐
│ Client  │ ──────────────────────────> │  Server  │
│         │ <────── JWT access token ── │          │
│         │       + refresh token       │          │
└─────────┘                             └──────────┘

Access token expires → Client sends x-refresh-token header
Server validates → Returns x-new-access-token + new x-refresh-token
```

- **Login**: `POST /v1/auth` with Firebase ID token → JWT access (3h) + refresh (7d)
- **API access**: `Authorization: Bearer <access_token>`
- **Auto-refresh**: `x-refresh-token` header on 401 → `x-new-access-token` on 200
- **Password hashing**: Argon2 (via Firebase)

---

## Error Codes

All errors follow: `{ success: false, error: { code, message, details? }, meta: { locale } }`

| HTTP | Code | When |
|------|------|------|
| 400 | `VALIDATION_ERROR` | Invalid body/query/param |
| 401 | `UNAUTHORIZED` | Missing/invalid token |
| 402 | `PAYMENT_REQUIRED` | Payment needed |
| 403 | `FORBIDDEN` | Authenticated but not authorized |
| 404 | `NOT_FOUND` | Resource doesn't exist |
| 409 | `CONFLICT` | Duplicate/state conflict |
| 413 | `PAYLOAD_TOO_LARGE` | Body exceeds limit |
| 415 | `UNSUPPORTED_MEDIA_TYPE` | Wrong Content-Type |
| 422 | `UNPROCESSABLE_ENTITY` | Semantic error |
| 429 | `RATE_LIMITED` | Too many requests |
| 500 | `INTERNAL_ERROR` | Unexpected server error |
| 502 | `BAD_GATEWAY` | Upstream failure |
| 503 | `SERVICE_UNAVAILABLE` | Maintenance/overloaded |

---

## Performance

| Metric | Value |
|--------|-------|
| Target throughput | 100K+ req/s |
| DB pool | 4–20 connections, 5s acquire timeout |
| Idle timeout | 300s |
| Compression | Gzip + Brotli + Zstd |
| Release profile | opt-level=3, LTO, 1 codegen unit, stripped binary |
| Binary size | ~20MB (stripped) |

---

## Security

- JWT access (3h) + refresh (7d) token rotation
- All state-changing endpoints behind auth middleware
- Parameterized SQL queries (sqlx — no string interpolation)
- Input validation on all DTOs (validator derive)
- Argon2 password hashing
- CORS with explicit origins (permissive in dev)
- `unsafe_code = "forbid"` in Cargo.toml
- HTTP tracing layer for audit

---

## Development

```bash
# Check for issues
cargo clippy -- -D warnings
cargo audit

# Run tests
cargo test

# Build docs
~/bin/apinox build docs/reybi-api.yaml -o docs/apinox -f all

# Docker build (no host compiler needed)
docker build -f Dockerfile.build -t reybi-check .
```

### File Size Limit

Every file ≤ 500 lines. Use `wc -l src/**/*.rs` to verify.

### Pre-commit Checklist

- [ ] `cargo clippy -- -D warnings` passes (0 errors, 0 warnings)
- [ ] `cargo audit` clean
- [ ] All list endpoints use cursor pagination
- [ ] Responses follow `{ success, data, meta }` format
- [ ] CHANGELOG.md updated
- [ ] Apinox schema + docs updated

---

## Documentation

| Format | Path |
|--------|------|
| OpenAPI 3.0 | [docs/apinox/openapi/reybi-api.openapi.yaml](./docs/apinox/openapi/reybi-api.openapi.yaml) |
| Scalar HTML | [docs/apinox/docs/reybi-api.scalar.html](./docs/apinox/docs/reybi-api.scalar.html) |
| Postman Collection | [docs/apinox/postman/reybi-api.postman_collection.json](./docs/apinox/postman/reybi-api.postman_collection.json) |
| Insomnia | [docs/apinox/insomnia/reybi-api.insomnia.json](./docs/apinox/insomnia/reybi-api.insomnia.json) |
| Markdown | [docs/apinox/docs/reybi-api.md](./docs/apinox/docs/reybi-api.md) |
| Hurl Tests | [docs/apinox/hurl/reybi-api.hurl](./docs/apinox/hurl/reybi-api.hurl) |
| cURL Script | [docs/apinox/hurl/reybi-api.sh](./docs/apinox/hurl/reybi-api.sh) |

---

## Internationalization (i18n)

API supports **English (EN)** and **Bahasa Indonesia (ID)** responses.

### Switch Language

Priority order:
1. **Query param** `?locale=id` (highest priority)
2. **Header** `Accept-Language: id`
3. **Default** → English (`en`)

### Example

```bash
# English (default)
curl https://api.example.com/v1/products

# Indonesian via query param
curl https://api.example.com/v1/products?locale=id

# Indonesian via header
curl -H "Accept-Language: id" https://api.example.com/v1/products
```

### Response Format

```json
{
  "success": true,
  "data": [...],
  "meta": { "locale": "id" }
}
```

### Error Response (ID)

```json
{
  "success": false,
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Validasi gagal",
    "details": []
  },
  "meta": { "locale": "id" }
}
```

### Available Keys (25+)

| Key | EN | ID |
|-----|----|----|
| VALIDATION_ERROR | Validation failed | Validasi gagal |
| NOT_FOUND | Resource not found | Sumber daya tidak ditemukan |
| UNAUTHORIZED | Authentication required | Autentikasi diperlukan |
| FORBIDDEN | Access denied | Akses ditolak |
| CONFLICT | Resource conflict | Konflik sumber daya |
| RATE_LIMITED | Too many requests | Terlalu banyak permintaan |
| INTERNAL_ERROR | Internal server error | Kesalahan server internal |
| TOKEN_EXPIRED | Invalid or expired token | Token tidak valid atau kedaluwarsa |
| TOKEN_MISSING | Access token not provided | Token akses tidak disediakan |
| PRODUCT_NOT_FOUND | Product not found | Produk tidak ditemukan |
| ORDER_NOT_FOUND | Order not found | Pesanan tidak ditemukan |
| REGISTER_SUCCESS | Registration successful | Pendaftaran berhasil |
| PAYMENT_PENDING | Payment is pending | Pembayaran sedang diproses |

---

## Changelog

See [CHANGELOG.md](./CHANGELOG.md)

---

## License

MIT © [denisetiya](https://github.com/denisetiya)
