// Importações necessárias
use stellar_base::xdr::{Operation, OperationBody, PathPaymentStrictSendOp, AccountId, Int64, Asset};
use stellar_base::crypto;
use thiserror::Error;

// Estrutura para opções de pagamento
pub struct PathPaymentStrictSendOptions {
    pub send_asset: Asset,         // Ativo enviado
    pub send_amount: String,      // Quantidade enviada
    pub destination: String,      // Destino
    pub dest_asset: Asset,        // Ativo recebido
    pub dest_min: String,         // Quantidade mínima recebida
    pub path: Vec<Asset>,         // Caminho de pagamento
    pub source: Option<String>,   // Conta de origem (opcional)
}

// Erros possíveis no pagamento
#[derive(Error, Debug)]
pub enum PathPaymentStrictSendError {
    #[error("Ativo de envio não fornecido")]
    InvalidSendAsset,
    #[error("Quantidade de envio inválida")]
    InvalidSendAmount,
    #[error("Ativo de destino não fornecido")]
    InvalidDestAsset,
    #[error("Quantidade mínima de destino inválida")]
    InvalidDestMin,
    #[error("Conta de destino inválida")]
    InvalidDestination,
}

// Função para realizar pagamento
pub fn path_payment_strict_send(opts: PathPaymentStrictSendOptions) -> Result<Operation, PathPaymentStrictSendError> {
    // Validação da conta de destino
    let destination = decode_address_to_muxed_account(&opts.destination)?;

    // Validação do ativo enviado
    if !opts.send_asset.is_native() {
        return Err(PathPaymentStrictSendError::InvalidSendAsset);
    }

    // Validação da quantidade enviada
    if !is_valid_amount(&opts.send_amount) {
        return Err(PathPaymentStrictSendError::InvalidSendAmount);
    }

    // Validação do ativo recebido
    if !opts.dest_asset.is_native() {
        return Err(PathPaymentStrictSendError::InvalidDestAsset);
    }

    // Validação da quantidade mínima recebida
    if !is_valid_amount(&opts.dest_min) {
        return Err(PathPaymentStrictSendError::InvalidDestMin);
    }

    // Configuração da operação de pagamento
    let attributes = PathPaymentStrictSendOp {
        send_asset: opts.send_asset,
        send_amount: Int64 { value: to_xdr_amount(&opts.send_amount)? },
        destination,
        dest_asset: opts.dest_asset,
        dest_min: Int64 { value: to_xdr_amount(&opts.dest_min)? },
        path: opts.path,
    };

    // Criação da operação
    let op_attributes = OperationBody::PathPaymentStrictSend(attributes);
    let mut op = Operation {
        body: op_attributes,
        source_account: None,
    };

    // Adiciona conta de origem (opcional)
    if let Some(source) = opts.source {
        op.source_account = Some(crypto::KeyPair::from_public_key_str(&source)?.public_key().clone());
    }

    Ok(op)
}

// Função para decodificar endereço para AccountId
fn decode_address_to_muxed_account(address: &str) -> Result<AccountId, PathPaymentStrictSendError> {
    if crypto::KeyPair::is_valid_ed25519_public_key(address) {
        let keypair = crypto::KeyPair::from_public_key_str(address).map_err(|_| PathPaymentStrictSendError::InvalidDestination)?;
        Ok(keypair.xdr_account_id())
    } else {
        Err(PathPaymentStrictSendError::InvalidDestination)
    }
}

// Função para validar quantidade
fn is_valid_amount(amount: &str) -> bool {
    match amount.parse::<f64>() {
        Ok(value) => value > 0.0,
        Err(_) => false,
    }
}

// Função para converter quantidade para XDR
fn to_xdr_amount(amount: &str) -> Result<i64, PathPaymentStrictSendError> {
    match amount.parse::<f64>() {
        Ok(value) => Ok((value * 1_000_000.0) as i64),
        Err(_) => Err(PathPaymentStrictSendError::InvalidSendAmount),
    }
}

// Estrutura para opções de transferência
pub struct TransferOptions {
    pub destination: String,  // Destino
    pub amount: String,      // Quantidade
    pub send_asset: Asset,   // Ativo enviado
    pub dest_asset: Asset,   // Ativo recebido
    pub source: Option<String>,  // Conta de origem (opcional)
}

// Erros possíveis na transferência
#[derive(Debug, Error)]
pub enum TransferError {
    #[error("Conta de destino inválida")]
    InvalidDestination,
    #[error("Quantia inválida")]
    InvalidAmount,
}

// Implementação de conversão de erro
impl From<PathPaymentStrictSendError> for TransferError {
    fn from(_: PathPaymentStrictSendError) -> Self {
        TransferError::InvalidDestination
    }
}

// Função para realizar transferência
pub fn transfer(opts: TransferOptions) -> Result<Operation, TransferError> {
    // Validação da conta de destino
    if !crypto::KeyPair::is_valid_ed25519_public_key(&opts.destination) {
        return Err(TransferError::InvalidDestination);
    }

    // Validação da quantidade
    if !is_valid_amount(&opts.amount) {
        return Err(TransferError::InvalidAmount);
    }

    // Decodificação da conta de destino
    let destination = decode_address_to_muxed_account(&opts.destination)?;

    // Conversão da quantidade para XDR
    let amount = Int64 { value: to_xdr_amount(&opts.amount)? };

    // Configuração da operação de pagamento
    let attributes = PathPaymentStrictSendOp {
        send_asset: opts.send_asset,
        send_amount: amount,
        destination,
        dest_asset: opts.dest_asset,
        dest_min: amount,
        path: Vec::new(),
    };

    // Criação da operação
    let op_attributes = OperationBody::PathPaymentStrictSend(attributes);
    let mut op = Operation {
        body: op_attributes,
        source_account: None,
    };

    // Adiciona conta de origem (opcional)
    if let Some(source) = opts.source {
        op.source_account = Some(crypto::KeyPair::from_public_key_str(&source)?.public_key().clone());
    }

    Ok(op)
}