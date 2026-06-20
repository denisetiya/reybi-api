use crate::common::pagination::HasCursor;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AddCartRequest {
    pub product_id: String,
    pub quantity: i32,
    pub variant_id: Option<String>,
}

impl HasCursor for crate::models::Cart {
    fn cursor_value(&self) -> String {
        self.id.to_string()
    }
}
