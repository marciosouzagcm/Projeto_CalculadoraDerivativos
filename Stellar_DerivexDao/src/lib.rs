use std::hash::Hash;
use anyhow::Result;

/// Representa uma Exchange associada a um token específico.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Exchange {
    token: String,
    factory: String,
    server: String,
}

impl Exchange {
    pub fn new(token: String, factory: String, server: String) -> Self {
        Exchange { token, factory, server }
    }
}

impl std::fmt::Display for Exchange {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Exchange {{ token: {}, factory: {}, server: {} }}", self.token, self.factory, self.server)
    }
}


//Estrutura DerivexFactory


pub struct DerivexFactory {
    server: String,
    token_to_exchange: std::collections::HashMap<String, Exchange>,
    exchange_to_token: std::collections::HashMap<Exchange, String>,
}

impl DerivexFactory {
    pub fn new(server: String) -> Self {
        DerivexFactory {
            server,
            token_to_exchange: std::collections::HashMap::new(),
            exchange_to_token: std::collections::HashMap::new(),
        }
    }

    pub fn create_new_exchange(&mut self, token_address: &str) -> Result<Exchange, DerivexError> {
        if token_address.is_empty() {
            return Err(DerivexError::InvalidToken);
        }

        if self.token_to_exchange.contains_key(token_address) {
            return Err(DerivexError::DuplicateToken);
        }

        let exchange = Exchange::new(
            token_address.to_string(),
            "Factory".to_string(),
            self.server.clone(),
        );

        self.token_to_exchange
            .insert(token_address.to_string(), exchange.clone());
        self.exchange_to_token
            .insert(exchange.clone(), token_address.to_string());

        Ok(exchange)
    }

    pub fn get_exchange(&self, token_address: &str) -> Option<&Exchange> {
        self.token_to_exchange.get(token_address)
    }

    pub fn get_token(&self, exchange: &Exchange) -> Option<&String> {
        self.exchange_to_token.get(exchange)
    }

    pub fn remove_exchange(&mut self, token_address: &str) {
        if let Some(exchange) = self.token_to_exchange.remove(token_address) {
            self.exchange_to_token.remove(&exchange);
        }
    }
}


//Enum DerivexError


#[derive(Debug, thiserror::Error)]
pub enum DerivexError {
    #[error("Duplicate token address detected")]
    DuplicateToken,
    #[error("Invalid token address")]
    InvalidToken,
}


//Testes


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_exchange() {
        let mut factory = DerivexFactory::new("https://server.test".to_string());
        let exchange = factory.create_new_exchange("TOKEN123").unwrap();
        assert_eq!(exchange.token, "TOKEN123");
        assert_eq!(exchange.factory, "Factory");
        assert_eq!(exchange.server, "https://server.test");
    }

    #[test]
    fn test_duplicate_token() {
        let mut factory = DerivexFactory::new("https://server.test".to_string());
        factory.create_new_exchange("TOKEN123").unwrap();
        let result = factory.create_new_exchange("TOKEN123");
        assert!(result.is_err());
        assert_eq!(format!("{}", result.unwrap_err()), "Duplicate token address detected");
    }

    #[test]
    fn test_invalid_token() {
        let mut factory = DerivexFactory::new("https://server.test".to_string());
        let result = factory.create_new_exchange("");
        assert!(result.is_err());
        assert_eq!(format!("{}", result.unwrap_err()), "Invalid token address");
    }

    #[test]
    fn test_remove_exchange() {
        let mut factory = DerivexFactory::new("https://server.test".to_string());
        factory.create_new_exchange("TOKEN123").unwrap();
        factory.remove_exchange("TOKEN123");
        assert!(factory.get_exchange("TOKEN123").is_none());
    }
}