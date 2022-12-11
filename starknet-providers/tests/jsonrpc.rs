use std::io::Write;

use flate2::{write::GzEncoder, Compression};
use starknet_core::{
    types::{ContractArtifact, FieldElement},
    utils::{get_selector_from_name, get_storage_var_address},
};
use starknet_providers::jsonrpc::{models::*, HttpTransport, JsonRpcClient, JsonRpcClientError};
use url::Url;

fn create_jsonrpc_client() -> JsonRpcClient<HttpTransport> {
    JsonRpcClient::new(HttpTransport::new(
        Url::parse("https://starknet-goerli.rpc.zklend.com/").unwrap(),
    ))
}

fn create_contract_class() -> ContractClass {
    let artifact = serde_json::from_str::<ContractArtifact>(include_str!(
        "../../starknet-core/test-data/contracts/artifacts/oz_account.txt"
    ))
    .unwrap();

    let program_json = serde_json::to_string(&artifact.program).unwrap();
    let mut gzip_encoder = GzEncoder::new(Vec::new(), Compression::best());
    gzip_encoder.write_all(program_json.as_bytes()).unwrap();
    let compressed_program = gzip_encoder.finish().unwrap();

    ContractClass {
        program: compressed_program,
        entry_points_by_type: EntryPointsByType {
            constructor: artifact
                .entry_points_by_type
                .constructor
                .into_iter()
                .map(|item| ContractEntryPoint {
                    offset: item.offset.try_into().unwrap(),
                    selector: item.selector,
                })
                .collect(),
            external: artifact
                .entry_points_by_type
                .external
                .into_iter()
                .map(|item| ContractEntryPoint {
                    offset: item.offset.try_into().unwrap(),
                    selector: item.selector,
                })
                .collect(),
            l1_handler: artifact
                .entry_points_by_type
                .l1_handler
                .into_iter()
                .map(|item| ContractEntryPoint {
                    offset: item.offset.try_into().unwrap(),
                    selector: item.selector,
                })
                .collect(),
        },
        abi: None,
    }
}

#[tokio::test]
async fn jsonrpc_get_block_with_tx_hashes() {
    let rpc_client = create_jsonrpc_client();

    let block = rpc_client
        .get_block_with_tx_hashes(&BlockId::Tag(BlockTag::Latest))
        .await
        .unwrap();

    let block = match block {
        MaybePendingBlockWithTxHashes::Block(block) => block,
        _ => panic!("unexpected block response type"),
    };

    assert!(block.block_number > 0);
}

#[tokio::test]
async fn jsonrpc_get_block_with_txs() {
    let rpc_client = create_jsonrpc_client();

    let block = rpc_client
        .get_block_with_txs(&BlockId::Tag(BlockTag::Latest))
        .await
        .unwrap();

    let block = match block {
        MaybePendingBlockWithTxs::Block(block) => block,
        _ => panic!("unexpected block response type"),
    };

    assert!(block.block_number > 0);
}

#[tokio::test]
async fn jsonrpc_get_state_update() {
    let rpc_client = create_jsonrpc_client();

    let state_update = rpc_client
        .get_state_update(&BlockId::Tag(BlockTag::Latest))
        .await
        .unwrap();

    assert!(state_update.new_root > FieldElement::ZERO);
}

#[tokio::test]
async fn jsonrpc_get_storage_at() {
    let rpc_client = create_jsonrpc_client();

    // Checks L2 ETH balance via storage taking advantage of implementation detail
    let eth_balance = rpc_client
        .get_storage_at(
            FieldElement::from_hex_be(
                "049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
            )
            .unwrap(),
            get_storage_var_address(
                "ERC20_balances",
                &[FieldElement::from_hex_be(
                    "01352dd0ac2a462cb53e4f125169b28f13bd6199091a9815c444dcae83056bbc",
                )
                .unwrap()],
            )
            .unwrap(),
            &BlockId::Tag(BlockTag::Latest),
        )
        .await
        .unwrap();

    assert!(eth_balance > FieldElement::ZERO);
}

#[tokio::test]
async fn jsonrpc_get_transaction_by_hash() {
    let rpc_client = create_jsonrpc_client();

    let tx = rpc_client
        .get_transaction_by_hash(
            FieldElement::from_hex_be(
                "05b08d06a7f6422881d6461175f325844d179ca9018dbab5e92dc34e5c176ff1",
            )
            .unwrap(),
        )
        .await
        .unwrap();

    let tx = match tx {
        Transaction::Invoke(InvokeTransaction::V0(tx)) => tx,
        _ => panic!("unexpected tx response type"),
    };

    assert!(tx.entry_point_selector > FieldElement::ZERO);
}

#[tokio::test]
async fn jsonrpc_get_transaction_by_block_id_and_index() {
    let rpc_client = create_jsonrpc_client();

    let tx = rpc_client
        .get_transaction_by_block_id_and_index(&BlockId::Number(10_000), 1)
        .await
        .unwrap();

    let tx = match tx {
        Transaction::Invoke(InvokeTransaction::V0(tx)) => tx,
        _ => panic!("unexpected tx response type"),
    };

    assert!(tx.entry_point_selector > FieldElement::ZERO);
}

