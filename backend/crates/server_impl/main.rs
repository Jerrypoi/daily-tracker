#![allow(missing_docs)]

use axum::{
    Router,
    http::{HeaderValue, Method},
    middleware,
    routing::get,
};
use logging::init_logging;
use tower_http::cors::CorsLayer;

mod server_auth;
mod handler;
mod request_logger;
mod email;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    init_logging();

    let app = register_routes();
    let bind_addr = std::env::var("BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
    let listener = tokio::net::TcpListener::bind(&bind_addr).await.unwrap();
    println!("Server running on http://{}", bind_addr);
    axum::serve(listener, app).await.unwrap();
}

fn register_routes() -> Router {
    let api_routes = Router::new()
        .route("/topics", get(handler::get_topics).post(handler::create_topic))
        .route("/topics/:id", get(handler::get_topic_by_id).put(handler::update_topic))
        .route("/daily-tracks", get(handler::get_daily_tracks).post(handler::create_daily_track))
        .route(
            "/daily-tracks/:id",
            get(handler::get_daily_track_by_id)
                .put(handler::update_daily_track)
                .delete(handler::delete_daily_track),
        )
        .route_layer(middleware::from_fn(server_auth::auth_middleware));

    // API-key management is JWT-only: a programmatic caller must not be able to
    // mint or revoke other API keys using an API key.
    let api_key_routes = Router::new()
        .route(
            "/api-keys",
            get(handler::list_api_keys).post(handler::create_api_key),
        )
        .route("/api-keys/:id", axum::routing::delete(handler::revoke_api_key))
        .route_layer(middleware::from_fn(server_auth::jwt_only_middleware));

    let auth_routes = Router::new()
        .route("/register", axum::routing::post(handler::register))
        .route("/verify-email", axum::routing::post(handler::verify_email))
        .route("/login", axum::routing::post(handler::login));

    let allowed_origin = std::env::var("CORS_ORIGIN")
        .unwrap_or_else(|_| "http://localhost:5173".to_string());
    let cors = CorsLayer::new()
        .allow_origin(allowed_origin.parse::<HeaderValue>().expect("Invalid CORS_ORIGIN value"))
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([
            axum::http::header::AUTHORIZATION,
            axum::http::header::CONTENT_TYPE,
        ]);

    Router::new()
        .nest("/api/v1/auth", auth_routes)
        .nest("/api/v1", api_key_routes)
        .nest("/api/v1", api_routes)
        .layer(cors)
        .layer(middleware::from_fn(request_logger::log_request))
}
