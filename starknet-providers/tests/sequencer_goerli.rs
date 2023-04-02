use std::str::FromStr;

use starknet_core::types::{
    AccountTransaction, BlockId, CallL1Handler, FieldElement, InvokeFunctionTransactionRequest,
    L1Address,
};
use starknet_providers::{Provider, SequencerGatewayProvider};

fn create_sequencer_client() -> SequencerGatewayProvider {
    SequencerGatewayProvider::starknet_alpha_goerli()
}

fn felt_hex(hex: &str) -> FieldElement {
    FieldElement::from_hex_be(hex).unwrap()
}

fn felt_dec(dec: &str) -> FieldElement {
    FieldElement::from_dec_str(dec).unwrap()
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
                sender_address: FieldElement::from_hex_be(
                    "0x02643ad267d5f4035c57f33c4b521a539a0525f41ff8e885ce106b47a462ce5c",
                )
                .unwrap(),
                calldata: vec![
                    FieldElement::from_hex_be(
                        "0x0000000000000000000000000000000000000000000000000000000000000001",
                    )
                    .unwrap(),
                    FieldElement::from_hex_be(
                        "0x07394cbe418daa16e42b87ba67372d4ab4a5df0b05c6e554d158458ce245bc10",
                    )
                    .unwrap(),
                    FieldElement::from_hex_be(
                        "0x02f0b3c5710379609eb5495f1ecd348cb28167711b73609fe565a72734550354",
                    )
                    .unwrap(),
                    FieldElement::from_hex_be(
                        "0x0000000000000000000000000000000000000000000000000000000000000000",
                    )
                    .unwrap(),
                    FieldElement::from_hex_be(
                        "0x0000000000000000000000000000000000000000000000000000000000000003",
                    )
                    .unwrap(),
                    FieldElement::from_hex_be(
                        "0x0000000000000000000000000000000000000000000000000000000000000003",
                    )
                    .unwrap(),
                    FieldElement::from_hex_be(
                        "0x02643ad267d5f4035c57f33c4b521a539a0525f41ff8e885ce106b47a462ce5c",
                    )
                    .unwrap(),
                    FieldElement::from_hex_be(
                        "0x00000000000000000000000000000000000000000000003635c9adc5dea00000",
                    )
                    .unwrap(),
                    FieldElement::from_hex_be(
                        "0x0000000000000000000000000000000000000000000000000000000000000000",
                    )
                    .unwrap(),
                ],
                signature: vec![
                    FieldElement::from_hex_be(
                        "0x04404d064a487404a02b6eac3493938d4c5d4fc6fc44c5a573838790a3f0bd78",
                    )
                    .unwrap(),
                    FieldElement::from_hex_be(
                        "0x049cd4c19a92b4da46cc87f4a4f4b06dded9567db8bca8bf6279497777cf1fc2",
                    )
                    .unwrap(),
                ],
                max_fee: FieldElement::from_hex_be(
                    "0x000000000000000000000000000000000000000000000000016345785d8a0000",
                )
                .unwrap(),
                nonce: FieldElement::from_hex_be(
                    "0x0000000000000000000000000000000000000000000000000000000000000000",
                )
                .unwrap(),
            }),
            BlockId::Latest,
            false,
        )
        .await
        .unwrap();

    assert!(
        simulation
            .trace
            .function_invocation
            .unwrap()
            .execution_resources
            .n_steps
            > 0
    );
}

#[tokio::test]
async fn sequencer_goerli_can_get_nonce() {
    let client = create_sequencer_client();

    let nonce = client
        .get_nonce(
            FieldElement::from_hex_be(
                "0x0577abc3e3ab491af6fdc1e185b71a6d04f7e71a525f9f57c19fc36ed0655a39",
            )
            .unwrap(),
            BlockId::Latest,
        )
        .await
        .unwrap();

    assert!(nonce > FieldElement::ZERO);
}

#[tokio::test]
async fn sequencer_goerli_can_bulk_estimate_fee() {
    let client = create_sequencer_client();

    // Two txs from the same account with nonce 0 and 1
    let estimates = client
        .estimate_fee_bulk(
            &[
                AccountTransaction::InvokeFunction(InvokeFunctionTransactionRequest {
                    sender_address: felt_hex(
                        "0x5b5e9f6f6fb7d2647d81a8b2c2b99cbc9cc9d03d705576d7061812324dca5c0",
                    ),
                    calldata: vec![
                        felt_dec("1"),
                        felt_dec(
                    "3267429884791031784129188059026496191501564961518175231747906707757621165072"
                ),
                        felt_dec(
                    "1329909728320632088402217562277154056711815095720684343816173432540100887380"
                ),
                        felt_dec("0"),
                        felt_dec("3"),
                        felt_dec("3"),
                        felt_dec(
                    "2582978326697182094925044915479529632446801760547577461724830811224889140672"
                ),
                        felt_dec("1000000000000000000000"),
                        felt_dec("0"),
                    ],
                    signature: vec![
                        felt_dec(
                    "605417791026644483670811513828340231819682850475940872862750374884434792160"
                ),
                        felt_dec(
                    "3227162751686940146996647969343636789208985440255179192147422777151505011910"
                ),
                    ],
                    max_fee: felt_dec("0"),
                    nonce: felt_dec("0"),
                }),
                AccountTransaction::InvokeFunction(InvokeFunctionTransactionRequest {
                    sender_address: felt_hex(
                        "0x5b5e9f6f6fb7d2647d81a8b2c2b99cbc9cc9d03d705576d7061812324dca5c0",
                    ),
                    calldata: vec![
                        felt_dec("1"),
                        felt_dec(
                    "3267429884791031784129188059026496191501564961518175231747906707757621165072"
                ),
                        felt_dec(
                    "1329909728320632088402217562277154056711815095720684343816173432540100887380"
                ),
                        felt_dec("0"),
                        felt_dec("3"),
                        felt_dec("3"),
                        felt_dec(
                    "2582978326697182094925044915479529632446801760547577461724830811224889140672"
                ),
                        felt_dec("2000000000000000000000"),
                        felt_dec("0"),
                    ],
                    signature: vec![
                        felt_dec(
                    "2454731969569471949423549779477272094056061808345298145925675439909833863557"
                ),
                        felt_dec(
                    "724612237028642548263407980387909582237336146127278825566903814475468042134"
                ),
                    ],
                    max_fee: felt_dec("0"),
                    nonce: felt_dec("1"),
                }),
            ],
            BlockId::Latest,
            false,
        )
        .await
        .unwrap();

    assert_eq!(estimates.len(), 2);
}
