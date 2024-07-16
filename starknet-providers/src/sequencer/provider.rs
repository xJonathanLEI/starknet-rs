#![allow(deprecated)]

use std::any::Any;

use async_trait::async_trait;
use starknet_core::types::{
    BlockHashAndNumber, BlockId, BroadcastedDeclareTransaction,
    BroadcastedDeployAccountTransaction, BroadcastedInvokeTransaction, BroadcastedTransaction,
    ContractClass, DeclareTransactionResult, DeployAccountTransactionResult, EventFilter,
    EventsPage, FeeEstimate, Felt, FunctionCall, InvokeTransactionResult,
    MaybePendingBlockWithReceipts, MaybePendingBlockWithTxHashes, MaybePendingBlockWithTxs,
    MaybePendingStateUpdate, MsgFromL1, SimulatedTransaction, SimulationFlag,
    SimulationFlagForEstimateFee, StarknetError, SyncStatusType, Transaction,
    TransactionReceiptWithBlockInfo, TransactionStatus, TransactionTrace, TransactionTraceWithHash,
};

use crate::{
    provider::ProviderImplError,
    sequencer::{models::conversions::ConversionError, GatewayClientError},
    Provider, ProviderError, SequencerGatewayProvider,
};

use super::models::TransactionFinalityStatus;

#[allow(unused)]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
impl Provider for SequencerGatewayProvider {
    async fn spec_version(&self) -> Result<String, ProviderError> {
        Ok(String::from("0.7.1"))
    }

    async fn get_block_with_tx_hashes<B>(
        &self,
        block_id: B,
    ) -> Result<MaybePendingBlockWithTxHashes, ProviderError>
    where
        B: AsRef<BlockId> + Send + Sync,
    {
        Ok(self
            .get_block(block_id.as_ref().to_owned().into())
            .await?
            .try_into()?)
    }

    async fn get_block_with_txs<B>(
        &self,
        block_id: B,
    ) -> Result<MaybePendingBlockWithTxs, ProviderError>
    where
        B: AsRef<BlockId> + Send + Sync,
    {
        Ok(self
            .get_block(block_id.as_ref().to_owned().into())
            .await?
            .try_into()?)
    }

    async fn get_block_with_receipts<B>(
        &self,
        block_id: B,
    ) -> Result<MaybePendingBlockWithReceipts, ProviderError>
    where
        B: AsRef<BlockId> + Send + Sync,
    {
        Ok(self
            .get_block(block_id.as_ref().to_owned().into())
            .await?
            .try_into()?)
    }

    async fn get_state_update<B>(
        &self,
        block_id: B,
    ) -> Result<MaybePendingStateUpdate, ProviderError>
    where
        B: AsRef<BlockId> + Send + Sync,
    {
        Ok(self
            .get_state_update(block_id.as_ref().to_owned().into())
            .await?
            .try_into()?)
    }

    async fn get_storage_at<A, K, B>(
        &self,
        contract_address: A,
        key: K,
        block_id: B,
    ) -> Result<Felt, ProviderError>
    where
        A: AsRef<Felt> + Send + Sync,
        K: AsRef<Felt> + Send + Sync,
        B: AsRef<BlockId> + Send + Sync,
    {
        // Deprecated since Starknet v0.12.3
        Err(ProviderError::Other(Box::new(
            GatewayClientError::MethodNotSupported,
        )))
    }

    /// Gets the transaction status (possibly reflecting that the tx is still in
    /// the mempool, or dropped from it)
    async fn get_transaction_status<H>(
        &self,
        transaction_hash: H,
    ) -> Result<TransactionStatus, ProviderError>
    where
        H: AsRef<Felt> + Send + Sync,
    {
        let status = self
            .get_transaction_status(*transaction_hash.as_ref())
            .await?;

        // `NotReceived` is not a valid status for JSON-RPC. It's an error.
        if matches!(
            &status.finality_status,
            Some(TransactionFinalityStatus::NotReceived)
        ) {
            return Err(ProviderError::StarknetError(
                StarknetError::TransactionHashNotFound,
            ));
        }

        Ok(status.try_into()?)
    }

