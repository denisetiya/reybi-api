use sqlx::PgPool;
use crate::config::AppConfig;
use crate::middleware::AuthConfig;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub config: AppConfig,
    pub auth: AuthConfig,
}

impl AppState {
    pub fn new(db: PgPool, config: AppConfig) -> Self {
        let auth = AuthConfig::new(
            config.jwt_access_secret.clone(),
            config.jwt_refresh_secret.clone(),
            config.key_server.clone(),
        );
        Self { db, config, auth }
    }
}
