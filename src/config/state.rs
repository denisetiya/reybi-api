use crate::config::AppConfig;
use crate::middleware::AuthConfig;
use crate::utils::cache::Cache;
use crate::utils::firebase::FirebaseVerifier;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub config: AppConfig,
    pub auth: AuthConfig,
    pub cache: Cache,
    pub firebase: FirebaseVerifier,
}

impl AppState {
    pub fn new(db: PgPool, config: AppConfig, cache: Cache, firebase: FirebaseVerifier) -> Self {
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
            firebase,
        }
    }
}
