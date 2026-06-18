# Reybi API API Documentation

**Version:** `1.0.0`

Reybi API — Rust rewrite (Axum 0.8).
E-commerce platform with cart, orders, deposite, waste management.

## Cursor Pagination

All list endpoints use cursor-based pagination (NO offset). This ensures stable
pagination even when rows are inserted/deleted between requests.

### How It Works

```
GET /v1/products?cursor=eyJpZCI6NTB9&limit=25
```

1. **First page** — omit `cursor`, API returns first N items
2. **Next pages** — use `cursor` from `meta.pagination` of previous response
3. **Last page** — `has_more: false`, `cursor: null`

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
| `meta.pagination.cursor` | string\|null | Opaque token for next page. `null` when has_more=false |
| `meta.pagination.has_more` | bool | `true` if more items exist after this page |
| `meta.pagination.count` | int | Number of items in THIS page (not total) |

### Query Parameters

| Param | Type | Default | Max | Description |
|-------|------|---------|-----|-------------|
| `cursor` | string | — | — | Opaque token from previous page (omit for first page) |
| `limit` | integer | 25 | 100 | Items per page |

### Example Flow

```bash
# Page 1 — first 25 products
curl "http://localhost:3000/v1/products?limit=25"
# Response: meta.pagination.cursor = "eyJpZCI6MjV9", has_more = true

# Page 2 — use cursor from page 1
curl "http://localhost:3000/v1/products?cursor=eyJpZCI6MjV9&limit=25"
# Response: meta.pagination.cursor = "eyJpZCI6NTB9", has_more = true

# Page 3 — last page (say 12 items remain)
curl "http://localhost:3000/v1/products?cursor=eyJpZCI6NTB9&limit=25"
# Response: meta.pagination.cursor = null, has_more = false, count = 12
```

### Important

- **Cursor is opaque** — never parse or construct it client-side
- **has_more=false** means end of dataset
- **count** is current page items, not total
- **Order** — created_at DESC, id DESC
- **Default limit is 25** — specify limit for predictable page sizes

### Endpoints with Cursor Pagination

GET /v1/products, GET /v1/banners, GET /v1/banners/type/{type},
GET /v1/articles, GET /v1/carts/user/{user_id}, GET /v1/orders,
GET /v1/orders/user/{user_id}, GET /v1/deposites, GET /v1/deposites/user/{id},
GET /v1/landfills, GET /v1/trash/types, GET /v1/sallers/products/{id}

**Base URL:** `http://localhost:3000/v1`

## Environments

- **Development** — `http://localhost:3000/v1` — Development

## Authentication

**Default scheme:** `BearerAuth`

### BearerAuth (HTTP BEARER)

Firebase ID token (login) or JWT access token (API access)

**Header:** `Authorization`
**Prefix:** `Bearer `

## Table of Contents

