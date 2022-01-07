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
        let raw = include_str!("../../test-data/raw_gateway_responses/get_code/1_code.txt");

        let cc: ContractCode = serde_json::from_str(raw).unwrap();
        let abi = cc.abi.unwrap();

        assert_eq!(cc.bytecode.len(), 1347);
        if let AbiEntry::Constructor(c) = &abi[0] {
            assert_eq!(c.name, "constructor");
            assert_eq!(c.inputs.len(), 2);
        } else {
            panic!("Did not deserialize AbiEntry::Constructor properly")
        }

        if let AbiEntry::Function(f) = &abi[1] {
            assert_eq!(f.name, "execute");
            assert_eq!(f.inputs.len(), 5);
        } else {
            panic!("Did not deserialize AbiEntry::Function properly");
        }
        // TODO: use abi[9] to test "stateMutability" param

        // fake JSON response, built from a fixture in cairo-lang repo
        // https://github.com/starkware-libs/cairo-lang/blob/3d33c4e829a87bc3d88cf04ed6a489e788918b8b/src/starkware/starknet/compiler/starknet_preprocessor_test.py#L143
        let raw =
            include_str!("../../test-data/raw_gateway_responses/get_code/2_all_abi_types.txt");
        let cc: ContractCode = serde_json::from_str(raw).unwrap();
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
