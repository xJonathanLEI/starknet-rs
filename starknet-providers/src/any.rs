use async_trait::async_trait;
use starknet_core::types::{
    BlockHashAndNumber, BlockId, BroadcastedDeclareTransaction,
    BroadcastedDeployAccountTransaction, BroadcastedInvokeTransaction, BroadcastedTransaction,
    ContractClass, DeclareTransactionResult, DeployAccountTransactionResult, EventFilter,
    EventsPage, FeeEstimate, FieldElement, FunctionCall, InvokeTransactionResult,
    MaybePendingBlockWithTxHashes, MaybePendingBlockWithTxs, MaybePendingStateUpdate,
    MaybePendingTransactionReceipt, MsgFromL1, SimulatedTransaction, SimulationFlag,
    SimulationFlagForEstimateFee, SyncStatusType, Transaction, TransactionStatus, TransactionTrace,
    TransactionTraceWithHash,
};

use crate::{
    jsonrpc::{HttpTransport, JsonRpcClient},
    Provider, ProviderError, SequencerGatewayProvider,
};

/// A convenient Box-able type that implements the [Provider] trait. This can be useful when you
/// want to accept any built-in provider implementation from the library in your appliation, since
/// the [Provider] trait itself cannot be Box-ed due to the use of associated type.
///
/// A recommended pattern is to make your business logic code (e.g. functions) generic over the
/// [Provider] trait, while using this [AnyProvider] type for bootstrapping your application.
///
/// NOTE: This type was introduced when [Provider] was not Box-able. It should be reviewed whether
///       it's still needed anymore.
#[derive(Debug)]
pub enum AnyProvider {
    JsonRpcHttp(JsonRpcClient<HttpTransport>),
    SequencerGateway(SequencerGatewayProvider),
}

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
impl Provider for AnyProvider {
    async fn spec_version(&self) -> Result<String, ProviderError> {
        match self {
            Self::JsonRpcHttp(inner) => {
                <JsonRpcClient<HttpTransport> as Provider>::spec_version(inner).await
            }
            Self::SequencerGateway(inner) => {
                <SequencerGatewayProvider as Provider>::spec_version(inner).await
            }
        }
    }

    async fn get_block_with_tx_hashes<B>(
        &self,
        block_id: B,
    ) -> Result<MaybePendingBlockWithTxHashes, ProviderError>
    where
        B: AsRef<BlockId> + Send + Sync,
    {
        match self {
            Self::JsonRpcHttp(inner) => {
                <JsonRpcClient<HttpTransport> as Provider>::get_block_with_tx_hashes(
                    inner, block_id,
                )
                .await
            }
            Self::SequencerGateway(inner) => {
                <SequencerGatewayProvider as Provider>::get_block_with_tx_hashes(inner, block_id)
                    .await
            }
        }
    }

    async fn get_block_with_txs<B>(
        &self,
        block_id: B,
    ) -> Result<MaybePendingBlockWithTxs, ProviderError>
    where
        B: AsRef<BlockId> + Send + Sync,
    {
        match self {
            Self::JsonRpcHttp(inner) => {
                <JsonRpcClient<HttpTransport> as Provider>::get_block_with_txs(inner, block_id)
                    .await
            }
            Self::SequencerGateway(inner) => {
                <SequencerGatewayProvider as Provider>::get_block_with_txs(inner, block_id).await
            }
        }
    }

    async fn get_state_update<B>(
        &self,
        block_id: B,
    ) -> Result<MaybePendingStateUpdate, ProviderError>
    where
        B: AsRef<BlockId> + Send + Sync,
    {
        match self {
            Self::JsonRpcHttp(inner) => {
                <JsonRpcClient<HttpTransport> as Provider>::get_state_update(inner, block_id).await
            }
            Self::SequencerGateway(inner) => {
                <SequencerGatewayProvider as Provider>::get_state_update(inner, block_id).await
            }
        }
    }

    async fn get_storage_at<A, K, B>(
        &self,
        contract_address: A,
        key: K,
        block_id: B,
    ) -> Result<FieldElement, ProviderError>
    where
        A: AsRef<FieldElement> + Send + Sync,
        K: AsRef<FieldElement> + Send + Sync,
        B: AsRef<BlockId> + Send + Sync,
    {
        match self {
            Self::JsonRpcHttp(inner) => {
                <JsonRpcClient<HttpTransport> as Provider>::get_storage_at(
                    inner,
                    contract_address,
                    key,
                    block_id,
                )
                .await
            }
            Self::SequencerGateway(inner) => {
                <SequencerGatewayProvider as Provider>::get_storage_at(
                    inner,
                    contract_address,
                    key,
                    block_id,
                )
                .await
            }
        }
    }

