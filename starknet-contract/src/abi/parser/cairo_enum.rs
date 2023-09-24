use std::collections::HashMap;

use starknet_core::types::contract::AbiNamedMember;

use super::abi_types::{AbiType, AbiTypeAny};

#[derive(Debug, Clone)]
pub struct CairoEnum {
    pub abi: AbiTypeAny,
    /// Parsed types for each variants.
    pub variants: Vec<(String, AbiTypeAny)>,
    /// Variant name => (generic representation, is_generic).
    pub generic_variants: HashMap<String, (String, bool)>,
}

impl CairoEnum {
    /// Gets the name of the enum type.
    pub fn get_name(&self) -> String {
        self.abi.get_cairo_type_name()
    }

    /// Returns true if the enum is generic, false otherwise.
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

    /// Initializes a new instance from the abi name and it's variants.
    pub fn new(abi_name: &str, abi_variants: &Vec<AbiNamedMember>) -> CairoEnum {
        let abi = AbiTypeAny::from_string(abi_name);
        let mut variants: Vec<(String, AbiTypeAny)> = vec![];
        let mut generic_variants: HashMap<String, (String, bool)> = HashMap::new();

        for v in abi_variants {
            let name = v.name.clone();
            let mut v_abi = AbiTypeAny::from_string(&v.r#type.clone());

            if let AbiTypeAny::Generic(ref g) = abi {
                let cairo_gentys = g.get_cairo_types_gentys();
                let cairo_gentys = cairo_gentys
                    .iter()
                    .map(|(v1, v2)| (&v1[..], &v2[..]))
                    .collect();

                let (type_str, is_generic) = v_abi.apply_generic(cairo_gentys);

                generic_variants.insert(name.clone(), (type_str.clone(), is_generic));
            }

            variants.push((name.clone(), v_abi.clone()));
        }

        CairoEnum {
            abi,
            variants,
            generic_variants,
        }
    }

    /// Compares the generic types for each variants with an other `CairoEnum`.
    pub fn compare_generic_types(&self, existing_ce: &mut CairoEnum) {
        if let AbiTypeAny::Generic(_) = &self.abi {
            for (ev_name, ev_abi) in &mut existing_ce.variants {
                for (v_name, v_abi) in &self.variants {
                    if v_name != ev_name {
                        continue;
                    }
                    ev_abi.compare_generic(v_abi);
                }
            }
        }
    }
}
