use sqlx::PgPool;
use uuid::Uuid;
use crate::dto::ProductFilter;
use crate::errors::{AppError, AppResult};

pub struct ProductService;

#[derive(serde::Serialize)]
pub struct ProductWithRelations {
    #[serde(flatten)]
    pub product: crate::models::Product,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saller: Option<crate::models::Saller>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variants: Option<Vec<crate::models::VariantProduct>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reviews: Option<Vec<serde_json::Value>>,
}

impl ProductService {
    pub async fn list(db: &PgPool, filter: &ProductFilter) -> AppResult<Vec<crate::models::Product>> {
        let limit = filter.limit.unwrap_or(25).clamp(1, 100) + 1;
        let mut query = String::from("SELECT * FROM products");
        let mut conditions = Vec::new();

        if let Some(ref name) = filter.name {
            conditions.push(format!("name ILIKE '%{}%'", name.replace('\'', "''")));
        }
        if let Some(ref category) = filter.category {
            conditions.push(format!("category = '{}'", category.replace('\'', "''")));
        }
        if let Some(min) = filter.price_min {
            conditions.push(format!("price >= {min}"));
        }
        if let Some(max) = filter.price_max {
            conditions.push(format!("price <= {max}"));
        }
        if filter.stock == Some(true) {
            conditions.push("stock >= 1".into());
        }
        if filter.discount == Some(true) {
            conditions.push("discount IS NOT NULL AND discount > 0".into());
        }

        if let Some(ref cursor) = filter.cursor {
            if let Ok(cursor_id) = Uuid::parse_str(cursor) {
                conditions.push(format!("id < '{cursor_id}'"));
            }
        }

        if !conditions.is_empty() {
            query.push_str(" WHERE ");
            query.push_str(&conditions.join(" AND "));
        }

        query.push_str(" ORDER BY created_at DESC");
        query.push_str(&format!(" LIMIT {limit}"));

        let products: Vec<crate::models::Product> = sqlx::query_as(&query)
            .fetch_all(db)
            .await
            .map_err(|e| AppError::Internal(e.into()))?;

        Ok(products)
    }

    pub async fn get_by_id(
        db: &PgPool,
        id: Uuid,
    ) -> AppResult<Option<ProductWithRelations>> {
        let product = sqlx::query_as::<_, crate::models::Product>(
            "SELECT * FROM products WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(db)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        let product = match product {
            Some(p) => p,
            None => return Ok(None),
        };

        let saller = if let Some(sid) = product.saller_id {
            sqlx::query_as::<_, crate::models::Saller>(
                "SELECT * FROM sallers WHERE id = $1"
            )
            .bind(sid)
            .fetch_optional(db)
            .await
            .unwrap_or(None)
        } else {
            None
        };

        let variants = sqlx::query_as::<_, crate::models::VariantProduct>(
            "SELECT * FROM variant_products WHERE product_id = $1"
        )
        .bind(id)
        .fetch_all(db)
        .await
        .unwrap_or_default();

        Ok(Some(ProductWithRelations {
            product,
            saller,
            variants: Some(variants),
            reviews: None,
        }))
    }

    pub async fn create(
        db: &PgPool,
        name: &str,
        price: i64,
        stock: i64,
        description: &str,
        category: &str,
        location: Option<&str>,
        discount: Option<f64>,
        coin: Option<i64>,
        recommended: Option<bool>,
        saller_id: Option<Uuid>,
        thumbnail: Option<&str>,
        images: Option<serde_json::Value>,
    ) -> AppResult<crate::models::Product> {
        let id = Uuid::new_v4();
        sqlx::query_as::<_, crate::models::Product>(
            r#"INSERT INTO products (id, name, price, stock, description, category, location, discount, coin, recommended, saller_id, thumbnail, images)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
               RETURNING *"#
        )
        .bind(id)
        .bind(name)
        .bind(price as i32)
        .bind(stock as i32)
        .bind(description)
        .bind(category)
        .bind(location)
        .bind(discount)
        .bind(coin.map(|v| v as i32))
        .bind(recommended.unwrap_or(false))
        .bind(saller_id)
        .bind(thumbnail)
        .bind(images.unwrap_or(serde_json::json!({})))
        .fetch_one(db)
        .await
        .map_err(|e| AppError::Internal(e.into()))
    }

    pub async fn update(
        db: &PgPool,
        id: Uuid,
        name: Option<&str>,
        price: Option<i64>,
        stock: Option<i64>,
        description: Option<&str>,
        category: Option<&str>,
        location: Option<&str>,
        discount: Option<f64>,
        coin: Option<i64>,
        recommended: Option<bool>,
        thumbnail: Option<&str>,
        images: Option<serde_json::Value>,
    ) -> AppResult<crate::models::Product> {
        let existing = sqlx::query_as::<_, crate::models::Product>(
            "SELECT * FROM products WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(db)
        .await
        .map_err(|e| AppError::Internal(e.into()))?
        .ok_or_else(|| AppError::NotFound("Product not found".into()))?;

        sqlx::query_as::<_, crate::models::Product>(
            r#"UPDATE products SET
               name = COALESCE($2, name),
               price = COALESCE($3, price),
               stock = COALESCE($4, stock),
               description = COALESCE($5, description),
               category = COALESCE($6, category),
               location = COALESCE($7, location),
               discount = COALESCE($8, discount),
               coin = COALESCE($9, coin),
               recommended = COALESCE($10, recommended),
               thumbnail = COALESCE($11, thumbnail),
               images = COALESCE($12, images),
               updated_at = NOW()
               WHERE id = $1 RETURNING *"#
        )
        .bind(id)
        .bind(name.unwrap_or(&existing.name))
        .bind(price.map(|v| v as i32).unwrap_or(existing.price))
        .bind(stock.map(|v| v as i32).unwrap_or(existing.stock))
        .bind(description.unwrap_or(&existing.description))
        .bind(category.unwrap_or(&existing.category))
        .bind(location.unwrap_or(existing.location.as_deref().unwrap_or("")))
        .bind(discount.or(existing.discount))
        .bind(coin.map(|v| v as i32).or(existing.coin))
        .bind(recommended.or(existing.recommended))
        .bind(thumbnail.unwrap_or(existing.thumbnail.as_deref().unwrap_or("")))
        .bind(images)
        .fetch_one(db)
        .await
        .map_err(|e| AppError::Internal(e.into()))
    }

    pub async fn delete(db: &PgPool, id: Uuid) -> AppResult<()> {
        let result = sqlx::query("DELETE FROM products WHERE id = $1")
            .bind(id)
            .execute(db)
            .await
            .map_err(|e| AppError::Internal(e.into()))?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Product not found".into()));
        }
        Ok(())
    }

    pub async fn add_variant(
        db: &PgPool,
        product_id: Uuid,
        name: &str,
        price: i64,
        stock: i64,
        image: Option<&str>,
    ) -> AppResult<crate::models::VariantProduct> {
        let id = Uuid::new_v4();
        sqlx::query_as::<_, crate::models::VariantProduct>(
            r#"INSERT INTO variant_products (id, product_id, name, price, stock, image)
               VALUES ($1, $2, $3, $4, $5, $6) RETURNING *"#
        )
        .bind(id)
        .bind(product_id)
        .bind(name)
        .bind(price as i32)
        .bind(stock as i32)
        .bind(image)
        .fetch_one(db)
        .await
        .map_err(|e| AppError::Internal(e.into()))
    }
}
