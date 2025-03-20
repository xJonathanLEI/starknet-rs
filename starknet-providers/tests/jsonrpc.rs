use starknet_core::{
    types::{
        requests::{CallRequest, GetBlockTransactionCountRequest},
        BlockId, BlockTag, BroadcastedInvokeTransaction, BroadcastedInvokeTransactionV1,
        BroadcastedTransaction, ContractClass, DeclareTransaction, DeployAccountTransaction,
        EthAddress, EventFilter, ExecuteInvocation, ExecutionResult, Felt, FunctionCall,
        InvokeTransaction, MaybePendingBlockWithReceipts, MaybePendingBlockWithTxHashes,
        MaybePendingBlockWithTxs, MaybePendingStateUpdate, MsgFromL1, StarknetError,
        SyncStatusType, Transaction, TransactionExecutionStatus, TransactionReceipt,
        TransactionStatus, TransactionTrace,
    },
    utils::{get_selector_from_name, get_storage_var_address},
};
use starknet_providers::{
    jsonrpc::{HttpTransport, JsonRpcClient},
    Provider, ProviderError, ProviderRequestData, ProviderResponseData,
};
use url::Url;

fn create_jsonrpc_client() -> JsonRpcClient<HttpTransport> {
    let rpc_url = std::env::var("STARKNET_RPC")
        .unwrap_or_else(|_| "https://pathfinder.rpc.sepolia.starknet.rs/rpc/v0_7".into());
    JsonRpcClient::new(HttpTransport::new(Url::parse(&rpc_url).unwrap()))
}

