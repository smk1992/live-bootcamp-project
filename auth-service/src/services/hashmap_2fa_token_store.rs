use crate::domain::data_stores::{LoginAttemptId, TwoFACode, TwoFACodeStore, TwoFACodeStoreError};
use crate::domain::Email;
use std::collections::HashMap;

#[derive(Default)]
pub struct HashMap2FaTokenStore {
    codes: HashMap<Email, (LoginAttemptId, TwoFACode)>,
}

impl HashMap2FaTokenStore {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait::async_trait]
impl TwoFACodeStore for HashMap2FaTokenStore {
    async fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError> {
        self.codes.insert(email, (login_attempt_id, code));

        Ok(())
    }

    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError> {
        self.codes.remove(email);

        Ok(())
    }

    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError> {
        self.codes
            .get(email)
            .and_then(|v| Some(v.clone()))
            .ok_or(TwoFACodeStoreError::LoginAttemptIdNotFound)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_code() {
        let email = Email::parse("user@example.com").unwrap();
        let mut store = HashMap2FaTokenStore::default();
        store
            .add_code(
                email.clone(),
                LoginAttemptId::default(),
                TwoFACode::default(),
            )
            .await
            .unwrap();

        assert_eq!(store.codes.contains_key(&email), true);
    }

    #[tokio::test]
    async fn test_remove_code() {
        let email = Email::parse("user@example.com").unwrap();
        let mut store = HashMap2FaTokenStore::default();

        assert!(store
            .codes
            .insert(
                email.clone(),
                (LoginAttemptId::default(), TwoFACode::default()),
            ).is_none());

        assert!(store.remove_code(&email).await.is_ok());

        assert_eq!(store.codes.contains_key(&email), false);
    }

    #[tokio::test]
    async fn test_get_code_missing() {
        let email = Email::parse("user@example.com").unwrap();
        let store = HashMap2FaTokenStore::default();

        assert!(matches!(
            store.get_code(&email).await,
            Err(TwoFACodeStoreError::LoginAttemptIdNotFound)
        ));
    }

    #[tokio::test]
    async fn test_get_code_return_login_details() {
        let email = Email::parse("user@example.com").unwrap();
        let mut store = HashMap2FaTokenStore::default();

        let login_attempt = LoginAttemptId::default();
        let two_fa_code = TwoFACode::default();

        assert!(store
            .codes
            .insert(
                email.clone(),
                (login_attempt.clone(), two_fa_code.clone()),
            ).is_none());

        assert_eq!(store.get_code(&email).await, Ok((login_attempt, two_fa_code)));
    }
}
