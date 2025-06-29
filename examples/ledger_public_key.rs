use starknet::signers::{LedgerSigner, Signer};

#[tokio::main]
async fn main() {
    let path = "m/2645'/1195502025'/1470455285'/0'/0'/0";

    let ledger = LedgerSigner::new(path.try_into().expect("unable to parse path"))
        .await
        .expect("failed to initialize Starknet Ledger app");

    let public_key = ledger
        .get_public_key()
        .await
        .expect("failed to get public key");

    println!("Path: {path}");
    println!("Public key: {:#064x}", public_key.scalar());
}
