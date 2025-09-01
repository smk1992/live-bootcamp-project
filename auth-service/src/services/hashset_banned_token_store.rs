use crate::domain::data_stores::{BannedTokenStore};
use std::collections::HashSet;

#[derive(Default)]
pub struct HashSetBannedTokenStore {
    pub store: HashSet<String>,
}

#[async_trait::async_trait]
impl BannedTokenStore for HashSetBannedTokenStore {
    async fn add(&mut self, token: String) -> () {
        self.store.insert(token);
    }

    async fn contains(&self, token: &str) -> bool {
        self.store.contains(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add() {
        let mut store = HashSetBannedTokenStore::default();
        store.add("some_token".to_string()).await
    }
}
