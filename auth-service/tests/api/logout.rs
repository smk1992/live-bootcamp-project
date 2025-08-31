use crate::helpers::TestApp;
use auth_service::{utils::constants::JWT_COOKIE_NAME};
use reqwest::Url;

#[tokio::test]
async fn should_return_400_if_jwt_cookie_missing() {
    let app = TestApp::new().await;

    let response = app.post_logout().await;
    assert_eq!(response.status().as_u16(), 400);
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;

    app.cookie_jar.add_cookie_str(
        &format!(
            "{}=invalid; HttpOnly; SameSite=Lax; Secure; Path=/",
            JWT_COOKIE_NAME
        ),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );

    let response = app.post_logout().await;
    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn should_return_200_if_valid_jwt_cookie() {
    let app = TestApp::new().await;

    let email = "user@example.com";
    let password = "password";

    let response = app.post_signup(&serde_json::json!({
        "email": email,
        "password": password,
        "requires2FA": false
    })).await;
    assert_eq!(response.status().as_u16(), 201);

    let response = app.post_login(&serde_json::json!({
        "email": email,
        "password": password,
    })).await;
    assert_eq!(response.status().as_u16(), 200);

    let response = app.post_logout().await;
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn should_return_400_if_logout_called_twice_in_a_row() {
    let app = TestApp::new().await;

    let email = "user@example.com";
    let password = "password";

    // sign up
    let response = app.post_signup(&serde_json::json!({
        "email": email,
        "password": password,
        "requires2FA": false
    })).await;
    assert_eq!(response.status().as_u16(), 201);

    // login
    let response = app.post_login(&serde_json::json!({
        "email": email,
        "password": password,
    })).await;
    assert_eq!(response.status().as_u16(), 200);

    // logout
    let response = app.post_logout().await;
    assert_eq!(response.status().as_u16(), 200);

    // logout again
    let response = app.post_logout().await;
    assert_eq!(response.status().as_u16(), 400);
}
