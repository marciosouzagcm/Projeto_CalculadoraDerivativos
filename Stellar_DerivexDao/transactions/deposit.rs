use stellar_base::xdr::{Operation, OperationBody, PaymentOp, Int64}; // Importa tipos necessários para criar operações de pagamento.
use stellar_base::crypto::{strkey}; // Importa strkey para manipulação de chaves públicas.
use stellar_base::xdr::{Asset, AssetCode, Uint256}; // Importa Asset, AssetCode e Uint256 para manipulação de ativos e contas.
use thiserror::Error; // Importa a biblioteca thiserror para criar erros personalizados.

#[derive(Debug)]
pub struct DepositOptions {
    pub destination: String, // Endereço de destino do depósito.
    pub amount: String, // Quantidade a ser depositada.
    pub asset: Asset, // Ativo a ser depositado.
    pub source: Option<String>, // Conta de origem opcional.
}

#[derive(Debug, Error)]
pub enum DepositError {
    #[error("Destination is invalid")]
    InvalidDestination, // Erro para destino inválido.
    #[error("Amount is invalid")]
    InvalidAmount, // Erro para quantidade inválida.
    #[error("Source account is invalid")]
    InvalidSource, // Erro para conta de origem inválida.
}

pub fn deposit(opts: DepositOptions) -> Result<Operation, DepositError> {
    // Verifica se a chave pública de destino é válida.
    if !strkey::is_valid_ed25519_public_key(&opts.destination) {
        return Err(DepositError::InvalidDestination);
    }

    // Verifica se o valor do montante é válido.
    if !is_valid_amount(&opts.amount) {
        return Err(DepositError::InvalidAmount);
    }

    // Converte a chave pública de destino em AccountId.
    let destination_account_id = strkey::decode_ed25519_public_key(&opts.destination)
        .map_err(|_| DepositError::InvalidDestination)?;

    // Converte o valor do montante para um valor em XDR.
    let amount = to_xdr_amount(&opts.amount)?;

    // Define a operação de pagamento.
    let payment_op = PaymentOp {
        destination: destination_account_id,
        asset: opts.asset,
        amount: Int64 { value: amount },
    };

    let op_attributes = OperationBody::Payment(payment_op);

    // Cria a operação.
    let mut op = Operation::new(op_attributes);
    
    // Se uma conta fonte for fornecida, valida e a define.
    if let Some(source) = opts.source {
        let source_keypair = strkey::decode_ed25519_public_key(&source).map_err(|_| DepositError::InvalidSource)?;
        op.source_account = Some(source_keypair);
    }

    Ok(op)
}

// Verifica se o montante é válido (um número maior que zero).
fn is_valid_amount(amount: &str) -> bool {
    amount.parse::<f64>().map(|v| v > 0.0).unwrap_or(false)
}

// Converte o montante em string para um valor inteiro em XDR (micros Stellar).
fn to_xdr_amount(amount: &str) -> Result<i64, DepositError> {
    match amount.parse::<f64>() {
        Ok(value) => Ok((value * 1_000_000.0).round() as i64),  // Arredonda o valor para o inteiro mais próximo.
        Err(_) => Err(DepositError::InvalidAmount),
    }
}