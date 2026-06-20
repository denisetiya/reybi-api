use serde::Deserialize;
use serde_json::Value;
use crate::common::pagination::HasCursor;

#[derive(Debug, Deserialize)]
pub struct CreateDepositeRequest {
    pub address_id: String,
    pub r#type: String,
    pub pickup_date: String,
    pub pickup_time: String,
    pub coin: Option<i32>,
    pub images: Option<Value>,
    pub landfill_id: Option<String>,
    pub garbage_type: Vec<GarbageItem>,
}

#[derive(Debug, Deserialize)]
pub struct GarbageItem {
    pub trash_type_id: String,
    pub amount: i32,
}

impl HasCursor for crate::models::Deposite {
    fn cursor_value(&self) -> String { self.id.to_string() }
}
