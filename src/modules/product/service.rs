use sqlx::PgPool;
use uuid::Uuid;
use crate::errors::{AppError, AppResult};
use crate::models::Product;
use super::dto::{ProductFilter, CreateProductRequest, UpdateProductRequest, CreateVariantRequest};

pub struct ProductService;

impl ProductService {
    pub async fn list(db: &PgPool, filter: &ProductFilter) -> AppResult<Vec<Product>> {
        let limit = filter.limit.unwrap_or(25).clamp(1, 100);
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

    pub async fn create(db: &PgPool, data: CreateProductRequest) -> AppResult<Product> {
        let id = Uuid::new_v4();
        sqlx::query_as::<_, Product>(
            r#"INSERT INTO products (id, name, price, stock, description, category,
               location, discount, coin, recommended, saller_id, thumbnail, images)
               VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13) RETURNING *"#
        )
        .bind(id).bind(&data.name).bind(data.price).bind(data.stock)
        .bind(&data.description).bind(&data.category).bind(data.location.as_deref())
        .bind(data.discount).bind(data.coin).bind(data.recommended)
        .bind(data.saller_id).bind(data.thumbnail.as_deref()).bind(data.images)
        .fetch_one(db).await.map_err(|e| AppError::Internal(e.into()))
    }

    pub async fn update(db: &PgPool, id: Uuid, data: UpdateProductRequest) -> AppResult<Product> {
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
        .bind(data.name.as_deref().unwrap_or(&existing.name))
        .bind(data.price.unwrap_or(existing.price))
        .bind(data.stock.unwrap_or(existing.stock))
        .bind(data.description.as_deref().unwrap_or(&existing.description))
        .bind(data.category.as_deref().unwrap_or(&existing.category))
        .bind(data.location.as_deref().or(existing.location.as_deref()))
        .bind(data.discount.or(existing.discount))
        .bind(data.coin.map(|v| v as i64).or(existing.coin.map(|c| c as i64)))
        .bind(data.recommended.or(existing.recommended))
        .bind(data.thumbnail.as_deref().or(existing.thumbnail.as_deref()))
        .bind(data.images.or(Some(existing.images.clone())))
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
