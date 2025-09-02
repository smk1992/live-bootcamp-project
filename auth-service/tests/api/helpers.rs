use auth_service::utils::constants::test;
use auth_service::{AppState, Application, BannedStoreType, HashMap2FaTokenStore, HashMapUserStore, HashSetBannedTokenStore, MockEmailClient, TwoFACodeStoreType};
use reqwest::cookie::Jar;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub cookie_jar: Arc<Jar>,
    pub banned_token_store: BannedStoreType,
    pub two_fa_code_store: TwoFACodeStoreType,
    pub http_client: reqwest::Client,
}

impl TestApp {
    pub async fn new() -> Self {
        let banned_token_store = Arc::new(RwLock::new(HashSetBannedTokenStore::default()));
        let user_store = Arc::new(RwLock::new(HashMapUserStore::new()));
        let two_fa_code_store = Arc::new(RwLock::new(HashMap2FaTokenStore::new()));
        let mock_email_client = Arc::new(MockEmailClient);


        let app_state = AppState::new(
            user_store,
            banned_token_store.clone(),
            two_fa_code_store.clone(),
            mock_email_client
        );

        let app = Application::build(app_state, test::APP_ADDRESS)
            .await
            .expect("Failed to build app");

        let address = format!("http://{}", app.address.clone());

        // Run the auth service in a separate async task
        // to avoid blocking the main test thread.
        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        let cookie_jar = Arc::new(Jar::default());
        // Create a Reqwest http client instance
        let http_client = reqwest::Client::builder()
            .cookie_provider(cookie_jar.clone())
            .build()
            .unwrap();

        // Create new `TestApp` instance and return it
        TestApp {
            cookie_jar,
            http_client,
            address,
            banned_token_store,
            two_fa_code_store,
        }
    }

    pub async fn get_root(&self) -> reqwest::Response {
        self.http_client
            .get(&format!("{}/", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    // TODO: Implement helper functions for all other routes (signup, login, logout, verify-2fa, and verify-token)
    pub async fn post_signup<Body: serde::Serialize>(&self, body: &Body) -> reqwest::Response {
        self.http_client
            .post(self.url("/signup"))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_login<Body: serde::Serialize>(&self, body: &Body) -> reqwest::Response {
        self.http_client
            .post(self.url("/login"))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_logout(&self) -> reqwest::Response {
        self.http_client
            .post(self.url("/logout"))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_verify_2fa<Body: serde::Serialize>(&self, body: &Body) -> reqwest::Response {
        self.http_client
            .post(self.url("/verify-2fa"))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_verify_token<Body: serde::Serialize>(
        &self,
        body: &Body,
    ) -> reqwest::Response {
        self.http_client
            .post(self.url("/verify-token"))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    fn url(&self, path: &str) -> String {
        self.address.to_string() + path
    }
}

pub fn get_random_email() -> String {
    format!("{}@example.com", Uuid::new_v4())
}
