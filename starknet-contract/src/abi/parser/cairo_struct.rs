use starknet_core::types::contract::AbiNamedMember;
use std::collections::HashMap;

use super::abi_types::{AbiType, AbiTypeAny};

#[derive(Debug, Clone)]
pub struct CairoStruct {
    pub abi: AbiTypeAny,
    /// Parsed types for each member.
    pub members: Vec<(String, AbiTypeAny)>,
    /// Members name => (generic representation, is_generic).
    pub generic_members: HashMap<String, (String, bool)>,
}

impl CairoStruct {
    /// Gets the name of the struct type.
    pub fn get_name(&self) -> String {
        self.abi.get_cairo_type_name()
    }

    /// Returns true if the struct is generic, false otherwise.
    pub fn is_generic(&self) -> bool {
        matches!(self.abi, AbiTypeAny::Generic(_))
    }

    /// Returns the list of generic types, if any.
    pub fn get_gentys(&self) -> Vec<String> {
        if let AbiTypeAny::Generic(g) = &self.abi {
            g.get_gentys_only()
        } else {
            vec![]
        }
    }

    /// Initializes a new instance from the abi name and it's members.
    pub fn new(abi_name: &str, abi_members: &Vec<AbiNamedMember>) -> CairoStruct {
        let abi = AbiTypeAny::from_string(abi_name);
        let mut members: Vec<(String, AbiTypeAny)> = vec![];
        let mut generic_members: HashMap<String, (String, bool)> = HashMap::new();

        for m in abi_members {
            let name = m.name.clone();
            let mut m_abi = AbiTypeAny::from_string(&m.r#type.clone());

            if let AbiTypeAny::Generic(ref g) = abi {
                let cairo_gentys = g.get_cairo_types_gentys();
                let cairo_gentys = cairo_gentys
                    .iter()
                    .map(|(v1, v2)| (&v1[..], &v2[..]))
                    .collect();

                let (type_str, is_generic) = m_abi.apply_generic(cairo_gentys);

                generic_members.insert(name.clone(), (type_str.clone(), is_generic));
            }

            members.push((name.clone(), m_abi.clone()));
        }

        CairoStruct {
            abi,
            members,
            generic_members,
        }
    }

    /// Compares the generic types for each members with an other `CairoStruct`.
    pub fn compare_generic_types(&self, existing_cs: &mut CairoStruct) {
        if let AbiTypeAny::Generic(_) = &self.abi {
            for (em_name, em_abi) in &mut existing_cs.members {
                for (m_name, m_abi) in &self.members {
                    if m_name != em_name {
                        continue;
                    }
                    em_abi.compare_generic(m_abi);
                }
            }
        }
    }
}
