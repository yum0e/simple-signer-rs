use alloy_primitives::FixedBytes;
use clap::Parser;
use ethers::{
    core::k256::ecdsa::SigningKey,
    signers::{Signer, Wallet},
    types::Signature,
    utils::secret_key_to_address,
};

#[derive(Parser)]
struct Cli {
    message: String,
    private_key: FixedBytes<32>,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let message = args.message;

    let signing_key = Wallet::from_bytes(&args.private_key[..]).unwrap();
    let eth_address =
        secret_key_to_address(&SigningKey::from_slice(&args.private_key[..]).unwrap());
    let signature: Signature = signing_key.sign_message(&message).await.unwrap();
    let signature = hex::encode(&signature.to_vec()[..]);
    println!("Address: {}", eth_address);
    println!(r#"{{"signature": "{signature}", "message": "{message}"}}"#);
}
