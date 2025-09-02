use crate::helpers::TestApp;
use auth_service::domain::data_stores::{LoginAttemptId, TwoFACode};
use auth_service::domain::Email;
use auth_service::utils::constants::JWT_COOKIE_NAME;
// #[tokio::test]
// async fn verify_2fa_is_successful() {
//     let app = TestApp::new().await;
//     let response = app.post_verify_2fa().await;
//
//     assert_eq!(response.status().as_u16(), 200);
// }

#[tokio::test]
async fn should_return_422_for_malformed_request() {
    let app = TestApp::new().await;

    let test_cases = [
        serde_json::json!({
            "email": "user@example.com",
        }),
        serde_json::json!({
            "loginAttemptId": "12311",
            "2FACode": "111111"
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_verify_2fa(test_case).await;
        assert_eq!(response.status().as_u16(), 422);
    }
}

#[tokio::test]
async fn should_return_400_for_invalid_inputs() {
    let app = TestApp::new().await;

    let test_cases = [
        serde_json::json!({
            "email": "user@example.com",
            "loginAttemptId": "12311",
            "2FACode": "111111"
        }),
        serde_json::json!({
            "email": "example.com",
            "loginAttemptId": "12311",
            "2FACode": "111111"
        }),
        serde_json::json!({
            "email": "example.com",
            "loginAttemptId": "",
            "2FACode": "111111"
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_verify_2fa(test_case).await;
        assert_eq!(response.status().as_u16(), 400);
    }
}

#[tokio::test]
async fn should_return_401_for_incorrect_credentials() {
    let app = TestApp::new().await;

    let login_attempt = LoginAttemptId::default();
    let code = TwoFACode::default();
    let email = Email::parse("user@example.com").expect("setup email");

    app.two_fa_code_store
        .write()
        .await
        .add_code(email.clone(), login_attempt.clone(), code.clone())
        .await
        .expect("to insert 2FA Code");

    let test_cases = [
        serde_json::json!({
            "email": "other_user@example.com",
            "loginAttemptId": login_attempt.as_ref().to_string(),
            "2FACode": code.as_ref().to_string(),
        }),
        serde_json::json!({
            "email": email.to_string(),
            "loginAttemptId": LoginAttemptId::default().as_ref().to_string(),
            "2FACode": code.as_ref().to_string(),
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_verify_2fa(test_case).await;
        assert_eq!(response.status().as_u16(), 401);
    }
}

#[tokio::test]
async fn should_return_200_for_correct_credentials() {
    let app = TestApp::new().await;

    let login_attempt = LoginAttemptId::default();
    let code = TwoFACode::default();
    let email = Email::parse("user@example.com").expect("setup email");

    app.two_fa_code_store
        .write()
        .await
        .add_code(email.clone(), login_attempt.clone(), code.clone())
        .await
        .expect("to insert 2FA Code");

    let test_case = serde_json::json!({
        "email": email.to_string(),
        "loginAttemptId": login_attempt.as_ref().to_string(),
        "2FACode": code.as_ref().to_string(),
    });

    let response = app.post_verify_2fa(&test_case).await;
    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());
}

#[tokio::test]
async fn should_return_401_for_calling_twice() {
    let app = TestApp::new().await;

    let login_attempt = LoginAttemptId::default();
    let code = TwoFACode::default();
    let email = Email::parse("user@example.com").expect("setup email");

    app.two_fa_code_store
        .write()
        .await
        .add_code(email.clone(), login_attempt.clone(), code.clone())
        .await
        .expect("to insert 2FA Code");

    let test_case = serde_json::json!({
        "email": email.to_string(),
        "loginAttemptId": login_attempt.as_ref().to_string(),
        "2FACode": code.as_ref().to_string(),
    });

    let response = app.post_verify_2fa(&test_case).await;
    assert_eq!(response.status().as_u16(), 200);

    // 2nd call
    let response = app.post_verify_2fa(&test_case).await;
    assert_eq!(response.status().as_u16(), 401);
}
