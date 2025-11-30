use axum::{routing::post, Router};
use std::process::Command;

async fn deploy() -> &'static str {
    println!("Pulling latest changes from GitHub...");
    
    let output = Command::new("git")
        .arg("pull")
        .output()
        .expect("failed to pull");

    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    "✅ Updated from GitHub"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/deploy", post(deploy));

    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    println!("Listening on {}", addr);
}
