-- ============================================================================
--  Reybi API – baseline schema (idempotent: safe to run against existing
--  Prisma-managed DB).  Mirrors prisma/schema.prisma in reybi-api-app so the
--  Rust binary can read/write the same rows.
-- ============================================================================

CREATE TABLE IF NOT EXISTS "User" (
  id           TEXT PRIMARY KEY,
  "fbId"       TEXT UNIQUE,
  email        TEXT UNIQUE,
  name         TEXT DEFAULT 'Eco User',
  role         TEXT DEFAULT 'user',
  "phoneNumber" TEXT,
  "photoURL"   TEXT DEFAULT 'https://res.cloudinary.com/dst7qcigz/image/upload/v1732372933/user/mvntmplanmz49w32hqq7.jpg',
  "createdAt"  TIMESTAMPTZ DEFAULT now(),
  "updatedAt"  TIMESTAMPTZ DEFAULT now()
);

CREATE TABLE IF NOT EXISTS "Token" (
  id           TEXT PRIMARY KEY,
  "refreshToken" TEXT UNIQUE,
  "userId"     TEXT UNIQUE REFERENCES "User"(id) ON DELETE CASCADE,
  "createdAt"  TIMESTAMPTZ DEFAULT now(),
  "updatedAt"  TIMESTAMPTZ DEFAULT now()
);

CREATE TABLE IF NOT EXISTS "UserDetail" (
  id     TEXT PRIMARY KEY,
  "userId" TEXT UNIQUE REFERENCES "User"(id) ON DELETE CASCADE,
  exp    DOUBLE PRECISION,
  level  INTEGER,
  coin   INTEGER,
  badge  TEXT
);

CREATE TABLE IF NOT EXISTS "Address" (
  id           TEXT PRIMARY KEY,
  "userId"     TEXT UNIQUE REFERENCES "User"(id) ON DELETE CASCADE,
  address      TEXT NOT NULL,
  label        TEXT NOT NULL,
  "phoneNumber" TEXT NOT NULL,
  main         BOOLEAN DEFAULT false,
  "createdAt"  TIMESTAMPTZ DEFAULT now(),
  "updatedAt"  TIMESTAMPTZ DEFAULT now()
);

CREATE TABLE IF NOT EXISTS "Saller" (
  id           TEXT PRIMARY KEY,
  name         TEXT NOT NULL,
  image        TEXT,
  "totalProduct" INTEGER NOT NULL,
  "productSold" INTEGER DEFAULT 0,
  address      TEXT NOT NULL,
  rating       DOUBLE PRECISION NOT NULL,
  "createdAt"  TIMESTAMPTZ DEFAULT now(),
  "updatedAt"  TIMESTAMPTZ DEFAULT now()
);

CREATE TABLE IF NOT EXISTS "Product" (
  id          TEXT PRIMARY KEY,
  name        TEXT NOT NULL,
  price       INTEGER NOT NULL,
  coin        INTEGER NOT NULL,
  description TEXT NOT NULL,
  thumbnail   TEXT,
  images      JSONB NOT NULL,
  stock       INTEGER NOT NULL,
  location    TEXT,
  category    TEXT NOT NULL,
  discount    DOUBLE PRECISION,
  sold        INTEGER,
  available   INTEGER,
  rating      DOUBLE PRECISION,
  "sallerId"  TEXT REFERENCES "Saller"(id) ON DELETE CASCADE,
  recommended BOOLEAN DEFAULT false,
  "createdAt" TIMESTAMPTZ DEFAULT now(),
  "updatedAt" TIMESTAMPTZ DEFAULT now()
);
CREATE INDEX IF NOT EXISTS "Product_category_idx" ON "Product"(category);
CREATE INDEX IF NOT EXISTS "Product_sallerId_idx" ON "Product"("sallerId");

