use ethereum_types::U256;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ContractCode {
    pub bytecode: Vec<U256>,
    pub abi: Option<Vec<AbiEntry>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AbiEntry {
    #[serde(rename = "constructor")]
    Constructor(Constructor),
    #[serde(rename = "function")]
    Function(Function),
    #[serde(rename = "struct")]
    Struct(Struct),
    #[serde(rename = "l1_handler")]
    L1Handler(L1Handler),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Constructor {
    pub name: String,
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Function {
    pub name: String,
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Struct {
    pub name: String,
    pub size: u64,
    pub members: Vec<Member>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct L1Handler {
    pub name: String,
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Input {
    pub name: String,
    pub r#type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Output {
    pub name: String,
    pub r#type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Member {
    pub name: String,
    pub offset: u64,
    pub r#type: String,
}
