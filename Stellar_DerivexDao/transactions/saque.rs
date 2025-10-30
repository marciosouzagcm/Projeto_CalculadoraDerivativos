use stellar_base::{
    asset::Asset, 
    crypto::KeyPair, 
    xdr::{AccountId, Int64, Operation, OperationBody, PaymentOp},
};
use strkey::StrKey;
use thiserror::Error;

/// Estrutura de Opções de Saque

/// Opções para realizar um saque.
#[derive(Debug)]
pub struct SaqueOptions {
    /// Chave pública de destino.
    pub destino: String,
    /// Quantidade a ser sacada.
    pub amount: String,
    /// Ativo a ser sacado.
    pub asset: Asset,
    /// Chave pública da conta fonte (opcional).
    pub source: Option<String>,
}

/// Erros de Saque

/// Erros possíveis durante o saque.
#[derive(Debug, Error)]
pub enum SaqueError {
    /// Destino inválido.
    #[error("Destino inválido")]
    InvalidDestination,
    /// Quantia inválida.
    #[error("Quantia inválida")]
    InvalidAmount,
    /// Erro interno do Stellar.
    #[error("Erro interno do Stellar: {0}")]
    StellarError(#[from] stellar_base::error::Error),
}

/// Função de Saque

/// Cria uma operação de saque.
///
/// # Parâmetros
///
/// * `opts`: Opções de saque.
///
/// # Retornos
///
/// * `Ok(Operation)`: Operação de saque criada.
/// * `Err(SaqueError)`: Erro durante a criação da operação.
pub fn saque(opts: SaqueOptions) -> Result<Operation, SaqueError> {
    // Validação da chave pública de destino
    if !StrKey::is_valid_ed25519_public_key(&opts.destino) {
        return Err(SaqueError::InvalidDestination);
    }

    // Validação da quantidade
    if !is_valid_amount(&opts.amount) {
        return Err(SaqueError::InvalidAmount);
    }

    // Criação da chave pública de destino
    let destination_keypair = KeyPair::from_public_key(&opts.destino)?;

    // Criação do AccountId de destino
    let destination_account_id = destination_keypair.xdr_account_id();

    // Conversão da quantidade para XDR
    let amount = to_xdr_amount(&opts.amount)?;

    // Criação da operação de pagamento
    let attributes = PaymentOp {
        destination: destination_account_id,
        asset: opts.asset,
        amount: Int64 { value: amount },
    };

    // Criação da operação
    let op_attributes = OperationBody::Payment(attributes);
    let mut op = Operation::new(op_attributes);

    // Adiciona a conta fonte (se fornecida)
    if let Some(source) = opts.source {
        op.source_account = Some(KeyPair::from_public_key(&source)?.public_key().clone());
    }

    Ok(op)
}

/// Função de Validação de Quantidade

/// Verifica se a quantidade é válida.
///
/// # Parâmetros
///
/// * `amount`: Quantidade a ser validada.
///
/// # Retornos
///
/// * `true`: Quantidade válida.
/// * `false`: Quantidade inválida.
fn is_valid_amount(amount: &str) -> bool {
    match amount.parse::<f64>() {
        Ok(value) => value > 0.0,
        Err(_) => false,
    }
}

// Função para converter a quantidade para o formato XDR.
fn to_xdr_amount(amount: &str) -> Result<i64, SaqueError> {
    match amount.parse::<f64>() {
        Ok(value) => Ok((value * 1_000_000.0) as i64),
        Err(_) => Err(SaqueError::InvalidAmount),
    }
}