pub mod dao;
pub mod token; // Adicione esta linha para declarar o módulo token

/// Módulo para gerenciar exchanges e tokens na rede Stellar.

use stellar_sdk::Server; // Importa o módulo Server da biblioteca stellar_sdk.
use std::collections::HashMap; // Importa a estrutura HashMap da biblioteca padrão.
use std::error::Error; // Importa a trait Error para tratamento de erros.

#[derive(Clone)]
pub struct Exchange {
    token: String, // Campo para armazenar o token.
    factory: String, // Campo para armazenar a fábrica.
    server: Server, // Campo para armazenar o servidor.
}

impl Exchange {
    // Método para criar uma nova instância de Exchange.
    pub fn new(token: String, factory: String, server: Server) -> Self {
        Self { token, factory, server }
    }

    // Método para mostrar os detalhes da exchange.
    pub fn mostrar_detalhes(&self) {
        println!("Token: {}, Factory: {}, Server: {:?}", self.token, self.factory, self.server);
    }
}

pub struct DerivexFactory {
    server: Server, // Campo para armazenar o servidor.
    token_to_exchange: HashMap<String, Exchange>, // Mapa de tokens para exchanges.
    exchange_to_token: HashMap<Exchange, String>, // Mapa de exchanges para tokens.
}

impl DerivexFactory {
    // Método para criar uma nova instância de DerivexFactory.
    pub fn new(server: Server) -> Self {
        DerivexFactory {
            server,
            token_to_exchange: HashMap::new(),
            exchange_to_token: HashMap::new(),
        }
    }

    // Método para criar uma nova exchange.
    pub fn create_new_exchange(&mut self, token_address: String) -> Result<Exchange, Box<dyn Error>> {
        let exchange = Exchange::new(token_address.clone(), "Factory".to_string(), self.server.clone());
        self.token_to_exchange.insert(token_address.clone(), exchange.clone());
        self.exchange_to_token.insert(exchange.clone(), token_address);
        Ok(exchange)
    }

    // Método para obter uma exchange a partir de um endereço de token.
    pub fn get_exchange(&self, token_address: &str) -> Option<&Exchange> {
        self.token_to_exchange.get(token_address)
    }

    // Método para obter um token a partir de uma exchange.
    pub fn get_token(&self, exchange: &Exchange) -> Option<&String> {
        self.exchange_to_token.get(exchange)
    }
}