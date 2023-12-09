use rand::{rngs::StdRng, RngCore, SeedableRng};
use starknet_accounts::{ExecutionEncoding, SingleOwnerAccount};
use starknet_contract::ContractFactory;
use starknet_core::{
    chain_id,
    types::{contract::legacy::LegacyContractClass, BlockId, BlockTag, FieldElement},
};
use starknet_providers::{jsonrpc::HttpTransport, JsonRpcClient};
use starknet_signers::{LocalWallet, SigningKey};
use url::Url;

#[tokio::test]
async fn can_deploy_contract_to_alpha_goerli() {
    let rpc_url =
        std::env::var("STARKNET_RPC").unwrap_or("https://rpc-goerli-1.starknet.rs/rpc/v0.5".into());
    let provider = JsonRpcClient::new(HttpTransport::new(Url::parse(&rpc_url).unwrap()));
    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        FieldElement::from_hex_be(
            "00ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
        )
        .unwrap(),
    ));
    let address = FieldElement::from_hex_be(
        "04284d0741ee00d8e4d6a02d21c0be58665f0e6e187cf48c509b1ac39cdeca65",
    )
    .unwrap();
    let mut account = SingleOwnerAccount::new(
        provider,
        signer,
        address,
        chain_id::TESTNET,
        ExecutionEncoding::Legacy,
    );
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
        .send()
        .await;

    match result {
        Ok(_) => {}
        Err(err) => panic!("Contract deployment failed: {err}"),
    }
}
