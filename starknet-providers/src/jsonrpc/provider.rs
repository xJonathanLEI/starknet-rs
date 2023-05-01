use crate::{
    jsonrpc::{
        models::*, JsonRpcClient, JsonRpcClientError, JsonRpcError, JsonRpcTransport, RpcError,
    },
    Provider, ProviderError,
};

use async_trait::async_trait;
use starknet_core::types::{
    contract::{legacy::LegacyContractCode, CompiledClass, DeployedClass},
    AccountTransaction, AddTransactionResult, Block, BlockId, BlockTraces, CallContractResult,
    CallFunction, CallL1Handler, ContractAddresses, FeeEstimate, FieldElement, StateUpdate,
    TransactionInfo, TransactionReceipt, TransactionRequest, TransactionSimulationInfo,
    TransactionStatusInfo, TransactionTrace,
};

#[derive(Debug, thiserror::Error)]
pub enum JsonRpcProviderError<T> {
    #[error("Method not supported")]
    NotSupported,
    #[error("Failed to convert between JSON-RPC and sequencer gateway models")]
    ConversionError,
    #[error(transparent)]
    JsonError(serde_json::Error),
    #[error(transparent)]
    TransportError(T),
    #[error(transparent)]
    UnknownRpcError(JsonRpcError),
}

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
impl<T> Provider for JsonRpcClient<T>
where
    T: JsonRpcTransport + Sync + Send,
{
    type Error = JsonRpcProviderError<T::Error>;

    async fn add_transaction(
        &self,
        tx: TransactionRequest,
    ) -> Result<AddTransactionResult, ProviderError<Self::Error>> {
        match tx {
            TransactionRequest::Declare(tx) => self
                .add_declare_transaction(
                    &tx.try_into()
                        .map_err(|_| ProviderError::Other(Self::Error::ConversionError))?,
                )
                .await
                .map(|result| result.into())
                .map_err(|err| err.into()),
            TransactionRequest::InvokeFunction(tx) => self
                .add_invoke_transaction(&tx.into())
                .await
                .map(|result| result.into())
                .map_err(|err| err.into()),
            TransactionRequest::DeployAccount(tx) => self
                .add_deploy_account_transaction(&tx.into())
                .await
                .map(|result| result.into())
                .map_err(|err| err.into()),
        }
    }

    async fn get_contract_addresses(
        &self,
    ) -> Result<ContractAddresses, ProviderError<Self::Error>> {
        Err(ProviderError::Other(Self::Error::NotSupported))
    }

    async fn call_contract(
        &self,
        call_function: CallFunction,
        block_identifier: BlockId,
    ) -> Result<CallContractResult, ProviderError<Self::Error>> {
        self.call(
            <CallFunction as Into<FunctionCall>>::into(call_function),
            &block_identifier.into(),
        )
        .await
        .map(|result| CallContractResult { result })
        .map_err(|err| err.into())
    }

    async fn estimate_fee(
        &self,
        tx: AccountTransaction,
        block_identifier: BlockId,
        skip_validate: bool,
    ) -> Result<FeeEstimate, ProviderError<Self::Error>> {
        if skip_validate {
            return Err(ProviderError::Other(Self::Error::NotSupported));
        }

        let tx: BroadcastedTransaction = tx
            .try_into()
            .map_err(|_| ProviderError::Other(Self::Error::NotSupported))?;
        self.estimate_fee(tx, &block_identifier.into())
            .await
            .map(|est| est.into())
            .map_err(|err| err.into())
    }

    async fn estimate_fee_bulk(
        &self,
        _txs: &[AccountTransaction],
        _block_identifier: BlockId,
        _skip_validate: bool,
    ) -> Result<Vec<FeeEstimate>, ProviderError<Self::Error>> {
        Err(ProviderError::Other(Self::Error::NotSupported))
    }

    async fn estimate_message_fee(
        &self,
        _call_l1_handler: CallL1Handler,
        _block_identifier: BlockId,
    ) -> Result<FeeEstimate, ProviderError<Self::Error>> {
        Err(ProviderError::Other(Self::Error::NotSupported))
    }

    async fn simulate_transaction(
        &self,
        _tx: AccountTransaction,
        _block_identifier: BlockId,
        _skip_validate: bool,
    ) -> Result<TransactionSimulationInfo, ProviderError<Self::Error>> {
        Err(ProviderError::Other(Self::Error::NotSupported))
    }

    async fn get_block(
        &self,
        _block_identifier: BlockId,
    ) -> Result<Block, ProviderError<Self::Error>> {
        Err(ProviderError::Other(Self::Error::NotSupported))
    }

    async fn get_block_traces(
        &self,
        _block_identifier: BlockId,
    ) -> Result<BlockTraces, ProviderError<Self::Error>> {
        Err(ProviderError::Other(Self::Error::NotSupported))
    }

    async fn get_state_update(
        &self,
        _block_identifier: BlockId,
    ) -> Result<StateUpdate, ProviderError<Self::Error>> {
        Err(ProviderError::Other(Self::Error::NotSupported))
    }

    async fn get_code(
        &self,
        _contract_address: FieldElement,
        _block_identifier: BlockId,
    ) -> Result<LegacyContractCode, ProviderError<Self::Error>> {
        Err(ProviderError::Other(Self::Error::NotSupported))
    }

    async fn get_full_contract(
        &self,
        _contract_address: FieldElement,
        _block_identifier: BlockId,
    ) -> Result<DeployedClass, ProviderError<Self::Error>> {
        Err(ProviderError::Other(Self::Error::NotSupported))
    }

    async fn get_compiled_class_by_class_hash(
        &self,
        _class_hash: FieldElement,
        _block_identifier: BlockId,
    ) -> Result<CompiledClass, ProviderError<Self::Error>> {
        Err(ProviderError::Other(Self::Error::NotSupported))
    }

    async fn get_class_hash_at(
        &self,
        _contract_address: FieldElement,
        _block_identifier: BlockId,
    ) -> Result<FieldElement, ProviderError<Self::Error>> {
        Err(ProviderError::Other(Self::Error::NotSupported))
    }

    async fn get_class_by_hash(
        &self,
        _class_hash: FieldElement,
        _block_identifier: BlockId,
    ) -> Result<DeployedClass, ProviderError<Self::Error>> {
        Err(ProviderError::Other(Self::Error::NotSupported))
    }

    async fn get_storage_at(
        &self,
        contract_address: FieldElement,
        key: FieldElement,
        block_identifier: BlockId,
    ) -> Result<FieldElement, ProviderError<Self::Error>> {
        self.get_storage_at(contract_address, key, &block_identifier.into())
            .await
            .map_err(|err| err.into())
    }

    async fn get_nonce(
        &self,
        contract_address: FieldElement,
        block_identifier: BlockId,
    ) -> Result<FieldElement, ProviderError<Self::Error>> {
        self.get_nonce(&block_identifier.into(), contract_address)
            .await
            .map_err(|err| err.into())
    }

    async fn get_transaction_status(
        &self,
        transaction_hash: FieldElement,
    ) -> Result<TransactionStatusInfo, ProviderError<Self::Error>> {
        match self.get_transaction_receipt(transaction_hash).await {
            Ok(receipt) => Ok(receipt.into()),
            Err(JsonRpcClientError::RpcError(RpcError::Code(
                ErrorCode::TransactionHashNotFound,
            ))) => Ok(TransactionStatusInfo {
                block_hash: None,
                status: starknet_core::types::TransactionStatus::NotReceived,
                transaction_failure_reason: None,
            }),
            Err(err) => Err(err.into()),
        }
    }

    async fn get_transaction(
        &self,
        _transaction_hash: FieldElement,
    ) -> Result<TransactionInfo, ProviderError<Self::Error>> {
        Err(ProviderError::Other(Self::Error::NotSupported))
    }

    async fn get_transaction_receipt(
        &self,
        _transaction_hash: FieldElement,
    ) -> Result<TransactionReceipt, ProviderError<Self::Error>> {
        Err(ProviderError::Other(Self::Error::NotSupported))
    }

    async fn get_transaction_trace(
        &self,
        _transaction_hash: FieldElement,
    ) -> Result<TransactionTrace, ProviderError<Self::Error>> {
        Err(ProviderError::Other(Self::Error::NotSupported))
    }

    async fn get_block_hash_by_id(
        &self,
        _block_number: u64,
    ) -> Result<FieldElement, ProviderError<Self::Error>> {
        Err(ProviderError::Other(Self::Error::NotSupported))
    }

    async fn get_block_id_by_hash(
        &self,
        _block_hash: FieldElement,
    ) -> Result<u64, ProviderError<Self::Error>> {
        Err(ProviderError::Other(Self::Error::NotSupported))
    }

    async fn get_transaction_hash_by_id(
        &self,
        _transaction_number: u64,
    ) -> Result<FieldElement, ProviderError<Self::Error>> {
        Err(ProviderError::Other(Self::Error::NotSupported))
    }

    async fn get_transaction_id_by_hash(
        &self,
        _transaction_hash: FieldElement,
    ) -> Result<u64, ProviderError<Self::Error>> {
        Err(ProviderError::Other(Self::Error::NotSupported))
    }

    async fn get_last_batch_id(&self) -> Result<u64, ProviderError<Self::Error>> {
        Err(ProviderError::Other(Self::Error::NotSupported))
    }

    async fn get_l1_blockchain_id(&self) -> Result<u64, ProviderError<Self::Error>> {
        Err(ProviderError::Other(Self::Error::NotSupported))
    }
}

impl<T> From<JsonRpcClientError<T>> for ProviderError<JsonRpcProviderError<T>> {
    fn from(value: JsonRpcClientError<T>) -> Self {
        match value {
            JsonRpcClientError::JsonError(err) => {
                ProviderError::Other(JsonRpcProviderError::JsonError(err))
            }
            JsonRpcClientError::TransportError(err) => {
                ProviderError::Other(JsonRpcProviderError::TransportError(err))
            }
            JsonRpcClientError::RpcError(err) => match err {
                super::RpcError::Code(code) => ProviderError::StarknetError(code.into()),
                super::RpcError::Unknown(err) => {
                    ProviderError::Other(JsonRpcProviderError::UnknownRpcError(err))
                }
            },
        }
    }
}
