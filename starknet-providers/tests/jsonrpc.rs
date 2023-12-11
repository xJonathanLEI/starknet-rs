use starknet_core::{
    types::{
        BlockId, BlockTag, BroadcastedInvokeTransaction, BroadcastedInvokeTransactionV1,
        BroadcastedTransaction, ContractClass, DeclareTransaction, DeployAccountTransaction,
        EthAddress, EventFilter, ExecuteInvocation, ExecutionResult, FieldElement, FunctionCall,
        InvokeTransaction, MaybePendingBlockWithTxHashes, MaybePendingBlockWithTxs,
        MaybePendingStateUpdate, MaybePendingTransactionReceipt, MsgFromL1, StarknetError,
        SyncStatusType, Transaction, TransactionExecutionStatus, TransactionReceipt,
        TransactionStatus, TransactionTrace,
    },
    utils::{get_selector_from_name, get_storage_var_address},
};
use starknet_providers::{
    jsonrpc::{HttpTransport, JsonRpcClient},
    Provider, ProviderError,
};
use url::Url;

fn create_jsonrpc_client() -> JsonRpcClient<HttpTransport> {
    let rpc_url = std::env::var("STARKNET_RPC")
        .unwrap_or("https://juno.rpc.goerli.starknet.rs/rpc/v0_6".into());
    JsonRpcClient::new(HttpTransport::new(Url::parse(&rpc_url).unwrap()))
}

#[ignore = "nodes are incorrectly returning `0.6.0-rc5`"]
#[tokio::test]
async fn jsonrpc_spec_version() {
    let rpc_client = create_jsonrpc_client();

    let version = rpc_client.spec_version().await.unwrap();

    assert_eq!(version, "0.6.0");
}

#[tokio::test]
async fn jsonrpc_get_block_with_tx_hashes() {
    let rpc_client = create_jsonrpc_client();

    let block = rpc_client
        .get_block_with_tx_hashes(BlockId::Tag(BlockTag::Latest))
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
        .get_block_with_txs(BlockId::Tag(BlockTag::Latest))
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
        .get_state_update(BlockId::Tag(BlockTag::Latest))
        .await
        .unwrap();

    let state_update = match state_update {
        MaybePendingStateUpdate::Update(value) => value,
        _ => panic!("unexpected data type"),
    };

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
            BlockId::Tag(BlockTag::Latest),
        )
        .await
        .unwrap();

    assert!(eth_balance > FieldElement::ZERO);
}

#[tokio::test]
async fn jsonrpc_get_transaction_status_rejected() {
    let rpc_client = create_jsonrpc_client();

    let status = rpc_client
        .get_transaction_status(
            FieldElement::from_hex_be(
                "0x07362a9daa42d9e4be657ed5a50f7fc04ac2017714cddb6c88dc08f48a782632",
            )
            .unwrap(),
        )
        .await
        .unwrap();

    match status {
        TransactionStatus::Rejected => {}
        _ => panic!("unexpected transaction status"),
    }
}

#[tokio::test]
async fn jsonrpc_get_transaction_status_succeeded() {
    let rpc_client = create_jsonrpc_client();

    let status = rpc_client
        .get_transaction_status(
            FieldElement::from_hex_be(
                "0x042fe661cf973a9e62dbf587cfb6d1808e377f394e4fea2c62a4fd02b5ba3473",
            )
            .unwrap(),
        )
        .await
        .unwrap();

    match status {
        TransactionStatus::AcceptedOnL1(TransactionExecutionStatus::Succeeded) => {}
        _ => panic!("unexpected transaction status"),
    }
}

#[tokio::test]
async fn jsonrpc_get_transaction_status_reverted() {
    let rpc_client = create_jsonrpc_client();

    let status = rpc_client
        .get_transaction_status(
            FieldElement::from_hex_be(
                "0x03998d935e23ee0b4956c40e8a5f64f6767176e7e44981328295a2fc20e6892c",
            )
            .unwrap(),
        )
        .await
        .unwrap();

    match status {
        TransactionStatus::AcceptedOnL1(TransactionExecutionStatus::Reverted) => {}
        _ => panic!("unexpected transaction status"),
    }
}

