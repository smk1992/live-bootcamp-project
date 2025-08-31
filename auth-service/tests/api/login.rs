use auth_service::utils::constants::JWT_COOKIE_NAME;
use crate::helpers::{get_random_email, TestApp};

#[tokio::test]
async fn should_return_200_if_valid_credentials_and_2fa_disabled() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": false
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());
}

#[tokio::test]
async fn login_is_malformed() {
    let app = TestApp::new().await;

    let test_cases = [
        serde_json::json!({
            "email": "user@example.com",
        }),
        serde_json::json!({
            "password": ""
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_login(test_case).await;
        assert_eq!(response.status().as_u16(), 422);
    }
}

#[tokio::test]
async fn should_return_400_if_invalid_credentials() {
    let app = TestApp::new().await;

    let test_cases = [
        serde_json::json!({
            "email": "user",
            "password": "password"
        }),
        serde_json::json!({
            "email": "user@example.com",
            "password": ""
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_login(test_case).await;
        assert_eq!(response.status().as_u16(), 400);
    }
}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    let app = TestApp::new().await;

    let sign_up_response = app
        .post_signup(&serde_json::json!({
            "email": "user@example.com",
            "password": "password",
            "requires2FA": true
        }))
        .await;
    assert_eq!(sign_up_response.status().as_u16(), 201);

    let test_cases = [
        // wrong password
        serde_json::json!({
            "email": "user@example.com",
            "password": "wrong_password"
        }),
        // user does not exist
        serde_json::json!({
            "email": "other_user@example.com",
            "password": "password"
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_login(test_case).await;
        assert_eq!(response.status().as_u16(), 401);
    }
}
