//! Defines the arguments of the `abigen` macro.
//!
//! `ContractAbi` is expected to the argument
//! passed to the macro. We should then parse the
//! token stream to ensure the arguments are correct.
//!
//! At this moment, the macro supports two fashions:
//!
//! Loading from a file.
//!
//! abigen!(ContractName, "path/to/abi.json"
//!
//!
//! Loading from a literal string ABI.
//!
//! abigen!(ContractName, r#"
//!    [{ .... }]
//! "#);
//!
use std::fs::File;
use syn::{
    parse::{Parse, ParseStream, Result},
    Ident, LitStr, Token,
};
use starknet_core::types::contract::AbiEntry;

#[derive(Clone, Debug)]
pub(crate) struct ContractAbi {
    pub name: Ident,
    pub abi: Vec<AbiEntry>,
}

impl Parse for ContractAbi {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse::<Ident>()?;
        input.parse::<Token![,]>()?;

        // Path rooted to the Cargo.toml location.
        let json_path = input.parse::<LitStr>()?;
        
        let abi = serde_json::from_reader::<_, Vec<AbiEntry>>(
            File::open(json_path.value())
                .map_err(|e| {
                    syn::Error::new(json_path.span(), format!("JSON open file error: {}", e))
                })?
        )
            .map_err(|e| {
                syn::Error::new(json_path.span(), format!("JSON parse error: {}", e))
            })?;
        
        Ok(ContractAbi {
            name,
            abi,
        })
    }
}
