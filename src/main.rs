use axum::{routing::post, Router};
mod gpio;
use crate::gpio::buzzer::buzz;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/buzz",   post(buzz));

    let addr = "0.0.0.0:4000";
    println!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
