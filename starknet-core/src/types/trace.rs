pub use super::codegen::{
    CallType, EntryPointType, FeeEstimate, FunctionInvocation, NestedCall,
    SimulateTransactionsRequest, SimulateTransactionsRequestRef, SimulationFlag,
    TraceBlockTransactionsRequest, TraceBlockTransactionsRequestRef, TraceTransactionRequest,
    TraceTransactionRequestRef,
};

use super::{FieldElement, UfeHex};

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct TransactionTrace {
    /// An object describing the invocation of validation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate_invocation: Option<FunctionInvocation>,
    /// An object describing the invocation of a fee transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_transfer_invocation: Option<FunctionInvocation>,
    /// An object describing the invocation of a specific function.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execute_invocation: Option<ExecuteInvocation>,
    /// An object describing the invocation of a specific function.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_invocation: Option<FunctionInvocation>,
    /// An object describing the invocation of a specific function.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constructor_invocation: Option<FunctionInvocation>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct SimulatedTransaction {
    /// The transaction's trace
    pub transaction_trace: TransactionTrace,
    /// The transaction's resources and fee
    pub fee_estimation: FeeEstimate,
}

/// The execution trace and consuemd resources of the required transactions
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct SimulatedTransactions {
    pub simulated_transactions: Vec<SimulatedTransaction>,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct TransactionTraceWithHash {
    #[serde_as(as = "UfeHex")]
    pub transaction_hash: FieldElement,
    pub trace_root: TransactionTrace,
}
