use ethereum_types::U256;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ContractCode {
    pub bytecode: Vec<U256>,
    pub abi: Vec<AbiEntry>,
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
pub struct Constructor {
    pub name: String,
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
}

#[derive(Debug, Deserialize)]
pub struct Function {
    pub name: String,
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
}

#[derive(Debug, Deserialize)]
pub struct Struct {
    pub name: String,
    pub size: u64,
    pub members: Vec<Member>,
}

#[derive(Debug, Deserialize)]
pub struct L1Handler {
    pub name: String,
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
}

#[derive(Debug, Deserialize)]
pub struct Input {
    pub name: String,
    pub r#type: String,
}

#[derive(Debug, Deserialize)]
pub struct Output {
    pub name: String,
    pub r#type: String,
}

#[derive(Debug, Deserialize)]
pub struct Member {
    pub name: String,
    pub offset: u64,
    pub r#type: String,
}
