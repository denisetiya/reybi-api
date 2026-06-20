-- Views with snake_case column aliases so Rust sqlx queries work.
--
-- The DB has PascalCase singular tables (Prisma style) and Prisma auto-generates
-- lowercase plural views. But the underlying columns are camelCase
-- (`createdAt`, `updatedAt`, `userId`, ...) while Rust service queries use
-- snake_case. We REPLACE the views with explicit column lists that alias
-- camelCase → snake_case so no service code change is required.

-- ============ banners (Banner) ============
CREATE OR REPLACE VIEW banners AS
SELECT id, image, type, title, description,
       "createdAt" AS created_at,
       "updatedAt" AS updated_at
FROM "Banner";

-- ============ articles (Article) ============
CREATE OR REPLACE VIEW articles AS
SELECT id, thumbnail, header, content,
       "createdAt" AS created_at,
       "updatedAt" AS updated_at
FROM "Article";

-- ============ products (Product) ============
CREATE OR REPLACE VIEW products AS
SELECT id, name, price, coin, description, thumbnail, images, stock,
       location, category, discount, sold, available, rating,
       "sallerId" AS saller_id,
       recommended,
       "createdAt" AS created_at,
       "updatedAt" AS updated_at
FROM "Product";

-- ============ product_variants (VariantProduct) ============
CREATE OR REPLACE VIEW product_variants AS
SELECT id,
       "productId" AS product_id,
       name, price, stock, image
FROM "VariantProduct";

-- ============ carts (Cart) ============
CREATE OR REPLACE VIEW carts AS
SELECT id,
       "userId" AS user_id,
       "productId" AS product_id,
       quantity,
       "variantId" AS variant_id,
       "createdAt" AS created_at,
       "updatedAt" AS updated_at
FROM "Cart";

-- ============ addresses (Address) ============
CREATE OR REPLACE VIEW addresses AS
SELECT id,
       "userId" AS user_id,
       address, label,
       "phoneNumber" AS phone_number,
       main,
       "createdAt" AS created_at,
       "updatedAt" AS updated_at
FROM "Address";

-- ============ landfills (Landfills) ============
CREATE OR REPLACE VIEW landfills AS
SELECT id, name, address FROM "Landfills";

-- ============ trash_types (TrashType) ============
CREATE OR REPLACE VIEW trash_types AS
SELECT id, name, image,
       "createdAt" AS created_at,
       "updatedAt" AS updated_at
FROM "TrashType";

-- ============ deposites (Deposite) ============
CREATE OR REPLACE VIEW deposites AS
SELECT id,
       "userId" AS user_id,
       "addressId" AS address_id,
       type,
       "pickupDate" AS pickup_date,
       "pickupTime" AS pickup_time,
       coin, images,
       "landfillId" AS landfill_id,
       "createdAt" AS created_at,
       "updatedAt" AS updated_at
FROM "Deposite";

-- ============ garbage_details (garbageDetails) ============
CREATE OR REPLACE VIEW garbage_details AS
SELECT id,
       "trashTypeId" AS trash_type_id,
       "DepositeId" AS deposite_id,
       amount,
       "createdAt" AS created_at,
       "updatedAt" AS updated_at
FROM "garbageDetails";

-- ============ deposite_statuses (DepositeStatus) ============
CREATE OR REPLACE VIEW deposite_statuses AS
SELECT id,
       "depositId" AS deposit_id,
       ongoing, pickup, landfill,
       "createdAt" AS created_at
FROM "DepositeStatus";

-- ============ users (User) ============
CREATE OR REPLACE VIEW users AS
SELECT id,
       "fbId" AS fb_id,
       email, name, role,
       "phoneNumber" AS phone_number,
       "photoURL" AS photo_url,
       "createdAt" AS created_at,
       "updatedAt" AS updated_at
FROM "User";

-- ============ sallers (Saller) ============
CREATE OR REPLACE VIEW sallers AS
SELECT id, name, image,
       "totalProduct" AS total_product,
       "productSold" AS product_sold,
       address, rating,
       "createdAt" AS created_at,
       "updatedAt" AS updated_at
FROM "Saller";

-- ============ user_details (UserDetail) ============
CREATE OR REPLACE VIEW user_details AS
SELECT id,
       "userId" AS user_id,
       exp, level, coin, badge
FROM "UserDetail";

-- ============ tokens (Token) ============
CREATE OR REPLACE VIEW tokens AS
SELECT id,
       "refreshToken" AS refresh_token,
       "userId" AS user_id,
       "createdAt" AS created_at,
       "updatedAt" AS updated_at
FROM "Token";

-- ============ orders (Order) ============
CREATE OR REPLACE VIEW orders AS
SELECT id,
       "userId" AS user_id,
       "productId" AS product_id,
       quantity, coin,
       "createdAt" AS created_at,
       "updatedAt" AS updated_at
FROM "Order";

-- ============ payment_history (PaymentHistory) ============
CREATE OR REPLACE VIEW payment_history AS
SELECT id,
       "orderId" AS order_id,
       method, type, amount,
       "vaNumber" AS va_number,
       "linkQr" AS link_qr,
       "midtransId" AS midtrans_id,
       status,
       "createdAt" AS created_at,
       "updatedAt" AS updated_at
FROM "PaymentHistory";

-- ============ product_deliveries (ProductDelivery) ============
CREATE OR REPLACE VIEW product_deliveries AS
SELECT id,
       "orderId" AS order_id,
       status,
       "trackingNumber" AS tracking_number,
       history,
       "estimatedDelivery" AS estimated_delivery,
       "createdAt" AS created_at,
       "updatedAt" AS updated_at
FROM "ProductDelivery";

-- ============ review_products (reviewProduct) ============
CREATE OR REPLACE VIEW review_products AS
SELECT id,
       "ProductId" AS product_id,
       "userId" AS user_id,
       comment, images, rating,
       "createdAt" AS created_at,
       "updatedAt" AS updated_at
FROM "reviewProduct";