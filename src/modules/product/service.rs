use sqlx::PgPool;
use uuid::Uuid;
use serde_json::Value;
use crate::errors::{AppError, AppResult};
use crate::models::Product;
use super::dto::{ProductFilter, CreateVariantRequest};

pub struct ProductService;

impl ProductService {
    pub async fn list(db: &PgPool, filter: &ProductFilter) -> AppResult<Vec<Product>> {
        let limit = filter.limit.unwrap_or(25).min(100).max(1);
        if let Some(cat) = &filter.category {
            let rows = sqlx::query_as::<_, Product>(
                "SELECT * FROM products WHERE category = $1 ORDER BY created_at DESC LIMIT $2"
            )
            .bind(cat)
            .bind(limit + 1)
            .fetch_all(db).await.map_err(|e| AppError::Internal(e.into()))?;
            return Ok(rows);
        }
        if let Some(q) = &filter.search {
            let pattern = format!("%{}%", q.replace('%', "%%"));
            let rows = sqlx::query_as::<_, Product>(
                "SELECT * FROM products WHERE name ILIKE $1 ORDER BY created_at DESC LIMIT $2"
            )
            .bind(&pattern)
            .bind(limit + 1)
            .fetch_all(db).await.map_err(|e| AppError::Internal(e.into()))?;
            return Ok(rows);
        }
        let rows = sqlx::query_as::<_, Product>(
            "SELECT * FROM products ORDER BY created_at DESC LIMIT $1"
        )
        .bind(limit + 1)
        .fetch_all(db).await.map_err(|e| AppError::Internal(e.into()))?;
        Ok(rows)
    }

    pub async fn get_by_id(db: &PgPool, id: Uuid) -> AppResult<Option<Product>> {
        sqlx::query_as::<_, Product>("SELECT * FROM products WHERE id = $1")
            .bind(id)
            .fetch_optional(db).await.map_err(|e| AppError::Internal(e.into()))
    }

    pub async fn create(
        db: &PgPool, name: &str, price: i32, stock: i32,
        description: &str, category: &str, location: Option<&str>,
        discount: Option<f64>, coin: Option<i32>, recommended: Option<bool>,
        saller_id: Option<Uuid>, thumbnail: Option<&str>, images: Option<Value>,
    ) -> AppResult<Product> {
        let id = Uuid::new_v4();
        sqlx::query_as::<_, Product>(
            r#"INSERT INTO products (id, name, price, stock, description, category,
               location, discount, coin, recommended, saller_id, thumbnail, images)
               VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13) RETURNING *"#
        )
        .bind(id).bind(name).bind(price).bind(stock)
        .bind(description).bind(category).bind(location)
        .bind(discount).bind(coin).bind(recommended)
        .bind(saller_id).bind(thumbnail).bind(images)
        .fetch_one(db).await.map_err(|e| AppError::Internal(e.into()))
    }

    pub async fn update(
        db: &PgPool, id: Uuid,
        name: Option<&str>, price: Option<i32>, stock: Option<i32>,
        description: Option<&str>, category: Option<&str>,
        location: Option<&str>, discount: Option<f64>, coin: Option<i32>,
        recommended: Option<bool>, thumbnail: Option<&str>, images: Option<Value>,
    ) -> AppResult<Product> {
        let existing = Self::get_by_id(db, id).await?
            .ok_or_else(|| AppError::NotFound("Product not found".into()))?;
        sqlx::query_as::<_, Product>(
            r#"UPDATE products SET
               name=$2, price=$3, stock=$4, description=$5, category=$6,
               location=$7, discount=$8, coin=$9, recommended=$10,
               thumbnail=$11, images=$12, updated_at=NOW()
               WHERE id=$1 RETURNING *"#
        )
        .bind(id)
        .bind(name.unwrap_or(&existing.name))
        .bind(price.unwrap_or(existing.price))
        .bind(stock.unwrap_or(existing.stock))
        .bind(description.unwrap_or(&existing.description))
        .bind(category.unwrap_or(&existing.category))
        .bind(location.or(existing.location.as_deref()))
        .bind(discount.or(existing.discount))
        .bind(coin.map(|v| v as i64).or(existing.coin.map(|c| c as i64)))
        .bind(recommended.or(existing.recommended))
        .bind(thumbnail.or(existing.thumbnail.as_deref()))
        .bind(images.or(Some(existing.images.clone())))
        .fetch_one(db).await.map_err(|e| AppError::Internal(e.into()))
    }

    pub async fn delete(db: &PgPool, id: Uuid) -> AppResult<()> {
        let r = sqlx::query("DELETE FROM products WHERE id = $1")
            .bind(id).execute(db).await.map_err(|e| AppError::Internal(e.into()))?;
        if r.rows_affected() == 0 { return Err(AppError::NotFound("Product not found".into())); }
        Ok(())
    }

    pub async fn add_variant(
        db: &PgPool, product_id: Uuid, data: CreateVariantRequest,
    ) -> AppResult<crate::models::VariantProduct> {
        let id = Uuid::new_v4();
        sqlx::query_as::<_, crate::models::VariantProduct>(
            r#"INSERT INTO product_variants (id, product_id, name, price, stock, image)
               VALUES ($1,$2,$3,$4,$5,$6) RETURNING *"#
        )
        .bind(id).bind(product_id).bind(&data.name)
        .bind(data.price).bind(data.stock).bind(&data.image)
        .fetch_one(db).await.map_err(|e| AppError::Internal(e.into()))
    }
}