CREATE TABLE IF NOT EXISTS "VariantProduct" (
  id        TEXT PRIMARY KEY,
  "productId" TEXT NOT NULL REFERENCES "Product"(id) ON DELETE CASCADE,
  name      TEXT NOT NULL,
  price     INTEGER NOT NULL,
  stock     INTEGER NOT NULL,
  image     TEXT
);
CREATE INDEX IF NOT EXISTS "VariantProduct_productId_idx" ON "VariantProduct"("productId");

CREATE TABLE IF NOT EXISTS "Cart" (
  id        TEXT PRIMARY KEY,
  "userId"  TEXT NOT NULL REFERENCES "User"(id) ON DELETE CASCADE,
  "productId" TEXT NOT NULL REFERENCES "Product"(id) ON DELETE CASCADE,
  "variantId" TEXT REFERENCES "VariantProduct"(id) ON DELETE CASCADE,
  quantity  INTEGER NOT NULL,
  "createdAt" TIMESTAMPTZ DEFAULT now(),
  "updatedAt" TIMESTAMPTZ DEFAULT now(),
  CONSTRAINT "Cart_userId_productId_key" UNIQUE ("userId", "productId")
);
CREATE INDEX IF NOT EXISTS "Cart_userId_idx" ON "Cart"("userId");

CREATE TABLE IF NOT EXISTS "reviewProduct" (
  id        TEXT PRIMARY KEY,
  "ProductId" TEXT NOT NULL REFERENCES "Product"(id) ON DELETE CASCADE,
  "userId"  TEXT NOT NULL REFERENCES "User"(id) ON DELETE CASCADE,
  comment   TEXT NOT NULL,
  images    JSONB,
  rating    DOUBLE PRECISION NOT NULL,
  "createdAt" TIMESTAMPTZ DEFAULT now(),
  "updatedAt" TIMESTAMPTZ DEFAULT now()
);
CREATE INDEX IF NOT EXISTS "reviewProduct_ProductId_idx" ON "reviewProduct"("ProductId");
CREATE INDEX IF NOT EXISTS "reviewProduct_userId_idx"   ON "reviewProduct"("userId");

CREATE TABLE IF NOT EXISTS "Banner" (
  id          TEXT PRIMARY KEY,
  image       TEXT NOT NULL,
  type        TEXT,
  title       TEXT,
  description TEXT,
  "createdAt" TIMESTAMPTZ DEFAULT now(),
  "updatedAt" TIMESTAMPTZ DEFAULT now()
);

CREATE TABLE IF NOT EXISTS "Article" (
  id        TEXT PRIMARY KEY,
  thumbnail TEXT NOT NULL,
  header    TEXT NOT NULL,
  content   TEXT NOT NULL,
  "createdAt" TIMESTAMPTZ DEFAULT now(),
  "updatedAt" TIMESTAMPTZ DEFAULT now()
);

CREATE TABLE IF NOT EXISTS "TrashType" (
  id        TEXT PRIMARY KEY,
  name      TEXT NOT NULL,
  image     TEXT NOT NULL,
  "createdAt" TIMESTAMPTZ DEFAULT now(),
  "updatedAt" TIMESTAMPTZ DEFAULT now()
);

