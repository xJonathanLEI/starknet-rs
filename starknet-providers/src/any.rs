use async_trait::async_trait;
use starknet_core::types::{
    contract::{legacy::LegacyContractCode, CompiledClass, DeployedClass},
    AccountTransaction, AddTransactionResult, Block, BlockId, BlockTraces, CallContractResult,
    CallFunction, CallL1Handler, ContractAddresses, FeeEstimate, FieldElement, StateUpdate,
    TransactionInfo, TransactionReceipt, TransactionRequest, TransactionSimulationInfo,
    TransactionStatusInfo, TransactionTrace,
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
#[derive(Debug)]
pub enum AnyProvider {
    JsonRpcHttp(JsonRpcClient<HttpTransport>),
    SequencerGateway(SequencerGatewayProvider),
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub enum AnyProviderError {
    JsonRpcHttp(<JsonRpcClient<HttpTransport> as Provider>::Error),
    SequencerGateway(<SequencerGatewayProvider as Provider>::Error),
}

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
impl Provider for AnyProvider {
    type Error = AnyProviderError;

    async fn add_transaction(
        &self,
        tx: TransactionRequest,
    ) -> Result<AddTransactionResult, ProviderError<Self::Error>> {
        match self {
            Self::JsonRpcHttp(inner) => {
                Ok(<JsonRpcClient<HttpTransport> as Provider>::add_transaction(inner, tx).await?)
            }
            Self::SequencerGateway(inner) => {
                Ok(<SequencerGatewayProvider as Provider>::add_transaction(inner, tx).await?)
            }
        }
    }

    async fn get_contract_addresses(
        &self,
    ) -> Result<ContractAddresses, ProviderError<Self::Error>> {
        match self {
            Self::JsonRpcHttp(inner) => Ok(
                <JsonRpcClient<HttpTransport> as Provider>::get_contract_addresses(inner).await?,
            ),
            Self::SequencerGateway(inner) => {
                Ok(<SequencerGatewayProvider as Provider>::get_contract_addresses(inner).await?)
            }
        }
    }

    async fn call_contract(
        &self,
        call_function: CallFunction,
        block_identifier: BlockId,
    ) -> Result<CallContractResult, ProviderError<Self::Error>> {
        match self {
            Self::JsonRpcHttp(inner) => {
                Ok(<JsonRpcClient<HttpTransport> as Provider>::call_contract(
                    inner,
                    call_function,
                    block_identifier,
                )
                .await?)
            }
            Self::SequencerGateway(inner) => {
                Ok(<SequencerGatewayProvider as Provider>::call_contract(
                    inner,
                    call_function,
                    block_identifier,
                )
                .await?)
            }
        }
    }

    async fn estimate_fee(
        &self,
        tx: AccountTransaction,
        block_identifier: BlockId,
        skip_validate: bool,
    ) -> Result<FeeEstimate, ProviderError<Self::Error>> {
        match self {
            Self::JsonRpcHttp(inner) => {
                Ok(<JsonRpcClient<HttpTransport> as Provider>::estimate_fee(
                    inner,
                    tx,
                    block_identifier,
                    skip_validate,
                )
                .await?)
            }
            Self::SequencerGateway(inner) => {
                Ok(<SequencerGatewayProvider as Provider>::estimate_fee(
                    inner,
                    tx,
                    block_identifier,
                    skip_validate,
                )
                .await?)
            }
        }
    }

    async fn estimate_fee_bulk(
        &self,
        txs: &[AccountTransaction],
        block_identifier: BlockId,
        skip_validate: bool,
    ) -> Result<Vec<FeeEstimate>, ProviderError<Self::Error>> {
        match self {
            Self::JsonRpcHttp(inner) => Ok(
                <JsonRpcClient<HttpTransport> as Provider>::estimate_fee_bulk(
                    inner,
                    txs,
                    block_identifier,
                    skip_validate,
                )
                .await?,
            ),
            Self::SequencerGateway(inner) => {
                Ok(<SequencerGatewayProvider as Provider>::estimate_fee_bulk(
                    inner,
                    txs,
                    block_identifier,
                    skip_validate,
                )
                .await?)
            }
        }
    }

    async fn estimate_message_fee(
        &self,
        call_l1_handler: CallL1Handler,
        block_identifier: BlockId,
    ) -> Result<FeeEstimate, ProviderError<Self::Error>> {
        match self {
            Self::JsonRpcHttp(inner) => Ok(
                <JsonRpcClient<HttpTransport> as Provider>::estimate_message_fee(
                    inner,
                    call_l1_handler,
                    block_identifier,
                )
                .await?,
            ),
            Self::SequencerGateway(inner) => Ok(
                <SequencerGatewayProvider as Provider>::estimate_message_fee(
                    inner,
                    call_l1_handler,
                    block_identifier,
                )
                .await?,
            ),
        }
    }

    async fn simulate_transaction(
        &self,
        tx: AccountTransaction,
        block_identifier: BlockId,
        skip_validate: bool,
    ) -> Result<TransactionSimulationInfo, ProviderError<Self::Error>> {
        match self {
            Self::JsonRpcHttp(inner) => Ok(
                <JsonRpcClient<HttpTransport> as Provider>::simulate_transaction(
                    inner,
                    tx,
                    block_identifier,
                    skip_validate,
                )
                .await?,
            ),
            Self::SequencerGateway(inner) => Ok(
                <SequencerGatewayProvider as Provider>::simulate_transaction(
                    inner,
                    tx,
                    block_identifier,
                    skip_validate,
                )
                .await?,
            ),
        }
    }

    async fn get_block(
        &self,
        block_identifier: BlockId,
    ) -> Result<Block, ProviderError<Self::Error>> {
        match self {
            Self::JsonRpcHttp(inner) => Ok(<JsonRpcClient<HttpTransport> as Provider>::get_block(
                inner,
                block_identifier,
            )
            .await?),
            Self::SequencerGateway(inner) => Ok(<SequencerGatewayProvider as Provider>::get_block(
                inner,
                block_identifier,
            )
            .await?),
        }
    }

    async fn get_block_traces(
        &self,
        block_identifier: BlockId,
    ) -> Result<BlockTraces, ProviderError<Self::Error>> {
        match self {
            Self::JsonRpcHttp(inner) => Ok(
                <JsonRpcClient<HttpTransport> as Provider>::get_block_traces(
                    inner,
                    block_identifier,
                )
                .await?,
            ),
            Self::SequencerGateway(inner) => Ok(
                <SequencerGatewayProvider as Provider>::get_block_traces(inner, block_identifier)
                    .await?,
            ),
        }
    }

    async fn get_state_update(
        &self,
        block_identifier: BlockId,
    ) -> Result<StateUpdate, ProviderError<Self::Error>> {
        match self {
            Self::JsonRpcHttp(inner) => Ok(
                <JsonRpcClient<HttpTransport> as Provider>::get_state_update(
                    inner,
                    block_identifier,
                )
                .await?,
            ),
            Self::SequencerGateway(inner) => Ok(
                <SequencerGatewayProvider as Provider>::get_state_update(inner, block_identifier)
                    .await?,
            ),
        }
    }

    async fn get_code(
        &self,
        contract_address: FieldElement,
        block_identifier: BlockId,
    ) -> Result<LegacyContractCode, ProviderError<Self::Error>> {
        match self {
            Self::JsonRpcHttp(inner) => Ok(<JsonRpcClient<HttpTransport> as Provider>::get_code(
                inner,
                contract_address,
                block_identifier,
            )
            .await?),
            Self::SequencerGateway(inner) => Ok(<SequencerGatewayProvider as Provider>::get_code(
                inner,
                contract_address,
                block_identifier,
            )
            .await?),
        }
    }

    async fn get_full_contract(
        &self,
        contract_address: FieldElement,
        block_identifier: BlockId,
    ) -> Result<DeployedClass, ProviderError<Self::Error>> {
        match self {
            Self::JsonRpcHttp(inner) => Ok(
                <JsonRpcClient<HttpTransport> as Provider>::get_full_contract(
                    inner,
                    contract_address,
                    block_identifier,
                )
                .await?,
            ),
            Self::SequencerGateway(inner) => {
                Ok(<SequencerGatewayProvider as Provider>::get_full_contract(
                    inner,
                    contract_address,
                    block_identifier,
                )
                .await?)
            }
        }
    }

    async fn get_compiled_class_by_class_hash(
        &self,
        class_hash: FieldElement,
        block_identifier: BlockId,
    ) -> Result<CompiledClass, ProviderError<Self::Error>> {
        match self {
            Self::JsonRpcHttp(inner) => Ok(
                <JsonRpcClient<HttpTransport> as Provider>::get_compiled_class_by_class_hash(
                    inner,
                    class_hash,
                    block_identifier,
                )
                .await?,
            ),
            Self::SequencerGateway(inner) => Ok(
                <SequencerGatewayProvider as Provider>::get_compiled_class_by_class_hash(
                    inner,
                    class_hash,
                    block_identifier,
                )
                .await?,
            ),
        }
    }

    async fn get_class_hash_at(
        &self,
        contract_address: FieldElement,
        block_identifier: BlockId,
    ) -> Result<FieldElement, ProviderError<Self::Error>> {
        match self {
            Self::JsonRpcHttp(inner) => Ok(
                <JsonRpcClient<HttpTransport> as Provider>::get_class_hash_at(
                    inner,
                    contract_address,
                    block_identifier,
                )
                .await?,
            ),
            Self::SequencerGateway(inner) => {
                Ok(<SequencerGatewayProvider as Provider>::get_class_hash_at(
                    inner,
                    contract_address,
                    block_identifier,
                )
                .await?)
            }
        }
    }

    async fn get_class_by_hash(
        &self,
        class_hash: FieldElement,
        block_identifier: BlockId,
    ) -> Result<DeployedClass, ProviderError<Self::Error>> {
        match self {
            Self::JsonRpcHttp(inner) => Ok(
                <JsonRpcClient<HttpTransport> as Provider>::get_class_by_hash(
                    inner,
                    class_hash,
                    block_identifier,
                )
                .await?,
            ),
            Self::SequencerGateway(inner) => {
                Ok(<SequencerGatewayProvider as Provider>::get_class_by_hash(
                    inner,
                    class_hash,
                    block_identifier,
                )
                .await?)
            }
        }
    }

    async fn get_storage_at(
        &self,
        contract_address: FieldElement,
        key: FieldElement,
        block_identifier: BlockId,
    ) -> Result<FieldElement, ProviderError<Self::Error>> {
        match self {
            Self::JsonRpcHttp(inner) => {
                Ok(<JsonRpcClient<HttpTransport> as Provider>::get_storage_at(
                    inner,
                    contract_address,
                    key,
                    block_identifier,
                )
                .await?)
            }
            Self::SequencerGateway(inner) => {
                Ok(<SequencerGatewayProvider as Provider>::get_storage_at(
                    inner,
                    contract_address,
                    key,
                    block_identifier,
                )
                .await?)
            }
        }
    }

    async fn get_nonce(
        &self,
        contract_address: FieldElement,
        block_identifier: BlockId,
    ) -> Result<FieldElement, ProviderError<Self::Error>> {
        match self {
            Self::JsonRpcHttp(inner) => Ok(<JsonRpcClient<HttpTransport> as Provider>::get_nonce(
                inner,
                contract_address,
                block_identifier,
            )
            .await?),
            Self::SequencerGateway(inner) => Ok(<SequencerGatewayProvider as Provider>::get_nonce(
                inner,
                contract_address,
                block_identifier,
            )
            .await?),
        }
    }

    async fn get_transaction_status(
        &self,
        transaction_hash: FieldElement,
    ) -> Result<TransactionStatusInfo, ProviderError<Self::Error>> {
        match self {
            Self::JsonRpcHttp(inner) => Ok(
                <JsonRpcClient<HttpTransport> as Provider>::get_transaction_status(
                    inner,
                    transaction_hash,
                )
                .await?,
            ),
            Self::SequencerGateway(inner) => Ok(
                <SequencerGatewayProvider as Provider>::get_transaction_status(
                    inner,
                    transaction_hash,
                )
                .await?,
            ),
        }
    }

    async fn get_transaction(
        &self,
        transaction_hash: FieldElement,
    ) -> Result<TransactionInfo, ProviderError<Self::Error>> {
        match self {
            Self::JsonRpcHttp(inner) => {
                Ok(<JsonRpcClient<HttpTransport> as Provider>::get_transaction(
                    inner,
                    transaction_hash,
                )
                .await?)
            }
            Self::SequencerGateway(inner) => Ok(
                <SequencerGatewayProvider as Provider>::get_transaction(inner, transaction_hash)
                    .await?,
            ),
        }
    }

    async fn get_transaction_receipt(
        &self,
        transaction_hash: FieldElement,
    ) -> Result<TransactionReceipt, ProviderError<Self::Error>> {
        match self {
            Self::JsonRpcHttp(inner) => Ok(
                <JsonRpcClient<HttpTransport> as Provider>::get_transaction_receipt(
                    inner,
                    transaction_hash,
                )
                .await?,
            ),
            Self::SequencerGateway(inner) => Ok(
                <SequencerGatewayProvider as Provider>::get_transaction_receipt(
                    inner,
                    transaction_hash,
                )
                .await?,
            ),
        }
    }

    async fn get_transaction_trace(
        &self,
        transaction_hash: FieldElement,
    ) -> Result<TransactionTrace, ProviderError<Self::Error>> {
        match self {
            Self::JsonRpcHttp(inner) => Ok(
                <JsonRpcClient<HttpTransport> as Provider>::get_transaction_trace(
                    inner,
                    transaction_hash,
                )
                .await?,
            ),
            Self::SequencerGateway(inner) => Ok(
                <SequencerGatewayProvider as Provider>::get_transaction_trace(
                    inner,
                    transaction_hash,
                )
                .await?,
            ),
        }
    }

    async fn get_block_hash_by_id(
        &self,
        block_number: u64,
    ) -> Result<FieldElement, ProviderError<Self::Error>> {
        match self {
            Self::JsonRpcHttp(inner) => Ok(
                <JsonRpcClient<HttpTransport> as Provider>::get_block_hash_by_id(
                    inner,
                    block_number,
                )
                .await?,
            ),
            Self::SequencerGateway(inner) => Ok(
                <SequencerGatewayProvider as Provider>::get_block_hash_by_id(inner, block_number)
                    .await?,
            ),
        }
    }

    async fn get_block_id_by_hash(
        &self,
        block_hash: FieldElement,
    ) -> Result<u64, ProviderError<Self::Error>> {
        match self {
            Self::JsonRpcHttp(inner) => Ok(
                <JsonRpcClient<HttpTransport> as Provider>::get_block_id_by_hash(inner, block_hash)
                    .await?,
            ),
            Self::SequencerGateway(inner) => Ok(
                <SequencerGatewayProvider as Provider>::get_block_id_by_hash(inner, block_hash)
                    .await?,
            ),
        }
    }

    async fn get_transaction_hash_by_id(
        &self,
        transaction_number: u64,
    ) -> Result<FieldElement, ProviderError<Self::Error>> {
        match self {
            Self::JsonRpcHttp(inner) => Ok(
                <JsonRpcClient<HttpTransport> as Provider>::get_transaction_hash_by_id(
                    inner,
                    transaction_number,
                )
                .await?,
            ),
            Self::SequencerGateway(inner) => Ok(
                <SequencerGatewayProvider as Provider>::get_transaction_hash_by_id(
                    inner,
                    transaction_number,
                )
                .await?,
            ),
        }
    }

    async fn get_transaction_id_by_hash(
        &self,
        transaction_hash: FieldElement,
    ) -> Result<u64, ProviderError<Self::Error>> {
        match self {
            Self::JsonRpcHttp(inner) => Ok(
                <JsonRpcClient<HttpTransport> as Provider>::get_transaction_id_by_hash(
                    inner,
                    transaction_hash,
                )
                .await?,
            ),
            Self::SequencerGateway(inner) => Ok(
                <SequencerGatewayProvider as Provider>::get_transaction_id_by_hash(
                    inner,
                    transaction_hash,
                )
                .await?,
            ),
        }
    }

    async fn get_last_batch_id(&self) -> Result<u64, ProviderError<Self::Error>> {
        match self {
            Self::JsonRpcHttp(inner) => {
                Ok(<JsonRpcClient<HttpTransport> as Provider>::get_last_batch_id(inner).await?)
            }
            Self::SequencerGateway(inner) => {
                Ok(<SequencerGatewayProvider as Provider>::get_last_batch_id(inner).await?)
            }
        }
    }

    async fn get_l1_blockchain_id(&self) -> Result<u64, ProviderError<Self::Error>> {
        match self {
            Self::JsonRpcHttp(inner) => {
                Ok(<JsonRpcClient<HttpTransport> as Provider>::get_l1_blockchain_id(inner).await?)
            }
            Self::SequencerGateway(inner) => {
                Ok(<SequencerGatewayProvider as Provider>::get_l1_blockchain_id(inner).await?)
            }
        }
    }
}

impl From<ProviderError<<JsonRpcClient<HttpTransport> as Provider>::Error>>
    for ProviderError<AnyProviderError>
{
    fn from(value: ProviderError<<JsonRpcClient<HttpTransport> as Provider>::Error>) -> Self {
        match value {
            ProviderError::StarknetError(inner) => Self::StarknetError(inner),
            ProviderError::RateLimited => Self::RateLimited,
            ProviderError::Other(inner) => Self::Other(AnyProviderError::JsonRpcHttp(inner)),
        }
    }
}

impl From<ProviderError<<SequencerGatewayProvider as Provider>::Error>>
    for ProviderError<AnyProviderError>
{
    fn from(value: ProviderError<<SequencerGatewayProvider as Provider>::Error>) -> Self {
        match value {
            ProviderError::StarknetError(inner) => Self::StarknetError(inner),
            ProviderError::RateLimited => Self::RateLimited,
            ProviderError::Other(inner) => Self::Other(AnyProviderError::SequencerGateway(inner)),
        }
    }
}
