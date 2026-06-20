use serde::Deserialize;
use serde_json::Value;
#[derive(Debug, Deserialize)]
pub struct CreateReviewRequest {
    pub product_id: String,
    pub comment: String,
    pub rating: f64,
    pub images: Option<Value>,
}

#[derive(Debug, Deserialize, Default)]
pub struct UpdateReviewRequest {
    pub comment: Option<String>,
    pub rating: Option<f64>,
}
