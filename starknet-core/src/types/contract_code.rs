use super::{super::serde::unsigned_field_element::UfeHex, FieldElement};

use serde::{de::Error as DeError, Deserialize, Serialize, Serializer};
use serde_with::serde_as;

#[serde_as]
#[derive(Debug, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct ContractCode {
    #[serde_as(as = "Vec<UfeHex>")]
    pub bytecode: Vec<FieldElement>,
    pub abi: Option<Vec<AbiEntry>>,
}

#[derive(Debug, Clone)]
pub enum AbiEntry {
    Constructor(Constructor),
    Function(Function),
    Struct(Struct),
    L1Handler(L1Handler),
    Event(Event),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constructor {
    pub inputs: Vec<Input>,
    pub name: String,
    pub outputs: Vec<Output>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Function {
    pub inputs: Vec<Input>,
    pub name: String,
    pub outputs: Vec<Output>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_mutability: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Struct {
    pub members: Vec<Member>,
    pub name: String,
    pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct L1Handler {
    pub inputs: Vec<Input>,
    pub name: String,
    pub outputs: Vec<Output>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub data: Vec<EventData>,
    pub keys: Vec<()>, // Can't figure out what's in `keys`
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Input {
    pub name: String,
    pub r#type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Output {
    pub name: String,
    pub r#type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct EventData {
    pub name: String,
    pub r#type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Member {
    pub name: String,
    pub offset: u64,
    pub r#type: String,
}

// Manually implementing this so we can put `type` at the end:
// https://github.com/xJonathanLEI/starknet-rs/issues/216
impl Serialize for AbiEntry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct TypedValue<'a, T> {
            #[serde(flatten)]
            value: &'a T,
            r#type: &'static str,
        }

        match self {
            Self::Constructor(value) => TypedValue::serialize(
                &TypedValue {
                    value: &value,
                    r#type: "constructor",
                },
                serializer,
            ),
            Self::Function(value) => TypedValue::serialize(
                &TypedValue {
                    value: &value,
                    r#type: "function",
                },
                serializer,
            ),
            Self::Struct(value) => TypedValue::serialize(
                &TypedValue {
                    value: &value,
                    r#type: "struct",
                },
                serializer,
            ),
            Self::L1Handler(value) => TypedValue::serialize(
                &TypedValue {
                    value: &value,
                    r#type: "l1_handler",
                },
                serializer,
            ),
            Self::Event(value) => TypedValue::serialize(
                &TypedValue {
                    value: &value,
                    r#type: "event",
                },
                serializer,
            ),
        }
    }
}

// We need to manually implement this because `arbitrary_precision` doesn't work with `tag`:
//   https://github.com/serde-rs/serde/issues/1183
impl<'de> Deserialize<'de> for AbiEntry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let temp_value = serde_json::Value::deserialize(deserializer)?;
        match &temp_value["type"] {
            serde_json::Value::String(type_str) => match &type_str[..] {
                "constructor" => Ok(AbiEntry::Constructor(
                    Constructor::deserialize(temp_value).map_err(|err| {
                        DeError::custom(format!("invalid constructor variant: {}", err))
                    })?,
                )),
                "function" => Ok(AbiEntry::Function(
                    Function::deserialize(temp_value).map_err(|err| {
                        DeError::custom(format!("invalid function variant: {}", err))
                    })?,
                )),
                "struct" => Ok(AbiEntry::Struct(Struct::deserialize(temp_value).map_err(
                    |err| DeError::custom(format!("invalid struct variant: {}", err)),
                )?)),
                "l1_handler" => Ok(AbiEntry::L1Handler(
                    L1Handler::deserialize(temp_value).map_err(|err| {
                        DeError::custom(format!("invalid l1_handler variant: {}", err))
                    })?,
                )),
                "event" => Ok(AbiEntry::Event(Event::deserialize(temp_value).map_err(
                    |err| DeError::custom(format!("invalid event variant: {}", err)),
                )?)),
                _ => Err(DeError::custom(format!(
                    "unknown ABI entry type: {}",
                    type_str
                ))),
            },
            _ => Err(DeError::custom("invalid type field")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
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
            assert_eq!(f.state_mutability, None);
        } else {
            panic!("Did not deserialize AbiEntry::Function properly");
        }

        if let AbiEntry::Function(f) = &abi[9] {
            assert_eq!(f.name, "is_valid_signature");
            assert_eq!(f.inputs.len(), 3);
            assert_eq!(f.state_mutability, Some(String::from("view")));
        } else {
            panic!("Did not deserialize AbiEntry::Function properly");
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_contract_code_deser_all_abi_types() {
        // $ curl "https://alpha4.starknet.io/feeder_gateway/get_code?contractAddress=0x06ef97a90be1c0458f6e7bd1faf05021f2d81211f658155df0c5c97a39eb2d12"
        // Contract built from: https://github.com/starkware-libs/cairo-lang/blob/3d33c4e829a87bc3d88cf04ed6a489e788918b8b/src/starkware/starknet/compiler/starknet_preprocessor_test.py#L143
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
            assert_eq!(f.name, "g");
            assert_eq!(f.outputs.len(), 1);
            assert_eq!(f.state_mutability, Some(String::from("view")));
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
