use eframe::egui;
use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};
use thiserror::Error;
use anyhow::Result;

#[derive(Clone, PartialEq, Eq)]
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

impl Hash for Exchange {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.token.hash(state);
        self.factory.hash(state);
        self.server.hash(state);
    }
}

impl fmt::Display for Exchange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Exchange {{ token: {}, factory: {}, server: {} }}",
            self.token, self.factory, self.server
        )
    }
}

pub struct DerivexFactory {
    server: String,
    token_to_exchange: HashMap<String, Exchange>,
    exchange_to_token: HashMap<Exchange, String>,
}

impl Default for DerivexFactory {
    fn default() -> Self {
        DerivexFactory {
            server: String::new(),
            token_to_exchange: HashMap::new(),
            exchange_to_token: HashMap::new(),
        }
    }
}

impl DerivexFactory {
    pub fn new(server: String) -> Self {
        DerivexFactory {
            server,
            token_to_exchange: HashMap::new(),
            exchange_to_token: HashMap::new(),
        }
    }

    pub fn create_new_exchange(&mut self, token_address: &str) -> Result<Exchange, DerivexError> {
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

#[derive(Debug, Error)]
pub enum DerivexError {
    #[error("Duplicate token address detected")]
    DuplicateToken,
    #[error("Invalid token address")]
    InvalidToken,
}

#[derive(Default)]
struct MyApp {
    server: String,
    token_address: String,
    result: Option<Result<Exchange, DerivexError>>,
    factory: DerivexFactory,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Derivex Factory");
            ui.horizontal(|ui| {
                ui.label("Server:");
                ui.text_edit_singleline(&mut self.server);
            });
            ui.horizontal(|ui| {
                ui.label("Token Address:");
                ui.text_edit_singleline(&mut self.token_address);
            });
            if ui.button("Criar Exchange").clicked() {
                self.factory = DerivexFactory::new(self.server.clone());
                self.result = Some(self.factory.create_new_exchange(&self.token_address));
            }
            if let Some(result) = &self.result {
                match result {
                    Ok(exchange) => {
                        ui.label(format!("Exchange criada: {}", exchange));
                    }
                    Err(e) => {
                        ui.label(format!("Erro: {}", e));
                    }
                }
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "Derivex Factory",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(MyApp::default()))))
}