    async fn get_transaction_status<H>(
        &self,
        transaction_hash: H,
    ) -> Result<TransactionStatus, ProviderError>
    where
        H: AsRef<FieldElement> + Send + Sync,
    {
        match self {
            Self::JsonRpcHttp(inner) => {
                <JsonRpcClient<HttpTransport> as Provider>::get_transaction_status(
                    inner,
                    transaction_hash,
                )
                .await
            }
            Self::SequencerGateway(inner) => {
                <SequencerGatewayProvider as Provider>::get_transaction_status(
                    inner,
                    transaction_hash,
                )
                .await
            }
        }
    }

    async fn get_transaction_by_hash<H>(
        &self,
        transaction_hash: H,
    ) -> Result<Transaction, ProviderError>
    where
        H: AsRef<FieldElement> + Send + Sync,
    {
        match self {
            Self::JsonRpcHttp(inner) => {
                <JsonRpcClient<HttpTransport> as Provider>::get_transaction_by_hash(
                    inner,
                    transaction_hash,
                )
                .await
            }
            Self::SequencerGateway(inner) => {
                <SequencerGatewayProvider as Provider>::get_transaction_by_hash(
                    inner,
                    transaction_hash,
                )
                .await
            }
        }
    }

    async fn get_transaction_by_block_id_and_index<B>(
        &self,
        block_id: B,
        index: u64,
    ) -> Result<Transaction, ProviderError>
    where
        B: AsRef<BlockId> + Send + Sync,
    {
        match self {
            Self::JsonRpcHttp(inner) => {
                <JsonRpcClient<HttpTransport> as Provider>::get_transaction_by_block_id_and_index(
                    inner, block_id, index,
                )
                .await
            }
            Self::SequencerGateway(inner) => {
                <SequencerGatewayProvider as Provider>::get_transaction_by_block_id_and_index(
                    inner, block_id, index,
                )
                .await
            }
        }
    }

    async fn get_transaction_receipt<H>(
        &self,
        transaction_hash: H,
    ) -> Result<MaybePendingTransactionReceipt, ProviderError>
    where
        H: AsRef<FieldElement> + Send + Sync,
    {
        match self {
            Self::JsonRpcHttp(inner) => {
                <JsonRpcClient<HttpTransport> as Provider>::get_transaction_receipt(
                    inner,
                    transaction_hash,
                )
                .await
            }
            Self::SequencerGateway(inner) => {
                <SequencerGatewayProvider as Provider>::get_transaction_receipt(
                    inner,
                    transaction_hash,
                )
                .await
            }
        }
    }

    async fn get_class<B, H>(
        &self,
        block_id: B,
        class_hash: H,
    ) -> Result<ContractClass, ProviderError>
    where
        B: AsRef<BlockId> + Send + Sync,
        H: AsRef<FieldElement> + Send + Sync,
    {
        match self {
            Self::JsonRpcHttp(inner) => {
                <JsonRpcClient<HttpTransport> as Provider>::get_class(inner, block_id, class_hash)
                    .await
            }
            Self::SequencerGateway(inner) => {
                <SequencerGatewayProvider as Provider>::get_class(inner, block_id, class_hash).await
            }
        }
    }

    async fn get_class_hash_at<B, A>(
        &self,
        block_id: B,
        contract_address: A,
    ) -> Result<FieldElement, ProviderError>
    where
        B: AsRef<BlockId> + Send + Sync,
        A: AsRef<FieldElement> + Send + Sync,
    {
        match self {
            Self::JsonRpcHttp(inner) => {
                <JsonRpcClient<HttpTransport> as Provider>::get_class_hash_at(
                    inner,
                    block_id,
                    contract_address,
                )
                .await
            }
            Self::SequencerGateway(inner) => {
                <SequencerGatewayProvider as Provider>::get_class_hash_at(
                    inner,
                    block_id,
                    contract_address,
                )
                .await
            }
        }
    }

    async fn get_class_at<B, A>(
        &self,
        block_id: B,
        contract_address: A,
    ) -> Result<ContractClass, ProviderError>
    where
        B: AsRef<BlockId> + Send + Sync,
        A: AsRef<FieldElement> + Send + Sync,
    {
        match self {
            Self::JsonRpcHttp(inner) => {
                <JsonRpcClient<HttpTransport> as Provider>::get_class_at(
                    inner,
                    block_id,
                    contract_address,
                )
                .await
            }
            Self::SequencerGateway(inner) => {
                <SequencerGatewayProvider as Provider>::get_class_at(
                    inner,
                    block_id,
                    contract_address,
                )
                .await
            }
        }
    }

