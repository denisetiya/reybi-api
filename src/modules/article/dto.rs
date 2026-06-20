use crate::common::pagination::HasCursor;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateArticleRequest {
    pub thumbnail: String,
    pub header: String,
    pub content: String,
}

impl HasCursor for crate::models::Article {
    fn cursor_value(&self) -> String {
        self.id.to_string()
    }
}
