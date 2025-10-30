use stellar_base::crypto::strkey; // Importa strkey para manipulação de chaves públicas.
use stellar_base::Network; // Importa Network para especificar a rede Stellar.
use stellar_base::client::Client; // Importa Client para comunicação com o servidor Stellar.
use stellar_base::error::Error; // Importa Error para tratamento de erros.
use stellar_base::xdr::{AccountBalance, AccountId}; // Importa AccountBalance e AccountId para manipulação de contas.

#[derive(Debug)]
enum Asset {
    Native,
    CreditAlphanum4(String, String),
    CreditAlphanum12(String, String),
}

impl TryFrom<AccountBalance> for Asset {
    type Error = Error;

    fn try_from(balance: AccountBalance) -> Result<Self, Self::Error> {
        match balance.asset_type {
            "native" => Ok(Asset::Native),
            "credit_alphanum4" => Ok(Asset::CreditAlphanum4(
                balance.asset_code.clone(),
                balance.asset_issuer.clone(),
            )),
            "credit_alphanum12" => Ok(Asset::CreditAlphanum12(
                balance.asset_code.clone(),
                balance.asset_issuer.clone(),
            )),
            _ => Err(Error::InvalidAssetType),
        }
    }
}

pub async fn verificar_saldo(account_id: &str) -> Result<bool, Error> {
    // Verifica se a chave pública da conta é válida.
    if strkey::decode_check(account_id).is_err() {
        return Err(Error::InvalidAccountId);
    }

    let network = Network::Public; // Define a rede como pública.
    let client = Client::new(network); // Cria um novo cliente para comunicação com o servidor Stellar.
    let account = client.account(account_id).await?; // Obtém a conta do servidor.

    // Verifica se a conta possui saldos.
    if account.balances.is_empty() {
        return Ok(false);
    }

    // Itera sobre os saldos da conta.
    for balance in account.balances {
        let asset = Asset::try_from(balance)?;
        match asset {
            Asset::Native => {
                // Verifica se o saldo do ativo nativo é maior que zero.
                if balance.balance.parse::<f64>()? > 0.0 {
                    return Ok(true);
                }
            }
            _ => {}
        }
    }

    Ok(false)
}