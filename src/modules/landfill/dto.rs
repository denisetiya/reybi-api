use serde::Deserialize;
use crate::common::pagination::HasCursor;

#[derive(Debug, Deserialize)]
pub struct CreateLandfillRequest {
    pub name: String,
    pub address: String,
}

impl HasCursor for crate::models::Landfill {
    fn cursor_value(&self) -> String { self.id.to_string() }
}
