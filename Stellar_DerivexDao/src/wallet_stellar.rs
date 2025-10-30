use std::io;
use stellar_sdk::{Keypair, Server, Network, TransactionBuilder, Memo, Asset};
use sha2::{Sha256, Digest};
use mnemonic::Mnemonic;
use tokio;
use soroban_sdk;

/// Função principal do programa. Apresenta opções ao usuário e direciona para a funcionalidade correspondente.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Lê a entrada do usuário e direciona a execução
    let resposta = input("Escolha uma opção:\n1 - Criar nova carteira\n2 - Verificar carteira existente\n3 - Assinar e enviar transação\n4 - Rastrear transação\n5 - Enviar criptoativo\nDigite o número da opção: ");

    // Usa `match` para tratar as diferentes opções de entrada do usuário
    match resposta.as_str() {
        "1" => criar_carteira(),
        "2" => {
            let stellar_address = input("Informe o endereço público da carteira Stellar para verificação: ");
            verificar_carteira(&stellar_address).await?; // Chama função assíncrona para verificar carteira
        },
        "3" => {
            let secret_key = input("Informe a chave secreta da sua carteira Stellar: ");
            enviar_transacao_com_memo(&secret_key).await?; // Envia uma transação com um memo
        },
        "4" => {
            let hash_transacao = input("Informe o hash da transação para rastrear: ");
            rastrear_transacao(&hash_transacao).await?; // Rastreia uma transação existente
        },
        "5" => {
            let secret_key = input("Informe a chave secreta da sua carteira Stellar: ");
            let destino = input("Informe o endereço público da carteira destino: ");
            let quantidade: f64 = input("Informe a quantidade de XLM a ser enviada: ").parse()?; // Converte a entrada para f64
            let memo = input("Informe o MEMO: ");
            enviar_criptoativo(&secret_key, &destino, quantidade, &memo).await?; // Envia XLM para outro endereço
        },
        _ => println!("Opção inválida. Por favor, escolha 1, 2, 3, 4 ou 5."),
    }

    Ok(())
}

/// Função para criar uma nova carteira Stellar
fn criar_carteira() {
    // Gera uma frase mnemônica para recuperação da carteira
    let mnemo = Mnemonic::new("english");
    let mnemonic = mnemo.generate(128);

    // Solicita o nome do usuário para associar à carteira
    let nome_completo = input("Informe seu nome completo para vincular à carteira: ");
    
    // Gera um par de chaves público/privado
    let pair = Keypair::new();

    // Exibe informações da nova carteira ao usuário
    println!("\nNova carteira Stellar criada com sucesso!");
    println!("Nome completo vinculado à carteira: {}", nome_completo);
    println!("Chave pública (para receber XLM): {}", pair.public_key());
    println!("Chave secreta (guarde em segurança): {}", pair.secret().to_string());
    println!("Frase mnemônica (guarde em segurança): {}", mnemonic);

    // TODO: Adicionar lógica para armazenar a carteira de forma segura, como em um arquivo criptografado
}

/// Função assíncrona para verificar o saldo de uma carteira Stellar
async fn verificar_carteira(stellar_address: &str) -> anyhow::Result<()> {
    // Conexão com o servidor Stellar
    let server = Server::new("https://horizon.stellar.org");

    // Solicita informações da conta usando o endereço público
    let account = server.accounts().account_id(stellar_address).call().await?;

    // Obtém informações de saldo
    let balance_info = account
        .balances
        .iter()
        .find(|balance| balance.asset_type == "native")
        .unwrap_or_else(|| panic!("Nenhum saldo encontrado para XLM.")); // Melhor tratamento de erros

    println!("\nCarteira Stellar verificada com sucesso:");
    println!("Endereço público (Stellar Address): {}", stellar_address);
    println!("Saldo em Stellar (XLM): {}", balance_info.balance);

    Ok(())
}

/// Envia uma transação com um MEMO específico
async fn enviar_transacao_com_memo(secret_key: &str) -> anyhow::Result<()> {
    // Converte a chave secreta em um par de chaves
    let pair = Keypair::from_secret(secret_key);
    let server = Server::new("https://horizon.stellar.org");

    let account = server.load_account(&pair.public_key()).await?;
    
    // Prepara uma transação com um MEMO de texto
    let transaction = TransactionBuilder::new(&account, Network::PUBLIC, 100)
        .add_text_memo("Transação com Memo")
        .set_timeout(30)
        .build();

    // Assina e envia a transação
    let transaction = transaction.sign(&pair);
    let response = server.submit_transaction(&transaction).await?;

    println!("Transação enviada com sucesso! Hash da transação: {}", response.hash);

    Ok(())
}

/// Rastreia informações de uma transação com base no hash
async fn rastrear_transacao(hash_transacao: &str) -> anyhow::Result<()> {
    let server = Server::new("https://horizon.stellar.org");

    // Obtém detalhes da transação pelo hash
    let transacao = server.transactions().transaction(hash_transacao).call().await?;

    println!("\nTransação rastreada com sucesso!");
    println!("Hash da transação: {}", transacao.hash);
    println!("Estado da transação: {}", transacao.successful);
    println!("Data da transação: {}", transacao.created_at);

    Ok(())
}

/// Envia XLM de uma carteira para outra
async fn enviar_criptoativo(secret_key: &str, destino: &str, quantidade: f64, memo: &str) -> anyhow::Result<()> {
    // Converte a chave secreta em um par de chaves
    let pair = Keypair::from_secret(secret_key);
    let server = Server::new("https://horizon.stellar.org");

    // Carrega a conta do remetente
    let account = server.load_account(&pair.public_key()).await?;

    // Cria uma transação de pagamento
    let transaction = TransactionBuilder::new(&account, Network::PUBLIC, 100)
        .append_payment_op(destino, quantidade, Asset::native()) // Envia XLM
        .add_text_memo(memo) // Adiciona o memo fornecido
        .set_timeout(30)
        .build();

    // Assina e envia a transação
    let transaction = transaction.sign(&pair);
    let response = server.submit_transaction(&transaction).await?;

    println!("Transação enviada com sucesso! Hash da transação: {}", response.hash);

    Ok(())
}

/// Lê a entrada do usuário a partir do terminal
fn input(mensagem: &str) -> String {
    println!("{}", mensagem);
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Erro ao ler entrada");
    entrada.trim().to_string() // Remove espaços e quebras de linha
}
