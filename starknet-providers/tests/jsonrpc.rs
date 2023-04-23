use starknet_core::{
    types::FieldElement,
    utils::{get_selector_from_name, get_storage_var_address},
};
use starknet_providers::jsonrpc::{
    models::*, HttpTransport, JsonRpcClient, JsonRpcClientError, RpcError,
};
use url::Url;

fn create_jsonrpc_client() -> JsonRpcClient<HttpTransport> {
    JsonRpcClient::new(HttpTransport::new(
        Url::parse("https://starknet-goerli.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161")
            .unwrap(),
    ))
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
        JsonRpcClientError::RpcError(RpcError::Code(ErrorCode::TransactionHashNotFound)) => {
            // TXN_HASH_NOT_FOUND
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

    let class = match class {
        ContractClass::Legacy(class) => class,
        _ => panic!("unexpected class type"),
    };

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

    let class = match class {
        ContractClass::Legacy(class) => class,
        _ => panic!("unexpected class type"),
    };

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

// NOTE: `addXxxxTransaction` methods are harder to test here since they require signatures. These
// are integration tests anyways, so we might as well just leave the job to th tests in
// `starknet-accounts`.
