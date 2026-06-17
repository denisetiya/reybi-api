use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Standard success response
#[derive(Debug, Serialize)]
pub struct SuccessResponse<T: Serialize> {
    pub success: bool,
    pub data: T,
    pub meta: ResponseMeta,
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T: Serialize> {
    pub success: bool,
    pub data: Vec<T>,
    pub meta: PaginationMeta,
}

#[derive(Debug, Serialize)]
pub struct ResponseMeta {
    pub locale: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<PaginationInfo>,
}

#[derive(Debug, Serialize)]
pub struct PaginationInfo {
    pub cursor: Option<String>,
    pub has_more: bool,
    pub count: usize,
}

#[derive(Debug, Serialize)]
pub struct PaginationMeta {
    pub locale: String,
    pub pagination: PaginationInfo,
}

/// Pagination query params
#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    pub cursor: Option<String>,
    pub limit: Option<i64>,
}

impl PaginationQuery {
    pub fn limit_or_default(&self) -> i64 {
        self.limit.unwrap_or(25).clamp(1, 100)
    }

    pub fn take(&self) -> i64 {
        self.limit_or_default() + 1
    }
}

/// Auth DTOs
#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    pub firebase_token: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub email: String,
    pub name: String,
    pub role: String,
    pub photo_url: Option<String>,
    pub phone_number: Option<String>,
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub name: String,
    pub phone_number: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ResetPasswordRequest {
    pub email: String,
}

/// Product DTOs
#[derive(Debug, Deserialize)]
pub struct ProductFilter {
    pub cursor: Option<String>,
    pub limit: Option<i64>,
    pub name: Option<String>,
    pub category: Option<String>,
    pub price_min: Option<i64>,
    pub price_max: Option<i64>,
    pub popular: Option<bool>,
    pub discount: Option<bool>,
    pub stock: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CreateProductRequest {
    pub name: String,
    pub price: i64,
    pub stock: i64,
    pub description: String,
    pub category: String,
    pub location: Option<String>,
    pub discount: Option<f64>,
    pub coin: Option<i64>,
    pub recommended: Option<bool>,
    pub saller_id: Option<Uuid>,
    pub thumbnail: Option<String>,
    pub images: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProductRequest {
    pub name: Option<String>,
    pub price: Option<i64>,
    pub stock: Option<i64>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub location: Option<String>,
    pub discount: Option<f64>,
    pub coin: Option<i64>,
    pub recommended: Option<bool>,
    pub thumbnail: Option<String>,
    pub images: Option<serde_json::Value>,
}

/// Cart DTOs
#[derive(Debug, Deserialize)]
pub struct AddCartRequest {
    pub product_id: Uuid,
    pub quantity: i64,
    pub variant_id: Option<Uuid>,
}

/// Order DTOs
#[derive(Debug, Deserialize)]
pub struct PaymentInfo {
    pub method: String,
    pub r#type: String,
    pub amount: f64,
}

#[derive(Debug, Deserialize)]
pub struct CreateOrderRequest {
    pub product_id: Uuid,
    pub quantity: i64,
    pub coin: Option<i64>,
    pub product_price: f64,
    pub customer_name: String,
    pub customer_email: String,
    pub payment: PaymentInfo,
}

/// Deposite DTOs
#[derive(Debug, Deserialize)]
pub struct GarbageTypeItem {
    pub trash_type_id: Uuid,
    pub amount: i64,
}

#[derive(Debug, Deserialize)]
pub struct CreateDepositeRequest {
    pub address_id: Uuid,
    pub r#type: String,
    pub landfill_id: Option<Uuid>,
    pub pickup_date: String,
    pub pickup_time: String,
    pub coins: Option<i64>,
    pub images: Option<serde_json::Value>,
    pub garbage_type: Vec<GarbageTypeItem>,
}

/// Review DTOs
#[derive(Debug, Deserialize)]
pub struct CreateReviewRequest {
    pub product_id: Uuid,
    pub comment: String,
    pub rating: f64,
    pub images: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateReviewRequest {
    pub comment: Option<String>,
    pub rating: Option<f64>,
}

/// Article DTOs
#[derive(Debug, Deserialize)]
pub struct CreateArticleRequest {
    pub thumbnail: String,
    pub header: String,
    pub content: String,
}

/// Banner DTOs
#[derive(Debug, Deserialize)]
pub struct CreateBannerRequest {
    pub image: String,
    pub r#type: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
}

/// TrashType DTOs
#[derive(Debug, Deserialize)]
pub struct CreateTrashTypeRequest {
    pub name: String,
    pub image: Option<String>,
}

/// Landfill DTOs
#[derive(Debug, Deserialize)]
pub struct CreateLandfillRequest {
    pub name: String,
    pub address: String,
}

/// Address DTOs
#[derive(Debug, Deserialize)]
pub struct CreateAddressRequest {
    pub address: String,
    pub label: String,
    pub phone_number: String,
    pub main: Option<bool>,
}

/// Profile DTOs
#[derive(Debug, Deserialize)]
pub struct UpdateProfileRequest {
    pub name: Option<String>,
    pub photo_url: Option<String>,
    pub role: Option<String>,
    pub address: Option<String>,
    pub phone_number: Option<String>,
    pub exp: Option<f64>,
    pub level: Option<i64>,
    pub coin: Option<i64>,
}

/// Variant DTOs
#[derive(Debug, Deserialize)]
pub struct CreateVariantRequest {
    pub name: String,
    pub price: i64,
    pub stock: i64,
    pub image: Option<String>,
}