#[tokio::test]
async fn jsonrpc_spec_version() {
    let rpc_client = create_jsonrpc_client();

    let version = rpc_client.spec_version().await.unwrap();

    assert_eq!(version, "0.7.1");
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
async fn jsonrpc_get_block_with_receipts() {
    let rpc_client = create_jsonrpc_client();

    let block = rpc_client
        .get_block_with_receipts(BlockId::Tag(BlockTag::Latest))
        .await
        .unwrap();

    let block = match block {
        MaybePendingBlockWithReceipts::Block(block) => block,
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

    assert!(state_update.new_root > Felt::ZERO);
}

#[tokio::test]
async fn jsonrpc_get_storage_at() {
    let rpc_client = create_jsonrpc_client();

    // Checks L2 ETH balance via storage taking advantage of implementation detail
    let eth_balance = rpc_client
        .get_storage_at(
            Felt::from_hex("049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7")
                .unwrap(),
            get_storage_var_address(
                "ERC20_balances",
                &[Felt::from_hex(
                    "03f47d3911396b6d579fd7848cf576286ab6f96dda977915d6c7b10f3dd2315b",
                )
                .unwrap()],
            )
            .unwrap(),
            BlockId::Tag(BlockTag::Latest),
        )
        .await
        .unwrap();

    assert!(eth_balance > Felt::ZERO);
}

// Test case `jsonrpc_get_transaction_status_rejected` was removed as there is no `REJECTED`
// transaction on the Sepolia network.

#[tokio::test]
async fn jsonrpc_get_transaction_status_succeeded() {
    let rpc_client = create_jsonrpc_client();

    let status = rpc_client
        .get_transaction_status(
            Felt::from_hex("03f786ecc4955a2602c91a291328518ef866cb7f3d50e4b16fd42282952623aa")
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
            Felt::from_hex("02f00c7f28df2197196440747f97baa63d0851e3b0cfc2efedb6a88a7ef78cb1")
                .unwrap(),
        )
        .await
        .unwrap();

    match status {
        TransactionStatus::AcceptedOnL1(TransactionExecutionStatus::Reverted) => {}
        _ => panic!("unexpected transaction status"),
    }
}

// Test case `jsonrpc_get_transaction_by_hash_invoke_v0` was removed as there is no `INVOKE` v0
// transaction on the Sepolia network.

#[tokio::test]
async fn jsonrpc_get_transaction_by_hash_invoke_v1() {
    let rpc_client = create_jsonrpc_client();

    let tx = rpc_client
        .get_transaction_by_hash(
            Felt::from_hex("03f786ecc4955a2602c91a291328518ef866cb7f3d50e4b16fd42282952623aa")
                .unwrap(),
        )
        .await
        .unwrap();

    let tx = match tx {
        Transaction::Invoke(InvokeTransaction::V1(tx)) => tx,
        _ => panic!("unexpected tx response type"),
    };

    assert!(tx.sender_address > Felt::ZERO);
}

#[tokio::test]
async fn jsonrpc_get_transaction_by_hash_l1_handler() {
    let rpc_client = create_jsonrpc_client();

    let tx = rpc_client
        .get_transaction_by_hash(
            Felt::from_hex("0785c2ada3f53fbc66078d47715c27718f92e6e48b96372b36e5197de69b82b5")
                .unwrap(),
        )
        .await
        .unwrap();

    let tx = match tx {
        Transaction::L1Handler(tx) => tx,
        _ => panic!("unexpected tx response type"),
    };

    assert!(tx.entry_point_selector > Felt::ZERO);
}

#[tokio::test]
async fn jsonrpc_get_transaction_by_hash_declare_v0() {
    let rpc_client = create_jsonrpc_client();

    let tx = rpc_client
        .get_transaction_by_hash(
            Felt::from_hex("030a541df2547ed9f94602c35daf61ce3a8e179ec75d26cbe34e0ec61f823695")
                .unwrap(),
        )
        .await
        .unwrap();

    let tx = match tx {
        Transaction::Declare(DeclareTransaction::V0(tx)) => tx,
        _ => panic!("unexpected tx response type"),
    };

    assert!(tx.sender_address > Felt::ZERO);
}

#[tokio::test]
async fn jsonrpc_get_transaction_by_hash_declare_v1() {
    let rpc_client = create_jsonrpc_client();

    let tx = rpc_client
        .get_transaction_by_hash(
            Felt::from_hex("01936a09e5aaee208fc0f7cc826e126d421c3ac9aca2c789605e1e919e399185")
                .unwrap(),
        )
        .await
        .unwrap();

    let tx = match tx {
        Transaction::Declare(DeclareTransaction::V1(tx)) => tx,
        _ => panic!("unexpected tx response type"),
    };

    assert!(tx.sender_address > Felt::ZERO);
}

#[tokio::test]
async fn jsonrpc_get_transaction_by_hash_declare_v2() {
    let rpc_client = create_jsonrpc_client();

    let tx = rpc_client
        .get_transaction_by_hash(
            Felt::from_hex("004cacc2bbdd5ec77b20e908f311ab27d6495b69761e929bb24ba02632716944")
                .unwrap(),
        )
        .await
        .unwrap();

    let tx = match tx {
        Transaction::Declare(DeclareTransaction::V2(tx)) => tx,
        _ => panic!("unexpected tx response type"),
    };

    assert!(tx.sender_address > Felt::ZERO);
}

#[tokio::test]
async fn jsonrpc_get_transaction_by_hash_declare_v3() {
    let rpc_client = create_jsonrpc_client();

    let tx = rpc_client
        .get_transaction_by_hash(
            Felt::from_hex("054270d103c875a613e013d1fd555edcff2085feca9d7b4532243a8257fd5cf3")
                .unwrap(),
        )
        .await
        .unwrap();

    let tx = match tx {
        Transaction::Declare(DeclareTransaction::V3(tx)) => tx,
        _ => panic!("unexpected tx response type"),
    };

    assert!(tx.sender_address > Felt::ZERO);
}

// Test case `jsonrpc_get_transaction_by_hash_deploy` was removed as there is no `DEPLOY`
// transaction on the Sepolia network.

#[tokio::test]
async fn jsonrpc_get_transaction_by_hash_deploy_account_v1() {
    let rpc_client = create_jsonrpc_client();

    let tx = rpc_client
        .get_transaction_by_hash(
            Felt::from_hex("024ed6b82e2f6d3a811ec180a25c1ccd0bdc7bdba8ebd709de2ed697a1e82193")
                .unwrap(),
        )
        .await
        .unwrap();

    let tx = match tx {
        Transaction::DeployAccount(DeployAccountTransaction::V1(tx)) => tx,
        _ => panic!("unexpected tx response type"),
    };

    assert!(tx.class_hash > Felt::ZERO);
}

#[tokio::test]
async fn jsonrpc_get_transaction_by_hash_deploy_account_v3() {
    let rpc_client = create_jsonrpc_client();

    let tx = rpc_client
        .get_transaction_by_hash(
            Felt::from_hex("011c67fb3a9a623b3190c9ac41ebf7f5dd421f2583344c498a30a7280c660f01")
                .unwrap(),
        )
        .await
        .unwrap();

    let tx = match tx {
        Transaction::DeployAccount(DeployAccountTransaction::V3(tx)) => tx,
        _ => panic!("unexpected tx response type"),
    };

    assert!(tx.class_hash > Felt::ZERO);
}

#[tokio::test]
async fn jsonrpc_get_transaction_by_block_id_and_index() {
    let rpc_client = create_jsonrpc_client();

    let tx = rpc_client
        .get_transaction_by_block_id_and_index(BlockId::Number(10_000), 1)
        .await
        .unwrap();

    let tx = match tx {
        Transaction::Invoke(InvokeTransaction::V3(tx)) => tx,
        _ => panic!("unexpected tx response type"),
    };

    assert!(tx.sender_address > Felt::ZERO);
}

#[tokio::test]
async fn jsonrpc_get_transaction_by_hash_non_existent_tx() {
    let rpc_client = create_jsonrpc_client();

    let err = rpc_client
        .get_transaction_by_hash(Felt::from_hex("1234").unwrap())
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
            Felt::from_hex("03f786ecc4955a2602c91a291328518ef866cb7f3d50e4b16fd42282952623aa")
                .unwrap(),
        )
        .await
        .unwrap();

    assert!(receipt.block.is_block());

    let receipt = match receipt.receipt {
        TransactionReceipt::Invoke(receipt) => receipt,
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
            Felt::from_hex("02f00c7f28df2197196440747f97baa63d0851e3b0cfc2efedb6a88a7ef78cb1")
                .unwrap(),
        )
        .await
        .unwrap();

    assert!(receipt.block.is_block());

    let receipt = match receipt.receipt {
        TransactionReceipt::Invoke(receipt) => receipt,
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

    let tx_hash =
        Felt::from_hex("0785c2ada3f53fbc66078d47715c27718f92e6e48b96372b36e5197de69b82b5").unwrap();

    let tx = rpc_client.get_transaction_by_hash(tx_hash).await.unwrap();
    let receipt = rpc_client.get_transaction_receipt(tx_hash).await.unwrap();

    let tx = match tx {
        Transaction::L1Handler(tx) => tx,
        _ => panic!("unexpected tx type"),
    };

    assert!(receipt.block.is_block());

    let receipt = match receipt.receipt {
        TransactionReceipt::L1Handler(receipt) => receipt,
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
            Felt::from_hex("01936a09e5aaee208fc0f7cc826e126d421c3ac9aca2c789605e1e919e399185")
                .unwrap(),
        )
        .await
        .unwrap();

    assert!(receipt.block.is_block());

    let receipt = match receipt.receipt {
        TransactionReceipt::Declare(receipt) => receipt,
        _ => panic!("unexpected receipt response type"),
    };

    match receipt.execution_result {
        ExecutionResult::Succeeded => {}
        _ => panic!("unexpected execution result"),
    }
}

// Test case `jsonrpc_get_transaction_receipt_deploy` was removed as there is no `DEPLOY`
// transaction on the Sepolia network.

#[tokio::test]
async fn jsonrpc_get_transaction_receipt_deploy_account() {
    let rpc_client = create_jsonrpc_client();

    let receipt = rpc_client
        .get_transaction_receipt(
            Felt::from_hex("024ed6b82e2f6d3a811ec180a25c1ccd0bdc7bdba8ebd709de2ed697a1e82193")
                .unwrap(),
        )
        .await
        .unwrap();

    assert!(receipt.block.is_block());

    let receipt = match receipt.receipt {
        TransactionReceipt::DeployAccount(receipt) => receipt,
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
            Felt::from_hex("07b3e05f48f0c69e4a65ce5e076a66271a527aff2c34ce1083ec6e1526997a69")
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
            Felt::from_hex("01a736d6ed154502257f02b1ccdf4d9d1089f80811cd6acad48e6b6a9d1f2003")
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
            Felt::from_hex("041a78e741e5af2fec34b695679bc6891742439f7afb8484ecd7766661ad02bf")
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(
        class_hash,
        Felt::from_hex("07b3e05f48f0c69e4a65ce5e076a66271a527aff2c34ce1083ec6e1526997a69").unwrap()
    );
}

#[tokio::test]
async fn jsonrpc_get_class_at() {
    let rpc_client = create_jsonrpc_client();

    let class = rpc_client
        .get_class_at(
            BlockId::Tag(BlockTag::Latest),
            Felt::from_hex("041a78e741e5af2fec34b695679bc6891742439f7afb8484ecd7766661ad02bf")
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

    assert_eq!(count, 6);
}

#[tokio::test]
async fn jsonrpc_call() {
    let rpc_client = create_jsonrpc_client();

    // Checks L2 ETH balance
    let eth_balance = rpc_client
        .call(
            &FunctionCall {
                contract_address: Felt::from_hex(
                    "049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
                )
                .unwrap(),
                entry_point_selector: get_selector_from_name("balanceOf").unwrap(),
                calldata: vec![Felt::from_hex(
                    "03f47d3911396b6d579fd7848cf576286ab6f96dda977915d6c7b10f3dd2315b",
                )
                .unwrap()],
            },
            BlockId::Tag(BlockTag::Latest),
        )
        .await
        .unwrap();

    assert!(eth_balance[0] > Felt::ZERO);
}

#[tokio::test]
async fn jsonrpc_estimate_fee() {
    let rpc_client = create_jsonrpc_client();

    let estimate = rpc_client
        .estimate_fee_single(
            BroadcastedTransaction::Invoke(BroadcastedInvokeTransaction::V1(
                BroadcastedInvokeTransactionV1 {
                    max_fee: Felt::ZERO,
                    signature: vec![
                        Felt::from_hex(
                            "0024bd9efc809227bbcdfbd5a38b9255562184f944336c662037865dddda7a98",
                        )
                        .unwrap(),
                        Felt::from_hex(
                            "0647f552129f367c1053caeb722c3e1d5719032e229c08dbfde988bd87c9cc3e",
                        )
                        .unwrap(),
                    ],
                    nonce: Felt::ONE,
                    sender_address: Felt::from_hex(
                        "047e5089068f45ed6f7e1396157cd2346dfecbf1c77f396c03d45db3b164f5a0",
                    )
                    .unwrap(),
                    calldata: vec![
                        Felt::from_hex("1").unwrap(),
                        Felt::from_hex(
                            "049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
                        )
                        .unwrap(),
                        Felt::from_hex(
                            "0083afd3f4caedc6eebf44246fe54e38c95e3179a5ec9ea81740eca5b482d12e",
                        )
                        .unwrap(),
                        Felt::from_hex("3").unwrap(),
                        Felt::from_hex("1234").unwrap(),
                        Felt::from_hex("64").unwrap(),
                        Felt::from_hex("0").unwrap(),
                    ],
                    is_query: true,
                },
            )),
            [],
            BlockId::Tag(BlockTag::Latest),
        )
        .await
        .unwrap();

    assert!(estimate.gas_consumed > Felt::ZERO);
    assert!(estimate.gas_price > Felt::ZERO);
    assert!(estimate.overall_fee > Felt::ZERO);
}

#[tokio::test]
async fn jsonrpc_estimate_message_fee() {
    let rpc_client = create_jsonrpc_client();

    let estimate = rpc_client
        .estimate_message_fee(
            MsgFromL1 {
                from_address: EthAddress::from_hex("0x8453FC6Cd1bCfE8D4dFC069C400B433054d47bDc")
                    .unwrap(),
                to_address: Felt::from_hex(
                    "04c5772d1914fe6ce891b64eb35bf3522aeae1315647314aac58b01137607f3f",
                )
                .unwrap(),
                entry_point_selector: Felt::from_hex(
                    "02d757788a8d8d6f21d1cd40bce38a8222d70654214e96ff95d8086e684fbee5",
                )
                .unwrap(),
                payload: vec![Felt::ONE, Felt::ONE, Felt::ONE],
            },
            BlockId::Tag(BlockTag::Latest),
        )
        .await
        .unwrap();

    assert!(estimate.gas_consumed > Felt::ZERO);
    assert!(estimate.gas_price > Felt::ZERO);
    assert!(estimate.overall_fee > Felt::ZERO);
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

    assert!(id.block_hash > Felt::ZERO);
    assert!(id.block_number > 0);
}

#[tokio::test]
async fn jsonrpc_chain_id() {
    let rpc_client = create_jsonrpc_client();

    let chain_id = rpc_client.chain_id().await.unwrap();
    assert!(chain_id > Felt::ZERO);
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
                from_block: Some(BlockId::Number(10000)),
                to_block: Some(BlockId::Number(20000)),
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
            Felt::from_hex("047e5089068f45ed6f7e1396157cd2346dfecbf1c77f396c03d45db3b164f5a0")
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(nonce, Felt::ONE);
}

#[tokio::test]
async fn jsonrpc_trace_invoke() {
    let rpc_client = create_jsonrpc_client();

    let trace = rpc_client
        .trace_transaction(
            Felt::from_hex("006e02663371638622afe541481561f482b7a7cee4a7ce512080d492bf51616f")
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
            Felt::from_hex("034b1713234d9ac35dcd687afa997e7382229a51173de427b893977bae683104")
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
            Felt::from_hex("02ea7560de087494dd76d39bb10281ee51e2f8c590ad4b9848ab149171862a50")
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
            Felt::from_hex("0109156fb80fc5c7b112818e4c95fe44749cdd4d7217a8aa4e99be905cdd7e32")
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
            Felt::from_hex("07a7dd7e084a762aba238fe5ba27bda822a9f9e456ed49baeeb64a2e2ebc1586")
                .unwrap(),
        )
        .await
        .unwrap();

    match trace {
        TransactionTrace::DeployAccount(_) => {}
        _ => panic!("unexpected trace type"),
    }
}

#[tokio::test]
async fn jsonrpc_batch() {
    let rpc_client = create_jsonrpc_client();

    let responses = rpc_client
        .batch_requests([
            ProviderRequestData::GetBlockTransactionCount(GetBlockTransactionCountRequest {
                block_id: BlockId::Number(20_000),
            }),
            ProviderRequestData::Call(CallRequest {
                request: FunctionCall {
                    contract_address: Felt::from_hex(
                        "049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
                    )
                    .unwrap(),
                    entry_point_selector: get_selector_from_name("balanceOf").unwrap(),
                    calldata: vec![Felt::from_hex(
                        "03f47d3911396b6d579fd7848cf576286ab6f96dda977915d6c7b10f3dd2315b",
                    )
                    .unwrap()],
                },
                block_id: BlockId::Tag(BlockTag::Latest),
            }),
        ])
        .await
        .unwrap();

    match &responses[0] {
        ProviderResponseData::GetBlockTransactionCount(count) => {
            assert_eq!(*count, 6);
        }
        _ => panic!("unexpected response type"),
    }

    match &responses[1] {
        ProviderResponseData::Call(eth_balance) => {
            assert!(eth_balance[0] > Felt::ZERO);
        }
        _ => panic!("unexpected response type"),
    }
}

// NOTE: `addXxxxTransaction` methods are harder to test here since they require signatures. These
// are integration tests anyways, so we might as well just leave the job to th tests in
// `starknet-accounts`.
