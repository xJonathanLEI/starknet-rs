use starknet::{
    accounts::{Account, ExecutionEncoding, SingleOwnerAccount},
    core::{
        chain_id,
        types::{BlockId, BlockTag, Call, Felt},
        utils::get_selector_from_name,
    },
    macros::felt,
    providers::{
        jsonrpc::{HttpTransport, JsonRpcClient},
        Url,
    },
    signers::LedgerSigner,
};

#[tokio::main]
async fn main() {
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
    let address = Felt::from_hex("YOUR_ACCOUNT_CONTRACT_ADDRESS_IN_HEX_HERE").unwrap();
    let eth_token_address =
        Felt::from_hex("0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7")
            .unwrap();

    let mut account = SingleOwnerAccount::new(
        provider,
        signer,
        address,
        chain_id::SEPOLIA,
        ExecutionEncoding::New,
    );

    // `SingleOwnerAccount` defaults to checking nonce and estimating fees against the latest
    // block. Optionally change the target block to pending with the following line:
    account.set_block_id(BlockId::Tag(BlockTag::Pending));

    let result = account
        .execute_v3(vec![Call {
            to: eth_token_address,
            selector: get_selector_from_name("transfer").unwrap(),
            calldata: vec![felt!("0x1234"), felt!("100"), Felt::ZERO],
        }])
        .send()
        .await
        .unwrap();

    println!("Transaction hash: {:#064x}", result.transaction_hash);
}