CREATE TABLE IF NOT EXISTS "Landfills" (
  id      TEXT PRIMARY KEY,
  name    TEXT NOT NULL,
  address TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS "Deposite" (
  id        TEXT PRIMARY KEY,
  "userId"  TEXT NOT NULL REFERENCES "User"(id) ON DELETE CASCADE,
  "addressId" TEXT NOT NULL REFERENCES "Address"(id) ON DELETE CASCADE,
  type      TEXT NOT NULL,
  "pickupDate" TEXT NOT NULL,
  "pickupTime" TEXT NOT NULL,
  coin      INTEGER,
  images    JSONB NOT NULL,
  "landfillId" TEXT REFERENCES "Landfills"(id) ON DELETE SET NULL,
  "createdAt" TIMESTAMPTZ DEFAULT now(),
  "updatedAt" TIMESTAMPTZ DEFAULT now()
);
CREATE INDEX IF NOT EXISTS "Deposite_userId_idx"     ON "Deposite"("userId");
CREATE INDEX IF NOT EXISTS "Deposite_landfillId_idx" ON "Deposite"("landfillId");

CREATE TABLE IF NOT EXISTS "garbageDetails" (
  id           TEXT PRIMARY KEY,
  "trashTypeId" TEXT NOT NULL REFERENCES "TrashType"(id) ON DELETE CASCADE,
  "DepositeId" TEXT NOT NULL REFERENCES "Deposite"(id) ON DELETE CASCADE,
  amount       INTEGER NOT NULL,
  "createdAt"  TIMESTAMPTZ DEFAULT now(),
  "updatedAt"  TIMESTAMPTZ DEFAULT now()
);
CREATE INDEX IF NOT EXISTS "garbageDetails_DepositeId_idx"  ON "garbageDetails"("DepositeId");
CREATE INDEX IF NOT EXISTS "garbageDetails_trashTypeId_idx" ON "garbageDetails"("trashTypeId");

CREATE TABLE IF NOT EXISTS "DepositeStatus" (
  id        TEXT PRIMARY KEY,
  "depositId" TEXT NOT NULL REFERENCES "Deposite"(id) ON DELETE CASCADE,
  ongoing   BOOLEAN DEFAULT true,
  pickup    BOOLEAN DEFAULT false,
  landfill  BOOLEAN DEFAULT false,
  "createdAt" TIMESTAMPTZ DEFAULT now()
);
CREATE INDEX IF NOT EXISTS "DepositeStatus_depositId_idx" ON "DepositeStatus"("depositId");

CREATE TABLE IF NOT EXISTS "Order" (
  id        TEXT PRIMARY KEY,
  "userId"  TEXT NOT NULL REFERENCES "User"(id) ON DELETE CASCADE,
  "productId" TEXT NOT NULL REFERENCES "Product"(id) ON DELETE CASCADE,
  quantity  INTEGER NOT NULL,
  coin      INTEGER,
  "createdAt" TIMESTAMPTZ DEFAULT now(),
  "updatedAt" TIMESTAMPTZ DEFAULT now()
);
CREATE INDEX IF NOT EXISTS "Order_userId_idx"    ON "Order"("userId");
CREATE INDEX IF NOT EXISTS "Order_productId_idx" ON "Order"("productId");

CREATE TABLE IF NOT EXISTS "PaymentHistory" (
  id          TEXT PRIMARY KEY,
  "orderId"   TEXT UNIQUE NOT NULL REFERENCES "Order"(id) ON DELETE CASCADE,
  method      TEXT NOT NULL,
  type        TEXT,
  amount      DOUBLE PRECISION NOT NULL,
  "vaNumber"  JSONB,
  "linkQr"    JSONB,
  "midtransId" TEXT UNIQUE,
  status      TEXT DEFAULT 'pending',
  "createdAt" TIMESTAMPTZ DEFAULT now(),
  "updatedAt" TIMESTAMPTZ DEFAULT now()
);

CREATE TABLE IF NOT EXISTS "ProductDelivery" (
  id                TEXT PRIMARY KEY,
  "orderId"         TEXT UNIQUE NOT NULL REFERENCES "Order"(id) ON DELETE CASCADE,
  status            TEXT NOT NULL,
  "trackingNumber"  TEXT NOT NULL,
  history           JSONB NOT NULL,
  "estimatedDelivery" TIMESTAMPTZ NOT NULL,
  "createdAt"       TIMESTAMPTZ DEFAULT now(),
  "updatedAt"       TIMESTAMPTZ DEFAULT now()
);

-- ============================================================================
--  _sqlx_migrations table is created automatically by sqlx::migrate!() on
--  first run if absent.
-- ============================================================================