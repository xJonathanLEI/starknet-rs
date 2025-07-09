use starknet::{
    accounts::{AccountFactory, ArgentAccountFactory},
    core::{chain_id, types::Felt},
    macros::felt,
    providers::{
        jsonrpc::{HttpTransport, JsonRpcClient},
        Url,
    },
    signers::{LocalWallet, SigningKey},
};

#[tokio::main]
async fn main() {
    // Latest hash as of 2024-12-01. For demo only.
    let class_hash = felt!("0x036078334509b514626504edc9fb252328d1a240e4e948bef8d0c08dff45927f");

    // Anything you like here as salt
    let salt = felt!("12345678");

    let provider = JsonRpcClient::new(HttpTransport::new(
        Url::parse("https://starknet-sepolia.public.blastapi.io/rpc/v0_9").unwrap(),
    ));

    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        Felt::from_hex("YOUR_PRIVATE_KEY_IN_HEX_HERE").unwrap(),
    ));

    let factory = ArgentAccountFactory::new(class_hash, chain_id::SEPOLIA, None, signer, provider)
        .await
        .unwrap();

    let deployment = factory.deploy_v3(salt);

    let est_fee = deployment.estimate_fee().await.unwrap();

    // In an actual application you might want to add a buffer to the amount
    println!(
        "Fund at least {} wei to {:#064x}",
        est_fee.overall_fee,
        deployment.address()
    );
    println!("Press ENTER after account is funded to continue deployment...");
    std::io::stdin().read_line(&mut String::new()).unwrap();

    let result = deployment.send().await;
    match result {
        Ok(tx) => {
            println!("Transaction hash: {:#064x}", tx.transaction_hash);
            println!("Account: {:#064x}", tx.contract_address);
        }
        Err(err) => {
            eprintln!("Error: {err}");
        }
    }
}