    async fn get_transaction_by_hash<H>(
        &self,
        transaction_hash: H,
    ) -> Result<Transaction, ProviderError>
    where
        H: AsRef<Felt> + Send + Sync,
    {
        Ok(self
            .get_transaction(*transaction_hash.as_ref())
            .await?
            .try_into()?)
    }

    async fn get_transaction_by_block_id_and_index<B>(
        &self,
        block_id: B,
        index: u64,
    ) -> Result<Transaction, ProviderError>
    where
        B: AsRef<BlockId> + Send + Sync,
    {
        let mut block = self.get_block(block_id.as_ref().to_owned().into()).await?;

        let index = index as usize;
        if index < block.transactions.len() {
            Ok(block.transactions.remove(index).try_into()?)
        } else {
            Err(ProviderError::StarknetError(
                StarknetError::InvalidTransactionIndex,
            ))
        }
    }

    async fn get_transaction_receipt<H>(
        &self,
        transaction_hash: H,
    ) -> Result<TransactionReceiptWithBlockInfo, ProviderError>
    where
        H: AsRef<Felt> + Send + Sync,
    {
        // Deprecated since Starknet v0.12.3
        Err(ProviderError::Other(Box::new(
            GatewayClientError::MethodNotSupported,
        )))
    }

    async fn get_class<B, H>(
        &self,
        block_id: B,
        class_hash: H,
    ) -> Result<ContractClass, ProviderError>
    where
        B: AsRef<BlockId> + Send + Sync,
        H: AsRef<Felt> + Send + Sync,
    {
        Ok(self
            .get_class_by_hash(*class_hash.as_ref(), block_id.as_ref().to_owned().into())
            .await?
            .try_into()?)
    }

    async fn get_class_hash_at<B, A>(
        &self,
        block_id: B,
        contract_address: A,
    ) -> Result<Felt, ProviderError>
    where
        B: AsRef<BlockId> + Send + Sync,
        A: AsRef<Felt> + Send + Sync,
    {
        // Deprecated since Starknet v0.12.3
        Err(ProviderError::Other(Box::new(
            GatewayClientError::MethodNotSupported,
        )))
    }

    async fn get_class_at<B, A>(
        &self,
        block_id: B,
        contract_address: A,
    ) -> Result<ContractClass, ProviderError>
    where
        B: AsRef<BlockId> + Send + Sync,
        A: AsRef<Felt> + Send + Sync,
    {
        // Deprecated since Starknet v0.12.3
        Err(ProviderError::Other(Box::new(
            GatewayClientError::MethodNotSupported,
        )))
    }

    async fn get_block_transaction_count<B>(&self, block_id: B) -> Result<u64, ProviderError>
    where
        B: AsRef<BlockId> + Send + Sync,
    {
        let block = self.get_block(block_id.as_ref().to_owned().into()).await?;
        Ok(block.transactions.len() as u64)
    }

    async fn call<R, B>(&self, request: R, block_id: B) -> Result<Vec<Felt>, ProviderError>
    where
        R: AsRef<FunctionCall> + Send + Sync,
        B: AsRef<BlockId> + Send + Sync,
    {
        // Deprecated since Starknet v0.12.3
        Err(ProviderError::Other(Box::new(
            GatewayClientError::MethodNotSupported,
        )))
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
        // Deprecated since Starknet v0.12.3
        Err(ProviderError::Other(Box::new(
            GatewayClientError::MethodNotSupported,
        )))
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
        // Deprecated since Starknet v0.12.3
        Err(ProviderError::Other(Box::new(
            GatewayClientError::MethodNotSupported,
        )))
    }

    async fn block_number(&self) -> Result<u64, ProviderError> {
        let block = self.get_block(super::models::BlockId::Latest).await?;
        Ok(block.block_number.ok_or(ConversionError)?)
    }

    async fn block_hash_and_number(&self) -> Result<BlockHashAndNumber, ProviderError> {
        let block = self.get_block(super::models::BlockId::Latest).await?;
        Ok(BlockHashAndNumber {
            block_hash: block.block_hash.ok_or(ConversionError)?,
            block_number: block.block_number.ok_or(ConversionError)?,
        })
    }

    async fn chain_id(&self) -> Result<Felt, ProviderError> {
        Ok(self.chain_id)
    }

