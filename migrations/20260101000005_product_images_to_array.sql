-- =============================================================
--  Fix: Product.images stored as object, should be array
-- -------------------------------------------------------------
--  Seed script originally inserted images as a JSON object
--  ({img_X_1: "...", img_X_2: "..."}) but the Prisma schema
--  expects an array of URLs (["...", "...", "..."]).
--  Frontend iterates the array; object breaks the UI.
--
--  This migration normalizes any object-shaped images into an
--  array of values (order is JSON object key order, not stable
--  — but for our seed data with sequential keys img_X_1..N, the
--  result is preserved in insertion order).
-- =============================================================

UPDATE "Product"
SET images = COALESCE(
    (SELECT jsonb_agg(value)
       FROM jsonb_each(images)),
    '[]'::jsonb
)
WHERE jsonb_typeof(images) = 'object'
  AND images <> '{}'::jsonb;

-- Empty objects → empty array
UPDATE "Product"
SET images = '[]'::jsonb
WHERE jsonb_typeof(images) = 'object'
  AND images = '{}'::jsonb;