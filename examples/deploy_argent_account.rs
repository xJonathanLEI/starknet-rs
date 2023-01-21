use starknet::{
    accounts::{AccountFactory, ArgentAccountFactory},
    core::{chain_id, types::FieldElement},
    macros::felt,
    providers::SequencerGatewayProvider,
    signers::{LocalWallet, SigningKey},
};

#[tokio::main]
async fn main() {
    // Latest hashes as of 2023-01-21. For demo only.
    let proxy_hash = felt!("0x025ec026985a3bf9d0cc1fe17326b245dfdc3ff89b8fde106542a3ea56c5a918");
    let impl_hash = felt!("0x033434ad846cdd5f23eb73ff09fe6fddd568284a0fb7d1be20ee482f044dabe2");

    // Anything you like here as salt
    let salt = felt!("12345678");

    let provider = SequencerGatewayProvider::starknet_alpha_goerli();
    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        FieldElement::from_hex_be("YOUR_PRIVATE_KEY_IN_HEX_HERE").unwrap(),
    ));

    let factory = ArgentAccountFactory::new(
        proxy_hash,
        impl_hash,
        chain_id::TESTNET,
        FieldElement::ZERO,
        signer,
        provider,
    )
    .await
    .unwrap();

    let deployment = factory.deploy(salt);

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
