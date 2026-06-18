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
| `password` | `string` | ✅ | — |
| `name` | `string` | ✅ | — |
| `email` | `string` | ✅ | — |

#### Example cURL

```bash
curl -X POST 'http://localhost:3000/v1/auth/register' \
  -d '{}'
```

#### Responses

##### ✅ 201 `Success — created`

*default*

```json
{
  "content": {
    "badge": "gold",
    "coin": 500,
    "email": "user@example.com",
    "exp": 1500.5,
    "id": "usr_1234567890",
    "level": 12,
    "name": "John Doe",
    "phoneNumber": "+6281234567890",
    "photoURL": "https://example.com/avatar.jpg",
    "role": "user"
  },
  "message": "Success",
  "meta": {
    "locale": "en"
  },
  "statusCode": 201
}
```

##### ⚠️ 401 `Unauthorized`

*default*

```json
{
  "error": "Token not found",
  "message": "Unauthorized",
  "statusCode": 401
}
```

##### ⚠️ 400 `Bad Request`

*default*

```json
{
  "error": "Invalid input",
  "message": "Bad Request",
  "statusCode": 400
}
```

##### ⚠️ 429 `Rate limit exceeded`

*default*

```json
{
  "error": "Rate limit exceeded",
  "message": "Too Many Requests",
  "statusCode": 429
}
```

##### ❌ 500 `Internal Server Error`

*default*

