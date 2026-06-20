use axum::{middleware as mw, Router};
use sqlx::postgres::PgPoolOptions;
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use reybi_api::common::locale;
use reybi_api::config::{AppConfig, AppState};
use reybi_api::middleware;
use reybi_api::utils::cache::Cache;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "reybi_api=debug,tower_http=debug".into()),
        )
        .init();

    dotenvy::dotenv().ok();
    let config = AppConfig::from_env();

    let pool = PgPoolOptions::new()
        .max_connections(20)
        .min_connections(4)
        .acquire_timeout(std::time::Duration::from_secs(5))
        .idle_timeout(std::time::Duration::from_secs(300))
        .connect(&config.database_url)
        .await
        .expect("Failed to connect to database");

    // run pending migrations (idempotent — skips already-applied files)
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run database migrations");
    tracing::info!("✓ database migrations applied");

    let cache = Cache::connect(&config.redis_url).await;
    let state = AppState::new(pool, config.clone(), cache);

    let api_routes = Router::new()
        .nest("/auth", reybi_api::modules::auth::routes::routes())
        .nest("/products", reybi_api::modules::product::routes::routes())
        .nest("/banners", reybi_api::modules::banner::routes::routes())
        .nest("/articles", reybi_api::modules::article::routes::routes())
        .nest("/profile", reybi_api::modules::profile::routes::routes())
        .nest("/reviews", reybi_api::modules::review::routes::routes())
        .nest("/carts", reybi_api::modules::cart::routes::routes())
        .nest("/orders", reybi_api::modules::order::routes::routes())
        .nest("/deposites", reybi_api::modules::deposite::routes::routes())
        .nest("/landfills", reybi_api::modules::landfill::routes::routes())
        .nest("/trash", reybi_api::modules::trash::routes::routes())
        .nest("/addresses", reybi_api::modules::address::routes::routes())
        .nest("/sallers", reybi_api::modules::saller::routes::routes())
        .nest("/payments", reybi_api::modules::payment::routes::routes());

    let app = Router::new()
        .nest("/v1", api_routes)
        .layer(mw::from_fn_with_state(state.clone(), middleware::jwt_auth))
        .layer(mw::from_fn(locale::locale_middleware))
        .layer(CompressionLayer::new())
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state.clone());

    let addr = format!("{}:{}", config.host, config.port);
    tracing::info!("Server starting on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
