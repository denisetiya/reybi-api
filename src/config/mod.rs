pub mod state;

use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub jwt_access_secret: String,
    pub jwt_refresh_secret: String,
    pub host: String,
    pub port: u16,
    pub upload_dir: String,
    pub firebase_credentials_path: String,
    pub key_server: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            jwt_access_secret: env::var("JWT_ACCESS_SECRET").expect("JWT_ACCESS_SECRET must be set"),
            jwt_refresh_secret: env::var("JWT_REFRESH_SECRET").expect("JWT_REFRESH_SECRET must be set"),
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".into()),
            port: env::var("PORT").unwrap_or_else(|_| "3000".into()).parse().expect("PORT must be a number"),
            upload_dir: env::var("UPLOAD_DIR").unwrap_or_else(|_| "./uploads".into()),
            firebase_credentials_path: env::var("FIREBASE_CREDENTIALS_PATH").unwrap_or_else(|_| "firebase-credentials.json".into()),
            key_server: env::var("KEY_SERVER").expect("KEY_SERVER must be set"),
        }
    }
}

pub use state::AppState;
