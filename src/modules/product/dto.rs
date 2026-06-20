use serde::Deserialize;
use serde_json::Value;
use crate::common::pagination::HasCursor;

#[derive(Debug, Deserialize)]
pub struct ProductFilter {
    pub cursor: Option<String>,
    pub limit: Option<i64>,
    pub category: Option<String>,
    pub search: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateProductRequest {
    pub name: String,
    pub price: i32,
    pub stock: i32,
    pub description: String,
    pub category: String,
    pub location: Option<String>,
    pub discount: Option<f64>,
    pub coin: Option<i32>,
    pub recommended: Option<bool>,
    pub saller_id: Option<String>,
    pub thumbnail: Option<String>,
    pub images: Option<Value>,
}

#[derive(Debug, Deserialize, Default)]
pub struct UpdateProductRequest {
    pub name: Option<String>,
    pub price: Option<i32>,
    pub stock: Option<i32>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub location: Option<String>,
    pub discount: Option<f64>,
    pub coin: Option<i32>,
    pub recommended: Option<bool>,
    pub thumbnail: Option<String>,
    pub images: Option<Value>,
}

#[derive(Debug, Deserialize)]
pub struct CreateVariantRequest {
    pub name: String,
    pub price: i32,
    pub stock: i32,
    pub image: Option<String>,
}

impl HasCursor for crate::models::Product {
    fn cursor_value(&self) -> String { self.id.to_string() }
}
