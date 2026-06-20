use axum::{middleware as mw, Router};
use mimalloc::MiMalloc;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;
use tower_http::limit::RequestBodyLimitLayer;
use tower_http::services::ServeDir;
use tower_http::set_header::SetResponseHeaderLayer;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};

use reybi_api::common::locale;
use reybi_api::config::{AppConfig, AppState};
use reybi_api::middleware;
use reybi_api::utils::cache::Cache;
use reybi_api::utils::firebase::FirebaseVerifier;

/// Multi-threaded heap allocator with better scaling than the system malloc
/// under concurrent load — fewer locks, fewer cross-thread bounces.  This is
/// the single global allocator for the whole process; everything allocates
/// through it.
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "reybi_api=info,tower_http=warn".into()),
        )
        .init();

    dotenvy::dotenv().ok();
    let config = AppConfig::from_env();

    // Postgres pool
    let pool = PgPoolOptions::new()
        .max_connections(config.pg_max_connections)
        .min_connections(config.pg_min_connections)
        .acquire_timeout(Duration::from_secs(config.pg_acquire_timeout_secs))
        .idle_timeout(Some(Duration::from_secs(300)))
        .max_lifetime(Some(Duration::from_secs(1800)))
        // SET statement_timeout on every new connection — aborts any query
        // that runs longer than this so a slow query can't pin a pool slot
        // forever.  Pair with sqlx's acquire_timeout for full coverage.
        .after_connect({
            let ms = config.pg_statement_timeout_ms;
            move |conn, _meta| {
                Box::pin(async move {
                    use sqlx::Executor;
                    conn.execute(format!("SET statement_timeout = {ms}").as_str())
                        .await?;
                    Ok(())
                })
            }
        })
        .connect(&config.database_url)
        .await
        .expect("Failed to connect to database");

    // Pre-warm pool — acquire `min_connections` to avoid first-request latency spike.
    {
        let mut warm = Vec::new();
        for _ in 0..4 {
            warm.push(pool.acquire().await.expect("pre-warm acquire"));
        }
        // drop the guards here, returning the live connections to the pool
    }
    tracing::info!("✓ db pool pre-warmed (4 conns ready)");

    // run pending migrations (idempotent — skips already-applied files)
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run database migrations");
    tracing::info!("✓ database migrations applied");

    let cache = Cache::connect(&config.redis_url).await;
    let firebase = FirebaseVerifier::new(&config.firebase_project_id).await;
    let state = AppState::new(pool, config.clone(), cache, firebase);

    // Admin seeding — promote the email in ADMIN_EMAIL env to role='admin'.
    // Idempotent: a no-op if the user doesn't exist yet (they'll be created
    // on first Firebase login and the role assignment will be reapplied).
    if let Ok(admin_email) = std::env::var("ADMIN_EMAIL") {
        if !admin_email.is_empty() {
            match sqlx::query(
                "UPDATE users SET role = 'admin' WHERE email = $1 AND (role IS NULL OR role <> 'admin')",
            )
            .bind(&admin_email)
            .execute(&state.db)
            .await
            {
                Ok(r) if r.rows_affected() > 0 => {
                    tracing::info!("✓ admin role granted to {}", admin_email);
                }
                Ok(_) => {
                    tracing::debug!("admin {} already up-to-date or not yet registered", admin_email);
                }
                Err(e) => tracing::warn!("admin seed failed: {}", e),
            }
        }
    }

    // Public routes — no JWT required.  Skip auth/locale middleware for these.
    let public_routes = Router::new()
        .nest("/auth", reybi_api::modules::auth::routes::routes())
        .nest("/products", reybi_api::modules::product::routes::public_routes())
        .nest("/banners", reybi_api::modules::banner::routes::public_routes())
        .nest("/articles", reybi_api::modules::article::routes::public_routes());

    // Admin-only routes — JWT + role == "admin".  Mounted INSIDE user_routes
    // so the `jwt_auth` middleware decodes the JWT once and `require_admin`
    // reuses the injected `Claims` from the request extensions.
    let admin_routes = Router::new()
        .nest("/banners", reybi_api::modules::banner::routes::protected_routes())
        .nest("/articles", reybi_api::modules::article::routes::protected_routes())
        .nest("/landfills", reybi_api::modules::landfill::routes::routes())
        .nest("/trash", reybi_api::modules::trash::routes::routes())
        .nest("/orders", reybi_api::modules::order::routes::admin_routes())
        .nest(
            "/deposites",
            reybi_api::modules::deposite::routes::admin_routes(),
        )
        .nest("/sallers", reybi_api::modules::saller::routes::routes())
        .route_layer(mw::from_fn(middleware::require_admin));

    // Authenticated user routes — any valid JWT.
    let user_routes = Router::new()
        .nest(
            "/products",
            reybi_api::modules::product::routes::protected_routes(),
        )
        .nest("/profile", reybi_api::modules::profile::routes::routes())
        .nest("/carts", reybi_api::modules::cart::routes::routes())
        .nest("/orders", reybi_api::modules::order::routes::user_routes())
        .nest(
            "/deposites",
            reybi_api::modules::deposite::routes::user_routes(),
        )
        .nest("/addresses", reybi_api::modules::address::routes::routes())
        .nest("/reviews", reybi_api::modules::review::routes::routes())
        .nest("/payments", reybi_api::modules::payment::routes::routes())
        .merge(admin_routes)
        .route_layer(mw::from_fn_with_state(state.clone(), middleware::jwt_auth))
        .layer(mw::from_fn(locale::locale_middleware));

    let api_routes = Router::new().merge(public_routes).merge(user_routes);

    // Static file serving — uploads/images served straight off disk by
    // `tower-http::services::ServeDir`.  Bypasses the router / middleware
    // stack entirely so no per-request work happens for static hits, and
    // serves a precompressed `.br`/`.gz` sibling when the client accepts it.
    let upload_dir = std::env::var("UPLOAD_DIR").unwrap_or_else(|_| "uploads".to_string());
    let uploads_route = Router::new().nest_service(
        "/uploads",
        ServeDir::new(&upload_dir)
            .precompressed_gzip()
            .precompressed_br(),
    );

    let app = Router::new()
        .merge(uploads_route)
        .nest("/v1", api_routes)
        .route_layer(TimeoutLayer::new(Duration::from_secs(30)))
        .route_layer(RequestBodyLimitLayer::new(5 * 1024 * 1024)) // 5 MB
        // ETag / 304 — buffers GET bodies, returns 304 on If-None-Match hit.
        // Placed inside compression so the hash is over the uncompressed body.
        .layer(mw::from_fn(middleware::etag::etag_middleware))
        // Security headers — cheap, applied once at the outermost layer
        .layer(SetResponseHeaderLayer::if_not_present(
            axum::http::header::X_CONTENT_TYPE_OPTIONS,
            axum::http::HeaderValue::from_static("nosniff"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            axum::http::header::X_FRAME_OPTIONS,
            axum::http::HeaderValue::from_static("DENY"),
        ))
        // Compression — Fastest quality minimizes CPU at the cost of ratio
        .layer(CompressionLayer::new().quality(tower_http::CompressionLevel::Fastest))
        .layer(CorsLayer::permissive())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(tracing::Level::INFO))
                .on_response(DefaultOnResponse::new().level(tracing::Level::INFO)),
        )
        .with_state(state.clone());

    let addr = format!("{}:{}", config.host, config.port);

    // Warm the hottest public caches in the background by hitting our own
    // HTTP endpoints once the listener is up.  Using real HTTP guarantees the
    // populated cache keys match exactly what live requests read, and exercises
    // the real handler path.  Best-effort — failures just leave caches cold.
    {
        let base = format!("http://127.0.0.1:{}", config.port);
        tokio::spawn(async move {
            // Give the server a beat to start accepting connections.
            tokio::time::sleep(Duration::from_millis(500)).await;
            let client = reqwest::Client::new();
            for path in ["/v1/banners", "/v1/articles", "/v1/products"] {
                let url = format!("{base}{path}");
                match client.get(&url).send().await {
                    Ok(r) => tracing::info!("cache warm {} -> {}", path, r.status()),
                    Err(e) => tracing::warn!("cache warm {} failed: {}", path, e),
                }
            }
            tracing::info!("✓ hot caches warmed");
        });
    }

    tracing::info!("Server starting on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