- [auth (3 endpoints)](#auth)
- [products (6 endpoints)](#products)
- [banners (3 endpoints)](#banners)
- [articles (5 endpoints)](#articles)
- [profile (2 endpoints)](#profile)
- [reviews (2 endpoints)](#reviews)
- [carts (3 endpoints)](#carts)
- [orders (4 endpoints)](#orders)
- [deposites (3 endpoints)](#deposites)
- [landfills (4 endpoints)](#landfills)
- [trash (4 endpoints)](#trash)
- [addresses (2 endpoints)](#addresses)
- [sallers (1 endpoint)](#sallers)

## auth

Authentication (Firebase + JWT)

### 🔵 `/auth/register` 

**Register new user** — `POST`

**Tags:** `auth`

#### Request Body

**Content-Type:** `json`
**Actual Content-Type:** `application/json`
**Required:** ✅

**Schema:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | `string` | ✅ | — |
| `password` | `string` | ✅ | — |
| `email` | `string` | ✅ | — |

#### Example cURL

```bash
curl -X POST 'http://localhost:3000/v1/auth/register' \
  -d '{}'
```

#### Responses

##### ✅ 201 `Register new user - created`

*default*

```json
{
  "data": {
    "created_at": "2025-01-01T00:00:00",
    "email": "newuser@example.com",
    "fb_id": "firebase-uid-123",
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "name": "Jane Doe",
    "role": "user",
    "updated_at": "2025-01-01T00:00:00"
  },
  "meta": {
    "locale": "en"
  },
  "success": true
}
```

##### ⚠️ 400 `Validation failed`

*default*

```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "details": [],
    "message": "Validation failed"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ⚠️ 401 `Authentication required`

*default*

```json
{
  "error": {
    "code": "UNAUTHORIZED",
    "details": [],
    "message": "Authentication required"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ⚠️ 429 `Too many requests`

*default*

```json
{
  "error": {
    "code": "RATE_LIMITED",
    "details": [],
    "message": "Too many requests"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ❌ 500 `Internal server error`

*default*

```json
{
  "error": {
    "code": "INTERNAL_ERROR",
    "details": [],
    "message": "Internal server error"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

---

### 🔵 `/auth/reset-password` 

**Request password reset** — `POST`

**Tags:** `auth`

#### Request Body

**Content-Type:** `json`
**Actual Content-Type:** `application/json`
**Required:** ✅

**Schema:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `email` | `string` | ✅ | — |

#### Example cURL

```bash
curl -X POST 'http://localhost:3000/v1/auth/reset-password' \
  -d '{}'
```

#### Responses

##### ✅ 201 `Request password reset - created`

*default*

```json
{
  "data": {
    "email": "user@example.com"
  },
  "meta": {
    "locale": "en"
  },
  "success": true
}
```

##### ⚠️ 400 `Validation failed`

*default*

```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "details": [],
    "message": "Validation failed"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ⚠️ 401 `Authentication required`

*default*

```json
{
  "error": {
    "code": "UNAUTHORIZED",
    "details": [],
    "message": "Authentication required"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ⚠️ 429 `Too many requests`

*default*

```json
{
  "error": {
    "code": "RATE_LIMITED",
    "details": [],
    "message": "Too many requests"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ❌ 500 `Internal server error`

*default*

```json
{
  "error": {
    "code": "INTERNAL_ERROR",
    "details": [],
    "message": "Internal server error"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

---

### 🔵 `/auth` 

**Login (Firebase token → JWT)** — `POST`

**Tags:** `auth`

#### Request Body

**Content-Type:** `json`
**Actual Content-Type:** `application/json`
**Required:** ✅

**Schema:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|

#### Example cURL

```bash
curl -X POST 'http://localhost:3000/v1/auth' \
  -d '{}'
```

#### Responses

##### ✅ 201 `Login (Firebase token → JWT) - created`

*default*

```json
{
  "data": {
    "access_token": "eyJhbGciOiJIUzI1NiIs...",
    "expires_in": 3600,
    "refresh_token": "eyJhbGciOiJIUzI1NiIs...",
    "token_type": "Bearer",
    "user": {
      "created_at": "2025-01-01T00:00:00",
      "email": "user@example.com",
      "fb_id": "firebase-uid-123",
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "name": "John Doe",
      "phone_number": "+628123456789",
      "photo_url": "https://example.com/photo.jpg",
      "role": "user",
      "updated_at": "2025-01-01T00:00:00"
    }
  },
  "meta": {
    "locale": "en"
  },
  "success": true
}
```

##### ⚠️ 400 `Validation failed`

*default*

```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "details": [],
    "message": "Validation failed"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ⚠️ 401 `Authentication required`

*default*

```json
{
  "error": {
    "code": "UNAUTHORIZED",
    "details": [],
    "message": "Authentication required"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ⚠️ 429 `Too many requests`

*default*

```json
{
  "error": {
    "code": "RATE_LIMITED",
    "details": [],
    "message": "Too many requests"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ❌ 500 `Internal server error`

*default*

```json
{
  "error": {
    "code": "INTERNAL_ERROR",
    "details": [],
    "message": "Internal server error"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

---

## products

Product catalog

### 🟢 `/products` 

**List products (public, no auth required)** — `GET`

**Tags:** `products`

#### Query Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `cursor` | `string` | ❌ | — |
| `limit` | `integer` | ❌ | — |
| `category` | `string` | ❌ | — |
| `search` | `string` | ❌ | — |

#### Example cURL

```bash
curl -X GET 'http://localhost:3000/v1/products?cursor=example-cursor&limit=1&category=example-category&search=example-search'
```

#### Responses

##### ✅ 200 `List products (public, no auth required) - paginated`

*default*

```json
{
  "data": [
    {
      "available": 50,
      "category": "clothing",
      "coin": 10,
      "created_at": "2025-01-01T00:00:00",
      "description": "Premium cotton t-shirt",
      "discount": 10.5,
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "images": [
        "https://example.com/img1.jpg"
      ],
      "location": "Jakarta",
      "name": "T-Shirt",
      "price": 150000,
      "rating": 4.5,
      "recommended": true,
      "saller_id": "660e8400-e29b-41d4-a716-446655440001",
      "sold": 50,
      "stock": 100,
      "thumbnail": "https://example.com/thumb.jpg",
      "updated_at": "2025-01-01T00:00:00"
    }
  ],
  "meta": {
    "locale": "en",
    "pagination": {
      "count": 25,
      "cursor": "eyJpZCI6MjV9",
      "has_more": true
    }
  },
  "success": true
}
```

##### ⚠️ 404 `Resource not found`

*default*

```json
{
  "error": {
    "code": "NOT_FOUND",
    "details": [],
    "message": "Resource not found"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ⚠️ 429 `Too many requests`

*default*

```json
{
  "error": {
    "code": "RATE_LIMITED",
    "details": [],
    "message": "Too many requests"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ❌ 500 `Internal server error`

*default*

```json
{
  "error": {
    "code": "INTERNAL_ERROR",
    "details": [],
    "message": "Internal server error"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

---

### 🟢 `/products/{id}` 

**Get product by ID** — `GET`

**Tags:** `products`

#### Path Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `id` | `string` | ✅ | — |

#### Example cURL

```bash
curl -X GET 'http://localhost:3000/v1/products/id-example'
```

#### Responses

##### ✅ 200 `Get product by ID - success`

*default*

```json
{
  "data": {
    "available": 50,
    "category": "clothing",
    "coin": 10,
    "created_at": "2025-01-01T00:00:00",
    "description": "Premium cotton t-shirt",
    "discount": 10.5,
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "images": [
      "https://example.com/img1.jpg"
    ],
    "location": "Jakarta",
    "name": "T-Shirt",
    "price": 150000,
    "rating": 4.5,
    "recommended": true,
    "saller_id": "660e8400-e29b-41d4-a716-446655440001",
    "sold": 50,
    "stock": 100,
    "thumbnail": "https://example.com/thumb.jpg",
    "updated_at": "2025-01-01T00:00:00"
  },
  "meta": {
    "locale": "en"
  },
  "success": true
}
```

##### ⚠️ 404 `Resource not found`

*default*

```json
{
  "error": {
    "code": "NOT_FOUND",
    "details": [],
    "message": "Resource not found"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ⚠️ 429 `Too many requests`

*default*

```json
{
  "error": {
    "code": "RATE_LIMITED",
    "details": [],
    "message": "Too many requests"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ❌ 500 `Internal server error`

*default*

```json
{
  "error": {
    "code": "INTERNAL_ERROR",
    "details": [],
    "message": "Internal server error"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

---

### 🟠 `/products/{id}` 

**Update product** — `PUT`

**Tags:** `products`

#### Path Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `id` | `string` | ✅ | — |

#### Request Body

**Content-Type:** `json`
**Actual Content-Type:** `application/json`
**Required:** ✅

**Schema:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|

#### Example cURL

```bash
curl -X PUT 'http://localhost:3000/v1/products/id-example' \
  -d '{}'
```

#### Responses

##### ✅ 200 `Update product - success`

*default*

```json
{
  "data": {
    "available": 50,
    "category": "clothing",
    "coin": 10,
    "created_at": "2025-01-01T00:00:00",
    "description": "Premium cotton t-shirt",
    "discount": 10.5,
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "images": [
      "https://example.com/img1.jpg"
    ],
    "location": "Jakarta",
    "name": "T-Shirt",
    "price": 150000,
    "rating": 4.5,
    "recommended": true,
    "saller_id": "660e8400-e29b-41d4-a716-446655440001",
    "sold": 50,
    "stock": 100,
    "thumbnail": "https://example.com/thumb.jpg",
    "updated_at": "2025-01-01T00:00:00"
  },
  "meta": {
    "locale": "en"
  },
  "success": true
}
```

##### ⚠️ 400 `Validation failed`

*default*

```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "details": [],
    "message": "Validation failed"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ⚠️ 404 `Resource not found`

*default*

```json
{
  "error": {
    "code": "NOT_FOUND",
    "details": [],
    "message": "Resource not found"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ⚠️ 429 `Too many requests`

*default*

```json
{
  "error": {
    "code": "RATE_LIMITED",
    "details": [],
    "message": "Too many requests"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ❌ 500 `Internal server error`

*default*

```json
{
  "error": {
    "code": "INTERNAL_ERROR",
    "details": [],
    "message": "Internal server error"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

---

### 🔴 `/products/{id}` 

**Delete product** — `DELETE`

**Tags:** `products`

#### Path Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `id` | `string` | ✅ | — |

#### Example cURL

```bash
curl -X DELETE 'http://localhost:3000/v1/products/id-example'
```

#### Responses

##### ✅ 200 `Delete product - success`

*default*

```json
{
  "message": "Deleted successfully",
  "meta": {
    "locale": "en"
  },
  "success": true
}
```

##### ⚠️ 404 `Resource not found`

*default*

```json
{
  "error": {
    "code": "NOT_FOUND",
    "details": [],
    "message": "Resource not found"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ⚠️ 429 `Too many requests`

*default*

```json
{
  "error": {
    "code": "RATE_LIMITED",
    "details": [],
    "message": "Too many requests"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ❌ 500 `Internal server error`

*default*

```json
{
  "error": {
    "code": "INTERNAL_ERROR",
    "details": [],
    "message": "Internal server error"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

---

### 🔵 `/products/variant/{id}` 

**Add product variant** — `POST`

**Tags:** `products`

#### Path Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `id` | `string` | ✅ | — |

#### Request Body

**Content-Type:** `json`
**Actual Content-Type:** `application/json`
**Required:** ✅

**Schema:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | `string` | ✅ | — |
| `price` | `integer` | ✅ | — |
| `stock` | `integer` | ✅ | — |
| `image` | `string` | ❌ | — |

#### Example cURL

```bash
curl -X POST 'http://localhost:3000/v1/products/variant/id-example' \
  -d '{}'
```

#### Responses

##### ✅ 201 `Add product variant - created`

*default*

```json
{
  "data": {
    "id": "770e8400-e29b-41d4-a716-446655440002",
    "image": "https://example.com/variant.jpg",
    "name": "Size L - Black",
    "price": 160000,
    "product_id": "550e8400-e29b-41d4-a716-446655440000",
    "stock": 30
  },
  "meta": {
    "locale": "en"
  },
  "success": true
}
```

##### ⚠️ 400 `Validation failed`

*default*

```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "details": [],
    "message": "Validation failed"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ⚠️ 429 `Too many requests`

*default*

```json
{
  "error": {
    "code": "RATE_LIMITED",
    "details": [],
    "message": "Too many requests"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ❌ 500 `Internal server error`

*default*

```json
{
  "error": {
    "code": "INTERNAL_ERROR",
    "details": [],
    "message": "Internal server error"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

---

### 🔵 `/products/create` 

**Create product** — `POST`

**Tags:** `products`

#### Request Body

**Content-Type:** `json`
**Actual Content-Type:** `application/json`
**Required:** ✅

**Schema:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `recommended` | `boolean` | ❌ | — |
| `images` | `object` | ❌ | — |
| `thumbnail` | `string` | ❌ | — |
| `coin` | `integer` | ❌ | — |
| `category` | `string` | ✅ | — |
| `discount` | `integer` | ❌ | — |
| `name` | `string` | ✅ | — |
| `stock` | `integer` | ✅ | — |
| `saller_id` | `string` | ❌ | — |
| `description` | `string` | ✅ | — |
| `price` | `integer` | ✅ | — |
| `location` | `string` | ❌ | — |

#### Example cURL

```bash
curl -X POST 'http://localhost:3000/v1/products/create' \
  -d '{}'
```

#### Responses

##### ✅ 201 `Create product - created`

*default*

```json
{
  "data": {
    "available": 50,
    "category": "clothing",
    "coin": 10,
    "created_at": "2025-01-01T00:00:00",
    "description": "Premium cotton t-shirt",
    "discount": 10.5,
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "images": [
      "https://example.com/img1.jpg"
    ],
    "location": "Jakarta",
    "name": "T-Shirt",
    "price": 150000,
    "rating": 4.5,
    "recommended": true,
    "saller_id": "660e8400-e29b-41d4-a716-446655440001",
    "sold": 50,
    "stock": 100,
    "thumbnail": "https://example.com/thumb.jpg",
    "updated_at": "2025-01-01T00:00:00"
  },
  "meta": {
    "locale": "en"
  },
  "success": true
}
```

##### ⚠️ 400 `Validation failed`

*default*

```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "details": [],
    "message": "Validation failed"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ⚠️ 429 `Too many requests`

*default*

```json
{
  "error": {
    "code": "RATE_LIMITED",
    "details": [],
    "message": "Too many requests"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ❌ 500 `Internal server error`

*default*

```json
{
  "error": {
    "code": "INTERNAL_ERROR",
    "details": [],
    "message": "Internal server error"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

---

## banners

Banner management

### 🟢 `/banners/type/{type}` 

**List banners by type** — `GET`

**Tags:** `banners`

#### Path Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `type` | `string` | ✅ | — |

#### Example cURL

```bash
curl -X GET 'http://localhost:3000/v1/banners/type/type-example'
```

#### Responses

##### ✅ 200 `List banners by type - paginated`

*default*

```json
{
  "data": [
    {
      "created_at": "2025-01-01T00:00:00",
      "description": "Up to 50% off",
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "image": "https://example.com/banner.jpg",
      "title": "Summer Sale",
      "type": "home",
      "updated_at": "2025-01-01T00:00:00"
    }
  ],
  "meta": {
    "locale": "en",
    "pagination": {
      "count": 25,
      "cursor": "eyJpZCI6MjV9",
      "has_more": true
    }
  },
  "success": true
}
```

##### ⚠️ 404 `Resource not found`

*default*

```json
{
  "error": {
    "code": "NOT_FOUND",
    "details": [],
    "message": "Resource not found"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ⚠️ 429 `Too many requests`

*default*

```json
{
  "error": {
    "code": "RATE_LIMITED",
    "details": [],
    "message": "Too many requests"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ❌ 500 `Internal server error`

*default*

```json
{
  "error": {
    "code": "INTERNAL_ERROR",
    "details": [],
    "message": "Internal server error"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

---

### 🔵 `/banners/create` 

**Create banner** — `POST`

**Tags:** `banners`

#### Request Body

**Content-Type:** `json`
**Actual Content-Type:** `application/json`
**Required:** ✅

**Schema:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `image` | `string` | ✅ | — |
| `type` | `string` | ❌ | — |

#### Example cURL

```bash
curl -X POST 'http://localhost:3000/v1/banners/create' \
  -d '{}'
```

#### Responses

##### ✅ 201 `Create banner - created`

*default*

```json
{
  "data": {
    "created_at": "2025-01-01T00:00:00",
    "description": "Up to 50% off",
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "image": "https://example.com/banner.jpg",
    "title": "Summer Sale",
    "type": "home",
    "updated_at": "2025-01-01T00:00:00"
  },
  "meta": {
    "locale": "en"
  },
  "success": true
}
```

##### ⚠️ 400 `Validation failed`

*default*

```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "details": [],
    "message": "Validation failed"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ⚠️ 429 `Too many requests`

*default*

```json
{
  "error": {
    "code": "RATE_LIMITED",
    "details": [],
    "message": "Too many requests"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ❌ 500 `Internal server error`

*default*

```json
{
  "error": {
    "code": "INTERNAL_ERROR",
    "details": [],
    "message": "Internal server error"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

---

### 🟢 `/banners` 

**List all banners (public)** — `GET`

**Tags:** `banners`

#### Query Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `cursor` | `string` | ❌ | — |
| `limit` | `integer` | ❌ | — |

#### Example cURL

```bash
curl -X GET 'http://localhost:3000/v1/banners?cursor=example-cursor&limit=1'
```

#### Responses

##### ✅ 200 `List all banners (public) - paginated`

*default*

```json
{
  "data": [
    {
      "created_at": "2025-01-01T00:00:00",
      "description": "Up to 50% off",
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "image": "https://example.com/banner.jpg",
      "title": "Summer Sale",
      "type": "home",
      "updated_at": "2025-01-01T00:00:00"
    }
  ],
  "meta": {
    "locale": "en",
    "pagination": {
      "count": 25,
      "cursor": "eyJpZCI6MjV9",
      "has_more": true
    }
  },
  "success": true
}
```

##### ⚠️ 404 `Resource not found`

*default*

```json
{
  "error": {
    "code": "NOT_FOUND",
    "details": [],
    "message": "Resource not found"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ⚠️ 429 `Too many requests`

*default*

```json
{
  "error": {
    "code": "RATE_LIMITED",
    "details": [],
    "message": "Too many requests"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ❌ 500 `Internal server error`

*default*

```json
{
  "error": {
    "code": "INTERNAL_ERROR",
    "details": [],
    "message": "Internal server error"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

---

## articles

Article content

### 🟢 `/articles/{id}` 

**Get article by ID** — `GET`

**Tags:** `articles`

#### Path Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `id` | `string` | ✅ | — |

#### Example cURL

```bash
curl -X GET 'http://localhost:3000/v1/articles/id-example'
```

#### Responses

##### ✅ 200 `Get article by ID - success`

*default*

```json
{
  "data": {
    "content": "Full article content here...",
    "created_at": "2025-01-01T00:00:00",
    "header": "How to Recycle",
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "thumbnail": "https://example.com/article.jpg",
    "updated_at": "2025-01-01T00:00:00"
  },
  "meta": {
    "locale": "en"
  },
  "success": true
}
```

##### ⚠️ 404 `Resource not found`

*default*

```json
{
  "error": {
    "code": "NOT_FOUND",
    "details": [],
    "message": "Resource not found"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ⚠️ 429 `Too many requests`

*default*

```json
{
  "error": {
    "code": "RATE_LIMITED",
    "details": [],
    "message": "Too many requests"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ❌ 500 `Internal server error`

*default*

```json
{
  "error": {
    "code": "INTERNAL_ERROR",
    "details": [],
    "message": "Internal server error"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

---

### 🟠 `/articles/{id}` 

**Update article** — `PUT`

**Tags:** `articles`

#### Path Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `id` | `string` | ✅ | — |

#### Example cURL

```bash
curl -X PUT 'http://localhost:3000/v1/articles/id-example'
```

#### Responses

##### ✅ 200 `Update article - success`

*default*

```json
{
  "data": {
    "content": "Full article content here...",
    "created_at": "2025-01-01T00:00:00",
    "header": "How to Recycle",
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "thumbnail": "https://example.com/article.jpg",
    "updated_at": "2025-01-01T00:00:00"
  },
  "meta": {
    "locale": "en"
  },
  "success": true
}
```

##### ⚠️ 400 `Validation failed`

*default*

```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "details": [],
    "message": "Validation failed"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ⚠️ 404 `Resource not found`

*default*

```json
{
  "error": {
    "code": "NOT_FOUND",
    "details": [],
    "message": "Resource not found"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ⚠️ 429 `Too many requests`

*default*

```json
{
  "error": {
    "code": "RATE_LIMITED",
    "details": [],
    "message": "Too many requests"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ❌ 500 `Internal server error`

*default*

```json
{
  "error": {
    "code": "INTERNAL_ERROR",
    "details": [],
    "message": "Internal server error"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

---

### 🔴 `/articles/{id}` 

**Delete article** — `DELETE`

**Tags:** `articles`

#### Path Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `id` | `string` | ✅ | — |

#### Example cURL

```bash
curl -X DELETE 'http://localhost:3000/v1/articles/id-example'
```

#### Responses

##### ✅ 200 `Delete article - success`

*default*

```json
{
  "message": "Deleted successfully",
  "meta": {
    "locale": "en"
  },
  "success": true
}
```

##### ⚠️ 404 `Resource not found`

*default*

```json
{
  "error": {
    "code": "NOT_FOUND",
    "details": [],
    "message": "Resource not found"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ⚠️ 429 `Too many requests`

*default*

```json
{
  "error": {
    "code": "RATE_LIMITED",
    "details": [],
    "message": "Too many requests"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ❌ 500 `Internal server error`

*default*

```json
{
  "error": {
    "code": "INTERNAL_ERROR",
    "details": [],
    "message": "Internal server error"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

---

### 🟢 `/articles` 

**List articles (public)** — `GET`

**Tags:** `articles`

#### Query Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `cursor` | `string` | ❌ | — |
| `limit` | `integer` | ❌ | — |

#### Example cURL

```bash
curl -X GET 'http://localhost:3000/v1/articles?cursor=example-cursor&limit=1'
```

#### Responses

##### ✅ 200 `List articles (public) - paginated`

*default*

```json
{
  "data": [
    {
      "content": "Full article content here...",
      "created_at": "2025-01-01T00:00:00",
      "header": "How to Recycle",
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "thumbnail": "https://example.com/article.jpg",
      "updated_at": "2025-01-01T00:00:00"
    }
  ],
  "meta": {
    "locale": "en",
    "pagination": {
      "count": 25,
      "cursor": "eyJpZCI6MjV9",
      "has_more": true
    }
  },
  "success": true
}
```

##### ⚠️ 404 `Resource not found`

*default*

```json
{
  "error": {
    "code": "NOT_FOUND",
    "details": [],
    "message": "Resource not found"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ⚠️ 429 `Too many requests`

*default*

```json
{
  "error": {
    "code": "RATE_LIMITED",
    "details": [],
    "message": "Too many requests"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ❌ 500 `Internal server error`

*default*

```json
{
  "error": {
    "code": "INTERNAL_ERROR",
    "details": [],
    "message": "Internal server error"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

---

### 🔵 `/articles/create` 

**Create article** — `POST`

**Tags:** `articles`

#### Request Body

**Content-Type:** `json`
**Actual Content-Type:** `application/json`
**Required:** ✅

**Schema:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `header` | `string` | ✅ | — |
| `content` | `string` | ✅ | — |
| `thumbnail` | `string` | ✅ | — |

#### Example cURL

```bash
curl -X POST 'http://localhost:3000/v1/articles/create' \
  -d '{}'
```

#### Responses

##### ✅ 201 `Create article - created`

*default*

```json
{
  "data": {
    "content": "Full article content here...",
    "created_at": "2025-01-01T00:00:00",
    "header": "How to Recycle",
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "thumbnail": "https://example.com/article.jpg",
    "updated_at": "2025-01-01T00:00:00"
  },
  "meta": {
    "locale": "en"
  },
  "success": true
}
```

##### ⚠️ 400 `Validation failed`

*default*

```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "details": [],
    "message": "Validation failed"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ⚠️ 429 `Too many requests`

*default*

```json
{
  "error": {
    "code": "RATE_LIMITED",
    "details": [],
    "message": "Too many requests"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ❌ 500 `Internal server error`

*default*

```json
{
  "error": {
    "code": "INTERNAL_ERROR",
    "details": [],
    "message": "Internal server error"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

---

## profile

User profile

### 🟢 `/profile/{email}` 

**Get user profile** — `GET`

**Tags:** `profile`

#### Path Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `email` | `string` | ✅ | — |

#### Example cURL

```bash
curl -X GET 'http://localhost:3000/v1/profile/email-example'
```

#### Responses

##### ✅ 200 `Get user profile - success`

*default*

```json
{
  "data": {
    "badge": "gold",
    "coin": 200,
    "exp": 150.5,
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "level": 5,
    "user_id": "660e8400-e29b-41d4-a716-446655440001"
  },
  "meta": {
    "locale": "en"
  },
  "success": true
}
```

##### ⚠️ 404 `Resource not found`

*default*

```json
{
  "error": {
    "code": "NOT_FOUND",
    "details": [],
    "message": "Resource not found"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ⚠️ 429 `Too many requests`

*default*

```json
{
  "error": {
    "code": "RATE_LIMITED",
    "details": [],
    "message": "Too many requests"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ❌ 500 `Internal server error`

*default*

```json
{
  "error": {
    "code": "INTERNAL_ERROR",
    "details": [],
    "message": "Internal server error"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

---

### 🟠 `/profile/{email}` 

**Update user profile** — `PUT`

**Tags:** `profile`

#### Path Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `email` | `string` | ✅ | — |

#### Request Body

**Content-Type:** `json`
**Actual Content-Type:** `application/json`
**Required:** ✅

**Schema:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `photo_url` | `string` | ❌ | — |
| `name` | `string` | ❌ | — |
| `role` | `string` | ❌ | — |
| `phone_number` | `string` | ❌ | — |

#### Example cURL

```bash
curl -X PUT 'http://localhost:3000/v1/profile/email-example' \
  -d '{}'
```

#### Responses

##### ✅ 200 `Update user profile - success`

*default*

```json
{
  "data": {
    "badge": "gold",
    "coin": 200,
    "exp": 150.5,
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "level": 5,
    "user_id": "660e8400-e29b-41d4-a716-446655440001"
  },
  "meta": {
    "locale": "en"
  },
  "success": true
}
```

##### ⚠️ 400 `Validation failed`

*default*

```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "details": [],
    "message": "Validation failed"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ⚠️ 404 `Resource not found`

*default*

```json
{
  "error": {
    "code": "NOT_FOUND",
    "details": [],
    "message": "Resource not found"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ⚠️ 429 `Too many requests`

*default*

```json
{
  "error": {
    "code": "RATE_LIMITED",
    "details": [],
    "message": "Too many requests"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ❌ 500 `Internal server error`

*default*

```json
{
  "error": {
    "code": "INTERNAL_ERROR",
    "details": [],
    "message": "Internal server error"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

---

## reviews

Product reviews

### 🟠 `/reviews/{id}` 

**Update review (auth required)** — `PUT`

**Tags:** `reviews`

#### Path Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `id` | `string` | ✅ | — |

#### Request Body

**Content-Type:** `json`
**Actual Content-Type:** `application/json`
**Required:** ✅

**Schema:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `comment` | `string` | ❌ | — |
| `rating` | `integer` | ❌ | — |

#### Example cURL

```bash
curl -X PUT 'http://localhost:3000/v1/reviews/id-example' \
  -d '{}'
```

#### Responses

##### ✅ 200 `Update review (auth required) - success`

*default*

```json
{
  "data": {
    "comment": "Great product!",
    "created_at": "2025-01-01T00:00:00",
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "images": [],
    "product_id": "660e8400-e29b-41d4-a716-446655440001",
    "rating": 5.0,
    "updated_at": "2025-01-01T00:00:00",
    "user_id": "770e8400-e29b-41d4-a716-446655440002"
  },
  "meta": {
    "locale": "en"
  },
  "success": true
}
```

##### ⚠️ 400 `Validation failed`

*default*

```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "details": [],
    "message": "Validation failed"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ⚠️ 404 `Resource not found`

*default*

```json
{
  "error": {
    "code": "NOT_FOUND",
    "details": [],
    "message": "Resource not found"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ⚠️ 429 `Too many requests`

*default*

```json
{
  "error": {
    "code": "RATE_LIMITED",
    "details": [],
    "message": "Too many requests"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ❌ 500 `Internal server error`

*default*

```json
{
  "error": {
    "code": "INTERNAL_ERROR",
    "details": [],
    "message": "Internal server error"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

---

### 🔵 `/reviews` 

**Create review (auth required)** — `POST`

**Tags:** `reviews`

#### Request Body

**Content-Type:** `json`
**Actual Content-Type:** `application/json`
**Required:** ✅

**Schema:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `rating` | `integer` | ✅ | — |
| `images` | `object` | ❌ | — |
| `comment` | `string` | ✅ | — |
| `product_id` | `string` | ✅ | — |

#### Example cURL

```bash
curl -X POST 'http://localhost:3000/v1/reviews' \
  -d '{}'
```

#### Responses

##### ✅ 201 `Create review (auth required) - created`

*default*

```json
{
  "data": {
    "comment": "Great product!",
    "created_at": "2025-01-01T00:00:00",
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "images": [],
    "product_id": "660e8400-e29b-41d4-a716-446655440001",
    "rating": 5.0,
    "updated_at": "2025-01-01T00:00:00",
    "user_id": "770e8400-e29b-41d4-a716-446655440002"
  },
  "meta": {
    "locale": "en"
  },
  "success": true
}
```

##### ⚠️ 400 `Validation failed`

*default*

```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "details": [],
    "message": "Validation failed"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ⚠️ 429 `Too many requests`

*default*

```json
{
  "error": {
    "code": "RATE_LIMITED",
    "details": [],
    "message": "Too many requests"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ❌ 500 `Internal server error`

*default*

```json
{
  "error": {
    "code": "INTERNAL_ERROR",
    "details": [],
    "message": "Internal server error"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

---

## carts

Shopping cart

### 🟢 `/carts/user/{user_id}` 

**Get user cart** — `GET`

**Tags:** `carts`

#### Path Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `user_id` | `string` | ✅ | — |

#### Query Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `cursor` | `string` | ❌ | — |
| `limit` | `integer` | ❌ | — |

#### Example cURL

```bash
curl -X GET 'http://localhost:3000/v1/carts/user/user_id-example?cursor=example-cursor&limit=1'
```

#### Responses

##### ✅ 200 `Get user cart - success`

*default*

```json
{
  "data": {
    "created_at": "2025-01-01T00:00:00",
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "product_id": "770e8400-e29b-41d4-a716-446655440002",
    "quantity": 2,
    "updated_at": "2025-01-01T00:00:00",
    "user_id": "660e8400-e29b-41d4-a716-446655440001",
    "variant_id": null
  },
  "meta": {
    "locale": "en"
  },
  "success": true
}
```

##### ⚠️ 404 `Resource not found`

*default*

```json
{
  "error": {
    "code": "NOT_FOUND",
    "details": [],
    "message": "Resource not found"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ⚠️ 429 `Too many requests`

*default*

```json
{
  "error": {
    "code": "RATE_LIMITED",
    "details": [],
    "message": "Too many requests"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ❌ 500 `Internal server error`

*default*

```json
{
  "error": {
    "code": "INTERNAL_ERROR",
    "details": [],
    "message": "Internal server error"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

---

### 🔵 `/carts/user/{user_id}` 

**Add item to cart** — `POST`

**Tags:** `carts`

#### Path Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `user_id` | `string` | ✅ | — |

#### Request Body

**Content-Type:** `json`
**Actual Content-Type:** `application/json`
**Required:** ✅

**Schema:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `product_id` | `string` | ✅ | — |
| `variant_id` | `string` | ❌ | — |
| `quantity` | `integer` | ✅ | — |

#### Example cURL

```bash
curl -X POST 'http://localhost:3000/v1/carts/user/user_id-example' \
  -d '{}'
```

#### Responses

##### ✅ 201 `Add item to cart - created`

*default*

```json
{
  "data": {
    "created_at": "2025-01-01T00:00:00",
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "product_id": "770e8400-e29b-41d4-a716-446655440002",
    "quantity": 2,
    "updated_at": "2025-01-01T00:00:00",
    "user_id": "660e8400-e29b-41d4-a716-446655440001",
    "variant_id": null
  },
  "meta": {
    "locale": "en"
  },
  "success": true
}
```

##### ⚠️ 400 `Validation failed`

*default*

```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "details": [],
    "message": "Validation failed"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ⚠️ 429 `Too many requests`

*default*

```json
{
  "error": {
    "code": "RATE_LIMITED",
    "details": [],
    "message": "Too many requests"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ❌ 500 `Internal server error`

*default*

```json
{
  "error": {
    "code": "INTERNAL_ERROR",
    "details": [],
    "message": "Internal server error"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

---

### 🔴 `/carts/item/{id}` 

**Remove cart item** — `DELETE`

**Tags:** `carts`

#### Path Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `id` | `string` | ✅ | — |

#### Example cURL

```bash
curl -X DELETE 'http://localhost:3000/v1/carts/item/id-example'
```

#### Responses

##### ✅ 200 `Remove cart item - success`

*default*

```json
{
  "message": "Deleted successfully",
  "meta": {
    "locale": "en"
  },
  "success": true
}
```

##### ⚠️ 404 `Resource not found`

*default*

```json
{
  "error": {
    "code": "NOT_FOUND",
    "details": [],
    "message": "Resource not found"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ⚠️ 429 `Too many requests`

*default*

```json
{
  "error": {
    "code": "RATE_LIMITED",
    "details": [],
    "message": "Too many requests"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ❌ 500 `Internal server error`

*default*

```json
{
  "error": {
    "code": "INTERNAL_ERROR",
    "details": [],
    "message": "Internal server error"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

---

## orders

Order management

### 🟢 `/orders` 

**List all orders (admin)** — `GET`

**Tags:** `orders`

#### Query Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `cursor` | `string` | ❌ | — |
| `limit` | `integer` | ❌ | — |

#### Example cURL

```bash
curl -X GET 'http://localhost:3000/v1/orders?cursor=example-cursor&limit=1'
```

#### Responses

##### ✅ 200 `List all orders (admin) - paginated`

*default*

```json
{
  "data": [
    {
      "coin": 10,
      "created_at": "2025-01-01T00:00:00",
      "delivery": {
        "estimated_delivery": "2025-01-05T00:00:00",
        "status": "processing",
        "tracking_number": ""
      },
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "payment": {
        "amount": 150000.0,
        "method": "qris",
        "status": "pending"
      },
      "product_id": "770e8400-e29b-41d4-a716-446655440002",
      "quantity": 1,
      "updated_at": "2025-01-01T00:00:00",
      "user_id": "660e8400-e29b-41d4-a716-446655440001"
    }
  ],
  "meta": {
    "locale": "en",
    "pagination": {
      "count": 25,
      "cursor": "eyJpZCI6MjV9",
      "has_more": true
    }
  },
  "success": true
}
```

##### ⚠️ 404 `Resource not found`

*default*

```json
{
  "error": {
    "code": "NOT_FOUND",
    "details": [],
    "message": "Resource not found"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ⚠️ 429 `Too many requests`

*default*

```json
{
  "error": {
    "code": "RATE_LIMITED",
    "details": [],
    "message": "Too many requests"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ❌ 500 `Internal server error`

*default*

```json
{
  "error": {
    "code": "INTERNAL_ERROR",
    "details": [],
    "message": "Internal server error"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

---

### 🟢 `/orders/user/{user_id}` 

**Get user orders** — `GET`

**Tags:** `orders`

#### Path Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `user_id` | `string` | ✅ | — |

#### Query Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `cursor` | `string` | ❌ | — |

#### Example cURL

```bash
curl -X GET 'http://localhost:3000/v1/orders/user/user_id-example?cursor=example-cursor'
```

#### Responses

##### ✅ 200 `Get user orders - success`

*default*

```json
{
  "data": {
    "coin": 10,
    "created_at": "2025-01-01T00:00:00",
    "delivery": {
      "estimated_delivery": "2025-01-05T00:00:00",
      "status": "processing",
      "tracking_number": ""
    },
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "payment": {
      "amount": 150000.0,
      "method": "qris",
      "status": "pending"
    },
    "product_id": "770e8400-e29b-41d4-a716-446655440002",
    "quantity": 1,
    "updated_at": "2025-01-01T00:00:00",
    "user_id": "660e8400-e29b-41d4-a716-446655440001"
  },
  "meta": {
    "locale": "en"
  },
  "success": true
}
```

##### ⚠️ 404 `Resource not found`

*default*

```json
{
  "error": {
    "code": "NOT_FOUND",
    "details": [],
    "message": "Resource not found"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ⚠️ 429 `Too many requests`

*default*

```json
{
  "error": {
    "code": "RATE_LIMITED",
    "details": [],
    "message": "Too many requests"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ❌ 500 `Internal server error`

*default*

```json
{
  "error": {
    "code": "INTERNAL_ERROR",
    "details": [],
    "message": "Internal server error"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

---

### 🔵 `/orders/user/{user_id}` 

**Create order (auth required)** — `POST`

**Tags:** `orders`

#### Path Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `user_id` | `string` | ✅ | — |

#### Request Body

**Content-Type:** `json`
**Actual Content-Type:** `application/json`
**Required:** ✅

**Schema:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `payment` | `object` | ✅ | — |
| `product_id` | `string` | ✅ | — |
| `coin` | `integer` | ❌ | — |
| `quantity` | `integer` | ✅ | — |

#### Example cURL

```bash
curl -X POST 'http://localhost:3000/v1/orders/user/user_id-example' \
  -d '{}'
```

#### Responses

##### ✅ 201 `Create order (auth required) - created`

*default*

```json
{
  "data": {
    "coin": 10,
    "created_at": "2025-01-01T00:00:00",
    "delivery": {
      "estimated_delivery": "2025-01-05T00:00:00",
      "status": "processing",
      "tracking_number": ""
    },
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "payment": {
      "amount": 150000.0,
      "method": "qris",
      "status": "pending"
    },
    "product_id": "770e8400-e29b-41d4-a716-446655440002",
    "quantity": 1,
    "updated_at": "2025-01-01T00:00:00",
    "user_id": "660e8400-e29b-41d4-a716-446655440001"
  },
  "meta": {
    "locale": "en"
  },
  "success": true
}
```

##### ⚠️ 400 `Validation failed`

*default*

```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "details": [],
    "message": "Validation failed"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ⚠️ 429 `Too many requests`

*default*

```json
{
  "error": {
    "code": "RATE_LIMITED",
    "details": [],
    "message": "Too many requests"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ❌ 500 `Internal server error`

*default*

```json
{
  "error": {
    "code": "INTERNAL_ERROR",
    "details": [],
    "message": "Internal server error"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

---

### 🔴 `/orders/{id}` 

**Cancel order (auth required)** — `DELETE`

**Tags:** `orders`

#### Path Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `id` | `string` | ✅ | — |

#### Example cURL

```bash
curl -X DELETE 'http://localhost:3000/v1/orders/id-example'
```

#### Responses

##### ✅ 200 `Cancel order (auth required) - success`

*default*

```json
{
  "message": "Deleted successfully",
  "meta": {
    "locale": "en"
  },
  "success": true
}
```

##### ⚠️ 404 `Resource not found`

*default*

```json
{
  "error": {
    "code": "NOT_FOUND",
    "details": [],
    "message": "Resource not found"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ⚠️ 429 `Too many requests`

*default*

```json
{
  "error": {
    "code": "RATE_LIMITED",
    "details": [],
    "message": "Too many requests"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ❌ 500 `Internal server error`

*default*

```json
{
  "error": {
    "code": "INTERNAL_ERROR",
    "details": [],
    "message": "Internal server error"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

---

## deposites

Waste deposite / pickup requests

### 🟢 `/deposites` 

**List all deposites (admin)** — `GET`

**Tags:** `deposites`

#### Query Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `cursor` | `string` | ❌ | — |
| `limit` | `integer` | ❌ | — |

#### Example cURL

```bash
curl -X GET 'http://localhost:3000/v1/deposites?cursor=example-cursor&limit=1'
```

#### Responses

##### ✅ 200 `List all deposites (admin) - paginated`

*default*

```json
{
  "data": [
    {
      "address_id": "770e8400-e29b-41d4-a716-446655440002",
      "coin": 50,
      "created_at": "2025-01-01T00:00:00",
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "images": [],
      "landfill_id": null,
      "pickup_date": "2025-01-15",
      "pickup_time": "10:00",
      "type": "plastic",
      "updated_at": "2025-01-01T00:00:00",
      "user_id": "660e8400-e29b-41d4-a716-446655440001"
    }
  ],
  "meta": {
    "locale": "en",
    "pagination": {
      "count": 25,
      "cursor": "eyJpZCI6MjV9",
      "has_more": true
    }
  },
  "success": true
}
```

##### ⚠️ 404 `Resource not found`

*default*

```json
{
  "error": {
    "code": "NOT_FOUND",
    "details": [],
    "message": "Resource not found"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ⚠️ 429 `Too many requests`

*default*

```json
{
  "error": {
    "code": "RATE_LIMITED",
    "details": [],
    "message": "Too many requests"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ❌ 500 `Internal server error`

*default*

```json
{
  "error": {
    "code": "INTERNAL_ERROR",
    "details": [],
    "message": "Internal server error"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

---

### 🔵 `/deposites` 

**Create deposite (auth required)** — `POST`

**Tags:** `deposites`

#### Request Body

**Content-Type:** `json`
**Actual Content-Type:** `application/json`
**Required:** ✅

**Schema:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `coin` | `integer` | ❌ | — |
| `type` | `string` | ✅ | — |
| `address_id` | `string` | ✅ | — |
| `pickup_date` | `string` | ✅ | — |
| `garbage_type` | `array` | ✅ | — |
| `pickup_time` | `string` | ✅ | — |
| `images` | `object` | ❌ | — |
| `landfill_id` | `string` | ❌ | — |

#### Example cURL

```bash
curl -X POST 'http://localhost:3000/v1/deposites' \
  -d '{}'
```

#### Responses

##### ✅ 201 `Create deposite (auth required) - created`

*default*

```json
{
  "data": {
    "address_id": "770e8400-e29b-41d4-a716-446655440002",
    "coin": 50,
    "created_at": "2025-01-01T00:00:00",
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "images": [],
    "landfill_id": null,
    "pickup_date": "2025-01-15",
    "pickup_time": "10:00",
    "type": "plastic",
    "updated_at": "2025-01-01T00:00:00",
    "user_id": "660e8400-e29b-41d4-a716-446655440001"
  },
  "meta": {
    "locale": "en"
  },
  "success": true
}
```

##### ⚠️ 400 `Validation failed`

*default*

```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "details": [],
    "message": "Validation failed"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ⚠️ 429 `Too many requests`

*default*

```json
{
  "error": {
    "code": "RATE_LIMITED",
    "details": [],
    "message": "Too many requests"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ❌ 500 `Internal server error`

*default*

```json
{
  "error": {
    "code": "INTERNAL_ERROR",
    "details": [],
    "message": "Internal server error"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

---

### 🟢 `/deposites/user/{id}` 

**Get user deposites** — `GET`

**Tags:** `deposites`

#### Path Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `id` | `string` | ✅ | — |

#### Query Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `cursor` | `string` | ❌ | — |

#### Example cURL

```bash
curl -X GET 'http://localhost:3000/v1/deposites/user/id-example?cursor=example-cursor'
```

#### Responses

##### ✅ 200 `Get user deposites - success`

*default*

```json
{
  "data": {
    "address_id": "770e8400-e29b-41d4-a716-446655440002",
    "coin": 50,
    "created_at": "2025-01-01T00:00:00",
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "images": [],
    "landfill_id": null,
    "pickup_date": "2025-01-15",
    "pickup_time": "10:00",
    "type": "plastic",
    "updated_at": "2025-01-01T00:00:00",
    "user_id": "660e8400-e29b-41d4-a716-446655440001"
  },
  "meta": {
    "locale": "en"
  },
  "success": true
}
```

##### ⚠️ 404 `Resource not found`

*default*

```json
{
  "error": {
    "code": "NOT_FOUND",
    "details": [],
    "message": "Resource not found"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ⚠️ 429 `Too many requests`

*default*

```json
{
  "error": {
    "code": "RATE_LIMITED",
    "details": [],
    "message": "Too many requests"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ❌ 500 `Internal server error`

*default*

```json
{
  "error": {
    "code": "INTERNAL_ERROR",
    "details": [],
    "message": "Internal server error"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

---

## landfills

Landfill locations

### 🟢 `/landfills` 

**List landfills (public)** — `GET`

**Tags:** `landfills`

#### Query Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `cursor` | `string` | ❌ | — |
| `limit` | `integer` | ❌ | — |

#### Example cURL

```bash
curl -X GET 'http://localhost:3000/v1/landfills?cursor=example-cursor&limit=1'
```

#### Responses

##### ✅ 200 `List landfills (public) - paginated`

*default*

```json
{
  "data": [
    {
      "address": "Jl. TB Simatupang",
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "name": "TPS Jakarta Selatan"
    }
  ],
  "meta": {
    "locale": "en",
    "pagination": {
      "count": 25,
      "cursor": "eyJpZCI6MjV9",
      "has_more": true
    }
  },
  "success": true
}
```

##### ⚠️ 404 `Resource not found`

*default*

```json
{
  "error": {
    "code": "NOT_FOUND",
    "details": [],
    "message": "Resource not found"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ⚠️ 429 `Too many requests`

*default*

```json
{
  "error": {
    "code": "RATE_LIMITED",
    "details": [],
    "message": "Too many requests"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ❌ 500 `Internal server error`

*default*

```json
{
  "error": {
    "code": "INTERNAL_ERROR",
    "details": [],
    "message": "Internal server error"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

---

### 🔵 `/landfills` 

**Create landfill (admin)** — `POST`

**Tags:** `landfills`

#### Request Body

**Content-Type:** `json`
**Actual Content-Type:** `application/json`
**Required:** ✅

**Schema:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | `string` | ✅ | — |
| `address` | `string` | ✅ | — |

#### Example cURL

```bash
curl -X POST 'http://localhost:3000/v1/landfills' \
  -d '{}'
```

#### Responses

##### ✅ 201 `Create landfill (admin) - created`

*default*

```json
{
  "data": {
    "address": "Jl. TB Simatupang",
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "name": "TPS Jakarta Selatan"
  },
  "meta": {
    "locale": "en"
  },
  "success": true
}
```

##### ⚠️ 400 `Validation failed`

*default*

```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "details": [],
    "message": "Validation failed"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ⚠️ 429 `Too many requests`

*default*

```json
{
  "error": {
    "code": "RATE_LIMITED",
    "details": [],
    "message": "Too many requests"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ❌ 500 `Internal server error`

*default*

```json
{
  "error": {
    "code": "INTERNAL_ERROR",
    "details": [],
    "message": "Internal server error"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

---

### 🟠 `/landfills/{id}` 

**Update landfill (admin)** — `PUT`

**Tags:** `landfills`

#### Path Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `id` | `string` | ✅ | — |

#### Example cURL

```bash
curl -X PUT 'http://localhost:3000/v1/landfills/id-example'
```

#### Responses

##### ✅ 200 `Update landfill (admin) - success`

*default*

```json
{
  "data": {
    "address": "Jl. TB Simatupang",
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "name": "TPS Jakarta Selatan"
  },
  "meta": {
    "locale": "en"
  },
  "success": true
}
```

##### ⚠️ 400 `Validation failed`

*default*

```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "details": [],
    "message": "Validation failed"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ⚠️ 404 `Resource not found`

*default*

```json
{
  "error": {
    "code": "NOT_FOUND",
    "details": [],
    "message": "Resource not found"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ⚠️ 429 `Too many requests`

*default*

```json
{
  "error": {
    "code": "RATE_LIMITED",
    "details": [],
    "message": "Too many requests"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ❌ 500 `Internal server error`

*default*

```json
{
  "error": {
    "code": "INTERNAL_ERROR",
    "details": [],
    "message": "Internal server error"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

---

### 🔴 `/landfills/{id}` 

**Delete landfill (admin)** — `DELETE`

**Tags:** `landfills`

#### Path Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `id` | `string` | ✅ | — |

#### Example cURL

```bash
curl -X DELETE 'http://localhost:3000/v1/landfills/id-example'
```

#### Responses

##### ✅ 200 `Delete landfill (admin) - success`

*default*

```json
{
  "message": "Deleted successfully",
  "meta": {
    "locale": "en"
  },
  "success": true
}
```

##### ⚠️ 404 `Resource not found`

*default*

```json
{
  "error": {
    "code": "NOT_FOUND",
    "details": [],
    "message": "Resource not found"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ⚠️ 429 `Too many requests`

*default*

```json
{
  "error": {
    "code": "RATE_LIMITED",
    "details": [],
    "message": "Too many requests"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ❌ 500 `Internal server error`

*default*

```json
{
  "error": {
    "code": "INTERNAL_ERROR",
    "details": [],
    "message": "Internal server error"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

---

## trash

Trash type management

### 🟠 `/trash/type/{id}` 

**Update trash type (admin)** — `PUT`

**Tags:** `trash`

#### Path Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `id` | `string` | ✅ | — |

#### Example cURL

```bash
curl -X PUT 'http://localhost:3000/v1/trash/type/id-example'
```

#### Responses

##### ✅ 200 `Update trash type (admin) - success`

*default*

```json
{
  "data": {
    "created_at": "2025-01-01T00:00:00",
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "image": "https://example.com/trash.jpg",
    "name": "Plastic",
    "updated_at": "2025-01-01T00:00:00"
  },
  "meta": {
    "locale": "en"
  },
  "success": true
}
```

##### ⚠️ 400 `Validation failed`

*default*

```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "details": [],
    "message": "Validation failed"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ⚠️ 404 `Resource not found`

*default*

```json
{
  "error": {
    "code": "NOT_FOUND",
    "details": [],
    "message": "Resource not found"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ⚠️ 429 `Too many requests`

*default*

```json
{
  "error": {
    "code": "RATE_LIMITED",
    "details": [],
    "message": "Too many requests"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ❌ 500 `Internal server error`

*default*

```json
{
  "error": {
    "code": "INTERNAL_ERROR",
    "details": [],
    "message": "Internal server error"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

---

### 🔴 `/trash/type/{id}` 

**Delete trash type (admin)** — `DELETE`

**Tags:** `trash`

#### Path Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `id` | `string` | ✅ | — |

#### Example cURL

```bash
curl -X DELETE 'http://localhost:3000/v1/trash/type/id-example'
```

#### Responses

##### ✅ 200 `Delete trash type (admin) - success`

*default*

```json
{
  "message": "Deleted successfully",
  "meta": {
    "locale": "en"
  },
  "success": true
}
```

##### ⚠️ 404 `Resource not found`

*default*

```json
{
  "error": {
    "code": "NOT_FOUND",
    "details": [],
    "message": "Resource not found"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ⚠️ 429 `Too many requests`

*default*

```json
{
  "error": {
    "code": "RATE_LIMITED",
    "details": [],
    "message": "Too many requests"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ❌ 500 `Internal server error`

*default*

```json
{
  "error": {
    "code": "INTERNAL_ERROR",
    "details": [],
    "message": "Internal server error"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

---

### 🟢 `/trash/types` 

**List trash types (public)** — `GET`

**Tags:** `trash`

#### Query Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `cursor` | `string` | ❌ | — |
| `limit` | `integer` | ❌ | — |

#### Example cURL

```bash
curl -X GET 'http://localhost:3000/v1/trash/types?cursor=example-cursor&limit=1'
```

#### Responses

##### ✅ 200 `List trash types (public) - paginated`

*default*

```json
{
  "data": [
    {
      "created_at": "2025-01-01T00:00:00",
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "image": "https://example.com/trash.jpg",
      "name": "Plastic",
      "updated_at": "2025-01-01T00:00:00"
    }
  ],
  "meta": {
    "locale": "en",
    "pagination": {
      "count": 25,
      "cursor": "eyJpZCI6MjV9",
      "has_more": true
    }
  },
  "success": true
}
```

##### ⚠️ 404 `Resource not found`

*default*

```json
{
  "error": {
    "code": "NOT_FOUND",
    "details": [],
    "message": "Resource not found"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ⚠️ 429 `Too many requests`

*default*

```json
{
  "error": {
    "code": "RATE_LIMITED",
    "details": [],
    "message": "Too many requests"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ❌ 500 `Internal server error`

*default*

```json
{
  "error": {
    "code": "INTERNAL_ERROR",
    "details": [],
    "message": "Internal server error"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

---

### 🔵 `/trash/types` 

**Create trash type (admin)** — `POST`

**Tags:** `trash`

#### Request Body

**Content-Type:** `json`
**Actual Content-Type:** `application/json`
**Required:** ✅

**Schema:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `image` | `string` | ❌ | — |
| `name` | `string` | ✅ | — |

#### Example cURL

```bash
curl -X POST 'http://localhost:3000/v1/trash/types' \
  -d '{}'
```

#### Responses

##### ✅ 201 `Create trash type (admin) - created`

*default*

```json
{
  "data": {
    "created_at": "2025-01-01T00:00:00",
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "image": "https://example.com/trash.jpg",
    "name": "Plastic",
    "updated_at": "2025-01-01T00:00:00"
  },
  "meta": {
    "locale": "en"
  },
  "success": true
}
```

##### ⚠️ 400 `Validation failed`

*default*

```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "details": [],
    "message": "Validation failed"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ⚠️ 429 `Too many requests`

*default*

```json
{
  "error": {
    "code": "RATE_LIMITED",
    "details": [],
    "message": "Too many requests"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ❌ 500 `Internal server error`

*default*

```json
{
  "error": {
    "code": "INTERNAL_ERROR",
    "details": [],
    "message": "Internal server error"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

---

## addresses

User addresses

### 🔵 `/addresses/user/{user_id}` 

**Create address (auth required)** — `POST`

**Tags:** `addresses`

#### Path Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `user_id` | `string` | ✅ | — |

#### Request Body

**Content-Type:** `json`
**Actual Content-Type:** `application/json`
**Required:** ✅

**Schema:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `phone_number` | `string` | ✅ | — |
| `main` | `boolean` | ❌ | — |
| `label` | `string` | ✅ | — |
| `address` | `string` | ✅ | — |

#### Example cURL

```bash
curl -X POST 'http://localhost:3000/v1/addresses/user/user_id-example' \
  -d '{}'
```

#### Responses

##### ✅ 201 `Create address (auth required) - created`

*default*

```json
{
  "data": {
    "address": "Jl. Sudirman No. 1",
    "created_at": "2025-01-01T00:00:00",
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "label": "Home",
    "main": true,
    "phone_number": "+628123456789",
    "updated_at": "2025-01-01T00:00:00",
    "user_id": "660e8400-e29b-41d4-a716-446655440001"
  },
  "meta": {
    "locale": "en"
  },
  "success": true
}
```

##### ⚠️ 400 `Validation failed`

*default*

```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "details": [],
    "message": "Validation failed"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ⚠️ 429 `Too many requests`

*default*

```json
{
  "error": {
    "code": "RATE_LIMITED",
    "details": [],
    "message": "Too many requests"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ❌ 500 `Internal server error`

*default*

```json
{
  "error": {
    "code": "INTERNAL_ERROR",
    "details": [],
    "message": "Internal server error"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

---

### 🟠 `/addresses/user/{user_id}` 

**Update address (auth required)** — `PUT`

**Tags:** `addresses`

#### Path Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `user_id` | `string` | ✅ | — |

#### Example cURL

```bash
curl -X PUT 'http://localhost:3000/v1/addresses/user/user_id-example'
```

#### Responses

##### ✅ 200 `Update address (auth required) - success`

*default*

```json
{
  "data": {
    "address": "Jl. Sudirman No. 1",
    "created_at": "2025-01-01T00:00:00",
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "label": "Home",
    "main": true,
    "phone_number": "+628123456789",
    "updated_at": "2025-01-01T00:00:00",
    "user_id": "660e8400-e29b-41d4-a716-446655440001"
  },
  "meta": {
    "locale": "en"
  },
  "success": true
}
```

##### ⚠️ 400 `Validation failed`

*default*

```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "details": [],
    "message": "Validation failed"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ⚠️ 404 `Resource not found`

*default*

```json
{
  "error": {
    "code": "NOT_FOUND",
    "details": [],
    "message": "Resource not found"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ⚠️ 429 `Too many requests`

*default*

```json
{
  "error": {
    "code": "RATE_LIMITED",
    "details": [],
    "message": "Too many requests"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ❌ 500 `Internal server error`

*default*

```json
{
  "error": {
    "code": "INTERNAL_ERROR",
    "details": [],
    "message": "Internal server error"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

---

## sallers

Seller product listing

### 🟢 `/sallers/products/{id}` 

**Get seller products** — `GET`

**Tags:** `sallers`

#### Path Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `id` | `string` | ✅ | — |

#### Query Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `cursor` | `string` | ❌ | — |
| `limit` | `integer` | ❌ | — |

#### Example cURL

```bash
curl -X GET 'http://localhost:3000/v1/sallers/products/id-example?cursor=example-cursor&limit=1'
```

#### Responses

##### ✅ 200 `Get seller products - success`

*default*

```json
{
  "data": {
    "available": 50,
    "category": "clothing",
    "coin": 10,
    "created_at": "2025-01-01T00:00:00",
    "description": "Premium cotton t-shirt",
    "discount": 10.5,
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "images": [
      "https://example.com/img1.jpg"
    ],
    "location": "Jakarta",
    "name": "T-Shirt",
    "price": 150000,
    "rating": 4.5,
    "recommended": true,
    "saller_id": "660e8400-e29b-41d4-a716-446655440001",
    "sold": 50,
    "stock": 100,
    "thumbnail": "https://example.com/thumb.jpg",
    "updated_at": "2025-01-01T00:00:00"
  },
  "meta": {
    "locale": "en"
  },
  "success": true
}
```

##### ⚠️ 404 `Resource not found`

*default*

```json
{
  "error": {
    "code": "NOT_FOUND",
    "details": [],
    "message": "Resource not found"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ⚠️ 429 `Too many requests`

*default*

```json
{
  "error": {
    "code": "RATE_LIMITED",
    "details": [],
    "message": "Too many requests"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

##### ❌ 500 `Internal server error`

*default*

```json
{
  "error": {
    "code": "INTERNAL_ERROR",
    "details": [],
    "message": "Internal server error"
  },
  "meta": {
    "locale": "en"
  },
  "success": false
}
```

---

