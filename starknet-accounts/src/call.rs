use starknet_types_core::felt::Felt;

#[derive(Debug, Clone)]
pub struct Call {
    pub to: Felt,
    pub selector: Felt,
    pub calldata: Vec<Felt>,
}
