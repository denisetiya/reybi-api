use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct ResetPasswordRequest {
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub refresh_token: String,
    pub user: super::service::UserSummary,
}
