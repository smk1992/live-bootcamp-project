use std::sync::Arc;
use tokio::sync::RwLock;
use auth_service::{AppState, Application, HashMapUserStore, HashSetBannedTokenStore};
use auth_service::utils::constants::prod;

#[tokio::main]
async fn main() {
    let banned_user_store = Arc::new(RwLock::new(HashSetBannedTokenStore::default()));
    let app_state = AppState::new(HashMapUserStore::new(), banned_user_store);

    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}