    async fn syncing(&self) -> Result<SyncStatusType, ProviderError> {
        Ok(SyncStatusType::NotSyncing)
    }

    async fn get_events(
        &self,
        filter: EventFilter,
        continuation_token: Option<String>,
        chunk_size: u64,
    ) -> Result<EventsPage, ProviderError> {
        Err(ProviderError::Other(Box::new(
            GatewayClientError::MethodNotSupported,
        )))
    }

    async fn get_nonce<B, A>(&self, block_id: B, contract_address: A) -> Result<Felt, ProviderError>
    where
        B: AsRef<BlockId> + Send + Sync,
        A: AsRef<Felt> + Send + Sync,
    {
        // Deprecated since Starknet v0.12.3
        Err(ProviderError::Other(Box::new(
            GatewayClientError::MethodNotSupported,
        )))
    }

    async fn add_invoke_transaction<I>(
        &self,
        invoke_transaction: I,
    ) -> Result<InvokeTransactionResult, ProviderError>
    where
        I: AsRef<BroadcastedInvokeTransaction> + Send + Sync,
    {
        let result = self
            .add_transaction(super::models::TransactionRequest::InvokeFunction(
                invoke_transaction.as_ref().to_owned().into(),
            ))
            .await?;

        Ok(InvokeTransactionResult {
            transaction_hash: result.transaction_hash,
        })
    }

    async fn add_declare_transaction<D>(
        &self,
        declare_transaction: D,
    ) -> Result<DeclareTransactionResult, ProviderError>
    where
        D: AsRef<BroadcastedDeclareTransaction> + Send + Sync,
    {
        let result = self
            .add_transaction(super::models::TransactionRequest::Declare(
                declare_transaction.as_ref().to_owned().try_into()?,
            ))
            .await?;

        Ok(DeclareTransactionResult {
            transaction_hash: result.transaction_hash,
            class_hash: result.class_hash.ok_or(ConversionError)?,
        })
    }

    async fn add_deploy_account_transaction<D>(
        &self,
        deploy_account_transaction: D,
    ) -> Result<DeployAccountTransactionResult, ProviderError>
    where
        D: AsRef<BroadcastedDeployAccountTransaction> + Send + Sync,
    {
        let result = self
            .add_transaction(super::models::TransactionRequest::DeployAccount(
                deploy_account_transaction.as_ref().to_owned().into(),
            ))
            .await?;

        Ok(DeployAccountTransactionResult {
            transaction_hash: result.transaction_hash,
            contract_address: result.address.ok_or(ConversionError)?,
        })
    }

    async fn trace_transaction<H>(
        &self,
        _transaction_hash: H,
    ) -> Result<TransactionTrace, ProviderError>
    where
        H: AsRef<Felt> + Send + Sync,
    {
        // With JSON-RPC v0.5.0 it's no longer possible to convert feeder traces to JSON-RPC traces. So we simply pretend that it's not supported here.
        //
        // This is fine as the feeder gateway is soon to be removed anyways.
        Err(ProviderError::Other(Box::new(
            GatewayClientError::MethodNotSupported,
        )))
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
        // With JSON-RPC v0.5.0 it's no longer possible to convert feeder traces to JSON-RPC traces. So we simply pretend that it's not supported here.
        //
        // This is fine as the feeder gateway is soon to be removed anyways.
        Err(ProviderError::Other(Box::new(
            GatewayClientError::MethodNotSupported,
        )))
    }

    async fn trace_block_transactions<B>(
        &self,
        block_id: B,
    ) -> Result<Vec<TransactionTraceWithHash>, ProviderError>
    where
        B: AsRef<BlockId> + Send + Sync,
    {
        // With JSON-RPC v0.5.0 it's no longer possible to convert feeder traces to JSON-RPC traces. So we simply pretend that it's not supported here.
        //
        // This is fine as the feeder gateway is soon to be removed anyways.
        Err(ProviderError::Other(Box::new(
            GatewayClientError::MethodNotSupported,
        )))
    }
}

impl ProviderImplError for GatewayClientError {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl From<GatewayClientError> for ProviderError {
    fn from(value: GatewayClientError) -> Self {
        Self::Other(Box::new(value))
    }
}
