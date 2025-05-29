use axum::{
    routing::{get, post},
    Router,
};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;
use tower_http::set_header::SetResponseHeaderLayer;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use axum::http::HeaderValue;
use prometheus::{Encoder, TextEncoder, Counter, Histogram, register_counter, register_histogram};
use once_cell::sync::Lazy;
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};

mod config;
mod models;
mod handlers;
mod services;
mod middleware;
mod error;

use services::{cache::CacheService, db::DbService};

static REQUEST_COUNT: Lazy<Counter> = Lazy::new(|| {
    register_counter!("link_shortener_requests_total", "Total number of requests").unwrap()
});

static RESPONSE_TIME: Lazy<Histogram> = Lazy::new(|| {
    register_histogram!("link_shortener_response_time_seconds", "Response time in seconds").unwrap()
});

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DbService>,
    pub cache: Arc<Mutex<CacheService>>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let config = &config::CONFIG;

    let pool = PgPoolOptions::new()
        .max_connections(100)
        .min_connections(10)
        .max_lifetime(Duration::from_secs(1800))
        .idle_timeout(Duration::from_secs(600))
        .connect(&config.database_url)
        .await
        .expect("Failed to create pool");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate database");

    let cache = CacheService::new(&config.redis_url)
        .await
        .expect("Failed to connect to Redis");

    let db_service = Arc::new(DbService::new(pool));
    let cache_service = Arc::new(Mutex::new(cache));

    let app_state = AppState {
        db: db_service,
        cache: cache_service,
    };

    let governor_conf = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(2) 
            .burst_size(100)
            .finish()
            .unwrap(),
    );

    let app = Router::new()
        .route("/", post(handlers::shorten::create_link))
        .route("/{code}", get(handlers::redirect::redirect))
        .route("/health", get(handlers::health::health_check))
        .route("/metrics", get(metrics_handler))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive())
                .layer(CompressionLayer::new())
                .layer(SetResponseHeaderLayer::if_not_present(
                    axum::http::header::CACHE_CONTROL,
                    HeaderValue::from_static("public, max-age=3600")
                ))
                .layer(SetResponseHeaderLayer::overriding(
                    axum::http::header::SERVER,
                    HeaderValue::from_static("link-shortener/1.0")
                ))
        )
        .layer(GovernorLayer {
            config: governor_conf,
        })
        .with_state(app_state);

    let addr: SocketAddr = format!("{}:{}", config.server_host, config.server_port)
        .parse()
        .expect("Failed to parse socket address");
    
    println!("Server starting on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");
        
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .expect("Failed to start server");
}

async fn metrics_handler() -> String {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = vec![];
    encoder.encode(&metric_families, &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
} 