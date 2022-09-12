use std::str::FromStr;

use starknet_core::types::{
    AccountTransaction, BlockId, CallL1Handler, FieldElement, InvokeFunctionTransactionRequest,
    L1Address,
};
use starknet_providers::{Provider, SequencerGatewayProvider};

fn create_sequencer_client() -> SequencerGatewayProvider {
    SequencerGatewayProvider::starknet_alpha_goerli()
}

#[tokio::test]
async fn sequencer_goerli_can_estimate_message_fee() {
    let client = create_sequencer_client();

    let estimate = client
        .estimate_message_fee(
            CallL1Handler {
                from_address: L1Address::from_str("0xc3511006c04ef1d78af4c8e0e74ec18a6e64ff9e")
                    .unwrap(),
                to_address: FieldElement::from_hex_be(
                    "0x073314940630fd6dcda0d772d4c972c4e0a9946bef9dabf4ef84eda8ef542b82",
                )
                .unwrap(),
                entry_point_selector: FieldElement::from_hex_be(
                    "0x02d757788a8d8d6f21d1cd40bce38a8222d70654214e96ff95d8086e684fbee5",
                )
                .unwrap(),
                payload: vec![
                    FieldElement::from_hex_be(
                        "0x03467d16d88e959aea455796fa10e26fb2ca07a9e10284988946cc85c7d21b50",
                    )
                    .unwrap(),
                    FieldElement::from_hex_be(
                        "0x0000000000000000000000000000000000000000000000000011c37937e08000",
                    )
                    .unwrap(),
                    FieldElement::from_hex_be(
                        "0x0000000000000000000000000000000000000000000000000000000000000000",
                    )
                    .unwrap(),
                ],
            },
            BlockId::Latest,
        )
        .await
        .unwrap();

    assert!(estimate.gas_usage > 0);
}

#[tokio::test]
async fn sequencer_goerli_can_simulate_transaction() {
    let client = create_sequencer_client();

    let simulation = client
        .simulate_transaction(
            AccountTransaction::InvokeFunction(InvokeFunctionTransactionRequest {
                contract_address: FieldElement::from_hex_be(
                    "0x07394cbe418daa16e42b87ba67372d4ab4a5df0b05c6e554d158458ce245bc10",
                )
                .unwrap(),
                entry_point_selector: FieldElement::from_hex_be(
                    "0x02f0b3c5710379609eb5495f1ecd348cb28167711b73609fe565a72734550354",
                )
                .unwrap(),
                calldata: vec![
                    FieldElement::from_hex_be(
                        "0x59b844bae1727516c6d5c40d2540f6f0a0eebc7eed2adf760515b45dbc20593",
                    )
                    .unwrap(),
                    FieldElement::from_dec_str("1000000000000000000000").unwrap(),
                    FieldElement::from_dec_str("0").unwrap(),
                ],
                signature: vec![],
                max_fee: FieldElement::ZERO,
            }),
            BlockId::Latest,
        )
        .await
        .unwrap();

    assert!(
        simulation
            .trace
            .function_invocation
            .execution_resources
            .n_steps
            > 0
    );
}
