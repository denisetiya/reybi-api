use serde::{Deserialize, Serialize};

/// Register request — Firebase idToken is passed via Authorization header
/// and validated server-side.  Optional body fields override display name
/// / photo URL (defaults come from the verified Firebase profile).
#[derive(Debug, Default, Deserialize)]
pub struct RegisterRequest {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub photo_url: Option<String>,
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