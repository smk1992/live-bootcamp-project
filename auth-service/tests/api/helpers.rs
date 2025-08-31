use std::sync::Arc;
use reqwest::cookie::Jar;
use auth_service::{AppState, Application, HashMapUserStore};
use uuid::Uuid;
use auth_service::utils::constants::test;

pub struct TestApp {
    pub address: String,
    pub cookie_jar: Arc<Jar>,
    pub http_client: reqwest::Client,
}

impl TestApp {
    pub async fn new() -> Self {
        let app_state = AppState::new(HashMapUserStore::new());
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
        let http_client = reqwest::Client::builder().cookie_provider(cookie_jar.clone()).build().unwrap();

        // Create new `TestApp` instance and return it
        TestApp {
            cookie_jar,
            http_client,
            address,
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

    pub async fn post_verify_2fa(&self) -> reqwest::Response {
        self.http_client
            .post(self.url("/verify-2fa"))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_verify_token<Body: serde::Serialize>(&self, body: &Body) -> reqwest::Response {
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
