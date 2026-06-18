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

##### ✅ 201 `Register new user - created`

##### ⚠️ 400 `Validation failed`

##### ⚠️ 401 `Authentication required`

##### ⚠️ 429 `Too many requests`

##### ❌ 500 `Internal server error`

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

##### ⚠️ 400 `Validation failed`

##### ⚠️ 401 `Authentication required`

##### ⚠️ 429 `Too many requests`

##### ❌ 500 `Internal server error`

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

##### ⚠️ 400 `Validation failed`

##### ⚠️ 401 `Authentication required`

##### ⚠️ 429 `Too many requests`

##### ❌ 500 `Internal server error`

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

##### ⚠️ 404 `Resource not found`

##### ⚠️ 429 `Too many requests`

##### ❌ 500 `Internal server error`

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

##### ⚠️ 404 `Resource not found`

##### ⚠️ 429 `Too many requests`

##### ❌ 500 `Internal server error`

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

##### ⚠️ 400 `Validation failed`

##### ⚠️ 404 `Resource not found`

##### ⚠️ 429 `Too many requests`

##### ❌ 500 `Internal server error`

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

##### ⚠️ 404 `Resource not found`

##### ⚠️ 429 `Too many requests`

##### ❌ 500 `Internal server error`

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
| `price` | `integer` | ✅ | — |
| `name` | `string` | ✅ | — |
| `image` | `string` | ❌ | — |
| `stock` | `integer` | ✅ | — |

#### Example cURL

```bash
curl -X POST 'http://localhost:3000/v1/products/variant/id-example' \
  -d '{}'
```

#### Responses

##### ✅ 201 `Add product variant - created`

##### ⚠️ 400 `Validation failed`

##### ⚠️ 429 `Too many requests`

##### ❌ 500 `Internal server error`

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
| `category` | `string` | ✅ | — |
| `saller_id` | `string` | ❌ | — |
| `recommended` | `boolean` | ❌ | — |
| `price` | `integer` | ✅ | — |
| `discount` | `integer` | ❌ | — |
| `stock` | `integer` | ✅ | — |
| `images` | `object` | ❌ | — |
| `location` | `string` | ❌ | — |
| `description` | `string` | ✅ | — |
| `thumbnail` | `string` | ❌ | — |
| `coin` | `integer` | ❌ | — |
| `name` | `string` | ✅ | — |

#### Example cURL

```bash
curl -X POST 'http://localhost:3000/v1/products/create' \
  -d '{}'
```

#### Responses

##### ✅ 201 `Create product - created`

##### ⚠️ 400 `Validation failed`

##### ⚠️ 429 `Too many requests`

##### ❌ 500 `Internal server error`

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

##### ⚠️ 404 `Resource not found`

##### ⚠️ 429 `Too many requests`

##### ❌ 500 `Internal server error`

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

##### ✅ 201 `Create banner - created`

##### ⚠️ 400 `Validation failed`

##### ⚠️ 429 `Too many requests`

##### ❌ 500 `Internal server error`

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

##### ⚠️ 404 `Resource not found`

##### ⚠️ 429 `Too many requests`

##### ❌ 500 `Internal server error`

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

##### ⚠️ 404 `Resource not found`

##### ⚠️ 429 `Too many requests`

##### ❌ 500 `Internal server error`

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

##### ⚠️ 400 `Validation failed`

##### ⚠️ 404 `Resource not found`

##### ⚠️ 429 `Too many requests`

##### ❌ 500 `Internal server error`

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

##### ⚠️ 404 `Resource not found`

##### ⚠️ 429 `Too many requests`

##### ❌ 500 `Internal server error`

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

##### ⚠️ 404 `Resource not found`

##### ⚠️ 429 `Too many requests`

##### ❌ 500 `Internal server error`

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
| `content` | `string` | ✅ | — |
| `header` | `string` | ✅ | — |
| `thumbnail` | `string` | ✅ | — |

#### Example cURL

```bash
curl -X POST 'http://localhost:3000/v1/articles/create' \
  -d '{}'
```

#### Responses

##### ✅ 201 `Create article - created`

##### ⚠️ 400 `Validation failed`

##### ⚠️ 429 `Too many requests`

##### ❌ 500 `Internal server error`

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

##### ⚠️ 404 `Resource not found`

##### ⚠️ 429 `Too many requests`

##### ❌ 500 `Internal server error`

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
| `role` | `string` | ❌ | — |
| `phone_number` | `string` | ❌ | — |
| `name` | `string` | ❌ | — |
| `photo_url` | `string` | ❌ | — |

#### Example cURL

```bash
curl -X PUT 'http://localhost:3000/v1/profile/email-example' \
  -d '{}'
```

#### Responses

##### ✅ 200 `Update user profile - success`

##### ⚠️ 400 `Validation failed`

##### ⚠️ 404 `Resource not found`

##### ⚠️ 429 `Too many requests`

##### ❌ 500 `Internal server error`

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
| `rating` | `integer` | ❌ | — |
| `comment` | `string` | ❌ | — |

#### Example cURL

```bash
curl -X PUT 'http://localhost:3000/v1/reviews/id-example' \
  -d '{}'
```

#### Responses

