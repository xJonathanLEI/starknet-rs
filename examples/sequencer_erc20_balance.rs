use starknet::{
    core::types::{BlockId, CallFunction, FieldElement},
    macros::{felt, selector},
    providers::{Provider, SequencerGatewayProvider},
};

#[tokio::main]
async fn main() {
    let provider = SequencerGatewayProvider::starknet_alpha_goerli();
    let tst_token_address =
        felt!("0x07394cbe418daa16e42b87ba67372d4ab4a5df0b05c6e554d158458ce245bc10");

    let call_result = provider
        .call_contract(
            CallFunction {
                contract_address: tst_token_address,
                entry_point_selector: selector!("balanceOf"),
                calldata: vec![FieldElement::from_hex_be(
                    "YOUR_ACCOUNT_CONTRACT_ADDRESS_IN_HEX_HERE",
                )
                .unwrap()],
            },
            BlockId::Latest,
        )
        .await
        .expect("failed to call contract");

    dbg!(call_result);
}
