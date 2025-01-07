pub struct TokenManager {
    tokens: Vec<i32>, // Owned tokens
}

impl TokenManager {
    /// Create a new TokenManager
    pub fn new() -> Self {
        TokenManager {
            tokens: Vec::new(),
        }
    }

    /// Extend the vector with new tokens
    pub fn extend_tokens(&mut self, new_tokens: &[i32]) {
        self.tokens.extend_from_slice(new_tokens);
    }

    /// Provide a temporary slice of the tokens
    pub fn get_tokens_slice(&self) -> &[i32] {
        &self.tokens
    }
}