#[tokio::test]
async fn jsonrpc_get_transaction_by_hash_invoke_v0() {
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
async fn jsonrpc_get_transaction_by_hash_invoke_v1() {
    let rpc_client = create_jsonrpc_client();

    let tx = rpc_client
        .get_transaction_by_hash(
            FieldElement::from_hex_be(
                "028736cab16e67b4bed7ec5805ecd2636e7e800c2b0311b561e43fb4987cd70a",
            )
            .unwrap(),
        )
        .await
        .unwrap();

    let tx = match tx {
        Transaction::Invoke(InvokeTransaction::V1(tx)) => tx,
        _ => panic!("unexpected tx response type"),
    };

    assert!(tx.sender_address > FieldElement::ZERO);
}

#[tokio::test]
async fn jsonrpc_get_transaction_by_hash_l1_handler() {
    let rpc_client = create_jsonrpc_client();

    let tx = rpc_client
        .get_transaction_by_hash(
            FieldElement::from_hex_be(
                "0374286ae28f201e61ffbc5b022cc9701208640b405ea34ea9799f97d5d2d23c",
            )
            .unwrap(),
        )
        .await
        .unwrap();

    let tx = match tx {
        Transaction::L1Handler(tx) => tx,
        _ => panic!("unexpected tx response type"),
    };

    assert!(tx.entry_point_selector > FieldElement::ZERO);
}

#[tokio::test]
async fn jsonrpc_get_transaction_by_hash_declare_v0() {
    let rpc_client = create_jsonrpc_client();

    let tx = rpc_client
        .get_transaction_by_hash(
            FieldElement::from_hex_be(
                "021933cb48e59c74caa4575a78e89e6046d043505e5600fd88af7f051d3610ca",
            )
            .unwrap(),
        )
        .await
        .unwrap();

    let tx = match tx {
        Transaction::Declare(DeclareTransaction::V0(tx)) => tx,
        _ => panic!("unexpected tx response type"),
    };

    assert!(tx.sender_address > FieldElement::ZERO);
}

#[tokio::test]
async fn jsonrpc_get_transaction_by_hash_declare_v1() {
    let rpc_client = create_jsonrpc_client();

    let tx = rpc_client
        .get_transaction_by_hash(
            FieldElement::from_hex_be(
                "0618cc1e0ed68521ae8ee33595db8b0e33adaa9548837d4c824c83e99ad18f37",
            )
            .unwrap(),
        )
        .await
        .unwrap();

    let tx = match tx {
        Transaction::Declare(DeclareTransaction::V1(tx)) => tx,
        _ => panic!("unexpected tx response type"),
    };

    assert!(tx.sender_address > FieldElement::ZERO);
}

#[tokio::test]
async fn jsonrpc_get_transaction_by_hash_declare_v2() {
    let rpc_client = create_jsonrpc_client();

    let tx = rpc_client
        .get_transaction_by_hash(
            FieldElement::from_hex_be(
                "06c3b4729c1a303cef6fa60754ab012cd0759f2e8cf55cf0c008e10b9d420ca2",
            )
            .unwrap(),
        )
        .await
        .unwrap();

    let tx = match tx {
        Transaction::Declare(DeclareTransaction::V2(tx)) => tx,
        _ => panic!("unexpected tx response type"),
    };

    assert!(tx.sender_address > FieldElement::ZERO);
}

#[tokio::test]
async fn jsonrpc_get_transaction_by_hash_deploy() {
    let rpc_client = create_jsonrpc_client();

    let tx = rpc_client
        .get_transaction_by_hash(
            FieldElement::from_hex_be(
                "018b1ef66488f0e48bcf0bcdb367148352fe9180bc5d6505e9af843e6a51ff5d",
            )
            .unwrap(),
        )
        .await
        .unwrap();

    let tx = match tx {
        Transaction::Deploy(tx) => tx,
        _ => panic!("unexpected tx response type"),
    };

    assert!(tx.class_hash > FieldElement::ZERO);
}

#[tokio::test]
async fn jsonrpc_get_transaction_by_hash_deploy_account_v1() {
    let rpc_client = create_jsonrpc_client();

    let tx = rpc_client
        .get_transaction_by_hash(
            FieldElement::from_hex_be(
                "058ba7cdaf437d3a3b9680e6cbb4169811cddfa693875812bd98a8b1d61278de",
            )
            .unwrap(),
        )
        .await
        .unwrap();

    let tx = match tx {
        Transaction::DeployAccount(DeployAccountTransaction::V1(tx)) => tx,
        _ => panic!("unexpected tx response type"),
    };

    assert!(tx.class_hash > FieldElement::ZERO);
}

#[tokio::test]
async fn jsonrpc_get_transaction_by_block_id_and_index() {
    let rpc_client = create_jsonrpc_client();

    let tx = rpc_client
        .get_transaction_by_block_id_and_index(BlockId::Number(10_000), 1)
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
        ProviderError::StarknetError(StarknetError::TransactionHashNotFound) => {
            // TXN_HASH_NOT_FOUND
        }
        _ => panic!("Unexpected error"),
    }
}

