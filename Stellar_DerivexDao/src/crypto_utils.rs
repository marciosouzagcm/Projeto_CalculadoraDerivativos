use ed25519_dalek::{Keypair, PublicKey, Signer, Verifier, Signature};
use rand::rngs::OsRng;

/// Assina uma mensagem e verifica a assinatura usando Ed25519
pub fn sign_and_verify() {
    // Gera um par de chaves aleatórias
    let mut csprng = OsRng {};
    let keypair = Keypair::generate(&mut csprng);

    // Mensagem a ser assinada
    let message: &[u8] = b"Mensagem de exemplo para assinatura";

    // Assina a mensagem com a chave privada
    let signature: Signature = keypair.sign(message);

    // Verifica a assinatura com a chave pública
    let public_key: PublicKey = keypair.public;
    let is_valid = public_key.verify(message, &signature).is_ok();

    if is_valid {
        println!("Assinatura válida!");
    } else {
        println!("Assinatura inválida!");
    }
}
