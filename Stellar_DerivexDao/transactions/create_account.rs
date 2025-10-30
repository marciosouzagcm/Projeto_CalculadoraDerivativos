use stellar_base::crypto; // Importa o módulo crypto da biblioteca stellar_base.
use stellar_base::xdr::{CreateAccountOp, Operation, OperationBody, Int64}; // Importa tipos necessários para criar operações de conta.
use stellar_base::error::Error as StellarError; // Renomeia o erro da biblioteca stellar_base para evitar conflitos.
use thiserror::Error; // Importa a biblioteca thiserror para criar erros personalizados.

#[derive(Debug)]
pub struct CreateAccountOptions {
    pub destination: String, // Endereço de destino da nova conta.
    pub starting_balance: String, // Saldo inicial da nova conta.
    pub source: Option<String>, // Conta de origem opcional.
}

#[derive(Debug, Error)]
pub enum CreateAccountError {
    #[error("Destino inválido")]
    InvalidDestination, // Erro para destino inválido.
    #[error("Saldo inicial inválido")]
    InvalidStartingBalance, // Erro para saldo inicial inválido.
    #[error("Erro interno do Stellar: {0}")]
    StellarError(#[from] StellarError), // Erro interno do Stellar.
}

pub fn create_account(opts: CreateAccountOptions) -> Result<Operation, CreateAccountError> {
    // Verifica se o destino é válido.
    match crypto::KeyPair::from_public_key_str(&opts.destination) {
        Ok(_) => (),
        Err(_) => return Err(CreateAccountError::InvalidDestination),
    }

    // Verifica se o saldo inicial é válido.
    if !is_valid_amount(&opts.starting_balance) {
        return Err(CreateAccountError::InvalidStartingBalance);
    }

    // Cria a operação de criação de conta.
    let destination_keypair = crypto::KeyPair::from_public_key_str(&opts.destination)?;
    let destination_account_id = destination_keypair.xdr_account_id();
    let starting_balance = to_xdr_amount(&opts.starting_balance)?;
    let attributes = CreateAccountOp {
        destination: destination_account_id,
        starting_balance: Int64 { value: starting_balance },
    };
    let op_attributes = OperationBody::CreateAccount(attributes);
    let mut op = Operation { body: op_attributes };

    // Define a conta de origem, se fornecida.
    if let Some(source) = opts.source {
        op.source_account = Some(crypto::KeyPair::from_public_key_str(&source)?.public_key().clone());
    }

    Ok(op)
}

// Função para verificar se a quantidade é válida.
fn is_valid_amount(amount: &str) -> bool {
    match amount.parse::<f64>() {
        Ok(value) => value > 0.0,
        Err(_) => false,
    }
}

// Função para converter a quantidade para o formato XDR.
fn to_xdr_amount(amount: &str) -> Result<i64, CreateAccountError> {
    match amount.parse::<f64>() {
        Ok(value) => Ok((value * 1_000_000.0) as i64),
        Err(_) => Err(CreateAccountError::InvalidStartingBalance),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_account_valid_inputs() {
        let opts = CreateAccountOptions {
            destination: "GCFX3TDSABDPWXU36YK3CVXQYV2JQV7F3HJL3UULN4VWU2YIXUMVMYCD".to_string(),
            starting_balance: "100.0".to_string(),
            source: Some("SBKDX7O3T7U44U6QF3BIPU5XVOPFXU6HNDWZZQCNFAPULGRWPPQXZW4Z".to_string()),
        };

        let result = create_account(opts);
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_account_invalid_destination() {
        let opts = CreateAccountOptions {
            destination: "INVALID_KEY".to_string(),
            starting_balance: "100.0".to_string(),
            source: None,
        };

        let result = create_account(opts);
        assert!(matches!(result, Err(CreateAccountError::InvalidDestination)));
    }

    #[test]
    fn test_create_account_invalid_balance() {
        let opts = CreateAccountOptions {
            destination: "GCFX3TDSABDPWXU36YK3CVXQYV2JQV7F3HJL3UULN4VWU2YIXUMVMYCD".to_string(),
            starting_balance: "-100.0".to_string(),
            source: None,
        };

        let result = create_account(opts);
        assert!(matches!(result, Err(CreateAccountError::InvalidStartingBalance)));
    }
}
