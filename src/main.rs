use axum::{routing::*};
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::CorsLayer;
use tower_http::compression::CompressionLayer;
use tower_http::trace::TraceLayer;

use reybi_api::config::{AppConfig, AppState};
use reybi_api::middleware::jwt_auth;

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
        .connect_lazy(&config.database_url)
        .expect("Failed to connect to database");

    let state = AppState::new(pool, config.clone());

    let auth_routes = Router::new()
        .route("/", post(reybi_api::routes::auth::login))
        .route("/register", post(reybi_api::routes::auth::register))
        .route("/reset-password", post(reybi_api::routes::auth::reset_password));

    let product_routes = Router::new()
        .route("/", get(reybi_api::routes::product::list_products))
        .route("/:id", get(reybi_api::routes::product::get_product)
            .put(reybi_api::routes::product::update_product)
            .delete(reybi_api::routes::product::delete_product))
        .route("/create", post(reybi_api::routes::product::create_product))
        .route("/variant/:id", post(reybi_api::routes::product::create_variant));

    let banner_routes = Router::new()
        .route("/", get(reybi_api::routes::banner::list_banners))
        .route("/type/:type", get(reybi_api::routes::banner::list_banners_by_type))
        .route("/create", post(reybi_api::routes::banner::create_banner));

    let article_routes = Router::new()
        .route("/", get(reybi_api::routes::article::list_articles))
        .route("/create", post(reybi_api::routes::article::create_article))
        .route("/:id", get(reybi_api::routes::article::get_article)
            .put(reybi_api::routes::article::update_article)
            .delete(reybi_api::routes::article::delete_article));

    let profile_routes = Router::new()
        .route("/:email", get(reybi_api::routes::profile::get_profile)
            .put(reybi_api::routes::profile::update_profile));

    let review_routes = Router::new()
        .route("/", post(reybi_api::routes::review::create_review))
        .route("/:id", put(reybi_api::routes::review::update_review));

    let cart_routes = Router::new()
        .route("/user/:user_id", get(reybi_api::routes::cart::get_cart)
            .post(reybi_api::routes::cart::add_cart))
        .route("/item/:id", delete(reybi_api::routes::cart::delete_cart));

    let order_routes = Router::new()
        .route("/", get(reybi_api::routes::order::get_all_orders))
        .route("/user/:user_id", get(reybi_api::routes::order::get_orders)
            .post(reybi_api::routes::order::create_order))
        .route("/:id", delete(reybi_api::routes::order::delete_order));

    let deposite_routes = Router::new()
        .route("/", get(reybi_api::routes::deposite::get_all_deposites)
            .post(reybi_api::routes::deposite::create_deposite))
        .route("/user/:id", get(reybi_api::routes::deposite::get_user_deposites));

    let landfill_routes = Router::new()
        .route("/", get(reybi_api::routes::landfill::list_landfills))
        .route("/create", post(reybi_api::routes::landfill::create_landfill))
        .route("/:id", put(reybi_api::routes::landfill::update_landfill)
            .delete(reybi_api::routes::landfill::delete_landfill));

    let trash_routes = Router::new()
        .route("/types", get(reybi_api::routes::trash::list_trash_types))
        .route("/type", post(reybi_api::routes::trash::create_trash_type))
        .route("/type/:id", put(reybi_api::routes::trash::update_trash_type)
            .delete(reybi_api::routes::trash::delete_trash_type));

    let address_routes = Router::new()
        .route("/user/:user_id", post(reybi_api::routes::address::create_address)
            .put(reybi_api::routes::address::update_address));

    let saller_routes = Router::new()
        .route("/products/:id", get(reybi_api::routes::saller::get_saller_products));

    let api_routes = Router::new()
        .nest("/auth", auth_routes)
        .nest("/products", product_routes)
        .nest("/banners", banner_routes)
        .nest("/articles", article_routes)
        .nest("/profile", profile_routes)
        .nest("/reviews", review_routes)
        .nest("/carts", cart_routes)
        .nest("/orders", order_routes)
        .nest("/deposites", deposite_routes)
        .nest("/landfills", landfill_routes)
        .nest("/trash", trash_routes)
        .nest("/addresses", address_routes)
        .nest("/sallers", saller_routes);

    let app = Router::new()
        .nest("/v1", api_routes)
        .layer(
            axum::middleware::from_fn_with_state(state.clone(), jwt_auth)
        )
        .layer(CompressionLayer::new())
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state.clone());

    let addr = format!("{}:{}", config.host, config.port);
    tracing::info!("Server starting on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
