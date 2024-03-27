use rand::{rngs::StdRng, RngCore, SeedableRng};
use starknet_accounts::{ExecutionEncoding, SingleOwnerAccount};
use starknet_contract::ContractFactory;
use starknet_core::types::{
    contract::legacy::LegacyContractClass, BlockId, BlockTag, FieldElement,
};
use starknet_providers::{jsonrpc::HttpTransport, JsonRpcClient};
use starknet_signers::{LocalWallet, SigningKey};
use url::Url;

/// Cairo short string encoding for `SN_SEPOLIA`.
const CHAIN_ID: FieldElement = FieldElement::from_mont([
    1555806712078248243,
    18446744073708869172,
    18446744073709551615,
    507980251676163170,
]);

#[tokio::test]
async fn can_deploy_contract_to_alpha_sepolia() {
    let rpc_url = std::env::var("STARKNET_RPC")
        .unwrap_or("https://pathfinder.rpc.sepolia.starknet.rs/rpc/v0_6".into());
    let provider = JsonRpcClient::new(HttpTransport::new(Url::parse(&rpc_url).unwrap()));
    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        FieldElement::from_hex_be(
            "00ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
        )
        .unwrap(),
    ));
    let address = FieldElement::from_hex_be(
        "0x059e738b86f82e11cd5b4afaccfce1d5166700c92fb87be59ad4af908e9bf866",
    )
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
        .deploy(
            vec![FieldElement::ONE],
            FieldElement::from_bytes_be(&salt_buffer).unwrap(),
            true,
        )
        .max_fee(FieldElement::from_dec_str("1000000000000000000").unwrap())
        .send()
        .await;

    match result {
        Ok(_) => {}
        Err(err) => panic!("Contract deployment failed: {err}"),
    }
}
