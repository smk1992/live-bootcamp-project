use crate::domain::data_stores::{
    UserStore, UserStoreError,
    UserStoreError::{IncorrectCredentials, UserAlreadyExists, UserNotFound},
};
use crate::domain::user::User;
use crate::domain::{Email, Password};
use std::collections::HashMap;

#[derive(Clone)]
pub struct HashMapUserStore {
    users: HashMap<Email, User>,
}

impl HashMapUserStore {
    pub fn new() -> Self {
        HashMapUserStore {
            users: HashMap::new(),
        }
    }
}

#[async_trait::async_trait]
impl UserStore for HashMapUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        if self.users.contains_key(&user.email) {
            return Err(UserAlreadyExists);
        }
        self.users.insert(user.email.clone(), user);

        Ok(())
    }

    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        self.users.get(email).cloned().ok_or(UserNotFound)
    }

    async fn validate_user(
        &self,
        email: &Email,
        password: &Password,
    ) -> Result<(), UserStoreError> {
        let user = self.get_user(email).await?;

        match &user.password == password {
            true => Ok(()),
            false => Err(IncorrectCredentials),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let mut store = HashMapUserStore::new();

        let user = User {
            email: Email::parse("a@abc.com").unwrap(),
            password: Password::parse("password").unwrap(),
            requires_2fa: true,
        };

        assert_eq!(store.add_user(user).await, Ok(()))
    }

    #[tokio::test]
    async fn test_add_user_already_exists() {
        let mut store = HashMapUserStore::new();

        let user = User::new(
            Email::parse("a@abc.com").unwrap(),
            Password::parse("password_a").unwrap(),
            true,
        );
        let user_two = User::new(
            Email::parse("a@abc.com").unwrap(),
            Password::parse("password").unwrap(),
            true,
        );

        assert_eq!(store.add_user(user).await, Ok(()));
        assert_eq!(store.add_user(user_two).await, Err(UserAlreadyExists))
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut store = HashMapUserStore::new();

        let email = Email::parse("user@example.com").unwrap();
        let password = Password::parse("password").unwrap();

        store
            .add_user(User::new(
                email.clone(),
                password.clone(),
                true,
            ))
            .await
            .expect("insert user failed");

        let user = store.get_user(&email).await.expect("Failed to find user");

        assert_eq!(user.email, email);
        assert_eq!(user.password, password);
        assert_eq!(user.requires_2fa, true);
    }

    #[tokio::test]
    async fn test_user_not_found() {
        let store = HashMapUserStore::new();
        let email = Email::parse("user@example.com").unwrap();

        let result = store.get_user(&email).await;
        assert!(matches!(result, Err(UserNotFound)));
    }

    #[tokio::test]
    async fn test_validate_user_not_found() {
        let store = HashMapUserStore::new();

        let email = Email::parse("user@example.com").unwrap();
        let password = Password::parse("password").unwrap();


        let result = store.validate_user(&email, &password).await;
        assert!(matches!(result, Err(UserNotFound)));
    }

    #[tokio::test]
    async fn test_validate_incorrect_password() {
        let mut store = HashMapUserStore::new();

        let email = Email::parse("user@example.com").unwrap();
        let password = Password::parse("password").unwrap();

        store
            .add_user(User::new(email.clone(), password, true))
            .await
            .expect("Failed to insert user");

        let result = store
            .validate_user(&email, &Password::parse("password111").unwrap())
            .await;
        assert!(matches!(result, Err(_)));
    }

    #[tokio::test]
    async fn test_validate_password() {
        let mut store = HashMapUserStore::new();

        let email = Email::parse("user@example.com").unwrap();
        let password = Password::parse("password").unwrap();

        store
            .add_user(User::new(email.clone(), password.clone(), true))
            .await
            .expect("Failed to insert user");

        let result = store.validate_user(&email, &password).await;
        assert!(matches!(result, Ok(())));
    }
}
