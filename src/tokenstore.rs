use std::collections::HashMap;
use rand::prelude::*;

pub struct TokenStore {
    tokens: HashMap<u64, u64>,
}

impl TokenStore {
    /// create a new TokenStore
    pub fn new() -> TokenStore {
        TokenStore {
            tokens: HashMap::new(),
        }
    }

    /// generate a new token and return it
    pub fn new_token(&mut self, id: u64) -> u64 {
	let mut rng = rand::thread_rng();
	let mut token: u64 = rng.gen();
	let old_tokens: Vec<u64> = self.tokens.clone().into_keys().collect();
	while old_tokens.contains(&token) {
	    token = rng.gen();
	}
	self.tokens.insert(token, id);
	token
    }
}
