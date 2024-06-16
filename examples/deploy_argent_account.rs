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
    // Latest hash as of 2023-09-15. For demo only.
    let class_hash = felt!("0x01a736d6ed154502257f02b1ccdf4d9d1089f80811cd6acad48e6b6a9d1f2003");

    // Anything you like here as salt
    let salt = felt!("12345678");

    let provider = JsonRpcClient::new(HttpTransport::new(
        Url::parse("https://starknet-sepolia.public.blastapi.io/rpc/v0_7").unwrap(),
    ));

    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        Felt::from_hex("YOUR_PRIVATE_KEY_IN_HEX_HERE").unwrap(),
    ));

    let factory =
        ArgentAccountFactory::new(class_hash, chain_id::SEPOLIA, Felt::ZERO, signer, provider)
            .await
            .unwrap();

    let deployment = factory.deploy_v1(salt);

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
            dbg!(tx);
        }
        Err(err) => {
            eprintln!("Error: {err}");
        }
    }
}
