use rand::prelude::*;
use std::collections::HashMap;

pub struct TokenStore {
    tokens: HashMap<usize, usize>,
}

impl TokenStore {
    /// create a new TokenStore
    pub fn new() -> TokenStore {
        TokenStore {
            tokens: HashMap::new(),
        }
    }

    /// generate a new token and return it
    pub fn new_token(&mut self, id: usize) -> Option<usize> {
        let old_tokens: Vec<usize> = self.tokens.clone().into_keys().collect();
        if old_tokens.len() >= usize::MAX {
            return None;
        }
        let mut rng = rand::thread_rng();
        let mut token: usize = rng.gen();
        while old_tokens.contains(&token) {
            token = rng.gen();
        }
        self.tokens.insert(token, id);
        Some(token)
    }

    /// check wether the tokenstore contains a specific token or not
    pub fn has_token(&self, token: &usize) -> bool {
        self.tokens.contains_key(token)
    }

    /// delete a token from the tokenstore. If it doesn't exist nothing happens
    pub fn delete_token(&mut self, token: &usize) {
        if let Some(_) = self.tokens.get(token) {
            let _ = self.tokens.remove(token);
        }
    }

    /// get the user id associated with a specific token
    pub fn get_id(&self, token: &usize) -> Option<&usize> {
        self.tokens.get(token)
    }
}
