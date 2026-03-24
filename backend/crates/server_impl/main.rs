#![allow(missing_docs)]

use axum::{
    Router,
    http::Method,
    middleware,
    routing::get,
};
use logging::init_logging;
use tower_http::cors::{Any, CorsLayer};

mod server_auth;
mod handler;
mod request_logger;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    init_logging();

    let app = register_routes();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("Server running on http://localhost:8080");
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

    let auth_routes = Router::new()
        .route("/register", axum::routing::post(handler::register))
        .route("/login", axum::routing::post(handler::login));

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers(Any);

    Router::new()
        .nest("/api/v1/auth", auth_routes)
        .nest("/api/v1", api_routes)
        .layer(cors)
        .layer(middleware::from_fn(request_logger::log_request))
}