    async fn get_block_transaction_count<B>(&self, block_id: B) -> Result<u64, ProviderError>
    where
        B: AsRef<BlockId> + Send + Sync,
    {
        match self {
            Self::JsonRpcHttp(inner) => {
                <JsonRpcClient<HttpTransport> as Provider>::get_block_transaction_count(
                    inner, block_id,
                )
                .await
            }
            Self::SequencerGateway(inner) => {
                <SequencerGatewayProvider as Provider>::get_block_transaction_count(inner, block_id)
                    .await
            }
        }
    }

    async fn call<R, B>(&self, request: R, block_id: B) -> Result<Vec<FieldElement>, ProviderError>
    where
        R: AsRef<FunctionCall> + Send + Sync,
        B: AsRef<BlockId> + Send + Sync,
    {
        match self {
            Self::JsonRpcHttp(inner) => {
                <JsonRpcClient<HttpTransport> as Provider>::call(inner, request, block_id).await
            }
            Self::SequencerGateway(inner) => {
                <SequencerGatewayProvider as Provider>::call(inner, request, block_id).await
            }
        }
    }

    async fn estimate_fee<R, S, B>(
        &self,
        request: R,
        simulation_flags: S,
        block_id: B,
    ) -> Result<Vec<FeeEstimate>, ProviderError>
    where
        R: AsRef<[BroadcastedTransaction]> + Send + Sync,
        S: AsRef<[SimulationFlagForEstimateFee]> + Send + Sync,
        B: AsRef<BlockId> + Send + Sync,
    {
        match self {
            Self::JsonRpcHttp(inner) => {
                <JsonRpcClient<HttpTransport> as Provider>::estimate_fee(
                    inner,
                    request,
                    simulation_flags,
                    block_id,
                )
                .await
            }
            Self::SequencerGateway(inner) => {
                <SequencerGatewayProvider as Provider>::estimate_fee(
                    inner,
                    request,
                    simulation_flags,
                    block_id,
                )
                .await
            }
        }
    }

    async fn estimate_message_fee<M, B>(
        &self,
        message: M,
        block_id: B,
    ) -> Result<FeeEstimate, ProviderError>
    where
        M: AsRef<MsgFromL1> + Send + Sync,
        B: AsRef<BlockId> + Send + Sync,
    {
        match self {
            Self::JsonRpcHttp(inner) => {
                <JsonRpcClient<HttpTransport> as Provider>::estimate_message_fee(
                    inner, message, block_id,
                )
                .await
            }
            Self::SequencerGateway(inner) => {
                <SequencerGatewayProvider as Provider>::estimate_message_fee(
                    inner, message, block_id,
                )
                .await
            }
        }
    }

    async fn block_number(&self) -> Result<u64, ProviderError> {
        match self {
            Self::JsonRpcHttp(inner) => {
                <JsonRpcClient<HttpTransport> as Provider>::block_number(inner).await
            }
            Self::SequencerGateway(inner) => {
                <SequencerGatewayProvider as Provider>::block_number(inner).await
            }
        }
    }

    async fn block_hash_and_number(&self) -> Result<BlockHashAndNumber, ProviderError> {
        match self {
            Self::JsonRpcHttp(inner) => {
                <JsonRpcClient<HttpTransport> as Provider>::block_hash_and_number(inner).await
            }
            Self::SequencerGateway(inner) => {
                <SequencerGatewayProvider as Provider>::block_hash_and_number(inner).await
            }
        }
    }

    async fn chain_id(&self) -> Result<FieldElement, ProviderError> {
        match self {
            Self::JsonRpcHttp(inner) => {
                <JsonRpcClient<HttpTransport> as Provider>::chain_id(inner).await
            }
            Self::SequencerGateway(inner) => {
                <SequencerGatewayProvider as Provider>::chain_id(inner).await
            }
        }
    }

    async fn syncing(&self) -> Result<SyncStatusType, ProviderError> {
        match self {
            Self::JsonRpcHttp(inner) => {
                <JsonRpcClient<HttpTransport> as Provider>::syncing(inner).await
            }
            Self::SequencerGateway(inner) => {
                <SequencerGatewayProvider as Provider>::syncing(inner).await
            }
        }
    }

    async fn get_events(
        &self,
        filter: EventFilter,
        continuation_token: Option<String>,
        chunk_size: u64,
    ) -> Result<EventsPage, ProviderError> {
        match self {
            Self::JsonRpcHttp(inner) => {
                <JsonRpcClient<HttpTransport> as Provider>::get_events(
                    inner,
                    filter,
                    continuation_token,
                    chunk_size,
                )
                .await
            }
            Self::SequencerGateway(inner) => {
                <SequencerGatewayProvider as Provider>::get_events(
                    inner,
                    filter,
                    continuation_token,
                    chunk_size,
                )
                .await
            }
        }
    }

