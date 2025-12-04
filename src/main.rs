mod gpio;
use gpio::buzzer::buzz;
use gpio::fan::{fan_off, fan_on};
mod api;
use api::count::count;
use api::ping::ping;
use axum::{
    Router,
    routing::{get, post},
};
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Mutex;
use http::header::HeaderValue;
use tower_http::cors::{Any, CorsLayer};

type Users = Arc<Mutex<Vec<Instant>>>;

#[tokio::main]
async fn main() {
    let users: Users = Arc::new(Mutex::new(Vec::new()));
    let cors = CorsLayer::new()
        .allow_origin(
            "https://mangathemango.github.io"
                .parse::<HeaderValue>()
                .unwrap(),
        )
        .allow_methods(Any)
        .allow_headers(Any);
        
    let app = Router::new()
        .route("/gpio/buzz", post(buzz))
        .route("/gpio/fan-on", post(fan_on))
        .route("/gpio/fan-off", post(fan_off))
        .route("/api/ping", post(ping))
        .route("/api/count", get(count))
        .with_state(users)
        .layer(cors);

    let addr = "0.0.0.0:4000";
    println!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
