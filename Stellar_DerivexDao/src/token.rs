use stellar_base::crypto::{MuxedAccount}; // Importa MuxedAccount para manipulação de contas muxed.
use stellar_base::operations::{Operation, PaymentOperation}; // Importa Operation e PaymentOperation para operações de pagamento.
use stellar_base::amount::Stroops; // Importa Stroops para manipulação de valores.
use stellar_base::xdr::{Asset, AssetCode4, Uint256}; // Importa Asset, AssetCode4 e Uint256 para manipulação de ativos e contas.
use soroban_sdk;


#[derive(Debug)]
pub enum TokenError {
    InvalidAmount, // Erro para quantidade inválida.
    ServerError(String), // Erro para problemas no servidor.
}

pub struct Token {
    pub public_key: String, // Chave pública do token.
    pub symbol: String, // Símbolo do token.
    pub server: Server, // Servidor Stellar.
}

impl Token {
    // Método para criar uma nova instância de Token.
    pub fn new(public_key: String, symbol: String) -> Self {
        let server = Server::new("(horizon.stellar.org)".to_string(), None).expect("Falha ao criar servidor");
        Token { public_key, symbol, server }
    }

    // Método para obter o saldo do token.
    pub async fn get_balance(&self) -> Result<f64, TokenError> {
        let account = self.server.load_account(&self.public_key).await.map_err(|e| TokenError::ServerError(e.to_string()))?;
        let balance = account.balances.iter().find(|b| b.asset_type == "credit_alphanum4" && b.asset_code == self.symbol)
            .map(|b| b.balance.parse::<f64>().unwrap_or(0.0));
        Ok(balance.unwrap_or(0.0))
    }

    // Método para criar uma emissão de tokens.
    pub async fn create_emission(&self, amount: f64) -> Result<(), TokenError> {
        if amount <= 0.0 {
            return Err(TokenError::InvalidAmount);
        }

        let source_account = self.server.load_account(&self.public_key).await.map_err(|e| TokenError::ServerError(e.to_string()))?;
        let destination = MuxedAccount::KeyTypeEd25519(Uint256::from_be_bytes(self.public_key.clone().into_bytes()));
        let operation = Operation::Payment(PaymentOperation {
            source_account: None,
            destination,
            asset: Asset::CreditAlphanum4Code(AssetCode4::new(self.symbol.clone().into_bytes())),
            amount: Stroops::new((amount * 1_000_000.0) as i64),
        });

        let transaction = TransactionBuilder::new(source_account.id.clone().into(), 100, Stroops::new(100))
            .add_operation(operation)
            .build()?;
        self.server.submit_transaction(transaction).await.map_err(|e| TokenError::ServerError(e.to_string()))?;
        Ok(())
    }

    // Método para transferir tokens.
    pub async fn transfer(&self, destination: String, amount: f64) -> Result<(), TokenError> {
        if amount <= 0.0 {
            return Err(TokenError::InvalidAmount);
        }

        let source_account = self.server.load_account(&self.public_key).await.map_err(|e| TokenError::ServerError(e.to_string()))?;
        let destination = MuxedAccount::KeyTypeEd25519(Uint256::from_be_bytes(destination.into_bytes()));
        let operation = Operation::Payment(PaymentOperation {
            source_account: None,
            destination,
            asset: Asset::CreditAlphanum4(AssetCode::new(self.symbol.clone().into_bytes())),
            amount: Stroops::new((amount * 1_000_000.0) as i64),
        });

        let transaction = TransactionBuilder::new(source_account.id.clone().into(), 100, Stroops::new(100))
            .add_operation(operation)
            .build()?;
        self.server.submit_transaction(transaction).await.map_err(|e| TokenError::ServerError(e.to_string()))?;
        Ok(())
    }
}