#[tokio::test]
async fn jsonrpc_get_transaction_receipt_invoke() {
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

    match receipt.execution_result {
        ExecutionResult::Succeeded => {}
        _ => panic!("unexpected execution result"),
    }
}

#[tokio::test]
async fn jsonrpc_get_transaction_receipt_invoke_reverted() {
    let rpc_client = create_jsonrpc_client();

    let receipt = rpc_client
        .get_transaction_receipt(
            FieldElement::from_hex_be(
                "0x555c9392299727de9d3d6c85dd5db94f63a0994e698386d85c12b16f71fbfd0",
            )
            .unwrap(),
        )
        .await
        .unwrap();

    let receipt = match receipt {
        MaybePendingTransactionReceipt::Receipt(TransactionReceipt::Invoke(receipt)) => receipt,
        _ => panic!("unexpected receipt response type"),
    };

    match receipt.execution_result {
        ExecutionResult::Reverted { .. } => {}
        _ => panic!("unexpected execution result"),
    }
}

#[tokio::test]
async fn jsonrpc_get_transaction_receipt_l1_handler() {
    let rpc_client = create_jsonrpc_client();

    let tx_hash = FieldElement::from_hex_be(
        "0374286ae28f201e61ffbc5b022cc9701208640b405ea34ea9799f97d5d2d23c",
    )
    .unwrap();

    let tx = rpc_client.get_transaction_by_hash(tx_hash).await.unwrap();
    let receipt = rpc_client.get_transaction_receipt(tx_hash).await.unwrap();

    let tx = match tx {
        Transaction::L1Handler(tx) => tx,
        _ => panic!("unexpected tx type"),
    };

    let receipt = match receipt {
        MaybePendingTransactionReceipt::Receipt(TransactionReceipt::L1Handler(receipt)) => receipt,
        _ => panic!("unexpected receipt response type"),
    };

    match receipt.execution_result {
        ExecutionResult::Succeeded => {}
        _ => panic!("unexpected execution result"),
    }

    assert_eq!(tx.parse_msg_to_l2().unwrap().hash(), receipt.message_hash);
}

#[tokio::test]
async fn jsonrpc_get_transaction_receipt_declare() {
    let rpc_client = create_jsonrpc_client();

    let receipt = rpc_client
        .get_transaction_receipt(
            FieldElement::from_hex_be(
                "021933cb48e59c74caa4575a78e89e6046d043505e5600fd88af7f051d3610ca",
            )
            .unwrap(),
        )
        .await
        .unwrap();

    let receipt = match receipt {
        MaybePendingTransactionReceipt::Receipt(TransactionReceipt::Declare(receipt)) => receipt,
        _ => panic!("unexpected receipt response type"),
    };

    match receipt.execution_result {
        ExecutionResult::Succeeded => {}
        _ => panic!("unexpected execution result"),
    }
}

