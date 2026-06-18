# Reybi API API Documentation

**Version:** `1.0.0`

Reybi API — Rust rewrite (Axum 0.8).
E-commerce platform with cart, orders, deposite, waste management.


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

##### ✅ 200 `JWT tokens + user data`

---

## products

Product catalog

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

##### ✅ 201 `Created variant`

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
| `stock` | `integer` | ✅ | — |
| `location` | `string` | ❌ | — |
| `images` | `object` | ❌ | — |
| `discount` | `integer` | ❌ | — |
| `coin` | `integer` | ❌ | — |
| `description` | `string` | ✅ | — |
| `category` | `string` | ✅ | — |
| `saller_id` | `string` | ❌ | — |
| `name` | `string` | ✅ | — |
| `thumbnail` | `string` | ❌ | — |
| `price` | `integer` | ✅ | — |

#### Example cURL

```bash
curl -X POST 'http://localhost:3000/v1/products/create' \
  -d '{}'
```

#### Responses

##### ✅ 201 `Created product`

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

##### ✅ 200 `Product data`

##### ⚠️ 404 `Product not found`

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

##### ✅ 200 `Updated product`

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

##### ✅ 200 `Product deleted`

---

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

##### ✅ 200 `Paginated products`

---

## banners

Banner management

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

---

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

---

### 🟢 `/banners` 

**List all banners (public)** — `GET`

**Tags:** `banners`

#### Query Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `limit` | `integer` | ❌ | — |

#### Example cURL

```bash
curl -X GET 'http://localhost:3000/v1/banners?limit=1'
```

#### Responses

##### ✅ 200 `Banners`

---

## articles

Article content

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
| `thumbnail` | `string` | ✅ | — |
| `header` | `string` | ✅ | — |

#### Example cURL

```bash
curl -X POST 'http://localhost:3000/v1/articles/create' \
  -d '{}'
```

---

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

---

### 🟢 `/articles` 

**List articles (public)** — `GET`

**Tags:** `articles`

#### Query Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `limit` | `integer` | ❌ | — |

#### Example cURL

```bash
curl -X GET 'http://localhost:3000/v1/articles?limit=1'
```

#### Responses

##### ✅ 200 `Articles`

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

---

## reviews

Product reviews

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
| `comment` | `string` | ✅ | — |
| `product_id` | `string` | ✅ | — |
| `images` | `object` | ❌ | — |
| `rating` | `integer` | ✅ | — |

#### Example cURL

```bash
curl -X POST 'http://localhost:3000/v1/reviews' \
  -d '{}'
```

---

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
| `limit` | `integer` | ❌ | — |

#### Example cURL

```bash
curl -X GET 'http://localhost:3000/v1/carts/user/user_id-example?limit=1'
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
| `quantity` | `integer` | ✅ | — |
| `product_id` | `string` | ✅ | — |

#### Example cURL

```bash
curl -X POST 'http://localhost:3000/v1/carts/user/user_id-example' \
  -d '{}'
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

---

## orders

Order management

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

---

### 🟢 `/orders` 

**List all orders (admin)** — `GET`

**Tags:** `orders`

#### Query Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `limit` | `integer` | ❌ | — |

#### Example cURL

```bash
curl -X GET 'http://localhost:3000/v1/orders?limit=1'
```

---

### 🟢 `/orders/user/{user_id}` 

**Get user orders** — `GET`

**Tags:** `orders`

#### Path Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `user_id` | `string` | ✅ | — |

#### Example cURL

```bash
curl -X GET 'http://localhost:3000/v1/orders/user/user_id-example'
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
| `product_id` | `string` | ✅ | — |
| `payment` | `object` | ✅ | — |
| `quantity` | `integer` | ✅ | — |
| `coin` | `integer` | ❌ | — |

#### Example cURL

```bash
curl -X POST 'http://localhost:3000/v1/orders/user/user_id-example' \
  -d '{}'
```

---

## deposites

Waste deposite / pickup requests

### 🟢 `/deposites/user/{id}` 

**Get user deposites** — `GET`

**Tags:** `deposites`

#### Path Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `id` | `string` | ✅ | — |

#### Example cURL

```bash
curl -X GET 'http://localhost:3000/v1/deposites/user/id-example'
```

---

### 🟢 `/deposites` 

**List all deposites (admin)** — `GET`

**Tags:** `deposites`

#### Query Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `limit` | `integer` | ❌ | — |

#### Example cURL

```bash
curl -X GET 'http://localhost:3000/v1/deposites?limit=1'
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
| `type` | `string` | ✅ | — |
| `pickup_time` | `string` | ✅ | — |
| `images` | `object` | ❌ | — |
| `garbage_type` | `array` | ✅ | — |
| `pickup_date` | `string` | ✅ | — |
| `coin` | `integer` | ❌ | — |
| `address_id` | `string` | ✅ | — |
| `landfill_id` | `string` | ❌ | — |

#### Example cURL

```bash
curl -X POST 'http://localhost:3000/v1/deposites' \
  -d '{}'
```

---

## landfills

Landfill locations

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

---

### 🟢 `/landfills` 

**List landfills (public)** — `GET`

**Tags:** `landfills`

#### Query Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `limit` | `integer` | ❌ | — |

#### Example cURL

```bash
curl -X GET 'http://localhost:3000/v1/landfills?limit=1'
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

---

### 🟢 `/trash/types` 

**List trash types (public)** — `GET`

**Tags:** `trash`

#### Query Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `limit` | `integer` | ❌ | — |

#### Example cURL

```bash
curl -X GET 'http://localhost:3000/v1/trash/types?limit=1'
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
| `limit` | `integer` | ❌ | — |

#### Example cURL

```bash
curl -X GET 'http://localhost:3000/v1/sallers/products/id-example?limit=1'
```

---

