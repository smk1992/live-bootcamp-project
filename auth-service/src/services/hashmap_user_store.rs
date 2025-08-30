use crate::domain::data_stores::{
    UserStore, UserStoreError,
    UserStoreError::{InvalidCredentials, UserAlreadyExists, UserNotFound},
};
use crate::domain::user::User;
use std::collections::HashMap;


#[derive(Clone)]
pub struct HashMapUserStore {
    users: HashMap<String, User>,
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
        if user.email.trim().len() == 0 {
            return Err(InvalidCredentials);
        }
        if user.password.len() < 8 {
            return Err(InvalidCredentials);
        }
        if !user.email.contains("@") {
            return Err(InvalidCredentials);
        }
        if self.users.contains_key(&user.email) {
            return Err(UserAlreadyExists);
        }

        self.users.insert(user.email.clone(), user);
        Ok(())
    }

    async fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        self.users.get(email).cloned().ok_or(UserNotFound)
    }

    async fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        let user = self.get_user(email).await?;

        match user.password == password {
            true => Ok(()),
            false => Err(InvalidCredentials),
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
            email: "a@abc.com".to_string(),
            password: "password".to_string(),
            requires_2fa: true,
        };

        assert_eq!(store.add_user(user).await, Ok(()))
    }

    #[tokio::test]
    async fn test_add_user_already_exists() {
        let mut store = HashMapUserStore::new();

        let user = User::new("a@abc.com".to_string(), "password_a".to_string(), true);
        let user_two = User::new("a@abc.com".to_string(), "password".to_string(), true);

        assert_eq!(store.add_user(user).await, Ok(()));
        assert_eq!(store.add_user(user_two).await, Err(UserAlreadyExists))
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut store = HashMapUserStore::new();

        let email = "user@example.com".to_string();

        store
            .add_user(User::new(email.clone(), "password".to_string(), true))
            .await
            .expect("insert user failed");

        let user = store.get_user(&email).await.expect("Failed to find user");

        assert_eq!(user.email, email);
        assert_eq!(user.password, "password");
        assert_eq!(user.requires_2fa, true);
    }

    #[tokio::test]
    async fn test_user_not_found() {
        let store = HashMapUserStore::new();

        let result = store.get_user("user@example.com").await;
        assert!(matches!(result, Err(UserNotFound)));
    }

    #[tokio::test]
    async fn test_validate_user_not_found() {
        let store = HashMapUserStore::new();

        let result = store.validate_user("user@example.com", "password").await;
        assert!(matches!(result, Err(UserNotFound)));
    }

    #[tokio::test]
    async fn test_validate_incorrect_password() {
        let mut store = HashMapUserStore::new();

        store
            .add_user(User::new(
                "user@example.com".to_string(),
                "password".to_string(),
                true,
            ))
            .await
            .expect("Failed to insert user");

        let result = store.validate_user("other@example.com", "password").await;
        assert!(matches!(result, Err(UserNotFound)));
    }

    #[tokio::test]
    async fn test_validate_password() {
        let mut store = HashMapUserStore::new();

        store
            .add_user(User::new(
                "user@example.com".to_string(),
                "password".to_string(),
                true,
            ))
            .await
            .expect("Failed to insert user");

        let result = store.validate_user("user@example.com", "password").await;
        assert!(matches!(result, Ok(())));
    }
}