#[tokio::test]
async fn jsonrpc_get_transaction_receipt_deploy() {
    let rpc_client = create_jsonrpc_client();

    let receipt = rpc_client
        .get_transaction_receipt(
            FieldElement::from_hex_be(
                "018b1ef66488f0e48bcf0bcdb367148352fe9180bc5d6505e9af843e6a51ff5d",
            )
            .unwrap(),
        )
        .await
        .unwrap();

    let receipt = match receipt {
        MaybePendingTransactionReceipt::Receipt(TransactionReceipt::Deploy(receipt)) => receipt,
        _ => panic!("unexpected receipt response type"),
    };

    match receipt.execution_result {
        ExecutionResult::Succeeded => {}
        _ => panic!("unexpected execution result"),
    }
}

#[tokio::test]
async fn jsonrpc_get_transaction_receipt_deploy_account() {
    let rpc_client = create_jsonrpc_client();

    let receipt = rpc_client
        .get_transaction_receipt(
            FieldElement::from_hex_be(
                "058ba7cdaf437d3a3b9680e6cbb4169811cddfa693875812bd98a8b1d61278de",
            )
            .unwrap(),
        )
        .await
        .unwrap();

    let receipt = match receipt {
        MaybePendingTransactionReceipt::Receipt(TransactionReceipt::DeployAccount(receipt)) => {
            receipt
        }
        _ => panic!("unexpected receipt response type"),
    };

    match receipt.execution_result {
        ExecutionResult::Succeeded => {}
        _ => panic!("unexpected execution result"),
    }
}

#[tokio::test]
async fn jsonrpc_get_class_cairo_0() {
    let rpc_client = create_jsonrpc_client();

    let class = rpc_client
        .get_class(
            BlockId::Tag(BlockTag::Latest),
            FieldElement::from_hex_be(
                "048dd59fabc729a5db3afdf649ecaf388e931647ab2f53ca3c6183fa480aa292",
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
async fn jsonrpc_get_class_cairo_1() {
    let rpc_client = create_jsonrpc_client();

    let class = rpc_client
        .get_class(
            BlockId::Tag(BlockTag::Latest),
            FieldElement::from_hex_be(
                "05dc48d64a0f3852a4ac2b06f9b2a801177f35952715f32d3a7ca60af235e762",
            )
            .unwrap(),
        )
        .await
        .unwrap();

    let class = match class {
        ContractClass::Sierra(class) => class,
        _ => panic!("unexpected class type"),
    };

    assert!(!class.sierra_program.is_empty());
}

#[tokio::test]
async fn jsonrpc_get_class_hash_at() {
    let rpc_client = create_jsonrpc_client();

    let class_hash = rpc_client
        .get_class_hash_at(
            BlockId::Tag(BlockTag::Latest),
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
            BlockId::Tag(BlockTag::Latest),
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
        .get_block_transaction_count(BlockId::Number(20_000))
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
            BlockId::Tag(BlockTag::Latest),
        )
        .await
        .unwrap();

    assert!(eth_balance[0] > FieldElement::ZERO);
}

#[tokio::test]
async fn jsonrpc_estimate_fee() {
    let rpc_client = create_jsonrpc_client();

    let estimate = rpc_client
        .estimate_fee_single(
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
                    // TODO: make use of query version tx for estimating fees
                    is_query: false,
                },
            )),
            [],
            BlockId::Tag(BlockTag::Latest),
        )
        .await
        .unwrap();

    assert!(estimate.gas_consumed > FieldElement::ZERO);
    assert!(estimate.gas_price > FieldElement::ZERO);
    assert!(estimate.overall_fee > FieldElement::ZERO);
}