#[tokio::test]
async fn jsonrpc_get_transaction_by_hash_non_existent_tx() {
    let rpc_client = create_jsonrpc_client();

    let err = rpc_client
        .get_transaction_by_hash(FieldElement::from_hex_be("1234").unwrap())
        .await
        .unwrap_err();

    match err {
        JsonRpcClientError::RpcError(err) => {
            // INVALID_TXN_HASH
            assert_eq!(err.code, 25);
        }
        _ => panic!("Unexpected error"),
    }
}

#[tokio::test]
async fn jsonrpc_get_transaction_receipt() {
    let rpc_client = create_jsonrpc_client();

    let receipt = rpc_client
        .get_transaction_receipt(
            FieldElement::from_hex_be(
                "05b08d06a7f6422881d6461175f325844d179ca9018dbab5e92dc34e5c176ff1",
            )
            .unwrap(),
        )
        .await
        .unwrap();

    let receipt = match receipt {
        MaybePendingTransactionReceipt::Receipt(TransactionReceipt::Invoke(receipt)) => receipt,
        _ => panic!("unexpected receipt response type"),
    };

    assert!(receipt.actual_fee > FieldElement::ZERO);
}

#[tokio::test]
async fn jsonrpc_get_class() {
    let rpc_client = create_jsonrpc_client();

    let class = rpc_client
        .get_class(
            &BlockId::Tag(BlockTag::Latest),
            FieldElement::from_hex_be(
                "025ec026985a3bf9d0cc1fe17326b245dfdc3ff89b8fde106542a3ea56c5a918",
            )
            .unwrap(),
        )
        .await
        .unwrap();

    assert!(!class.program.is_empty());
}

