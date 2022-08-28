use std::io::Write;

use flate2::{write::GzEncoder, Compression};
use starknet_core::{
    types::{ContractArtifact, FieldElement},
    utils::{get_selector_from_name, get_storage_var_address},
};
use starknet_providers::jsonrpc::{
    models::{
        BlockHashOrTag, BlockId, BlockTag, ContractClass, ContractEntryPoint, EntryPointsByType,
        EventFilter, FunctionCall, MaybePendingBlockWithTxHashes, MaybePendingBlockWithTxs,
        SyncStatusType, Transaction,
    },
    HttpTransport, JsonRpcClient, JsonRpcClientError,
};
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

    assert!(block.header.block_number > 0);
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

    assert!(block.header.block_number > 0);
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
            &BlockHashOrTag::Tag(BlockTag::Latest),
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
        Transaction::Invoke(tx) => tx,
        _ => panic!("unexpected tx response type"),
    };

    assert!(tx.function_call.entry_point_selector > FieldElement::ZERO);
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

    assert!(receipt.actual_fee > FieldElement::ZERO);
}

#[tokio::test]
async fn jsonrpc_get_class() {
    let rpc_client = create_jsonrpc_client();

    let class = rpc_client
        .get_class(
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
async fn jsonrpc_block_number() {
    let rpc_client = create_jsonrpc_client();

    let block_number = rpc_client.block_number().await.unwrap();
    assert!(block_number > 0);
}

#[tokio::test]
async fn jsonrpc_chain_id() {
    let rpc_client = create_jsonrpc_client();

    let chain_id = rpc_client.chain_id().await.unwrap();
    assert!(chain_id > FieldElement::ZERO);
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
                from_block: Some(234500),
                to_block: None,
                address: None,
                keys: None,
            },
            20,
            10,
        )
        .await
        .unwrap();

    assert_eq!(events.events.len(), 20);
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
            &BlockHashOrTag::Tag(BlockTag::Latest),
        )
        .await
        .unwrap();

    assert!(eth_balance[0] > FieldElement::ZERO);
}

#[tokio::test]
async fn jsonrpc_estimate_fee() {
    let rpc_client = create_jsonrpc_client();

    // Same as `jsonrpc_call`
    let estimate = rpc_client
        .estimate_fee(
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
            &BlockHashOrTag::Tag(BlockTag::Latest),
        )
        .await
        .unwrap();

    // There seems to be a bug in `gas_comsumed` causing it to be zero:
    //   https://github.com/eqlabs/pathfinder/issues/412
    assert!(estimate.gas_price > FieldElement::ZERO);
    assert!(estimate.overall_fee > FieldElement::ZERO);
}

#[tokio::test]
async fn jsonrpc_add_invoke_transaction() {
    let rpc_client = create_jsonrpc_client();

    // This is an invalid made-up transaction but the sequencer will happily accept it anyways
    let add_tx_result = rpc_client
        .add_invoke_transaction(
            &FunctionCall {
                contract_address: FieldElement::from_hex_be(
                    "049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
                )
                .unwrap(),
                entry_point_selector: get_selector_from_name("__execute__").unwrap(),
                calldata: vec![FieldElement::from_hex_be("1234").unwrap()],
            },
            vec![],
            FieldElement::ONE,
            FieldElement::ZERO,
        )
        .await
        .unwrap();

    assert!(add_tx_result.transaction_hash > FieldElement::ZERO);
}

#[tokio::test]
async fn jsonrpc_add_declare_transaction() {
    let rpc_client = create_jsonrpc_client();

    let add_tx_result = rpc_client
        .add_declare_transaction(&create_contract_class(), FieldElement::ZERO)
        .await
        .unwrap();

    assert!(add_tx_result.class_hash > FieldElement::ZERO);
}

#[tokio::test]
async fn jsonrpc_add_deploy_transaction() {
    let rpc_client = create_jsonrpc_client();

    let add_tx_result = rpc_client
        .add_deploy_transaction(
            FieldElement::ONE,
            vec![FieldElement::ONE],
            &create_contract_class(),
        )
        .await
        .unwrap();

    assert!(add_tx_result.contract_address > FieldElement::ZERO);
}
