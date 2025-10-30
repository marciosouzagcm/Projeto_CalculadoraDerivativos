/// Módulo para gerenciar exchanges e tokens na rede Stellar.

use stellar_rust_sdk::{Exchange, Server}; // Importa os módulos Exchange e Server da biblioteca stellar_rust_sdk.
use std::collections::HashMap; // Importa a estrutura HashMap da biblioteca padrão.
use std::error::Error; // Importa a trait Error para tratamento de erros.

/// Definição da classe DerivexFactory.
///
/// A classe DerivexFactory é responsável por gerenciar exchanges e tokens na rede Stellar.
pub struct DerivexFactory {
    /// Servidor Stellar.
    server: Server,
    /// Mapeamento de tokens para exchanges.
    token_to_exchange: HashMap<String, Exchange>,
    /// Mapeamento de exchanges para tokens.
    exchange_to_token: HashMap<Exchange, String>,
    /// Mapeamento de IDs para tokens.
    id_to_token: HashMap<u64, String>,
}

impl DerivexFactory {
    /// Cria uma nova instância de DerivexFactory.
    ///
    /// # Parâmetros
    /// * `server`: Servidor Stellar.
    ///
    /// # Retorno
    /// * `DerivexFactory`: Nova instância de DerivexFactory.
    pub fn new(server: Server) -> Self {
        DerivexFactory {
            server,
            token_to_exchange: HashMap::new(),
            exchange_to_token: HashMap::new(),
            id_to_token: HashMap::new(),
        }
    }

    /// Cria uma nova exchange associada ao token.
    ///
    /// # Parâmetros
    /// * `token_address`: Endereço do token.
    ///
    /// # Retorno
    /// * `Result<Exchange, Box<dyn Error>>`: Nova exchange criada ou erro.
    pub fn create_new_exchange(&mut self, token_address: String) -> Result<Exchange, Box<dyn Error>> {
        // Validação de dados.
        if token_address.is_empty() {
            return Err("Endereço de token inválido".into());
        }
        
        // Criação da exchange.
        let exchange = Exchange::new(&self.server, &token_address)?;
        
        // Armazenamento da exchange.
        self.token_to_exchange.insert(token_address.clone(), exchange.clone());
        self.exchange_to_token.insert(exchange.clone(), token_address.clone());
        
        Ok(exchange)
    }

    /// Consulta a exchange associada a um token.
    ///
    /// # Parâmetros
    /// * `token_address`: Endereço do token.
    ///
    /// # Retorno
    /// * `Option<&Exchange>`: Exchange associada ao token, se existir.
    pub fn get_exchange(&self, token_address: &str) -> Option<&Exchange> {
        self.token_to_exchange.get(token_address)
    }

    /// Consulta o token associado a uma exchange.
    ///
    /// # Parâmetros
    /// * `exchange`: Exchange.
    ///
    /// # Retorno
    /// * `Option<&String>`: Token associado à exchange, se existir.
    pub fn get_token(&self, exchange: &Exchange) -> Option<&String> {
        self.exchange_to_token.get(exchange)
    }

    /// Consulta o token associado a um ID específico.
    ///
    /// # Parâmetros
    /// * `token_id`: ID do token.
    ///
    /// # Retorno
    /// * `Option<&String>`: Token associado ao ID, se existir.
    pub fn get_token_with_id(&self, token_id: u64) -> Option<&String> {
        self.id_to_token.get(&token_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use stellar_rust_sdk::Server;

    #[test]
    fn test_create_new_exchange() {
        let server = Server::new("horizon.stellar.org");
        let mut factory = DerivexFactory::new(server);
        assert!(factory.create_new_exchange("TokenAddress".to_string()).is_ok());
    }

    #[test]
    fn test_get_exchange() {
        let server = Server::new("horizon.stellar.org");
        let mut factory = DerivexFactory::new(server);
        let exchange = factory.create_new_exchange("TokenAddress".to_string()).unwrap();
        assert!(factory.get_exchange("TokenAddress").is_some());
    }

    #[test]
    fn test_get_token() {
        let server = Server::new("horizon.stellar.org");
        let mut factory = DerivexFactory::new(server);
        let exchange = factory.create_new_exchange("TokenAddress".to_string()).unwrap();
        assert!(factory.get_token(&exchange).is_some());
    }

    #[test]
    fn test_get_token_with_id() {
        let server = Server::new("horizon.stellar.org");
        let mut factory = DerivexFactory::new(server);
        let exchange = factory.create_new_exchange("TokenAddress".to_string()).unwrap();
        factory.id_to_token.insert(1, "TokenAddress".to_string());

        assert_eq!(factory.get_token_with_id(1).unwrap(), "TokenAddress");
    }

    #[test]
    fn test_create_new_exchange_error() {
        let server = Server::new("horizon.stellar.org");
        let mut factory = DerivexFactory::new(server);

        assert!(factory.create_new_exchange(String::new()).is_err());
    }

    #[test]
    fn test_get_exchange_error() {
        let server = Server::new("horizon.stellar.org");
        let factory = DerivexFactory::new(server);

        assert!(factory.get_exchange("TokenAddress").is_none());
    }

    #[test]
    fn test_get_token_error() {
        let server = Server::new("horizon.stellar.org");
        let factory = DerivexFactory::new(server);
        let exchange = Exchange::new(&server, "TokenAddress").unwrap();

        assert!(factory.get_token(&exchange).is_none());
    }

    #[test]
    fn test_get_token_with_id_error() {
        let server = Server::new("horizon.stellar.org");
        let factory = DerivexFactory::new(server);

        assert!(factory.get_token_with_id(1).is_none());
    }
}