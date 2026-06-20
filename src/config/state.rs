use crate::config::AppConfig;
use crate::middleware::AuthConfig;
use crate::utils::cache::Cache;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub config: AppConfig,
    pub auth: AuthConfig,
    pub cache: Cache,
}

impl AppState {
    pub fn new(db: PgPool, config: AppConfig, cache: Cache) -> Self {
        let auth = AuthConfig::new(
            config.jwt_access_secret.clone(),
            config.jwt_refresh_secret.clone(),
            config.key_server.clone(),
        );
        Self {
            db,
            config,
            auth,
            cache,
        }
    }
}
