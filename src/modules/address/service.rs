use sqlx::PgPool;
use crate::errors::{AppError, AppResult};
use crate::models::Address;
use super::dto::CreateAddressRequest;

pub struct AddressService;

impl AddressService {
    pub async fn create(db: &PgPool, user_id: String, data: CreateAddressRequest) -> AppResult<Address> {
        let id = cuid2::create_id();
        sqlx::query_as::<_, Address>(
            r#"INSERT INTO addresses (id, user_id, address, label, phone_number, main) VALUES ($1,$2,$3,$4,$5,$6) RETURNING *"#
        ).bind(id).bind(&user_id).bind(&data.address).bind(&data.label)
        .bind(&data.phone_number).bind(data.main.unwrap_or(false))
        .fetch_one(db).await.map_err(|e| AppError::Internal(e.into()))
    }

    pub async fn update(db: &PgPool, user_id: String, data: CreateAddressRequest) -> AppResult<Address> {
        sqlx::query_as::<_, Address>(
            r#"UPDATE addresses SET address=$2, label=$3, phone_number=$4, main=$5, updated_at=NOW() WHERE user_id=$1 RETURNING *"#
        ).bind(&user_id).bind(&data.address).bind(&data.label)
        .bind(&data.phone_number).bind(data.main.unwrap_or(false))
        .fetch_optional(db).await.map_err(|e| AppError::Internal(e.into()))?
        .ok_or_else(|| AppError::NotFound("Address not found".into()))
    }
}
