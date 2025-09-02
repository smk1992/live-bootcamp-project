use auth_service::utils::constants::prod;
use auth_service::{
    AppState, Application, HashMap2FaTokenStore, HashMapUserStore, HashSetBannedTokenStore,
    MockEmailClient,
};
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let banned_user_store = Arc::new(RwLock::new(HashSetBannedTokenStore::default()));
    let user_store = Arc::new(RwLock::new(HashMapUserStore::new()));
    let two_fa_code_store = Arc::new(RwLock::new(HashMap2FaTokenStore::new()));
    let mock_email_client = Arc::new(MockEmailClient {});

    let app_state = AppState::new(
        user_store,
        banned_user_store,
        two_fa_code_store,
        mock_email_client,
    );

    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}
