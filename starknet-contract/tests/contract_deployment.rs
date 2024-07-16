use rand::{rngs::StdRng, RngCore, SeedableRng};
use starknet_accounts::{ExecutionEncoding, SingleOwnerAccount};
use starknet_contract::ContractFactory;
use starknet_core::types::{contract::legacy::LegacyContractClass, BlockId, BlockTag, Felt};
use starknet_providers::{jsonrpc::HttpTransport, JsonRpcClient};
use starknet_signers::{LocalWallet, SigningKey};
use url::Url;

/// Cairo short string encoding for `SN_SEPOLIA`.
const CHAIN_ID: Felt = Felt::from_raw([
    507980251676163170,
    18446744073709551615,
    18446744073708869172,
    1555806712078248243,
]);

#[tokio::test]
async fn can_deploy_contract_to_alpha_sepolia_with_invoke_v1() {
    #![allow(clippy::or_fun_call)]
    let rpc_url = std::env::var("STARKNET_RPC")
        .unwrap_or("https://pathfinder.rpc.sepolia.starknet.rs/rpc/v0_6".into());
    let provider = JsonRpcClient::new(HttpTransport::new(Url::parse(&rpc_url).unwrap()));
    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        Felt::from_hex("00ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap(),
    ));
    let address =
        Felt::from_hex("0x059e738b86f82e11cd5b4afaccfce1d5166700c92fb87be59ad4af908e9bf866")
            .unwrap();
    let mut account =
        SingleOwnerAccount::new(provider, signer, address, CHAIN_ID, ExecutionEncoding::New);
    account.set_block_id(BlockId::Tag(BlockTag::Pending));

    let artifact = serde_json::from_str::<LegacyContractClass>(include_str!(
        "../test-data/cairo0/artifacts/oz_account.txt"
    ))
    .unwrap();

    let factory = ContractFactory::new(artifact.class_hash().unwrap(), account);

    let mut salt_buffer = [0u8; 32];
    let mut rng = StdRng::from_entropy();
    rng.fill_bytes(&mut salt_buffer[1..]);

    let result = factory
        .deploy_v1(vec![Felt::ONE], Felt::from_bytes_be(&salt_buffer), true)
        .max_fee(Felt::from_dec_str("100000000000000000").unwrap())
        .send()
        .await;

    match result {
        Ok(_) => {}
        Err(err) => panic!("Contract deployment failed: {err}"),
    }
}

#[tokio::test]
async fn can_deploy_contract_to_alpha_sepolia_with_invoke_v3() {
    #![allow(clippy::or_fun_call)]
    let rpc_url = std::env::var("STARKNET_RPC")
        .unwrap_or("https://pathfinder.rpc.sepolia.starknet.rs/rpc/v0_6".into());
    let provider = JsonRpcClient::new(HttpTransport::new(Url::parse(&rpc_url).unwrap()));
    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        Felt::from_hex("00ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap(),
    ));
    let address =
        Felt::from_hex("0x034dd51aa591d174b60d1cb45e46dfcae47946fae1c5e62933bbf48effedde4d")
            .unwrap();
    let mut account =
        SingleOwnerAccount::new(provider, signer, address, CHAIN_ID, ExecutionEncoding::New);
    account.set_block_id(BlockId::Tag(BlockTag::Pending));

    let artifact = serde_json::from_str::<LegacyContractClass>(include_str!(
        "../test-data/cairo0/artifacts/oz_account.txt"
    ))
    .unwrap();

    let factory = ContractFactory::new(artifact.class_hash().unwrap(), account);

    let mut salt_buffer = [0u8; 32];
    let mut rng = StdRng::from_entropy();
    rng.fill_bytes(&mut salt_buffer[1..]);

    let result = factory
        .deploy_v3(vec![Felt::ONE], Felt::from_bytes_be(&salt_buffer), true)
        .gas(200000)
        .gas_price(500000000000000)
        .send()
        .await;

    match result {
        Ok(_) => {}
        Err(err) => panic!("Contract deployment failed: {err}"),
    }
}