#[tokio::test]
async fn jsonrpc_get_class_hash_at() {
    let rpc_client = create_jsonrpc_client();

    let class_hash = rpc_client
        .get_class_hash_at(
            &BlockId::Tag(BlockTag::Latest),
            FieldElement::from_hex_be(
                "06b3dab9c563083e7e74d9a7ab7649f7af4564cfef397f8e44233a1feffc7049",
            )
            .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(
        class_hash,
        FieldElement::from_hex_be(
            "025ec026985a3bf9d0cc1fe17326b245dfdc3ff89b8fde106542a3ea56c5a918"
        )
        .unwrap()
    );
}

#[tokio::test]
async fn jsonrpc_get_class_at() {
    let rpc_client = create_jsonrpc_client();

    let class = rpc_client
        .get_class_at(
            &BlockId::Tag(BlockTag::Latest),
            FieldElement::from_hex_be(
                "06b3dab9c563083e7e74d9a7ab7649f7af4564cfef397f8e44233a1feffc7049",
            )
            .unwrap(),
        )
        .await
        .unwrap();

    assert!(!class.program.is_empty());
}

#[tokio::test]
async fn jsonrpc_get_block_transaction_count() {
    let rpc_client = create_jsonrpc_client();

    let count = rpc_client
        .get_block_transaction_count(&BlockId::Number(20_000))
        .await
        .unwrap();

    assert_eq!(count, 4);
}

#[tokio::test]
async fn jsonrpc_call() {
    let rpc_client = create_jsonrpc_client();

    // Checks L2 ETH balance
    let eth_balance = rpc_client
        .call(
            &FunctionCall {
                contract_address: FieldElement::from_hex_be(
                    "049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
                )
                .unwrap(),
                entry_point_selector: get_selector_from_name("balanceOf").unwrap(),
                calldata: vec![FieldElement::from_hex_be(
                    "01352dd0ac2a462cb53e4f125169b28f13bd6199091a9815c444dcae83056bbc",
                )
                .unwrap()],
            },
            &BlockId::Tag(BlockTag::Latest),
        )
        .await
        .unwrap();

    assert!(eth_balance[0] > FieldElement::ZERO);
}

#[tokio::test]
async fn jsonrpc_estimate_fee() {
    let rpc_client = create_jsonrpc_client();

    let estimate = rpc_client
        .estimate_fee(
            &BroadcastedTransaction::Invoke(BroadcastedInvokeTransaction::V1(
                BroadcastedInvokeTransactionV1 {
                    max_fee: FieldElement::ZERO,
                    signature: vec![
                        FieldElement::from_hex_be(
                            "156a781f12e8743bd07e20a4484154fd0baccee95d9ea791c121c916ad44ee0",
                        )
                        .unwrap(),
                        FieldElement::from_hex_be(
                            "7228267473c670cbb86a644f8696973db978c51acde19431d3f1f8f100794c6",
                        )
                        .unwrap(),
                    ],
                    nonce: FieldElement::ZERO,
                    sender_address: FieldElement::from_hex_be(
                        "5b5e9f6f6fb7d2647d81a8b2c2b99cbc9cc9d03d705576d7061812324dca5c0",
                    )
                    .unwrap(),
                    calldata: vec![
                        FieldElement::from_hex_be("1").unwrap(),
                        FieldElement::from_hex_be(
                            "7394cbe418daa16e42b87ba67372d4ab4a5df0b05c6e554d158458ce245bc10",
                        )
                        .unwrap(),
                        FieldElement::from_hex_be(
                            "2f0b3c5710379609eb5495f1ecd348cb28167711b73609fe565a72734550354",
                        )
                        .unwrap(),
                        FieldElement::from_hex_be("0").unwrap(),
                        FieldElement::from_hex_be("3").unwrap(),
                        FieldElement::from_hex_be("3").unwrap(),
                        FieldElement::from_hex_be(
                            "5b5e9f6f6fb7d2647d81a8b2c2b99cbc9cc9d03d705576d7061812324dca5c0",
                        )
                        .unwrap(),
                        FieldElement::from_hex_be("3635c9adc5dea00000").unwrap(),
                        FieldElement::from_hex_be("0").unwrap(),
                    ],
                },
            )),
            &BlockId::Tag(BlockTag::Latest),
        )
        .await
        .unwrap();

    assert!(estimate.gas_consumed > 0);
    assert!(estimate.gas_price > 0);
    assert!(estimate.overall_fee > 0);
}

#[tokio::test]
async fn jsonrpc_block_number() {
    let rpc_client = create_jsonrpc_client();

    let block_number = rpc_client.block_number().await.unwrap();
    assert!(block_number > 0);
}

#[tokio::test]
async fn jsonrpc_block_hash_and_number() {
    let rpc_client = create_jsonrpc_client();

    let id = rpc_client.block_hash_and_number().await.unwrap();

    assert!(id.block_hash > FieldElement::ZERO);
    assert!(id.block_number > 0);
}

#[tokio::test]
async fn jsonrpc_chain_id() {
    let rpc_client = create_jsonrpc_client();

    let chain_id = rpc_client.chain_id().await.unwrap();
    assert!(chain_id > FieldElement::ZERO);
}

#[tokio::test]
async fn jsonrpc_pending_transactions() {
    let rpc_client = create_jsonrpc_client();

    rpc_client.pending_transactions().await.unwrap();
}

#[tokio::test]
async fn jsonrpc_syncing() {
    let rpc_client = create_jsonrpc_client();

    let syncing = rpc_client.syncing().await.unwrap();
    if let SyncStatusType::Syncing(sync_status) = syncing {
        assert!(sync_status.highest_block_num > 0);
    }
}

#[tokio::test]
async fn jsonrpc_get_events() {
    let rpc_client = create_jsonrpc_client();

    let events = rpc_client
        .get_events(
            EventFilter {
                from_block: Some(BlockId::Number(234500)),
                to_block: None,
                address: None,
                keys: None,
            },
            None,
            20,
        )
        .await
        .unwrap();

    assert_eq!(events.events.len(), 20);
}

#[tokio::test]
async fn jsonrpc_get_nonce() {
    let rpc_client = create_jsonrpc_client();

    let nonce = rpc_client
        .get_nonce(
            &BlockId::Tag(BlockTag::Latest),
            FieldElement::from_hex_be(
                "0661d341c2ba6f3c2b277e54d507e4b49b0c4d8973ac7366a035d0d3e8bdec47",
            )
            .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(nonce, FieldElement::ZERO);
}

#[tokio::test]
async fn jsonrpc_add_invoke_transaction() {
    let rpc_client = create_jsonrpc_client();

    // This is an invalid made-up transaction but the sequencer will happily accept it anyways
    let add_tx_result = rpc_client
        .add_invoke_transaction(&BroadcastedInvokeTransaction::V0(
            BroadcastedInvokeTransactionV0 {
                max_fee: FieldElement::ONE,
                signature: vec![],
                nonce: FieldElement::ZERO,
                contract_address: FieldElement::from_hex_be(
                    "049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
                )
                .unwrap(),
                entry_point_selector: get_selector_from_name("__execute__").unwrap(),
                calldata: vec![FieldElement::from_hex_be("1234").unwrap()],
            },
        ))
        .await
        .unwrap();

    assert!(add_tx_result.transaction_hash > FieldElement::ZERO);
}

#[tokio::test]
async fn jsonrpc_add_declare_transaction() {
    let rpc_client = create_jsonrpc_client();

    let add_tx_result = rpc_client
        .add_declare_transaction(&BroadcastedDeclareTransaction {
            max_fee: FieldElement::ZERO,
            version: 0,
            signature: vec![],
            nonce: FieldElement::ZERO,
            contract_class: create_contract_class(),
            sender_address: FieldElement::ONE,
        })
        .await
        .unwrap();

    assert!(add_tx_result.class_hash > FieldElement::ZERO);
}

// TODO: test deploy account transaction
