use super::{
    utils::{str_to_ident, str_to_type},
    Expandable,
};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use starknet_contract::abi::parser::{
    abi_types::{AbiType, AbiTypeAny},
    CairoFunction,
};
use starknet_core::types::contract::StateMutability;

impl Expandable for CairoFunction {
    fn expand_decl(&self) -> TokenStream2 {
        let func_name = str_to_ident(&self.name);

        let mut inputs: Vec<TokenStream2> = vec![];
        for (name, abi_type) in &self.inputs {
            let name = str_to_ident(name);
            let ty = str_to_type(&abi_type.to_rust_type());
            // We can pass a reference here as serialize always takes a reference.
            inputs.push(quote!(#name:&#ty));
        }

        let output = match self.state_mutability {
            StateMutability::View => match &self.output {
                Some(o) => {
                    let oty = str_to_type(&o.to_rust_type());
                    quote!(-> starknet::contract::abi::cairo_types::Result<#oty>)
                }
                None => quote!(),
            },
            StateMutability::External => {
                quote!(-> Result<starknet::core::types::InvokeTransactionResult,
                       starknet::accounts::AccountError<starknet::accounts::single_owner::SignError<starknet::signers::local_wallet::SignError>, <P as starknet::providers::Provider>::Error>
                       >
                )
            }
        };

        quote! {
            pub async fn #func_name(
                &self,
                #(#inputs),*
            ) #output
        }
    }

    fn expand_impl(&self) -> TokenStream2 {
        let decl = self.expand_decl();
        let func_name = &self.name;

        let mut serializations: Vec<TokenStream2> = vec![];
        for (name, abi_type) in &self.inputs {
            let name = str_to_ident(name);
            let ty = str_to_type(&abi_type.to_rust_type_path());

            let ser = match abi_type {
                AbiTypeAny::Tuple(_) => quote! {
                    calldata.extend(<#ty>::serialize(#name));
                },
                _ => quote!(calldata.extend(#ty::serialize(#name));),
            };
            serializations.push(ser);
        }

        let out_res = match &self.output {
            Some(o) => {
                let out_type_path = str_to_type(&o.to_rust_type_path());
                match o {
                    // Tuples type used as rust type path must be surrounded
                    // by LT/GT.
                    AbiTypeAny::Tuple(_) => quote!(<#out_type_path>::deserialize(&r, 0)),
                    _ => quote!(#out_type_path::deserialize(&r, 0)),
                }
            }
            None => quote!(),
        };

        match &self.state_mutability {
            StateMutability::View => quote! {
                #decl {
                    use starknet::contract::abi::cairo_types::{self, Error as CairoError};
                    use starknet::contract::abi::CairoType;
                    use starknet::core::types::{BlockId, BlockTag};

                    let mut calldata = vec![];
                    #(#serializations)*

                    let r = self.provider
                        .call(
                            starknet::core::types::FunctionCall {
                                contract_address: self.address,
                                entry_point_selector: starknet::macros::selector!(#func_name),
                                calldata,
                            },
                            BlockId::Tag(BlockTag::Pending),
                        )
                        .await.map_err(
                            |err|
                            starknet::contract::abi::cairo_types::Error::Deserialize(
                                format!("Deserialization error {}", err)))?;

                    #out_res
                }
            },
            StateMutability::External => quote! {
                // TODO: How can we add Fee configuration + estimate fee out of the box.
                // maybe two methods are generated, one for actually running, the other
                // for estimate the fees.
                // Or, we can add a config struct as the last argument? Or directly
                // at the initialization of the contract, we can give a config for
                // fees (manual, estimated + scale factor).
                // The estimate only may be done at the function level, to avoid
                // altering the contract instance itself and hence races.
                #decl {
                    use starknet::contract::abi::cairo_types::{self, Error as CairoError};
                    use starknet::contract::abi::CairoType;
                    use starknet::accounts::Account;

                    // TODO: I don't know how to easily store the SingleOwnerAccount
                    // and it's generic types without complexifiying the whole typing.
                    // So it's constructed at every call. There is surely a better approach.
                    let account = match &self.account {
                        Some(a) => std::sync::Arc::clone(&a),
                        // TODO: better error handling here.
                        _ => panic!("Account is required to send invoke transactions")
                    };

                    let mut calldata = vec![];
                    #(#serializations)*

                    let calls = vec![starknet::accounts::Call {
                        to: self.address,
                        selector: starknet::macros::selector!(#func_name),
                        calldata,
                    }];

                    let execution = account.execute(calls).fee_estimate_multiplier(2f64);
                    // TODO: we can have manual fee here, or it can also be estimate only.
                    let max_fee = execution.estimate_fee().await?.overall_fee.into();

                    execution
                        .max_fee(max_fee)
                        .send()
                        .await
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Expandable;
    use proc_macro2::TokenStream2;
    use quote::quote;
    use starknet::contract::abi::parser::{
        abi_types::{AbiType, AbiTypeAny},
        CairoFunction,
    };
    use starknet::core::types::contract::StateMutability;

    #[test]
    fn test_decl_basic() {
        let cf = CairoFunction {
            name: "my_func".to_string(),
            state_mutability: StateMutability::View,
            inputs: vec![
                ("v1".to_string(), AbiTypeAny::Basic("core::felt252".into())),
                ("v2".to_string(), AbiTypeAny::Basic("core::felt252".into())),
            ],
            output: Some(AbiTypeAny::Basic("core::felt252".into())),
        };
        let te1 = cf.expand_decl();
        let tef1: TokenStream2 = quote!(
            pub async fn my_func(&self, v1: &starknet::core::types::FieldElement, v2: &starknet::core::types::FieldElement) -> starknet::contract::abi::cairo_types::Result<starknet::core::types::FieldElement>
        );

        assert_eq!(te1.to_string(), tef1.to_string());
    }

    #[test]
    fn test_impl_basic() {
        let cf = CairoFunction {
            name: "my_func".to_string(),
            state_mutability: StateMutability::View,
            inputs: vec![
                ("v1".to_string(), AbiTypeAny::Basic("core::felt252".into())),
                ("v2".to_string(), AbiTypeAny::Basic("core::felt252".into())),
            ],
            output: Some(AbiTypeAny::Basic("core::felt252".into())),
        };
        let te1 = cf.expand_impl();

        #[rustfmt::skip]
        let tef1: TokenStream2 = quote!(
            pub async fn my_func(
                &self,
                v1: &starknet::core::types::FieldElement,
                v2: &starknet::core::types::FieldElement
            ) -> starknet::contract::abi::cairo_types::Result<starknet::core::types::FieldElement> {
                let mut calldata = vec![];
                calldata.extend(starknet::core::types::FieldElement::serialize(v1));
                calldata.extend(starknet::core::types::FieldElement::serialize(v2));

                let r = self.provider
                    .call(
                        starknet::core::types::FunctionCall {
                            contract_address: self.address,
                            entry_point_selector: starknet::macros::selector!("my_func"),
                            calldata,
                        },
                        BlockId::Tag(BlockTag::Pending),
                    )
                    .await.map_err(|err| starknet::contract::abi::cairo_types::Error::Deserialize(format!("Deserialization error {:}" , err)))?;

                starknet::core::types::FieldElement::deserialize(&r, 0)
            }
        );

        assert_eq!(te1.to_string(), tef1.to_string());
    }
}
