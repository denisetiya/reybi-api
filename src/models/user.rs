use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: String,
    pub fb_id: String,
    pub email: String,
    pub name: Option<String>,
    pub role: Option<String>,
    pub phone_number: Option<String>,
    pub photo_url: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserDetail {
    pub id: String,
    pub user_id: String,
    pub exp: Option<f64>,
    pub level: Option<i32>,
    pub coin: Option<i32>,
    pub badge: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Token {
    pub id: String,
    pub refresh_token: String,
    pub user_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Address {
    pub id: String,
    pub user_id: String,
    pub address: String,
    pub label: String,
    pub phone_number: String,
    pub main: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub price: i32,
    pub coin: Option<i32>,
    pub description: String,
    pub thumbnail: Option<String>,
    pub images: serde_json::Value,
    pub stock: i32,
    pub location: Option<String>,
    pub category: String,
    pub discount: Option<f64>,
    pub sold: Option<i32>,
    pub available: Option<i32>,
    pub rating: Option<f64>,
    pub saller_id: Option<String>,
    pub recommended: Option<bool>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct VariantProduct {
    pub id: String,
    pub product_id: String,
    pub name: String,
    pub price: i32,
    pub stock: i32,
    pub image: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Cart {
    pub id: String,
    pub user_id: String,
    pub product_id: String,
    pub quantity: i32,
    pub variant_id: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Order {
    pub id: String,
    pub user_id: String,
    pub product_id: String,
    pub quantity: i32,
    pub coin: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PaymentHistory {
    pub id: String,
    pub order_id: String,
    pub method: String,
    pub r#type: Option<String>,
    pub amount: f64,
    pub va_number: Option<serde_json::Value>,
    pub link_qr: Option<serde_json::Value>,
    pub midtrans_id: Option<String>,
    pub status: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ProductDelivery {
    pub id: String,
    pub order_id: String,
    pub status: String,
    pub tracking_number: String,
    pub history: serde_json::Value,
    pub estimated_delivery: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ReviewProduct {
    pub id: String,
    pub product_id: String,
    pub user_id: String,
    pub comment: String,
    pub images: Option<serde_json::Value>,
    pub rating: f64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Banner {
    pub id: String,
    pub image: String,
    pub r#type: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Article {
    pub id: String,
    pub thumbnail: String,
    pub header: String,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TrashType {
    pub id: String,
    pub name: String,
    pub image: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Deposite {
    pub id: String,
    pub user_id: String,
    pub address_id: String,
    pub r#type: String,
    pub pickup_date: String,
    pub pickup_time: String,
    pub coin: Option<i32>,
    pub images: serde_json::Value,
    pub landfill_id: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct GarbageDetail {
    pub id: String,
    pub trash_type_id: String,
    pub deposite_id: String,
    pub amount: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DepositeStatus {
    pub id: String,
    pub deposit_id: String,
    pub ongoing: Option<bool>,
    pub pickup: Option<bool>,
    pub landfill: Option<bool>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Landfill {
    pub id: String,
    pub name: String,
    pub address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Saller {
    pub id: String,
    pub name: String,
    pub image: Option<String>,
    pub total_product: i32,
    pub product_sold: Option<i32>,
    pub address: String,
    pub rating: Option<f64>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