    async fn get_nonce<B, A>(
        &self,
        block_id: B,
        contract_address: A,
    ) -> Result<FieldElement, ProviderError>
    where
        B: AsRef<BlockId> + Send + Sync,
        A: AsRef<FieldElement> + Send + Sync,
    {
        match self {
            Self::JsonRpcHttp(inner) => {
                <JsonRpcClient<HttpTransport> as Provider>::get_nonce(
                    inner,
                    block_id,
                    contract_address,
                )
                .await
            }
            Self::SequencerGateway(inner) => {
                <SequencerGatewayProvider as Provider>::get_nonce(inner, block_id, contract_address)
                    .await
            }
        }
    }

    async fn add_invoke_transaction<I>(
        &self,
        invoke_transaction: I,
    ) -> Result<InvokeTransactionResult, ProviderError>
    where
        I: AsRef<BroadcastedInvokeTransaction> + Send + Sync,
    {
        match self {
            Self::JsonRpcHttp(inner) => {
                <JsonRpcClient<HttpTransport> as Provider>::add_invoke_transaction(
                    inner,
                    invoke_transaction,
                )
                .await
            }
            Self::SequencerGateway(inner) => {
                <SequencerGatewayProvider as Provider>::add_invoke_transaction(
                    inner,
                    invoke_transaction,
                )
                .await
            }
        }
    }

    async fn add_declare_transaction<D>(
        &self,
        declare_transaction: D,
    ) -> Result<DeclareTransactionResult, ProviderError>
    where
        D: AsRef<BroadcastedDeclareTransaction> + Send + Sync,
    {
        match self {
            Self::JsonRpcHttp(inner) => {
                <JsonRpcClient<HttpTransport> as Provider>::add_declare_transaction(
                    inner,
                    declare_transaction,
                )
                .await
            }
            Self::SequencerGateway(inner) => {
                <SequencerGatewayProvider as Provider>::add_declare_transaction(
                    inner,
                    declare_transaction,
                )
                .await
            }
        }
    }

    async fn add_deploy_account_transaction<D>(
        &self,
        deploy_account_transaction: D,
    ) -> Result<DeployAccountTransactionResult, ProviderError>
    where
        D: AsRef<BroadcastedDeployAccountTransaction> + Send + Sync,
    {
        match self {
            Self::JsonRpcHttp(inner) => {
                <JsonRpcClient<HttpTransport> as Provider>::add_deploy_account_transaction(
                    inner,
                    deploy_account_transaction,
                )
                .await
            }
            Self::SequencerGateway(inner) => {
                <SequencerGatewayProvider as Provider>::add_deploy_account_transaction(
                    inner,
                    deploy_account_transaction,
                )
                .await
            }
        }
    }

    async fn trace_transaction<H>(
        &self,
        transaction_hash: H,
    ) -> Result<TransactionTrace, ProviderError>
    where
        H: AsRef<FieldElement> + Send + Sync,
    {
        match self {
            Self::JsonRpcHttp(inner) => {
                <JsonRpcClient<HttpTransport> as Provider>::trace_transaction(
                    inner,
                    transaction_hash,
                )
                .await
            }
            Self::SequencerGateway(inner) => {
                <SequencerGatewayProvider as Provider>::trace_transaction(inner, transaction_hash)
                    .await
            }
        }
    }

    async fn simulate_transactions<B, T, S>(
        &self,
        block_id: B,
        transactions: T,
        simulation_flags: S,
    ) -> Result<Vec<SimulatedTransaction>, ProviderError>
    where
        B: AsRef<BlockId> + Send + Sync,
        T: AsRef<[BroadcastedTransaction]> + Send + Sync,
        S: AsRef<[SimulationFlag]> + Send + Sync,
    {
        match self {
            Self::JsonRpcHttp(inner) => {
                <JsonRpcClient<HttpTransport> as Provider>::simulate_transactions(
                    inner,
                    block_id,
                    transactions,
                    simulation_flags,
                )
                .await
            }
            Self::SequencerGateway(inner) => {
                <SequencerGatewayProvider as Provider>::simulate_transactions(
                    inner,
                    block_id,
                    transactions,
                    simulation_flags,
                )
                .await
            }
        }
    }

    async fn trace_block_transactions<B>(
        &self,
        block_id: B,
    ) -> Result<Vec<TransactionTraceWithHash>, ProviderError>
    where
        B: AsRef<BlockId> + Send + Sync,
    {
        match self {
            Self::JsonRpcHttp(inner) => {
                <JsonRpcClient<HttpTransport> as Provider>::trace_block_transactions(
                    inner, block_id,
                )
                .await
            }
            Self::SequencerGateway(inner) => {
                <SequencerGatewayProvider as Provider>::trace_block_transactions(inner, block_id)
                    .await
            }
        }
    }
}
