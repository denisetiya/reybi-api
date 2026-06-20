use serde::Deserialize;
use crate::common::pagination::HasCursor;

#[derive(Debug, Deserialize)]
pub struct CreateOrderRequest {
    pub product_id: String,
    pub quantity: i32,
    pub coin: Option<i64>,
    pub payment: PaymentRequest,
}

#[derive(Debug, Deserialize)]
pub struct PaymentRequest {
    pub method: String,
    pub r#type: String,
    pub amount: f64,
}

impl HasCursor for crate::models::Order {
    fn cursor_value(&self) -> String { self.id.to_string() }
}
