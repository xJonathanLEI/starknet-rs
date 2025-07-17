use std::sync::Arc;
use secrecy::SecretString;
use starknet::{
    accounts::{ExecutionEncoding, SingleOwnerAccount},
    contract::ContractFactory,
    core::{
        chain_id,
        types::{contract::legacy::LegacyContractClass, Felt},
    },
    macros::felt,
    providers::{
        jsonrpc::{HttpTransport, JsonRpcClient},
        Url,
    },
    signers::{LocalWallet, SigningKey},
};

#[tokio::main]
async fn main() {
    // NOTE: you will need to declare this class first
    let contract_artifact: LegacyContractClass =
        serde_json::from_reader(std::fs::File::open("/path/to/contract/artifact.json").unwrap())
            .unwrap();
    let class_hash = contract_artifact.class_hash().unwrap();

    let provider = JsonRpcClient::new(HttpTransport::new(
        Url::parse("https://starknet-sepolia.public.blastapi.io/rpc/v0_9").unwrap(),
    ));

    let signer = LocalWallet::from(SigningKey::from_secret(
        SecretString::new("YOUR_PRIVATE_KEY_IN_HEX_HERE".into()),
    ).unwrap());
    let address = Felt::from_hex("YOUR_ACCOUNT_CONTRACT_ADDRESS_IN_HEX_HERE").unwrap();
    let account = SingleOwnerAccount::new(
        provider,
        signer,
        address,
        chain_id::SEPOLIA,
        ExecutionEncoding::New,
    );

    // Wrapping in `Arc` is meaningless here. It's just showcasing it could be done as
    // `Arc<Account>` implements `Account` too.
    let account = Arc::new(account);

    let contract_factory = ContractFactory::new(class_hash, account);
    contract_factory
        .deploy_v3(vec![felt!("123456")], felt!("1122"), false)
        .send()
        .await
        .expect("Unable to deploy contract");
}
