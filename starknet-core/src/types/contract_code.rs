use ethereum_types::U256;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ContractCode {
    pub bytecode: Vec<U256>,
    pub abi: Option<Vec<AbiEntry>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum AbiEntry {
    Constructor(Constructor),
    Function(Function),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contract_code_deser() {
        let raw = r#"{"bytecode": ["0x40780017fff7fff", "0x1", "0x208b7fff7fff7ffe", "0x400380007ffb7ffc", "0x400380017ffb7ffd", "0x482680017ffb8000", "0x3", "0x480280027ffb8000", "0x208b7fff7fff7ffe", "0x20780017fff7ffd", "0x3", "0x208b7fff7fff7ffe", "0x480a7ffb7fff8000", "0x480a7ffc7fff8000", "0x480080007fff8000", "0x400080007ffd7fff", "0x482480017ffd8001", "0x1", "0x482480017ffd8001", "0x1", "0xa0680017fff7ffe", "0x800000000000010fffffffffffffffffffffffffffffffffffffffffffffffb"], "abi": [{"type": "struct", "name": "ExternalStruct3", "members": [{"name": "x", "offset": 0, "type": "felt"}], "size": 1}, {"type": "struct", "name": "ExternalStruct2", "members": [{"name": "x", "offset": 0, "type": "(felt, ExternalStruct)"}], "size": 3}, {"type": "struct", "name": "ExternalStruct", "members": [{"name": "y", "offset": 0, "type": "(felt, felt)"}], "size": 2}, {"inputs": [], "name": "constructor", "outputs": [], "type": "constructor"}, {"inputs": [{"name": "a", "type": "felt"}, {"name": "arr_len", "type": "felt"}, {"name": "arr", "type": "felt*"}], "name": "f", "outputs": [{"name": "b", "type": "felt"}, {"name": "c", "type": "felt"}], "type": "function"}, {"inputs": [], "name": "g", "outputs": [{"name": "a", "type": "ExternalStruct3"}], "type": "function", "stateMutability": "view"}, {"inputs": [{"name": "from_address", "type": "felt"}, {"name": "a", "type": "ExternalStruct2"}], "name": "handler", "outputs": [], "type": "l1_handler"}]}"#;
        let cc: ContractCode = serde_json::from_str(raw).unwrap();
        assert_eq!(cc.bytecode.len(), 22);

        let abi = cc.abi.unwrap();

        if let AbiEntry::Struct(s) = &abi[0] {
            assert_eq!(s.name, "ExternalStruct3");
            assert_eq!(s.size, 1);
        } else {
            panic!("Did not deserialize AbiEntry::Struct properly");
        }

        if let AbiEntry::Constructor(c) = &abi[3] {
            assert_eq!(c.name, "constructor");
        } else {
            panic!("Did not deserialize AbiEntry::Constructor properly");
        }

        if let AbiEntry::Function(f) = &abi[5] {
            // TODO: stateMutability
            assert_eq!(f.name, "g");
            assert_eq!(f.outputs.len(), 1);
        } else {
            panic!("Did not deserialize AbiEntry::Function properly");
        }

        if let AbiEntry::L1Handler(h) = &abi[6] {
            assert_eq!(h.name, "handler");
            assert_eq!(h.inputs.len(), 2);
        } else {
            panic!("Did not deserialize AbiEntry::L1Handler properly");
        }
    }
}
