use serde::Deserialize;
use crate::common::pagination::HasCursor;

#[derive(Debug, Deserialize)]
pub struct CreateBannerRequest {
    pub image: String,
    pub r#type: Option<String>,
}

impl HasCursor for crate::models::Banner {
    fn cursor_value(&self) -> String { self.id.to_string() }
}
