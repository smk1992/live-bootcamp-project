use auth_service::{AppState, Application, HashMapUserStore};
use auth_service::utils::constants::prod;

#[tokio::main]
async fn main() {
    let app_state = AppState::new(HashMapUserStore::new());

    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}
