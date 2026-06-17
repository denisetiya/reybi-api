# Changelog

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
