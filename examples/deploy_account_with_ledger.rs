use starknet::{
    accounts::AccountFactory,
    core::chain_id,
    macros::felt,
    providers::{
        jsonrpc::{HttpTransport, JsonRpcClient},
        Url,
    },
    signers::LedgerSigner,
};
use starknet_accounts::OpenZeppelinAccountFactory;

#[tokio::main]
async fn main() {
    // OpenZeppelin account contract v0.13.0 compiled with cairo v2.6.3
    let class_hash = felt!("0x00e2eb8f5672af4e6a4e8a8f1b44989685e668489b0a25437733756c5a34a1d6");

    // Anything you like here as salt
    let salt = felt!("12345678");

    let provider = JsonRpcClient::new(HttpTransport::new(
        Url::parse("https://starknet-sepolia.public.blastapi.io/rpc/v0_7").unwrap(),
    ));

    let signer = LedgerSigner::new(
        "m/2645'/1195502025'/1470455285'/0'/0'/0"
            .try_into()
            .expect("unable to parse path"),
    )
    .await
    .expect("failed to initialize Starknet Ledger app");

    let factory = OpenZeppelinAccountFactory::new(class_hash, chain_id::SEPOLIA, signer, provider)
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
            println!("Transaction hash: {:#064x}", tx.transaction_hash);
            println!("Account: {:#064x}", tx.contract_address);
        }
        Err(err) => {
            eprintln!("Error: {err}");
        }
    }
}
