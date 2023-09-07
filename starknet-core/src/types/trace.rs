pub use super::codegen::{
    CallType, EntryPointType, FeeEstimate, FunctionInvocation, NestedCall,
    SimulateTransactionsRequest, SimulateTransactionsRequestRef, SimulationFlag,
    TraceBlockTransactionsRequest, TraceBlockTransactionsRequestRef, TraceTransactionRequest,
    TraceTransactionRequestRef,
};

use super::{FieldElement, UfeHex};

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct TransactionTraceWithHash {
    #[serde_as(as = "UfeHex")]
    pub transaction_hash: FieldElement,
    pub trace_root: TransactionTrace,
}

/// the execution trace of an invoke transaction
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct InvokeTransactionTrace {
    /// An object describing the invocation of validation.
    pub validate_invocation: FunctionInvocation,
    /// An object describing the invocation of a specific function.
    pub execute_invocation: ExecuteInvocation,
    /// An object describing the invocation of a fee transfer.
    pub fee_transfer_invocation: FunctionInvocation,
}

/// The execution trace of a declare transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct DeclareTransactionTrace {
    /// An object describing the invocation of validation.
    pub validate_invocation: FunctionInvocation,
    /// An object describing the invocation of a fee transfer.
    pub fee_transfer_invocation: FunctionInvocation,
}

/// The execution trace of a deploy account transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct DeployAccountTransactionTrace {
    /// An object describing the invocation of validation.
    pub validate_invocation: FunctionInvocation,
    /// The trace of the __execute__ call or constructor call, depending on the transaction type (none for declare transactions)
    pub constructor_invocation: FunctionInvocation,
    /// An object describing the invocation of a fee transfer.
    pub fee_transfer_invocation: FunctionInvocation,
}

/// The execution trace of an L1 handler transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct L1HandlerTransactionTrace {
    /// the trace of the __execute__ call or constructor call, depending on the transaction type (none for declare transactions).
    pub function_invocation: FunctionInvocation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TransactionTrace {
    Invoke(InvokeTransactionTrace),
    Declare(DeclareTransactionTrace),
    DeployAccount(DeployAccountTransactionTrace),
    L1Handler(L1HandlerTransactionTrace),
}

/// The trace of the __execute__ call or constructor call, depending on the transaction type (none for declare transactions)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged, deny_unknown_fields, rename_all = "camelCase")]
pub enum ExecuteInvocation {
    /// An object describing the invocation of a specific function
    Succeeded(FunctionInvocation),
    /// The revert reason for the failed execution
    Reverted(String),
}

/// The execution trace and consuemd resources of the required transactions
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct SimulatedTransaction {
    /// The transaction's trace
    pub transaction_trace: TransactionTrace,
    /// The transaction's resources and fee
    pub fee_estimation: FeeEstimate,
}
