use crate::common::pagination::HasCursor;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateTrashTypeRequest {
    pub name: String,
    pub image: Option<String>,
}

impl HasCursor for crate::models::TrashType {
    fn cursor_value(&self) -> String {
        self.id.to_string()
    }
}
