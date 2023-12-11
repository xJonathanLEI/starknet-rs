// This file originally lived in starknet-core/src/types/mod.rs, but was moved here as part of the
// migration to becoming jsonrpc-centric. This file, along with all other sequencer-related types,
// will be removed after the sequencer API is removed from the network.

// Re-export commonly used upstream types
pub use ethereum_types::Address as L1Address;

pub(crate) mod conversions;

mod block;
pub use block::{Block, BlockId, BlockStatus};

mod transaction;
pub use transaction::{
    DeclareTransaction, DeployAccountTransaction, DeployTransaction, EntryPointType,
    InvokeFunctionTransaction, L1HandlerTransaction, TransactionFailureReason, TransactionInfo,
    TransactionStatusInfo, TransactionType,
};

mod transaction_receipt;
pub use transaction_receipt::{
    BuiltinInstanceCounter, ConfirmedReceipt as ConfirmedTransactionReceipt, Event,
    ExecutionResources, L1ToL2Message, L2ToL1Message, TransactionExecutionStatus,
    TransactionFinalityStatus, TransactionStatus,
};

mod contract_addresses;
pub use contract_addresses::ContractAddresses;

mod transaction_request;
pub use transaction_request::{
    AddTransactionResult, AddTransactionResultCode,
    DeclareTransaction as DeclareTransactionRequest,
    DeclareV1Transaction as DeclareV1TransactionRequest,
    DeclareV2Transaction as DeclareV2TransactionRequest,
    DeclareV3Transaction as DeclareV3TransactionRequest,
    DeployAccountTransaction as DeployAccountTransactionRequest,
    DeployAccountV1Transaction as DeployAccountV1TransactionRequest,
    DeployAccountV3Transaction as DeployAccountV3TransactionRequest,
    InvokeFunctionTransaction as InvokeFunctionTransactionRequest,
    InvokeFunctionV1Transaction as InvokeFunctionV1TransactionRequest,
    InvokeFunctionV3Transaction as InvokeFunctionV3TransactionRequest, TransactionRequest,
};

mod contract;
pub use contract::{CompressedLegacyContractClass, DeployedClass};

pub mod state_update;
pub use state_update::StateUpdate;

pub mod trace;
pub use trace::{BlockTraces, TransactionTrace};

pub(crate) mod serde_impls;