##### ✅ 200 `Update review (auth required) - success`

##### ⚠️ 400 `Validation failed`

##### ⚠️ 404 `Resource not found`

##### ⚠️ 429 `Too many requests`

##### ❌ 500 `Internal server error`

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
| `comment` | `string` | ✅ | — |
| `rating` | `integer` | ✅ | — |
| `images` | `object` | ❌ | — |

#### Example cURL

```bash
curl -X POST 'http://localhost:3000/v1/reviews' \
  -d '{}'
```

#### Responses

##### ✅ 201 `Create review (auth required) - created`

##### ⚠️ 400 `Validation failed`

##### ⚠️ 429 `Too many requests`

##### ❌ 500 `Internal server error`

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

##### ⚠️ 404 `Resource not found`

##### ⚠️ 429 `Too many requests`

##### ❌ 500 `Internal server error`

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

##### ✅ 201 `Add item to cart - created`

##### ⚠️ 400 `Validation failed`

##### ⚠️ 429 `Too many requests`

##### ❌ 500 `Internal server error`

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

##### ⚠️ 404 `Resource not found`

##### ⚠️ 429 `Too many requests`

##### ❌ 500 `Internal server error`

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

##### ⚠️ 404 `Resource not found`

##### ⚠️ 429 `Too many requests`

##### ❌ 500 `Internal server error`

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

##### ⚠️ 404 `Resource not found`

##### ⚠️ 429 `Too many requests`

##### ❌ 500 `Internal server error`

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
| `coin` | `integer` | ❌ | — |
| `quantity` | `integer` | ✅ | — |
| `product_id` | `string` | ✅ | — |

#### Example cURL

```bash
curl -X POST 'http://localhost:3000/v1/orders/user/user_id-example' \
  -d '{}'
```

#### Responses

##### ✅ 201 `Create order (auth required) - created`

##### ⚠️ 400 `Validation failed`

##### ⚠️ 429 `Too many requests`

##### ❌ 500 `Internal server error`

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

##### ⚠️ 404 `Resource not found`

##### ⚠️ 429 `Too many requests`

##### ❌ 500 `Internal server error`

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

##### ⚠️ 404 `Resource not found`

##### ⚠️ 429 `Too many requests`

##### ❌ 500 `Internal server error`

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
| `garbage_type` | `array` | ✅ | — |
| `pickup_time` | `string` | ✅ | — |
| `type` | `string` | ✅ | — |
| `pickup_date` | `string` | ✅ | — |
| `address_id` | `string` | ✅ | — |
| `images` | `object` | ❌ | — |
| `coin` | `integer` | ❌ | — |

#### Example cURL

```bash
curl -X POST 'http://localhost:3000/v1/deposites' \
  -d '{}'
```

#### Responses

##### ✅ 201 `Create deposite (auth required) - created`

##### ⚠️ 400 `Validation failed`

##### ⚠️ 429 `Too many requests`

##### ❌ 500 `Internal server error`

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

##### ⚠️ 404 `Resource not found`

##### ⚠️ 429 `Too many requests`

##### ❌ 500 `Internal server error`

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

##### ⚠️ 404 `Resource not found`

##### ⚠️ 429 `Too many requests`

##### ❌ 500 `Internal server error`

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

##### ⚠️ 400 `Validation failed`

##### ⚠️ 429 `Too many requests`

##### ❌ 500 `Internal server error`

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

##### ⚠️ 400 `Validation failed`

##### ⚠️ 404 `Resource not found`

##### ⚠️ 429 `Too many requests`

##### ❌ 500 `Internal server error`

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

##### ⚠️ 404 `Resource not found`

##### ⚠️ 429 `Too many requests`

##### ❌ 500 `Internal server error`

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

##### ⚠️ 400 `Validation failed`

##### ⚠️ 404 `Resource not found`

##### ⚠️ 429 `Too many requests`

##### ❌ 500 `Internal server error`

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

##### ⚠️ 404 `Resource not found`

##### ⚠️ 429 `Too many requests`

##### ❌ 500 `Internal server error`

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

##### ⚠️ 404 `Resource not found`

##### ⚠️ 429 `Too many requests`

##### ❌ 500 `Internal server error`

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

##### ⚠️ 400 `Validation failed`

##### ⚠️ 429 `Too many requests`

##### ❌ 500 `Internal server error`

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
| `address` | `string` | ✅ | — |
| `phone_number` | `string` | ✅ | — |
| `main` | `boolean` | ❌ | — |
| `label` | `string` | ✅ | — |

#### Example cURL

```bash
curl -X POST 'http://localhost:3000/v1/addresses/user/user_id-example' \
  -d '{}'
```

#### Responses

##### ✅ 201 `Create address (auth required) - created`

##### ⚠️ 400 `Validation failed`

##### ⚠️ 429 `Too many requests`

##### ❌ 500 `Internal server error`

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

##### ⚠️ 400 `Validation failed`

##### ⚠️ 404 `Resource not found`

##### ⚠️ 429 `Too many requests`

##### ❌ 500 `Internal server error`

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

##### ⚠️ 404 `Resource not found`

##### ⚠️ 429 `Too many requests`

##### ❌ 500 `Internal server error`

---

