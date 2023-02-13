use starknet_core::types::FieldElement;
use std::prelude::v1::*;
#[derive(Debug, Clone)]
pub struct Call {
    pub to: FieldElement,
    pub selector: FieldElement,
    pub calldata: Vec<FieldElement>,
}