```json
{
  "error": "Internal error occurred",
  "message": "Internal Server Error",
  "statusCode": 500
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

##### ✅ 200 `Success`

*default*

```json
{
  "content": {
    "accessToken": "eyJhbGciOiJIUzI1NiIs...",
    "email": "user@example.com",
    "name": "John Doe",
    "phoneNumber": "+6281234567890",
    "photoURL": "https://example.com/avatar.jpg",
    "refreshToken": "eyJhbGciOiJIUzI1NiIs...",
    "role": "user"
  },
  "message": "Success",
  "meta": {
    "locale": "en"
  },
  "statusCode": 200
}
```

##### ⚠️ 401 `Unauthorized`

*default*

```json
{
  "error": "Token not found",
  "message": "Unauthorized",
  "statusCode": 401
}
```

##### ⚠️ 400 `Bad Request`

*default*

```json
{
  "error": "Invalid input",
  "message": "Bad Request",
  "statusCode": 400
}
```

##### ⚠️ 429 `Rate limit exceeded`

*default*

```json
{
  "error": "Rate limit exceeded",
  "message": "Too Many Requests",
  "statusCode": 429
}
```

##### ❌ 500 `Internal Server Error`

*default*

```json
{
  "error": "Internal error occurred",
  "message": "Internal Server Error",
  "statusCode": 500
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

##### ✅ 200 `Success`

*default*

```json
{
  "content": {
    "accessToken": "eyJhbGciOiJIUzI1NiIs...",
    "email": "user@example.com",
    "name": "John Doe",
    "phoneNumber": "+6281234567890",
    "photoURL": "https://example.com/avatar.jpg",
    "refreshToken": "eyJhbGciOiJIUzI1NiIs...",
    "role": "user"
  },
  "message": "Success",
  "meta": {
    "locale": "en"
  },
  "statusCode": 200
}
```

##### ⚠️ 401 `Unauthorized`

*default*

```json
{
  "error": "Token not found",
  "message": "Unauthorized",
  "statusCode": 401
}
```

##### ⚠️ 400 `Bad Request`

*default*

```json
{
  "error": "Invalid input",
  "message": "Bad Request",
  "statusCode": 400
}
```

##### ⚠️ 429 `Rate limit exceeded`

*default*

```json
{
  "error": "Rate limit exceeded",
  "message": "Too Many Requests",
  "statusCode": 429
}
```

##### ❌ 500 `Internal Server Error`

*default*

```json
{
  "error": "Internal error occurred",
  "message": "Internal Server Error",
  "statusCode": 500
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

##### ✅ 200 `Success — paginated list`

*default*

```json
{
  "content": [
    {
      "coin": 202,
      "discount": null,
      "id": "cmqhhc2z00064pd2degy2ymub",
      "location": "Medan",
      "name": "Eksklusif Botol #30",
      "price": 202082,
      "sold": 418,
      "stock": 123,
      "thumbnail": "https://picsum.photos/seed/prod-30/600/600"
    },
    {
      "coin": 202,
      "discount": null,
      "id": "cmqhhc2z00064pd2degy2ymub",
      "location": "Medan",
      "name": "Eksklusif Botol #30",
      "price": 202082,
      "sold": 418,
      "stock": 123,
      "thumbnail": "https://picsum.photos/seed/prod-30/600/600"
    }
  ],
  "message": "Success",
  "meta": {
    "hasMore": true,
    "locale": "en",
    "nextCursor": "eyJpZCI6MjV9"
  },
  "statusCode": 200
}
```

##### ⚠️ 400 `Bad Request`

*default*

```json
{
  "error": "Invalid input",
  "message": "Bad Request",
  "statusCode": 400
}
```

##### ⚠️ 404 `Not Found`

*default*

```json
{
  "error": "Resource not found",
  "message": "Not Found",
  "statusCode": 404
}
```

##### ⚠️ 429 `Rate limit exceeded`

*default*

```json
{
  "error": "Rate limit exceeded",
  "message": "Too Many Requests",
  "statusCode": 429
}
```

##### ❌ 500 `Internal Server Error`

*default*

```json
{
  "error": "Internal error occurred",
  "message": "Internal Server Error",
  "statusCode": 500
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

##### ✅ 200 `Success — paginated list`

*default*

```json
{
  "content": [
    {
      "coin": 202,
      "discount": null,
      "id": "cmqhhc2z00064pd2degy2ymub",
      "location": "Medan",
      "name": "Eksklusif Botol #30",
      "price": 202082,
      "sold": 418,
      "stock": 123,
      "thumbnail": "https://picsum.photos/seed/prod-30/600/600"
    },
    {
      "coin": 202,
      "discount": null,
      "id": "cmqhhc2z00064pd2degy2ymub",
      "location": "Medan",
      "name": "Eksklusif Botol #30",
      "price": 202082,
      "sold": 418,
      "stock": 123,
      "thumbnail": "https://picsum.photos/seed/prod-30/600/600"
    }
  ],
  "message": "Success",
  "meta": {
    "hasMore": true,
    "locale": "en",
    "nextCursor": "eyJpZCI6MjV9"
  },
  "statusCode": 200
}
```

##### ⚠️ 400 `Bad Request`

*default*

```json
{
  "error": "Invalid input",
  "message": "Bad Request",
  "statusCode": 400
}
```

##### ⚠️ 404 `Not Found`

*default*

```json
{
  "error": "Resource not found",
  "message": "Not Found",
  "statusCode": 404
}
```

##### ⚠️ 429 `Rate limit exceeded`

*default*

```json
{
  "error": "Rate limit exceeded",
  "message": "Too Many Requests",
  "statusCode": 429
}
```

##### ❌ 500 `Internal Server Error`

*default*

```json
{
  "error": "Internal error occurred",
  "message": "Internal Server Error",
  "statusCode": 500
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

##### ✅ 200 `Success`

*default*

```json
{
  "content": {
    "coin": 202,
    "discount": null,
    "id": "cmqhhc2z00064pd2degy2ymub",
    "location": "Medan",
    "name": "Eksklusif Botol #30",
    "price": 202082,
    "sold": 418,
    "stock": 123,
    "thumbnail": "https://picsum.photos/seed/prod-30/600/600"
  },
  "message": "Success",
  "meta": {
    "locale": "en"
  },
  "statusCode": 200
}
```

##### ⚠️ 401 `Unauthorized`

*default*

```json
{
  "error": "Token not found",
  "message": "Unauthorized",
  "statusCode": 401
}
```

##### ⚠️ 400 `Bad Request`

*default*

```json
{
  "error": "Invalid input",
  "message": "Bad Request",
  "statusCode": 400
}
```

##### ⚠️ 404 `Not Found`

*default*

```json
{
  "error": "Resource not found",
  "message": "Not Found",
  "statusCode": 404
}
```

##### ⚠️ 429 `Rate limit exceeded`

*default*

```json
{
  "error": "Rate limit exceeded",
  "message": "Too Many Requests",
  "statusCode": 429
}
```

##### ❌ 500 `Internal Server Error`

*default*

```json
{
  "error": "Internal error occurred",
  "message": "Internal Server Error",
  "statusCode": 500
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

##### ✅ 200 `Success`

*default*

```json
{
  "content": {
    "coin": 202,
    "discount": null,
    "id": "cmqhhc2z00064pd2degy2ymub",
    "location": "Medan",
    "name": "Eksklusif Botol #30",
    "price": 202082,
    "sold": 418,
    "stock": 123,
    "thumbnail": "https://picsum.photos/seed/prod-30/600/600"
  },
  "message": "Success",
  "meta": {
    "locale": "en"
  },
  "statusCode": 200
}
```

##### ⚠️ 401 `Unauthorized`

*default*

```json
{
  "error": "Token not found",
  "message": "Unauthorized",
  "statusCode": 401
}
```

##### ⚠️ 400 `Bad Request`

*default*

```json
{
  "error": "Invalid input",
  "message": "Bad Request",
  "statusCode": 400
}
```

##### ⚠️ 404 `Not Found`

*default*

```json
{
  "error": "Resource not found",
  "message": "Not Found",
  "statusCode": 404
}
```

##### ⚠️ 429 `Rate limit exceeded`

*default*

```json
{
  "error": "Rate limit exceeded",
  "message": "Too Many Requests",
  "statusCode": 429
}
```

##### ❌ 500 `Internal Server Error`

*default*

```json
{
  "error": "Internal error occurred",
  "message": "Internal Server Error",
  "statusCode": 500
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
| `image` | `string` | ❌ | — |
| `price` | `integer` | ✅ | — |
| `stock` | `integer` | ✅ | — |
| `name` | `string` | ✅ | — |

#### Example cURL

```bash
curl -X POST 'http://localhost:3000/v1/products/variant/id-example' \
  -d '{}'
```

#### Responses

##### ✅ 201 `Success — created`

*default*

```json
{
  "content": {
    "coin": 202,
    "discount": null,
    "id": "cmqhhc2z00064pd2degy2ymub",
    "location": "Medan",
    "name": "Eksklusif Botol #30",
    "price": 202082,
    "sold": 418,
    "stock": 123,
    "thumbnail": "https://picsum.photos/seed/prod-30/600/600"
  },
  "message": "Success",
  "meta": {
    "locale": "en"
  },
  "statusCode": 201
}
```

##### ⚠️ 401 `Unauthorized`

*default*

```json
{
  "error": "Token not found",
  "message": "Unauthorized",
  "statusCode": 401
}
```

##### ⚠️ 400 `Bad Request`

*default*

```json
{
  "error": "Invalid input",
  "message": "Bad Request",
  "statusCode": 400
}
```

##### ⚠️ 429 `Rate limit exceeded`

*default*

```json
{
  "error": "Rate limit exceeded",
  "message": "Too Many Requests",
  "statusCode": 429
}
```

##### ❌ 500 `Internal Server Error`

*default*

```json
{
  "error": "Internal error occurred",
  "message": "Internal Server Error",
  "statusCode": 500
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
| `images` | `object` | ❌ | — |
| `coin` | `integer` | ❌ | — |
| `discount` | `integer` | ❌ | — |
| `name` | `string` | ✅ | — |
| `stock` | `integer` | ✅ | — |
| `recommended` | `boolean` | ❌ | — |
| `thumbnail` | `string` | ❌ | — |
| `category` | `string` | ✅ | — |
| `description` | `string` | ✅ | — |
| `price` | `integer` | ✅ | — |
| `saller_id` | `string` | ❌ | — |
| `location` | `string` | ❌ | — |

#### Example cURL

```bash
curl -X POST 'http://localhost:3000/v1/products/create' \
  -d '{}'
```

#### Responses

##### ✅ 201 `Success — created`

*default*

```json
{
  "content": {
    "coin": 202,
    "discount": null,
    "id": "cmqhhc2z00064pd2degy2ymub",
    "location": "Medan",
    "name": "Eksklusif Botol #30",
    "price": 202082,
    "sold": 418,
    "stock": 123,
    "thumbnail": "https://picsum.photos/seed/prod-30/600/600"
  },
  "message": "Success",
  "meta": {
    "locale": "en"
  },
  "statusCode": 201
}
```

##### ⚠️ 401 `Unauthorized`

*default*

```json
{
  "error": "Token not found",
  "message": "Unauthorized",
  "statusCode": 401
}
```

##### ⚠️ 400 `Bad Request`

*default*

```json
{
  "error": "Invalid input",
  "message": "Bad Request",
  "statusCode": 400
}
```

##### ⚠️ 429 `Rate limit exceeded`

*default*

```json
{
  "error": "Rate limit exceeded",
  "message": "Too Many Requests",
  "statusCode": 429
}
```

##### ❌ 500 `Internal Server Error`

*default*

```json
{
  "error": "Internal error occurred",
  "message": "Internal Server Error",
  "statusCode": 500
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

##### ✅ 200 `Success — paginated list`

*default*

```json
{
  "content": [
    {
      "description": "Get 50% off all items",
      "id": "cmqhhc2z00001pd2degy2ymub",
      "image": "https://picsum.photos/seed/banner-1/800/400",
      "title": "Summer Sale",
      "type": "promo"
    },
    {
      "description": "Get 50% off all items",
      "id": "cmqhhc2z00001pd2degy2ymub",
      "image": "https://picsum.photos/seed/banner-1/800/400",
      "title": "Summer Sale",
      "type": "promo"
    }
  ],
  "message": "Success",
  "meta": {
    "hasMore": true,
    "locale": "en",
    "nextCursor": "eyJpZCI6MjV9"
  },
  "statusCode": 200
}
```

##### ⚠️ 400 `Bad Request`

*default*

```json
{
  "error": "Invalid input",
  "message": "Bad Request",
  "statusCode": 400
}
```

##### ⚠️ 404 `Not Found`

*default*

```json
{
  "error": "Resource not found",
  "message": "Not Found",
  "statusCode": 404
}
```

##### ⚠️ 429 `Rate limit exceeded`

*default*

```json
{
  "error": "Rate limit exceeded",
  "message": "Too Many Requests",
  "statusCode": 429
}
```

##### ❌ 500 `Internal Server Error`

*default*

```json
{
  "error": "Internal error occurred",
  "message": "Internal Server Error",
  "statusCode": 500
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
| `type` | `string` | ❌ | — |
| `image` | `string` | ✅ | — |

#### Example cURL

```bash
curl -X POST 'http://localhost:3000/v1/banners/create' \
  -d '{}'
```

#### Responses

##### ✅ 201 `Success — created`

*default*

```json
{
  "content": {
    "description": "Get 50% off all items",
    "id": "cmqhhc2z00001pd2degy2ymub",
    "image": "https://picsum.photos/seed/banner-1/800/400",
    "title": "Summer Sale",
    "type": "promo"
  },
  "message": "Success",
  "meta": {
    "locale": "en"
  },
  "statusCode": 201
}
```

##### ⚠️ 401 `Unauthorized`

*default*

```json
{
  "error": "Token not found",
  "message": "Unauthorized",
  "statusCode": 401
}
```

##### ⚠️ 400 `Bad Request`

*default*

```json
{
  "error": "Invalid input",
  "message": "Bad Request",
  "statusCode": 400
}
```

##### ⚠️ 429 `Rate limit exceeded`

*default*

```json
{
  "error": "Rate limit exceeded",
  "message": "Too Many Requests",
  "statusCode": 429
}
```

##### ❌ 500 `Internal Server Error`

*default*

```json
{
  "error": "Internal error occurred",
  "message": "Internal Server Error",
  "statusCode": 500
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

##### ✅ 200 `Success — paginated list`

*default*

```json
{
  "content": [
    {
      "description": "Get 50% off all items",
      "id": "cmqhhc2z00001pd2degy2ymub",
      "image": "https://picsum.photos/seed/banner-1/800/400",
      "title": "Summer Sale",
      "type": "promo"
    },
    {
      "description": "Get 50% off all items",
      "id": "cmqhhc2z00001pd2degy2ymub",
      "image": "https://picsum.photos/seed/banner-1/800/400",
      "title": "Summer Sale",
      "type": "promo"
    }
  ],
  "message": "Success",
  "meta": {
    "hasMore": true,
    "locale": "en",
    "nextCursor": "eyJpZCI6MjV9"
  },
  "statusCode": 200
}
```

##### ⚠️ 400 `Bad Request`

*default*

```json
{
  "error": "Invalid input",
  "message": "Bad Request",
  "statusCode": 400
}
```

##### ⚠️ 404 `Not Found`

*default*

```json
{
  "error": "Resource not found",
  "message": "Not Found",
  "statusCode": 404
}
```

##### ⚠️ 429 `Rate limit exceeded`

*default*

```json
{
  "error": "Rate limit exceeded",
  "message": "Too Many Requests",
  "statusCode": 429
}
```

##### ❌ 500 `Internal Server Error`

*default*

```json
{
  "error": "Internal error occurred",
  "message": "Internal Server Error",
  "statusCode": 500
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

##### ✅ 200 `Success — paginated list`

*default*

```json
{
  "content": [
    {
      "content": "Learn how to recycle properly...",
      "header": "Tips Recycling",
      "id": "cmqhhc2z00002pd2degy2ymub",
      "thumbnail": "https://picsum.photos/seed/article-1/800/400"
    },
    {
      "content": "Learn how to recycle properly...",
      "header": "Tips Recycling",
      "id": "cmqhhc2z00002pd2degy2ymub",
      "thumbnail": "https://picsum.photos/seed/article-1/800/400"
    }
  ],
  "message": "Success",
  "meta": {
    "hasMore": true,
    "locale": "en",
    "nextCursor": "eyJpZCI6MjV9"
  },
  "statusCode": 200
}
```

##### ⚠️ 400 `Bad Request`

*default*

```json
{
  "error": "Invalid input",
  "message": "Bad Request",
  "statusCode": 400
}
```

##### ⚠️ 404 `Not Found`

*default*

```json
{
  "error": "Resource not found",
  "message": "Not Found",
  "statusCode": 404
}
```

##### ⚠️ 429 `Rate limit exceeded`

*default*

```json
{
  "error": "Rate limit exceeded",
  "message": "Too Many Requests",
  "statusCode": 429
}
```

##### ❌ 500 `Internal Server Error`

*default*

```json
{
  "error": "Internal error occurred",
  "message": "Internal Server Error",
  "statusCode": 500
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

##### ✅ 200 `Success`

*default*

```json
{
  "content": {
    "content": "Learn how to recycle properly...",
    "header": "Tips Recycling",
    "id": "cmqhhc2z00002pd2degy2ymub",
    "thumbnail": "https://picsum.photos/seed/article-1/800/400"
  },
  "message": "Success",
  "meta": {
    "locale": "en"
  },
  "statusCode": 200
}
```

##### ⚠️ 401 `Unauthorized`

*default*

```json
{
  "error": "Token not found",
  "message": "Unauthorized",
  "statusCode": 401
}
```

##### ⚠️ 400 `Bad Request`

*default*

```json
{
  "error": "Invalid input",
  "message": "Bad Request",
  "statusCode": 400
}
```

##### ⚠️ 404 `Not Found`

*default*

```json
{
  "error": "Resource not found",
  "message": "Not Found",
  "statusCode": 404
}
```

##### ⚠️ 429 `Rate limit exceeded`

*default*

```json
{
  "error": "Rate limit exceeded",
  "message": "Too Many Requests",
  "statusCode": 429
}
```

##### ❌ 500 `Internal Server Error`

*default*

```json
{
  "error": "Internal error occurred",
  "message": "Internal Server Error",
  "statusCode": 500
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

##### ✅ 200 `Success`

*default*

```json
{
  "content": {
    "content": "Learn how to recycle properly...",
    "header": "Tips Recycling",
    "id": "cmqhhc2z00002pd2degy2ymub",
    "thumbnail": "https://picsum.photos/seed/article-1/800/400"
  },
  "message": "Success",
  "meta": {
    "locale": "en"
  },
  "statusCode": 200
}
```

##### ⚠️ 401 `Unauthorized`

*default*

```json
{
  "error": "Token not found",
  "message": "Unauthorized",
  "statusCode": 401
}
```

##### ⚠️ 400 `Bad Request`

*default*

```json
{
  "error": "Invalid input",
  "message": "Bad Request",
  "statusCode": 400
}
```

##### ⚠️ 404 `Not Found`

*default*

```json
{
  "error": "Resource not found",
  "message": "Not Found",
  "statusCode": 404
}
```

##### ⚠️ 429 `Rate limit exceeded`

*default*

```json
{
  "error": "Rate limit exceeded",
  "message": "Too Many Requests",
  "statusCode": 429
}
```

##### ❌ 500 `Internal Server Error`

*default*

```json
{
  "error": "Internal error occurred",
  "message": "Internal Server Error",
  "statusCode": 500
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

##### ✅ 200 `Success — paginated list`

*default*

```json
{
  "content": [
    {
      "content": "Learn how to recycle properly...",
      "header": "Tips Recycling",
      "id": "cmqhhc2z00002pd2degy2ymub",
      "thumbnail": "https://picsum.photos/seed/article-1/800/400"
    },
    {
      "content": "Learn how to recycle properly...",
      "header": "Tips Recycling",
      "id": "cmqhhc2z00002pd2degy2ymub",
      "thumbnail": "https://picsum.photos/seed/article-1/800/400"
    }
  ],
  "message": "Success",
  "meta": {
    "hasMore": true,
    "locale": "en",
    "nextCursor": "eyJpZCI6MjV9"
  },
  "statusCode": 200
}
```

##### ⚠️ 400 `Bad Request`

*default*

```json
{
  "error": "Invalid input",
  "message": "Bad Request",
  "statusCode": 400
}
```

##### ⚠️ 404 `Not Found`

*default*

```json
{
  "error": "Resource not found",
  "message": "Not Found",
  "statusCode": 404
}
```

##### ⚠️ 429 `Rate limit exceeded`

*default*

```json
{
  "error": "Rate limit exceeded",
  "message": "Too Many Requests",
  "statusCode": 429
}
```

##### ❌ 500 `Internal Server Error`

*default*

```json
{
  "error": "Internal error occurred",
  "message": "Internal Server Error",
  "statusCode": 500
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

##### ✅ 201 `Success — created`

*default*

```json
{
  "content": {
    "content": "Learn how to recycle properly...",
    "header": "Tips Recycling",
    "id": "cmqhhc2z00002pd2degy2ymub",
    "thumbnail": "https://picsum.photos/seed/article-1/800/400"
  },
  "message": "Success",
  "meta": {
    "locale": "en"
  },
  "statusCode": 201
}
```

##### ⚠️ 401 `Unauthorized`

*default*

```json
{
  "error": "Token not found",
  "message": "Unauthorized",
  "statusCode": 401
}
```

##### ⚠️ 400 `Bad Request`

*default*

```json
{
  "error": "Invalid input",
  "message": "Bad Request",
  "statusCode": 400
}
```

##### ⚠️ 429 `Rate limit exceeded`

*default*

```json
{
  "error": "Rate limit exceeded",
  "message": "Too Many Requests",
  "statusCode": 429
}
```

##### ❌ 500 `Internal Server Error`

*default*

```json
{
  "error": "Internal error occurred",
  "message": "Internal Server Error",
  "statusCode": 500
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

##### ✅ 200 `Success — paginated list`

*default*

```json
{
  "content": [
    {
      "badge": "gold",
      "coin": 500,
      "email": "user@example.com",
      "exp": 1500.5,
      "id": "usr_1234567890",
      "level": 12,
      "name": "John Doe",
      "phoneNumber": "+6281234567890",
      "photoURL": "https://example.com/avatar.jpg",
      "role": "user"
    },
    {
      "badge": "gold",
      "coin": 500,
      "email": "user@example.com",
      "exp": 1500.5,
      "id": "usr_1234567890",
      "level": 12,
      "name": "John Doe",
      "phoneNumber": "+6281234567890",
      "photoURL": "https://example.com/avatar.jpg",
      "role": "user"
    }
  ],
  "message": "Success",
  "meta": {
    "hasMore": true,
    "locale": "en",
    "nextCursor": "eyJpZCI6MjV9"
  },
  "statusCode": 200
}
```

##### ⚠️ 400 `Bad Request`

*default*

```json
{
  "error": "Invalid input",
  "message": "Bad Request",
  "statusCode": 400
}
```

##### ⚠️ 404 `Not Found`

*default*

```json
{
  "error": "Resource not found",
  "message": "Not Found",
  "statusCode": 404
}
```

##### ⚠️ 429 `Rate limit exceeded`

*default*

```json
{
  "error": "Rate limit exceeded",
  "message": "Too Many Requests",
  "statusCode": 429
}
```

##### ❌ 500 `Internal Server Error`

*default*

```json
{
  "error": "Internal error occurred",
  "message": "Internal Server Error",
  "statusCode": 500
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
| `name` | `string` | ❌ | — |
| `role` | `string` | ❌ | — |
| `phone_number` | `string` | ❌ | — |
| `photo_url` | `string` | ❌ | — |

#### Example cURL

```bash
curl -X PUT 'http://localhost:3000/v1/profile/email-example' \
  -d '{}'
```

#### Responses

##### ✅ 200 `Success`

*default*

```json
{
  "content": {
    "badge": "gold",
    "coin": 500,
    "email": "user@example.com",
    "exp": 1500.5,
    "id": "usr_1234567890",
    "level": 12,
    "name": "John Doe",
    "phoneNumber": "+6281234567890",
    "photoURL": "https://example.com/avatar.jpg",
    "role": "user"
  },
  "message": "Success",
  "meta": {
    "locale": "en"
  },
  "statusCode": 200
}
```

##### ⚠️ 401 `Unauthorized`

*default*

```json
{
  "error": "Token not found",
  "message": "Unauthorized",
  "statusCode": 401
}
```

##### ⚠️ 400 `Bad Request`

*default*

```json
{
  "error": "Invalid input",
  "message": "Bad Request",
  "statusCode": 400
}
```

##### ⚠️ 404 `Not Found`

*default*

```json
{
  "error": "Resource not found",
  "message": "Not Found",
  "statusCode": 404
}
```

##### ⚠️ 429 `Rate limit exceeded`

*default*

```json
{
  "error": "Rate limit exceeded",
  "message": "Too Many Requests",
  "statusCode": 429
}
```

##### ❌ 500 `Internal Server Error`

*default*

```json
{
  "error": "Internal error occurred",
  "message": "Internal Server Error",
  "statusCode": 500
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

##### ✅ 200 `Success`

*default*

```json
{
  "content": {
    "comment": "Great product!",
    "id": "cmqhhc2z00007pd2degy2ymub",
    "images": [
      "https://example.com/review/img1.jpg"
    ],
    "productId": "cmqhhc2z00064pd2degy2ymub",
    "rating": 5,
    "userId": "usr_1234567890"
  },
  "message": "Success",
  "meta": {
    "locale": "en"
  },
  "statusCode": 200
}
```

##### ⚠️ 401 `Unauthorized`

*default*

```json
{
  "error": "Token not found",
  "message": "Unauthorized",
  "statusCode": 401
}
```

##### ⚠️ 400 `Bad Request`

*default*

```json
{
  "error": "Invalid input",
  "message": "Bad Request",
  "statusCode": 400
}
```

##### ⚠️ 404 `Not Found`

*default*

```json
{
  "error": "Resource not found",
  "message": "Not Found",
  "statusCode": 404
}
```

##### ⚠️ 429 `Rate limit exceeded`

*default*

```json
{
  "error": "Rate limit exceeded",
  "message": "Too Many Requests",
  "statusCode": 429
}
```

##### ❌ 500 `Internal Server Error`

*default*

```json
{
  "error": "Internal error occurred",
  "message": "Internal Server Error",
  "statusCode": 500
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
| `product_id` | `string` | ✅ | — |
| `rating` | `integer` | ✅ | — |
| `comment` | `string` | ✅ | — |
| `images` | `object` | ❌ | — |

#### Example cURL

```bash
curl -X POST 'http://localhost:3000/v1/reviews' \
  -d '{}'
```

#### Responses

##### ✅ 201 `Success — created`

*default*

```json
{
  "content": {
    "comment": "Great product!",
    "id": "cmqhhc2z00007pd2degy2ymub",
    "images": [
      "https://example.com/review/img1.jpg"
    ],
    "productId": "cmqhhc2z00064pd2degy2ymub",
    "rating": 5,
    "userId": "usr_1234567890"
  },
  "message": "Success",
  "meta": {
    "locale": "en"
  },
  "statusCode": 201
}
```

##### ⚠️ 401 `Unauthorized`

*default*

```json
{
  "error": "Token not found",
  "message": "Unauthorized",
  "statusCode": 401
}
```

##### ⚠️ 400 `Bad Request`

*default*

```json
{
  "error": "Invalid input",
  "message": "Bad Request",
  "statusCode": 400
}
```

##### ⚠️ 429 `Rate limit exceeded`

*default*

```json
{
  "error": "Rate limit exceeded",
  "message": "Too Many Requests",
  "statusCode": 429
}
```

##### ❌ 500 `Internal Server Error`

*default*

```json
{
  "error": "Internal error occurred",
  "message": "Internal Server Error",
  "statusCode": 500
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

##### ✅ 200 `Success — paginated list`

*default*

```json
{
  "content": [
    {
      "badge": "gold",
      "coin": 500,
      "email": "user@example.com",
      "exp": 1500.5,
      "id": "usr_1234567890",
      "level": 12,
      "name": "John Doe",
      "phoneNumber": "+6281234567890",
      "photoURL": "https://example.com/avatar.jpg",
      "role": "user"
    },
    {
      "badge": "gold",
      "coin": 500,
      "email": "user@example.com",
      "exp": 1500.5,
      "id": "usr_1234567890",
      "level": 12,
      "name": "John Doe",
      "phoneNumber": "+6281234567890",
      "photoURL": "https://example.com/avatar.jpg",
      "role": "user"
    }
  ],
  "message": "Success",
  "meta": {
    "hasMore": true,
    "locale": "en",
    "nextCursor": "eyJpZCI6MjV9"
  },
  "statusCode": 200
}
```

##### ⚠️ 401 `Unauthorized`

*default*

```json
{
  "error": "Token not found",
  "message": "Unauthorized",
  "statusCode": 401
}
```

##### ⚠️ 400 `Bad Request`

*default*

```json
{
  "error": "Invalid input",
  "message": "Bad Request",
  "statusCode": 400
}
```

##### ⚠️ 404 `Not Found`

*default*

```json
{
  "error": "Resource not found",
  "message": "Not Found",
  "statusCode": 404
}
```

##### ⚠️ 429 `Rate limit exceeded`

*default*

```json
{
  "error": "Rate limit exceeded",
  "message": "Too Many Requests",
  "statusCode": 429
}
```

##### ❌ 500 `Internal Server Error`

*default*

```json
{
  "error": "Internal error occurred",
  "message": "Internal Server Error",
  "statusCode": 500
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
| `variant_id` | `string` | ❌ | — |
| `product_id` | `string` | ✅ | — |
| `quantity` | `integer` | ✅ | — |

#### Example cURL

```bash
curl -X POST 'http://localhost:3000/v1/carts/user/user_id-example' \
  -d '{}'
```

#### Responses

##### ✅ 201 `Success — created`

*default*

```json
{
  "content": {
    "badge": "gold",
    "coin": 500,
    "email": "user@example.com",
    "exp": 1500.5,
    "id": "usr_1234567890",
    "level": 12,
    "name": "John Doe",
    "phoneNumber": "+6281234567890",
    "photoURL": "https://example.com/avatar.jpg",
    "role": "user"
  },
  "message": "Success",
  "meta": {
    "locale": "en"
  },
  "statusCode": 201
}
```

##### ⚠️ 401 `Unauthorized`

*default*

```json
{
  "error": "Token not found",
  "message": "Unauthorized",
  "statusCode": 401
}
```

##### ⚠️ 400 `Bad Request`

*default*

```json
{
  "error": "Invalid input",
  "message": "Bad Request",
  "statusCode": 400
}
```

##### ⚠️ 429 `Rate limit exceeded`

*default*

```json
{
  "error": "Rate limit exceeded",
  "message": "Too Many Requests",
  "statusCode": 429
}
```

##### ❌ 500 `Internal Server Error`

*default*

```json
{
  "error": "Internal error occurred",
  "message": "Internal Server Error",
  "statusCode": 500
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

##### ✅ 200 `Success`

*default*

```json
{
  "content": {
    "id": "cmqhhc2z00003pd2degy2ymub",
    "product": {
      "id": "cmqhhc2z00064pd2degy2ymub",
      "name": "Eksklusif Botol #30",
      "price": 202082,
      "thumbnail": "https://picsum.photos/seed/prod-30/600/600"
    },
    "productId": "cmqhhc2z00064pd2degy2ymub",
    "quantity": 2,
    "totalPrice": 404164,
    "userId": "usr_1234567890"
  },
  "message": "Success",
  "meta": {
    "locale": "en"
  },
  "statusCode": 200
}
```

##### ⚠️ 401 `Unauthorized`

*default*

```json
{
  "error": "Token not found",
  "message": "Unauthorized",
  "statusCode": 401
}
```

##### ⚠️ 400 `Bad Request`

*default*

```json
{
  "error": "Invalid input",
  "message": "Bad Request",
  "statusCode": 400
}
```

##### ⚠️ 404 `Not Found`

*default*

```json
{
  "error": "Resource not found",
  "message": "Not Found",
  "statusCode": 404
}
```

##### ⚠️ 429 `Rate limit exceeded`

*default*

```json
{
  "error": "Rate limit exceeded",
  "message": "Too Many Requests",
  "statusCode": 429
}
```

##### ❌ 500 `Internal Server Error`

*default*

```json
{
  "error": "Internal error occurred",
  "message": "Internal Server Error",
  "statusCode": 500
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

##### ✅ 200 `Success — paginated list`

*default*

```json
{
  "content": [
    {
      "coin": 202,
      "delivery": "pending",
      "id": "cmqhhc2z00004pd2degy2ymub",
      "payment": "qris",
      "productId": "cmqhhc2z00064pd2degy2ymub",
      "quantity": 1,
      "status": "pending",
      "totalPrice": 202082,
      "userId": "usr_1234567890"
    },
    {
      "coin": 202,
      "delivery": "pending",
      "id": "cmqhhc2z00004pd2degy2ymub",
      "payment": "qris",
      "productId": "cmqhhc2z00064pd2degy2ymub",
      "quantity": 1,
      "status": "pending",
      "totalPrice": 202082,
      "userId": "usr_1234567890"
    }
  ],
  "message": "Success",
  "meta": {
    "hasMore": true,
    "locale": "en",
    "nextCursor": "eyJpZCI6MjV9"
  },
  "statusCode": 200
}
```

##### ⚠️ 400 `Bad Request`

*default*

```json
{
  "error": "Invalid input",
  "message": "Bad Request",
  "statusCode": 400
}
```

##### ⚠️ 404 `Not Found`

*default*

```json
{
  "error": "Resource not found",
  "message": "Not Found",
  "statusCode": 404
}
```

##### ⚠️ 429 `Rate limit exceeded`

*default*

```json
{
  "error": "Rate limit exceeded",
  "message": "Too Many Requests",
  "statusCode": 429
}
```

##### ❌ 500 `Internal Server Error`

*default*

```json
{
  "error": "Internal error occurred",
  "message": "Internal Server Error",
  "statusCode": 500
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

##### ✅ 200 `Success — paginated list`

*default*

```json
{
  "content": [
    {
      "badge": "gold",
      "coin": 500,
      "email": "user@example.com",
      "exp": 1500.5,
      "id": "usr_1234567890",
      "level": 12,
      "name": "John Doe",
      "phoneNumber": "+6281234567890",
      "photoURL": "https://example.com/avatar.jpg",
      "role": "user"
    },
    {
      "badge": "gold",
      "coin": 500,
      "email": "user@example.com",
      "exp": 1500.5,
      "id": "usr_1234567890",
      "level": 12,
      "name": "John Doe",
      "phoneNumber": "+6281234567890",
      "photoURL": "https://example.com/avatar.jpg",
      "role": "user"
    }
  ],
  "message": "Success",
  "meta": {
    "hasMore": true,
    "locale": "en",
    "nextCursor": "eyJpZCI6MjV9"
  },
  "statusCode": 200
}
```

##### ⚠️ 401 `Unauthorized`

*default*

```json
{
  "error": "Token not found",
  "message": "Unauthorized",
  "statusCode": 401
}
```

##### ⚠️ 400 `Bad Request`

*default*

```json
{
  "error": "Invalid input",
  "message": "Bad Request",
  "statusCode": 400
}
```

##### ⚠️ 404 `Not Found`

*default*

```json
{
  "error": "Resource not found",
  "message": "Not Found",
  "statusCode": 404
}
```

##### ⚠️ 429 `Rate limit exceeded`

*default*

```json
{
  "error": "Rate limit exceeded",
  "message": "Too Many Requests",
  "statusCode": 429
}
```

##### ❌ 500 `Internal Server Error`

*default*

```json
{
  "error": "Internal error occurred",
  "message": "Internal Server Error",
  "statusCode": 500
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

##### ✅ 201 `Success — created`

*default*

```json
{
  "content": {
    "badge": "gold",
    "coin": 500,
    "email": "user@example.com",
    "exp": 1500.5,
    "id": "usr_1234567890",
    "level": 12,
    "name": "John Doe",
    "phoneNumber": "+6281234567890",
    "photoURL": "https://example.com/avatar.jpg",
    "role": "user"
  },
  "message": "Success",
  "meta": {
    "locale": "en"
  },
  "statusCode": 201
}
```

##### ⚠️ 401 `Unauthorized`

*default*

```json
{
  "error": "Token not found",
  "message": "Unauthorized",
  "statusCode": 401
}
```

##### ⚠️ 400 `Bad Request`

*default*

```json
{
  "error": "Invalid input",
  "message": "Bad Request",
  "statusCode": 400
}
```

##### ⚠️ 429 `Rate limit exceeded`

*default*

```json
{
  "error": "Rate limit exceeded",
  "message": "Too Many Requests",
  "statusCode": 429
}
```

##### ❌ 500 `Internal Server Error`

*default*

```json
{
  "error": "Internal error occurred",
  "message": "Internal Server Error",
  "statusCode": 500
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

##### ✅ 200 `Success`

*default*

```json
{
  "content": {
    "coin": 202,
    "delivery": "pending",
    "id": "cmqhhc2z00004pd2degy2ymub",
    "payment": "qris",
    "productId": "cmqhhc2z00064pd2degy2ymub",
    "quantity": 1,
    "status": "pending",
    "totalPrice": 202082,
    "userId": "usr_1234567890"
  },
  "message": "Success",
  "meta": {
    "locale": "en"
  },
  "statusCode": 200
}
```

##### ⚠️ 401 `Unauthorized`

*default*

```json
{
  "error": "Token not found",
  "message": "Unauthorized",
  "statusCode": 401
}
```

##### ⚠️ 400 `Bad Request`

*default*

```json
{
  "error": "Invalid input",
  "message": "Bad Request",
  "statusCode": 400
}
```

##### ⚠️ 404 `Not Found`

*default*

```json
{
  "error": "Resource not found",
  "message": "Not Found",
  "statusCode": 404
}
```

##### ⚠️ 429 `Rate limit exceeded`

*default*

```json
{
  "error": "Rate limit exceeded",
  "message": "Too Many Requests",
  "statusCode": 429
}
```

##### ❌ 500 `Internal Server Error`

*default*

```json
{
  "error": "Internal error occurred",
  "message": "Internal Server Error",
  "statusCode": 500
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

##### ✅ 200 `Success — paginated list`

*default*

```json
{
  "content": [
    {
      "coin": 150,
      "garbageType": [
        {
          "amount": 2,
          "trashTypeId": "trash_001"
        }
      ],
      "id": "cmqhhc2z00005pd2degy2ymub",
      "images": [
        "https://example.com/deposite/img1.jpg"
      ],
      "landfillId": "landfill_001",
      "pickupDate": "2025-06-20",
      "pickupTime": "10:00",
      "userId": "usr_1234567890"
    },
    {
      "coin": 150,
      "garbageType": [
        {
          "amount": 2,
          "trashTypeId": "trash_001"
        }
      ],
      "id": "cmqhhc2z00005pd2degy2ymub",
      "images": [
        "https://example.com/deposite/img1.jpg"
      ],
      "landfillId": "landfill_001",
      "pickupDate": "2025-06-20",
      "pickupTime": "10:00",
      "userId": "usr_1234567890"
    }
  ],
  "message": "Success",
  "meta": {
    "hasMore": true,
    "locale": "en",
    "nextCursor": "eyJpZCI6MjV9"
  },
  "statusCode": 200
}
```

##### ⚠️ 400 `Bad Request`

*default*

```json
{
  "error": "Invalid input",
  "message": "Bad Request",
  "statusCode": 400
}
```

##### ⚠️ 404 `Not Found`

*default*

```json
{
  "error": "Resource not found",
  "message": "Not Found",
  "statusCode": 404
}
```

##### ⚠️ 429 `Rate limit exceeded`

*default*

```json
{
  "error": "Rate limit exceeded",
  "message": "Too Many Requests",
  "statusCode": 429
}
```

##### ❌ 500 `Internal Server Error`

*default*

```json
{
  "error": "Internal error occurred",
  "message": "Internal Server Error",
  "statusCode": 500
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
| `landfill_id` | `string` | ❌ | — |
| `type` | `string` | ✅ | — |
| `coin` | `integer` | ❌ | — |
| `address_id` | `string` | ✅ | — |
| `pickup_date` | `string` | ✅ | — |
| `garbage_type` | `array` | ✅ | — |
| `images` | `object` | ❌ | — |
| `pickup_time` | `string` | ✅ | — |

#### Example cURL

```bash
curl -X POST 'http://localhost:3000/v1/deposites' \
  -d '{}'
```

#### Responses

##### ✅ 201 `Success — created`

*default*

```json
{
  "content": {
    "coin": 150,
    "garbageType": [
      {
        "amount": 2,
        "trashTypeId": "trash_001"
      }
    ],
    "id": "cmqhhc2z00005pd2degy2ymub",
    "images": [
      "https://example.com/deposite/img1.jpg"
    ],
    "landfillId": "landfill_001",
    "pickupDate": "2025-06-20",
    "pickupTime": "10:00",
    "userId": "usr_1234567890"
  },
  "message": "Success",
  "meta": {
    "locale": "en"
  },
  "statusCode": 201
}
```

##### ⚠️ 401 `Unauthorized`

*default*

```json
{
  "error": "Token not found",
  "message": "Unauthorized",
  "statusCode": 401
}
```

##### ⚠️ 400 `Bad Request`

*default*

```json
{
  "error": "Invalid input",
  "message": "Bad Request",
  "statusCode": 400
}
```

##### ⚠️ 429 `Rate limit exceeded`

*default*

```json
{
  "error": "Rate limit exceeded",
  "message": "Too Many Requests",
  "statusCode": 429
}
```

##### ❌ 500 `Internal Server Error`

*default*

```json
{
  "error": "Internal error occurred",
  "message": "Internal Server Error",
  "statusCode": 500
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

##### ✅ 200 `Success — paginated list`

*default*

```json
{
  "content": [
    {
      "badge": "gold",
      "coin": 500,
      "email": "user@example.com",
      "exp": 1500.5,
      "id": "usr_1234567890",
      "level": 12,
      "name": "John Doe",
      "phoneNumber": "+6281234567890",
      "photoURL": "https://example.com/avatar.jpg",
      "role": "user"
    },
    {
      "badge": "gold",
      "coin": 500,
      "email": "user@example.com",
      "exp": 1500.5,
      "id": "usr_1234567890",
      "level": 12,
      "name": "John Doe",
      "phoneNumber": "+6281234567890",
      "photoURL": "https://example.com/avatar.jpg",
      "role": "user"
    }
  ],
  "message": "Success",
  "meta": {
    "hasMore": true,
    "locale": "en",
    "nextCursor": "eyJpZCI6MjV9"
  },
  "statusCode": 200
}
```

##### ⚠️ 401 `Unauthorized`

*default*

```json
{
  "error": "Token not found",
  "message": "Unauthorized",
  "statusCode": 401
}
```

##### ⚠️ 400 `Bad Request`

*default*

```json
{
  "error": "Invalid input",
  "message": "Bad Request",
  "statusCode": 400
}
```

##### ⚠️ 404 `Not Found`

*default*

```json
{
  "error": "Resource not found",
  "message": "Not Found",
  "statusCode": 404
}
```

##### ⚠️ 429 `Rate limit exceeded`

*default*

```json
{
  "error": "Rate limit exceeded",
  "message": "Too Many Requests",
  "statusCode": 429
}
```

##### ❌ 500 `Internal Server Error`

*default*

```json
{
  "error": "Internal error occurred",
  "message": "Internal Server Error",
  "statusCode": 500
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

##### ✅ 200 `Success — paginated list`

*default*

```json
{
  "content": [
    {
      "capacity": 50000,
      "contact": "+6281234567890",
      "id": "landfill_001",
      "location": "Jakarta",
      "name": "TPA Bantar Gebang",
      "operationalHours": "08:00-17:00"
    },
    {
      "capacity": 50000,
      "contact": "+6281234567890",
      "id": "landfill_001",
      "location": "Jakarta",
      "name": "TPA Bantar Gebang",
      "operationalHours": "08:00-17:00"
    }
  ],
  "message": "Success",
  "meta": {
    "hasMore": true,
    "locale": "en",
    "nextCursor": "eyJpZCI6MjV9"
  },
  "statusCode": 200
}
```

##### ⚠️ 400 `Bad Request`

*default*

```json
{
  "error": "Invalid input",
  "message": "Bad Request",
  "statusCode": 400
}
```

##### ⚠️ 404 `Not Found`

*default*

```json
{
  "error": "Resource not found",
  "message": "Not Found",
  "statusCode": 404
}
```

##### ⚠️ 429 `Rate limit exceeded`

*default*

```json
{
  "error": "Rate limit exceeded",
  "message": "Too Many Requests",
  "statusCode": 429
}
```

##### ❌ 500 `Internal Server Error`

*default*

```json
{
  "error": "Internal error occurred",
  "message": "Internal Server Error",
  "statusCode": 500
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
| `address` | `string` | ✅ | — |
| `name` | `string` | ✅ | — |

#### Example cURL

```bash
curl -X POST 'http://localhost:3000/v1/landfills' \
  -d '{}'
```

#### Responses

##### ✅ 201 `Success — created`

*default*

```json
{
  "content": {
    "capacity": 50000,
    "contact": "+6281234567890",
    "id": "landfill_001",
    "location": "Jakarta",
    "name": "TPA Bantar Gebang",
    "operationalHours": "08:00-17:00"
  },
  "message": "Success",
  "meta": {
    "locale": "en"
  },
  "statusCode": 201
}
```

##### ⚠️ 401 `Unauthorized`

*default*

```json
{
  "error": "Token not found",
  "message": "Unauthorized",
  "statusCode": 401
}
```

##### ⚠️ 400 `Bad Request`

*default*

```json
{
  "error": "Invalid input",
  "message": "Bad Request",
  "statusCode": 400
}
```

##### ⚠️ 429 `Rate limit exceeded`

*default*

```json
{
  "error": "Rate limit exceeded",
  "message": "Too Many Requests",
  "statusCode": 429
}
```

##### ❌ 500 `Internal Server Error`

*default*

```json
{
  "error": "Internal error occurred",
  "message": "Internal Server Error",
  "statusCode": 500
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

##### ✅ 200 `Success`

*default*

```json
{
  "content": {
    "capacity": 50000,
    "contact": "+6281234567890",
    "id": "landfill_001",
    "location": "Jakarta",
    "name": "TPA Bantar Gebang",
    "operationalHours": "08:00-17:00"
  },
  "message": "Success",
  "meta": {
    "locale": "en"
  },
  "statusCode": 200
}
```

##### ⚠️ 401 `Unauthorized`

*default*

```json
{
  "error": "Token not found",
  "message": "Unauthorized",
  "statusCode": 401
}
```

##### ⚠️ 400 `Bad Request`

*default*

```json
{
  "error": "Invalid input",
  "message": "Bad Request",
  "statusCode": 400
}
```

##### ⚠️ 404 `Not Found`

*default*

```json
{
  "error": "Resource not found",
  "message": "Not Found",
  "statusCode": 404
}
```

##### ⚠️ 429 `Rate limit exceeded`

*default*

```json
{
  "error": "Rate limit exceeded",
  "message": "Too Many Requests",
  "statusCode": 429
}
```

##### ❌ 500 `Internal Server Error`

*default*

```json
{
  "error": "Internal error occurred",
  "message": "Internal Server Error",
  "statusCode": 500
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

##### ✅ 200 `Success`

*default*

```json
{
  "content": {
    "capacity": 50000,
    "contact": "+6281234567890",
    "id": "landfill_001",
    "location": "Jakarta",
    "name": "TPA Bantar Gebang",
    "operationalHours": "08:00-17:00"
  },
  "message": "Success",
  "meta": {
    "locale": "en"
  },
  "statusCode": 200
}
```

##### ⚠️ 401 `Unauthorized`

*default*

```json
{
  "error": "Token not found",
  "message": "Unauthorized",
  "statusCode": 401
}
```

##### ⚠️ 400 `Bad Request`

*default*

```json
{
  "error": "Invalid input",
  "message": "Bad Request",
  "statusCode": 400
}
```

##### ⚠️ 404 `Not Found`

*default*

```json
{
  "error": "Resource not found",
  "message": "Not Found",
  "statusCode": 404
}
```

##### ⚠️ 429 `Rate limit exceeded`

*default*

```json
{
  "error": "Rate limit exceeded",
  "message": "Too Many Requests",
  "statusCode": 429
}
```

##### ❌ 500 `Internal Server Error`

*default*

```json
{
  "error": "Internal error occurred",
  "message": "Internal Server Error",
  "statusCode": 500
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

##### ✅ 200 `Success`

*default*

```json
{
  "content": {
    "category": "plastic",
    "description": "Clean plastic bottles accepted",
    "id": "trash_001",
    "image": "https://example.com/trash/plastic.jpg",
    "name": "Plastic Bottle",
    "pricePerKg": 2500
  },
  "message": "Success",
  "meta": {
    "locale": "en"
  },
  "statusCode": 200
}
```

##### ⚠️ 401 `Unauthorized`

*default*

```json
{
  "error": "Token not found",
  "message": "Unauthorized",
  "statusCode": 401
}
```

##### ⚠️ 400 `Bad Request`

*default*

```json
{
  "error": "Invalid input",
  "message": "Bad Request",
  "statusCode": 400
}
```

##### ⚠️ 404 `Not Found`

*default*

```json
{
  "error": "Resource not found",
  "message": "Not Found",
  "statusCode": 404
}
```

##### ⚠️ 429 `Rate limit exceeded`

*default*

```json
{
  "error": "Rate limit exceeded",
  "message": "Too Many Requests",
  "statusCode": 429
}
```

##### ❌ 500 `Internal Server Error`

*default*

```json
{
  "error": "Internal error occurred",
  "message": "Internal Server Error",
  "statusCode": 500
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

##### ✅ 200 `Success`

*default*

```json
{
  "content": {
    "category": "plastic",
    "description": "Clean plastic bottles accepted",
    "id": "trash_001",
    "image": "https://example.com/trash/plastic.jpg",
    "name": "Plastic Bottle",
    "pricePerKg": 2500
  },
  "message": "Success",
  "meta": {
    "locale": "en"
  },
  "statusCode": 200
}
```

##### ⚠️ 401 `Unauthorized`

*default*

```json
{
  "error": "Token not found",
  "message": "Unauthorized",
  "statusCode": 401
}
```

##### ⚠️ 400 `Bad Request`

*default*

```json
{
  "error": "Invalid input",
  "message": "Bad Request",
  "statusCode": 400
}
```

##### ⚠️ 404 `Not Found`

*default*

```json
{
  "error": "Resource not found",
  "message": "Not Found",
  "statusCode": 404
}
```

##### ⚠️ 429 `Rate limit exceeded`

*default*

```json
{
  "error": "Rate limit exceeded",
  "message": "Too Many Requests",
  "statusCode": 429
}
```

##### ❌ 500 `Internal Server Error`

*default*

```json
{
  "error": "Internal error occurred",
  "message": "Internal Server Error",
  "statusCode": 500
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

##### ✅ 200 `Success — paginated list`

*default*

```json
{
  "content": [
    {
      "category": "plastic",
      "description": "Clean plastic bottles accepted",
      "id": "trash_001",
      "image": "https://example.com/trash/plastic.jpg",
      "name": "Plastic Bottle",
      "pricePerKg": 2500
    },
    {
      "category": "plastic",
      "description": "Clean plastic bottles accepted",
      "id": "trash_001",
      "image": "https://example.com/trash/plastic.jpg",
      "name": "Plastic Bottle",
      "pricePerKg": 2500
    }
  ],
  "message": "Success",
  "meta": {
    "hasMore": true,
    "locale": "en",
    "nextCursor": "eyJpZCI6MjV9"
  },
  "statusCode": 200
}
```

##### ⚠️ 400 `Bad Request`

*default*

```json
{
  "error": "Invalid input",
  "message": "Bad Request",
  "statusCode": 400
}
```

##### ⚠️ 404 `Not Found`

*default*

```json
{
  "error": "Resource not found",
  "message": "Not Found",
  "statusCode": 404
}
```

##### ⚠️ 429 `Rate limit exceeded`

*default*

```json
{
  "error": "Rate limit exceeded",
  "message": "Too Many Requests",
  "statusCode": 429
}
```

##### ❌ 500 `Internal Server Error`

*default*

```json
{
  "error": "Internal error occurred",
  "message": "Internal Server Error",
  "statusCode": 500
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

##### ✅ 201 `Success — created`

*default*

```json
{
  "content": {
    "category": "plastic",
    "description": "Clean plastic bottles accepted",
    "id": "trash_001",
    "image": "https://example.com/trash/plastic.jpg",
    "name": "Plastic Bottle",
    "pricePerKg": 2500
  },
  "message": "Success",
  "meta": {
    "locale": "en"
  },
  "statusCode": 201
}
```

##### ⚠️ 401 `Unauthorized`

*default*

```json
{
  "error": "Token not found",
  "message": "Unauthorized",
  "statusCode": 401
}
```

##### ⚠️ 400 `Bad Request`

*default*

```json
{
  "error": "Invalid input",
  "message": "Bad Request",
  "statusCode": 400
}
```

##### ⚠️ 429 `Rate limit exceeded`

*default*

```json
{
  "error": "Rate limit exceeded",
  "message": "Too Many Requests",
  "statusCode": 429
}
```

##### ❌ 500 `Internal Server Error`

*default*

```json
{
  "error": "Internal error occurred",
  "message": "Internal Server Error",
  "statusCode": 500
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
| `label` | `string` | ✅ | — |
| `main` | `boolean` | ❌ | — |
| `address` | `string` | ✅ | — |

#### Example cURL

```bash
curl -X POST 'http://localhost:3000/v1/addresses/user/user_id-example' \
  -d '{}'
```

#### Responses

##### ✅ 201 `Success — created`

*default*

```json
{
  "content": {
    "badge": "gold",
    "coin": 500,
    "email": "user@example.com",
    "exp": 1500.5,
    "id": "usr_1234567890",
    "level": 12,
    "name": "John Doe",
    "phoneNumber": "+6281234567890",
    "photoURL": "https://example.com/avatar.jpg",
    "role": "user"
  },
  "message": "Success",
  "meta": {
    "locale": "en"
  },
  "statusCode": 201
}
```

##### ⚠️ 401 `Unauthorized`

*default*

```json
{
  "error": "Token not found",
  "message": "Unauthorized",
  "statusCode": 401
}
```

##### ⚠️ 400 `Bad Request`

*default*

```json
{
  "error": "Invalid input",
  "message": "Bad Request",
  "statusCode": 400
}
```

##### ⚠️ 429 `Rate limit exceeded`

*default*

```json
{
  "error": "Rate limit exceeded",
  "message": "Too Many Requests",
  "statusCode": 429
}
```

##### ❌ 500 `Internal Server Error`

*default*

```json
{
  "error": "Internal error occurred",
  "message": "Internal Server Error",
  "statusCode": 500
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

##### ✅ 201 `Success — created`

*default*

```json
{
  "content": {
    "badge": "gold",
    "coin": 500,
    "email": "user@example.com",
    "exp": 1500.5,
    "id": "usr_1234567890",
    "level": 12,
    "name": "John Doe",
    "phoneNumber": "+6281234567890",
    "photoURL": "https://example.com/avatar.jpg",
    "role": "user"
  },
  "message": "Success",
  "meta": {
    "locale": "en"
  },
  "statusCode": 201
}
```

##### ⚠️ 401 `Unauthorized`

*default*

```json
{
  "error": "Token not found",
  "message": "Unauthorized",
  "statusCode": 401
}
```

##### ⚠️ 400 `Bad Request`

*default*

```json
{
  "error": "Invalid input",
  "message": "Bad Request",
  "statusCode": 400
}
```

##### ⚠️ 404 `Not Found`

*default*

```json
{
  "error": "Resource not found",
  "message": "Not Found",
  "statusCode": 404
}
```

##### ⚠️ 429 `Rate limit exceeded`

*default*

```json
{
  "error": "Rate limit exceeded",
  "message": "Too Many Requests",
  "statusCode": 429
}
```

##### ❌ 500 `Internal Server Error`

*default*

```json
{
  "error": "Internal error occurred",
  "message": "Internal Server Error",
  "statusCode": 500
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

##### ✅ 200 `Success — paginated list`

*default*

```json
{
  "content": [
    {
      "coin": 202,
      "discount": null,
      "id": "cmqhhc2z00064pd2degy2ymub",
      "location": "Medan",
      "name": "Eksklusif Botol #30",
      "price": 202082,
      "sold": 418,
      "stock": 123,
      "thumbnail": "https://picsum.photos/seed/prod-30/600/600"
    },
    {
      "coin": 202,
      "discount": null,
      "id": "cmqhhc2z00064pd2degy2ymub",
      "location": "Medan",
      "name": "Eksklusif Botol #30",
      "price": 202082,
      "sold": 418,
      "stock": 123,
      "thumbnail": "https://picsum.photos/seed/prod-30/600/600"
    }
  ],
  "message": "Success",
  "meta": {
    "hasMore": true,
    "locale": "en",
    "nextCursor": "eyJpZCI6MjV9"
  },
  "statusCode": 200
}
```

##### ⚠️ 400 `Bad Request`

*default*

```json
{
  "error": "Invalid input",
  "message": "Bad Request",
  "statusCode": 400
}
```

##### ⚠️ 404 `Not Found`

*default*

```json
{
  "error": "Resource not found",
  "message": "Not Found",
  "statusCode": 404
}
```

##### ⚠️ 429 `Rate limit exceeded`

*default*

```json
{
  "error": "Rate limit exceeded",
  "message": "Too Many Requests",
  "statusCode": 429
}
```

##### ❌ 500 `Internal Server Error`

*default*

```json
{
  "error": "Internal error occurred",
  "message": "Internal Server Error",
  "statusCode": 500
}
```

---

