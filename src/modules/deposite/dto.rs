use serde::Deserialize;
use serde_json::Value;
use uuid::Uuid;
use crate::common::pagination::HasCursor;

#[derive(Debug, Deserialize)]
pub struct CreateDepositeRequest {
    pub address_id: Uuid,
    pub r#type: String,
    pub pickup_date: String,
    pub pickup_time: String,
    pub coin: Option<i32>,
    pub images: Option<Value>,
    pub landfill_id: Option<Uuid>,
    pub garbage_type: Vec<GarbageItem>,
}

#[derive(Debug, Deserialize)]
pub struct GarbageItem {
    pub trash_type_id: Uuid,
    pub amount: i32,
}

impl HasCursor for crate::models::Deposite {
    fn cursor_value(&self) -> String { self.id.to_string() }
}