#[tokio::test]
async fn jsonrpc_estimate_message_fee() {
    let rpc_client = create_jsonrpc_client();

    let estimate = rpc_client
        .estimate_message_fee(
            MsgFromL1 {
                from_address: EthAddress::from_hex("0x0000000000000000000000000000000000000001")
                    .unwrap(),
                to_address: FieldElement::from_hex_be(
                    "07f7a88dc030eed907b634e2968693801ff56fdf71156a08f2c8e24aeb95371c",
                )
                .unwrap(),
                entry_point_selector: FieldElement::from_hex_be(
                    "00654a5600553e6e9d7023c67f1f597cebe39b6ba6b2a6cd63d86ec96d49d909",
                )
                .unwrap(),
                payload: vec![FieldElement::ONE],
            },
            BlockId::Tag(BlockTag::Latest),
        )
        .await
        .unwrap();

    assert!(estimate.gas_consumed > FieldElement::ZERO);
    assert!(estimate.gas_price > FieldElement::ZERO);
    assert!(estimate.overall_fee > FieldElement::ZERO);
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
                to_block: Some(BlockId::Number(235000)),
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
            BlockId::Tag(BlockTag::Latest),
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
async fn jsonrpc_trace_invoke() {
    let rpc_client = create_jsonrpc_client();

    let trace = rpc_client
        .trace_transaction(
            FieldElement::from_hex_be(
                "06d2ea57520318e577328ee0da9c609344ed77c86375a6764acc0c5854ebf258",
            )
            .unwrap(),
        )
        .await
        .unwrap();

    let trace = match trace {
        TransactionTrace::Invoke(trace) => trace,
        _ => panic!("unexpected trace type"),
    };

    match trace.execute_invocation {
        ExecuteInvocation::Success(_) => {}
        _ => panic!("unexpected execution result"),
    }
}

#[tokio::test]
async fn jsonrpc_trace_invoke_reverted() {
    let rpc_client = create_jsonrpc_client();

    let trace = rpc_client
        .trace_transaction(
            FieldElement::from_hex_be(
                "0555c9392299727de9d3d6c85dd5db94f63a0994e698386d85c12b16f71fbfd0",
            )
            .unwrap(),
        )
        .await
        .unwrap();

    let trace = match trace {
        TransactionTrace::Invoke(trace) => trace,
        _ => panic!("unexpected trace type"),
    };

    match trace.execute_invocation {
        ExecuteInvocation::Reverted(_) => {}
        _ => panic!("unexpected execution result"),
    }
}

#[tokio::test]
async fn jsonrpc_trace_l1_handler() {
    let rpc_client = create_jsonrpc_client();

    let trace = rpc_client
        .trace_transaction(
            FieldElement::from_hex_be(
                "0374286ae28f201e61ffbc5b022cc9701208640b405ea34ea9799f97d5d2d23c",
            )
            .unwrap(),
        )
        .await
        .unwrap();

    match trace {
        TransactionTrace::L1Handler(_) => {}
        _ => panic!("unexpected trace type"),
    }
}

#[tokio::test]
async fn jsonrpc_trace_declare() {
    let rpc_client = create_jsonrpc_client();

    let trace = rpc_client
        .trace_transaction(
            FieldElement::from_hex_be(
                "021933cb48e59c74caa4575a78e89e6046d043505e5600fd88af7f051d3610ca",
            )
            .unwrap(),
        )
        .await
        .unwrap();

    match trace {
        TransactionTrace::Declare(_) => {}
        _ => panic!("unexpected trace type"),
    }
}

// DEPLOY transactions cannot be traced

#[tokio::test]
async fn jsonrpc_trace_deploy_account() {
    let rpc_client = create_jsonrpc_client();

    let trace = rpc_client
        .trace_transaction(
            FieldElement::from_hex_be(
                "058ba7cdaf437d3a3b9680e6cbb4169811cddfa693875812bd98a8b1d61278de",
            )
            .unwrap(),
        )
        .await
        .unwrap();

    match trace {
        TransactionTrace::DeployAccount(_) => {}
        _ => panic!("unexpected trace type"),
    }
}

// NOTE: `addXxxxTransaction` methods are harder to test here since they require signatures. These
// are integration tests anyways, so we might as well just leave the job to th tests in
// `starknet-accounts`.
