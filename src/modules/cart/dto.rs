use serde::Deserialize;
use uuid::Uuid;
use crate::common::pagination::HasCursor;

#[derive(Debug, Deserialize)]
pub struct AddCartRequest {
    pub product_id: Uuid,
    pub quantity: i32,
    pub variant_id: Option<Uuid>,
}

impl HasCursor for crate::models::Cart {
    fn cursor_value(&self) -> String { self.id.to_string() }
}
