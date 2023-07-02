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
    ExecutionResources, L1ToL2Message, L2ToL1Message, Receipt as TransactionReceipt,
    TransactionStatus,
};

mod contract_addresses;
pub use contract_addresses::ContractAddresses;

mod call_contract;
pub use call_contract::CallContractResult;

mod transaction_request;
pub use transaction_request::{
    AccountTransaction, AddTransactionResult, AddTransactionResultCode, CallFunction,
    CallL1Handler, DeclareTransaction as DeclareTransactionRequest,
    DeclareV1Transaction as DeclareV1TransactionRequest,
    DeclareV2Transaction as DeclareV2TransactionRequest,
    DeployAccountTransaction as DeployAccountTransactionRequest,
    InvokeFunctionTransaction as InvokeFunctionTransactionRequest, TransactionRequest,
};

mod contract;
pub use contract::{CompressedLegacyContractClass, DeployedClass};

pub mod state_update;
pub use state_update::StateUpdate;

mod fee;
pub use fee::{FeeEstimate, FeeUnit, TransactionSimulationInfo};

pub mod trace;
pub use trace::{BlockTraces, TransactionTrace};
