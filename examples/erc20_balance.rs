use starknet::{
    core::types::{BlockId, BlockTag, FieldElement, FunctionCall},
    macros::{felt, selector},
    providers::{
        jsonrpc::{HttpTransport, JsonRpcClient},
        Provider, Url,
    },
};

#[tokio::main]
async fn main() {
    let provider = JsonRpcClient::new(HttpTransport::new(
        Url::parse("https://starknet-testnet.public.blastapi.io/rpc/v0_6").unwrap(),
    ));

    let tst_token_address =
        felt!("0x07394cbe418daa16e42b87ba67372d4ab4a5df0b05c6e554d158458ce245bc10");

    let call_result = provider
        .call(
            FunctionCall {
                contract_address: tst_token_address,
                entry_point_selector: selector!("balanceOf"),
                calldata: vec![FieldElement::from_hex_be(
                    "YOUR_ACCOUNT_CONTRACT_ADDRESS_IN_HEX_HERE",
                )
                .unwrap()],
            },
            BlockId::Tag(BlockTag::Latest),
        )
        .await
        .expect("failed to call contract");

    dbg!(call_result);
}
