#![allow(deprecated)]

use async_trait::async_trait;
use starknet_core::types::{
    BlockHashAndNumber, BlockId, BroadcastedDeclareTransaction,
    BroadcastedDeployAccountTransaction, BroadcastedDeployTransaction,
    BroadcastedInvokeTransaction, BroadcastedTransaction, ContractClass, DeclareTransactionResult,
    DeployAccountTransactionResult, DeployTransactionResult, EventFilter, EventsPage, FeeEstimate,
    FieldElement, FunctionCall, InvokeTransactionResult, MaybePendingBlockWithTxHashes,
    MaybePendingBlockWithTxs, MaybePendingTransactionReceipt, StateUpdate, SyncStatusType,
    Transaction,
};

use crate::{sequencer::GatewayClientError, Provider, ProviderError, SequencerGatewayProvider};

#[allow(unused)]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
impl Provider for SequencerGatewayProvider {
    type Error = GatewayClientError;

    async fn get_block_with_tx_hashes<B>(
        &self,
        block_id: B,
    ) -> Result<MaybePendingBlockWithTxHashes, ProviderError<Self::Error>>
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
    ) -> Result<MaybePendingBlockWithTxs, ProviderError<Self::Error>>
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
    ) -> Result<StateUpdate, ProviderError<Self::Error>>
    where
        B: AsRef<BlockId> + Send + Sync,
    {
        Err(ProviderError::Other(Self::Error::MethodNotSupported))
    }

    async fn get_storage_at<A, K, B>(
        &self,
        contract_address: A,
        key: K,
        block_id: B,
    ) -> Result<FieldElement, ProviderError<Self::Error>>
    where
        A: AsRef<FieldElement> + Send + Sync,
        K: AsRef<FieldElement> + Send + Sync,
        B: AsRef<BlockId> + Send + Sync,
    {
        Err(ProviderError::Other(Self::Error::MethodNotSupported))
    }

    async fn get_transaction_by_hash<H>(
        &self,
        transaction_hash: H,
    ) -> Result<Transaction, ProviderError<Self::Error>>
    where
        H: AsRef<FieldElement> + Send + Sync,
    {
        Err(ProviderError::Other(Self::Error::MethodNotSupported))
    }

    async fn get_transaction_by_block_id_and_index<B>(
        &self,
        block_id: B,
        index: u64,
    ) -> Result<Transaction, ProviderError<Self::Error>>
    where
        B: AsRef<BlockId> + Send + Sync,
    {
        Err(ProviderError::Other(Self::Error::MethodNotSupported))
    }

    async fn get_transaction_receipt<H>(
        &self,
        transaction_hash: H,
    ) -> Result<MaybePendingTransactionReceipt, ProviderError<Self::Error>>
    where
        H: AsRef<FieldElement> + Send + Sync,
    {
        Err(ProviderError::Other(Self::Error::MethodNotSupported))
    }

    async fn get_class<B, H>(
        &self,
        block_id: B,
        class_hash: H,
    ) -> Result<ContractClass, ProviderError<Self::Error>>
    where
        B: AsRef<BlockId> + Send + Sync,
        H: AsRef<FieldElement> + Send + Sync,
    {
        Err(ProviderError::Other(Self::Error::MethodNotSupported))
    }

    async fn get_class_hash_at<B, A>(
        &self,
        block_id: B,
        contract_address: A,
    ) -> Result<FieldElement, ProviderError<Self::Error>>
    where
        B: AsRef<BlockId> + Send + Sync,
        A: AsRef<FieldElement> + Send + Sync,
    {
        Err(ProviderError::Other(Self::Error::MethodNotSupported))
    }

    async fn get_class_at<B, A>(
        &self,
        block_id: B,
        contract_address: A,
    ) -> Result<ContractClass, ProviderError<Self::Error>>
    where
        B: AsRef<BlockId> + Send + Sync,
        A: AsRef<FieldElement> + Send + Sync,
    {
        Err(ProviderError::Other(Self::Error::MethodNotSupported))
    }

    async fn get_block_transaction_count<B>(
        &self,
        block_id: B,
    ) -> Result<u64, ProviderError<Self::Error>>
    where
        B: AsRef<BlockId> + Send + Sync,
    {
        Err(ProviderError::Other(Self::Error::MethodNotSupported))
    }

    async fn call<R, B>(
        &self,
        request: R,
        block_id: B,
    ) -> Result<Vec<FieldElement>, ProviderError<Self::Error>>
    where
        R: AsRef<FunctionCall> + Send + Sync,
        B: AsRef<BlockId> + Send + Sync,
    {
        Err(ProviderError::Other(Self::Error::MethodNotSupported))
    }

    async fn estimate_fee<R, B>(
        &self,
        request: R,
        block_id: B,
    ) -> Result<FeeEstimate, ProviderError<Self::Error>>
    where
        R: AsRef<BroadcastedTransaction> + Send + Sync,
        B: AsRef<BlockId> + Send + Sync,
    {
        Err(ProviderError::Other(Self::Error::MethodNotSupported))
    }

    async fn block_number(&self) -> Result<u64, ProviderError<Self::Error>> {
        Err(ProviderError::Other(Self::Error::MethodNotSupported))
    }

    async fn block_hash_and_number(
        &self,
    ) -> Result<BlockHashAndNumber, ProviderError<Self::Error>> {
        Err(ProviderError::Other(Self::Error::MethodNotSupported))
    }

    async fn chain_id(&self) -> Result<FieldElement, ProviderError<Self::Error>> {
        Err(ProviderError::Other(Self::Error::MethodNotSupported))
    }

    async fn pending_transactions(&self) -> Result<Vec<Transaction>, ProviderError<Self::Error>> {
        Err(ProviderError::Other(Self::Error::MethodNotSupported))
    }

    async fn syncing(&self) -> Result<SyncStatusType, ProviderError<Self::Error>> {
        Err(ProviderError::Other(Self::Error::MethodNotSupported))
    }

    async fn get_events(
        &self,
        filter: EventFilter,
        continuation_token: Option<String>,
        chunk_size: u64,
    ) -> Result<EventsPage, ProviderError<Self::Error>> {
        Err(ProviderError::Other(Self::Error::MethodNotSupported))
    }

    async fn get_nonce<B, A>(
        &self,
        block_id: B,
        contract_address: A,
    ) -> Result<FieldElement, ProviderError<Self::Error>>
    where
        B: AsRef<BlockId> + Send + Sync,
        A: AsRef<FieldElement> + Send + Sync,
    {
        Err(ProviderError::Other(Self::Error::MethodNotSupported))
    }

    async fn add_invoke_transaction<I>(
        &self,
        invoke_transaction: I,
    ) -> Result<InvokeTransactionResult, ProviderError<Self::Error>>
    where
        I: AsRef<BroadcastedInvokeTransaction> + Send + Sync,
    {
        Err(ProviderError::Other(Self::Error::MethodNotSupported))
    }

    async fn add_declare_transaction<D>(
        &self,
        declare_transaction: D,
    ) -> Result<DeclareTransactionResult, ProviderError<Self::Error>>
    where
        D: AsRef<BroadcastedDeclareTransaction> + Send + Sync,
    {
        Err(ProviderError::Other(Self::Error::MethodNotSupported))
    }

    async fn add_deploy_transaction<D>(
        &self,
        deploy_transaction: D,
    ) -> Result<DeployTransactionResult, ProviderError<Self::Error>>
    where
        D: AsRef<BroadcastedDeployTransaction> + Send + Sync,
    {
        Err(ProviderError::Other(Self::Error::MethodNotSupported))
    }

    async fn add_deploy_account_transaction<D>(
        &self,
        deploy_account_transaction: D,
    ) -> Result<DeployAccountTransactionResult, ProviderError<Self::Error>>
    where
        D: AsRef<BroadcastedDeployAccountTransaction> + Send + Sync,
    {
        Err(ProviderError::Other(Self::Error::MethodNotSupported))
    }
}
