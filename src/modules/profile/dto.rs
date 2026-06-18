use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UpdateProfileRequest {
    pub name: Option<String>,
    pub photo_url: Option<String>,
    pub role: Option<String>,
    pub phone_number: Option<String>,
}
