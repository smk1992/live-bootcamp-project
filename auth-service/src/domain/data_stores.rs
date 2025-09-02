use rand::distr::{Alphanumeric, SampleString};
use uuid::Uuid;
use crate::domain::{Email, Password};
use crate::domain::user::User;

#[async_trait::async_trait]
pub trait UserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError>;
    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError>;
    async fn validate_user(&self, email: &Email, password: &Password) -> Result<User, UserStoreError>;
}

#[async_trait::async_trait]
pub trait BannedTokenStore {
    async fn add(&mut self, token: String) -> ();
    async fn contains(&self, token: &str) -> bool;
}

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    IncorrectCredentials,
    UnexpectedError,
}

// This trait represents the interface all concrete 2FA code stores should implement
#[async_trait::async_trait]
pub trait TwoFACodeStore {
    async fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError>;
    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError>;
    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError>;
}

#[derive(Debug, PartialEq)]
pub enum TwoFACodeStoreError {
    LoginAttemptIdNotFound,
    UnexpectedError,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoginAttemptId(String);

impl LoginAttemptId {
    pub fn parse(id: String) -> Result<Self, String> {
        match Uuid::parse_str(&id) {
            Err(err) => Err(err.to_string()),
            Ok(_) => Ok(LoginAttemptId(id)),
        }
    }
}

impl Default for LoginAttemptId {
    fn default() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}

impl AsRef<str> for LoginAttemptId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TwoFACode(String);

impl TwoFACode {
    pub fn parse(code: String) -> Result<Self, String> {
        if code.trim().len() != 6 {
            return Err("Invalid length".to_string())
        }

        Ok(Self(code))
    }
}

impl Default for TwoFACode {
    fn default() -> Self {
        let code = Alphanumeric.sample_string(&mut rand::rng(), 6);
        Self(code)
    }
}

impl AsRef<str> for TwoFACode {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
