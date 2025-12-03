use axum::{routing::post, Router};
mod gpio;
use crate::gpio::buzzer::buzz;
use crate::gpio::fan::{fan_on, fan_off};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("gpio/buzz",   post(buzz))
        .route("gpio/fan-on", post(fan_on))
        .route("gpio/fan-off",post(fan_off));

    let addr = "0.0.0.0:4000";
    println!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
