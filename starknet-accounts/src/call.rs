use starknet_core::types::Felt;

#[derive(Debug, Clone)]
pub struct Call {
    pub to: Felt,
    pub selector: Felt,
    pub calldata: Vec<Felt>,
}
