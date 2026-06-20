-- ============================================================================
-- 20260101000004_perf_indexes.sql
-- ----------------------------------------------------------------------------
--  Targeted B-tree + partial + GIN indexes for the actual query patterns in
--  the Rust services.  Base tables are PascalCase singular (Prisma convention);
--  queries hit the lowercase plural views which inherit the underlying indexes.
--
--  Design rules:
--    • Composite indexes ordered: equality columns first, then range/order-by.
--    • DESC on the cursor column to avoid an extra sort on pagination queries.
--    • Partial indexes for soft filters (e.g. `recommended = true`).
--    • Unique indexes where the column enforces identity (login, idempotency).
--    • GIN trgm on `name` for ILIKE search — far faster than seq scan.
-- ============================================================================

-- pg_trgm extension — required for GIN trigram indexes
CREATE EXTENSION IF NOT EXISTS pg_trgm;

-- 1. cursor pagination indexes (id DESC = newest first)
CREATE INDEX IF NOT EXISTS "Banner_id_desc_idx"
    ON "Banner" (id DESC);

CREATE INDEX IF NOT EXISTS "Article_id_desc_idx"
    ON "Article" (id DESC);

CREATE INDEX IF NOT EXISTS "TrashType_id_desc_idx"
    ON "TrashType" (id DESC);

CREATE INDEX IF NOT EXISTS "TrashType_name_asc_idx"
    ON "TrashType" (name ASC);

CREATE INDEX IF NOT EXISTS "Landfills_id_desc_idx"
    ON "Landfills" (id DESC);

CREATE INDEX IF NOT EXISTS "Landfills_name_asc_idx"
    ON "Landfills" (name ASC);

-- 2. composite cursor + filter (Product already has a category index — add (id DESC) for cursor)
-- Existing: "Product_category_idx" btree (category) — extend to composite for cursor
CREATE INDEX IF NOT EXISTS "Product_category_id_desc_idx"
    ON "Product" (category, id DESC);

-- Product saller products list
CREATE INDEX IF NOT EXISTS "Product_sallerId_created_idx"
    ON "Product" ("sallerId", "createdAt" DESC);

-- Banner filtered by type + cursor
CREATE INDEX IF NOT EXISTS "Banner_type_id_desc_idx"
    ON "Banner" (type, id DESC);

-- 3. user-scoped list indexes
-- Cart already has Cart_userId_idx + Cart_userId_productId_key (composite unique).
-- Add created_at for ORDER BY without sort.
CREATE INDEX IF NOT EXISTS "Cart_userId_created_idx"
    ON "Cart" ("userId", "createdAt" DESC);

-- Order already has Order_userId_idx + Order_productId_idx. Add created_at composite.
CREATE INDEX IF NOT EXISTS "Order_userId_created_idx"
    ON "Order" ("userId", "createdAt" DESC);

-- 4. auth + payment (unique lookups)
CREATE UNIQUE INDEX IF NOT EXISTS "User_email_unique"
    ON "User" (email);

CREATE UNIQUE INDEX IF NOT EXISTS "Token_refresh_unique"
    ON "Token" ("refreshToken");

-- 5. review lookups (lowercase table name, PascalCase columns from Prisma)
CREATE INDEX IF NOT EXISTS "reviewProduct_ProductId_idx"
    ON "reviewProduct" ("ProductId");

-- 6. product search (ILIKE %foo%) — GIN trigram
CREATE INDEX IF NOT EXISTS "Product_name_trgm_idx"
    ON "Product" USING GIN (name gin_trgm_ops);

-- 7. partial index for recommended products
CREATE INDEX IF NOT EXISTS "Product_recommended_idx"
    ON "Product" (id DESC)
    WHERE recommended = true;

-- 8. deposite + product delivery indexes (write-heavy paths)
CREATE INDEX IF NOT EXISTS "Deposite_userId_idx"
    ON "Deposite" ("userId");

CREATE INDEX IF NOT EXISTS "ProductDelivery_orderId_idx"
    ON "ProductDelivery" ("orderId");

-- ============================================================================
-- ANALYZE — refresh planner statistics after index build.
-- ============================================================================
ANALYZE "Banner";
ANALYZE "Article";
ANALYZE "Product";
ANALYZE "Cart";
ANALYZE "Order";
ANALYZE "Address";
ANALYZE "Token";
ANALYZE "User";
ANALYZE "PaymentHistory";
ANALYZE "reviewProduct";
ANALYZE "TrashType";
ANALYZE "Landfills";
ANALYZE "Deposite";
ANALYZE "ProductDelivery";
