use eframe::egui;
use stellar_base::crypto::KeyPair;
use stellar_base::xdr::{CreateAccountOp, Int64, AccountId, PublicKey, Uint256};
use thiserror::Error;

pub struct CreateAccountOptions {
    pub secret: String,
    pub starting_balance: String,
}

#[derive(Debug, Error)]
pub enum CreateAccountError {
    #[error("Chave secreta inválida: {0}")]
    InvalidSecretKey(String),
    #[error("Erro geral: {0}")]
    GeneralError(String),
}

pub fn create_account(opts: CreateAccountOptions) -> Result<CreateAccountOp, CreateAccountError> {
    let keypair = KeyPair::from_secret_seed(&opts.secret)
        .map_err(|_| CreateAccountError::InvalidSecretKey(opts.secret.clone()))?;
    let starting_balance = opts.starting_balance.parse::<i64>()
        .map_err(|_| CreateAccountError::GeneralError("Saldo inválido".to_string()))?;
    let public_key_bytes: Uint256 = stellar_base::xdr::Uint256 {
        value: keypair.public_key().as_bytes().to_owned(),
    };
    let operation = CreateAccountOp {
        destination: AccountId {
            value: PublicKey::PublicKeyTypeEd25519(public_key_bytes),
        },
        starting_balance: Int64 { value: starting_balance },
    };
    Ok(operation)
}

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "Criar Conta Stellar",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

#[derive(Default)]
struct MyApp {
    secret: String,
    starting_balance: String,
    result: Option<Result<CreateAccountOp, CreateAccountError>>,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Criar Conta Stellar");
            ui.horizontal(|ui| {
                ui.label("Chave Secreta:");
                ui.text_edit_singleline(&mut self.secret);
            });
            ui.horizontal(|ui| {
                ui.label("Saldo Inicial:");
                ui.text_edit_singleline(&mut self.starting_balance);
            });
            if ui.button("Criar Conta").clicked() {
                let opts = CreateAccountOptions {
                    secret: self.secret.clone(),
                    starting_balance: self.starting_balance.clone(),
                };
                self.result = Some(create_account(opts));
            }
            if let Some(result) = &self.result {
                match result {
                    Ok(operation) => {
                        ui.label(format!("Operação criada com sucesso: {:?}", operation));
                    }
                    Err(e) => {
                        ui.label(format!("Erro: {:?}", e));
                    }
                }
            }
        });
    }
}