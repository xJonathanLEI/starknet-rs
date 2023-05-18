#![allow(deprecated)]

use async_trait::async_trait;
use starknet_core::types::{
    BlockHashAndNumber, BlockId, BroadcastedDeclareTransaction,
    BroadcastedDeployAccountTransaction, BroadcastedDeployTransaction,
    BroadcastedInvokeTransaction, BroadcastedTransaction, ContractClass, DeclareTransactionResult,
    DeployAccountTransactionResult, DeployTransactionResult, EventFilter, EventsPage, FeeEstimate,
    FieldElement, FunctionCall, InvokeTransactionResult, MaybePendingBlockWithTxHashes,
    MaybePendingBlockWithTxs, MaybePendingTransactionReceipt, StarknetError, StateUpdate,
    SyncStatusType, Transaction,
};

use crate::{
    sequencer::{
        models::conversions::{ConversionError, TransactionWithReceipt},
        GatewayClientError,
    },
    Provider, ProviderError, SequencerGatewayProvider,
};

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
        Ok(self
            .get_state_update(block_id.as_ref().to_owned().into())
            .await?
            .into())
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
        Ok(self
            .get_storage_at(
                *contract_address.as_ref(),
                *key.as_ref(),
                block_id.as_ref().to_owned().into(),
            )
            .await?)
    }

    async fn get_transaction_by_hash<H>(
        &self,
        transaction_hash: H,
    ) -> Result<Transaction, ProviderError<Self::Error>>
    where
        H: AsRef<FieldElement> + Send + Sync,
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
    ) -> Result<Transaction, ProviderError<Self::Error>>
    where
        B: AsRef<BlockId> + Send + Sync,
    {
        let mut block = self.get_block(block_id.as_ref().to_owned().into()).await?;

        let index = index as usize;
        if index < block.transactions.len() {
            Ok(block.transactions.remove(index).try_into()?)
        } else {
            Err(ProviderError::<Self::Error>::StarknetError(
                StarknetError::InvalidTransactionIndex,
            ))
        }
    }

    async fn get_transaction_receipt<H>(
        &self,
        transaction_hash: H,
    ) -> Result<MaybePendingTransactionReceipt, ProviderError<Self::Error>>
    where
        H: AsRef<FieldElement> + Send + Sync,
    {
        let receipt = self
            .get_transaction_receipt(*transaction_hash.as_ref())
            .await?;

        // Even if it's `Received` we pretend it's not found to align with JSON-RPC
        if receipt.status == super::models::TransactionStatus::NotReceived
            || receipt.status == super::models::TransactionStatus::Received
        {
            Err(ProviderError::<Self::Error>::StarknetError(
                StarknetError::TransactionHashNotFound,
            ))
        } else {
            // JSON-RPC also sends tx type, which is not available in our receipt type
            let tx = self.get_transaction(*transaction_hash.as_ref()).await?;
            let tx = TransactionWithReceipt {
                transaction: tx,
                receipt,
            };

            Ok(tx.try_into()?)
        }
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
        Ok(self
            .get_class_hash_at(
                *contract_address.as_ref(),
                block_id.as_ref().to_owned().into(),
            )
            .await?)
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
        let block = self.get_block(block_id.as_ref().to_owned().into()).await?;
        Ok(block.transactions.len() as u64)
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
        Ok(self
            .call_contract(
                request.as_ref().to_owned().into(),
                block_id.as_ref().to_owned().into(),
            )
            .await?
            .result)
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
        Ok(self
            .estimate_fee(
                request.as_ref().to_owned().try_into()?,
                block_id.as_ref().to_owned().into(),
                false,
            )
            .await?
            .into())
    }

    async fn block_number(&self) -> Result<u64, ProviderError<Self::Error>> {
        let block = self.get_block(super::models::BlockId::Latest).await?;
        Ok(block.block_number.ok_or(ConversionError)?)
    }

    async fn block_hash_and_number(
        &self,
    ) -> Result<BlockHashAndNumber, ProviderError<Self::Error>> {
        let block = self.get_block(super::models::BlockId::Latest).await?;
        Ok(BlockHashAndNumber {
            block_hash: block.block_hash.ok_or(ConversionError)?,
            block_number: block.block_number.ok_or(ConversionError)?,
        })
    }

    async fn chain_id(&self) -> Result<FieldElement, ProviderError<Self::Error>> {
        Ok(self.chain_id)
    }

    async fn pending_transactions(&self) -> Result<Vec<Transaction>, ProviderError<Self::Error>> {
        let block = self.get_block(super::models::BlockId::Pending).await?;
        if block.status == super::models::BlockStatus::Pending {
            Ok(block
                .transactions
                .into_iter()
                .map(|tx| tx.try_into())
                .collect::<Result<_, _>>()?)
        } else {
            Ok(vec![])
        }
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
