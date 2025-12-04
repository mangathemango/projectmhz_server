use crate::Users;

pub async fn count(state: axum::extract::State<Users>) -> String {
    let mut users = state.lock().await;

    // Remove users older than 20s
    users.retain(|t| t.elapsed().as_secs() < 20);

    users.len().to_string()
}