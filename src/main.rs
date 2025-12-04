mod gpio;
use gpio::buzzer::buzz;
use gpio::fan::{fan_on, fan_off};
mod api;
use api::ping::ping;
use api::count::count;
use axum::{routing::{post, get}, Router};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::time::Instant;

type Users = Arc<Mutex<Vec<Instant>>>;

#[tokio::main]
async fn main() {
    let users: Users = Arc::new(Mutex::new(Vec::new()));

    let app = Router::new()
        .route("/gpio/buzz",   post(buzz))
        .route("/gpio/fan-on", post(fan_on))
        .route("/gpio/fan-off",post(fan_off))
        .route("/api/ping", post(ping))
        .route("/api/count", get(count))
        .with_state(users);

    let addr = "0.0.0.0:4000";
    println!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
