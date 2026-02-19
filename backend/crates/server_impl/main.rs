#![allow(missing_docs)]

use axum::{Router, middleware, routing::{get, post}};
use logging::init_logging;

mod server_auth;
mod handler;
mod request_logger;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    init_logging();

    let app = register_routes();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

fn register_routes() -> Router {
    let api_routes = Router::new()
        .route("/topics", get(handler::get_topics).post(handler::create_topic))
        .route("/topics/{id}", get(handler::get_topic_by_id))
        .route("/daily-tracks", get(handler::get_daily_tracks).post(handler::create_daily_track))
        .route("/daily-tracks/{id}", get(handler::get_daily_track_by_id));

    Router::new()
        .nest("/api/v1", api_routes)
        .layer(middleware::from_fn(request_logger::log_request))
}
