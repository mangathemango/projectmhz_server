use crate::Users;
use std::time::Instant;

pub async fn ping(state: axum::extract::State<Users>) {
    let mut users = state.lock().await;
    users.push(Instant::now());
}