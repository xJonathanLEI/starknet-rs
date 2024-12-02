use alloc::vec::*;

use crate::types::Felt;

/// A contract call as part of a multi-call execution request.
#[derive(Debug, Clone)]
pub struct Call {
    /// Address of the contract being invoked.
    pub to: Felt,
    /// Entrypoint selector of the function being invoked.
    pub selector: Felt,
    /// List of calldata to be sent for the call.
    pub calldata: Vec<Felt>,
}
