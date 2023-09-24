//! Expands the contract first implementation with
//! default configuration for provider and account, if any.
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

use syn::Ident;

pub struct CairoContract;

impl CairoContract {
    pub fn expand(contract_name: Ident) -> TokenStream2 {
        quote! {

            #[derive(Debug)]
            pub struct #contract_name<P>
            where
                P: starknet::providers::Provider + Send + Sync, <P as starknet::providers::Provider>::Error: 'static
            {
                pub address: starknet::core::types::FieldElement,
                pub provider: std::sync::Arc<P>,
                pub account: std::option::Option<std::sync::Arc<starknet::accounts::SingleOwnerAccount<std::sync::Arc<P>, starknet::signers::LocalWallet>>>,
            }

            impl<P> #contract_name<P>
            where
                P: starknet::providers::Provider + Send + Sync, <P as starknet::providers::Provider>::Error: 'static
             {
                pub fn new(
                    address: starknet::core::types::FieldElement,
                    provider: std::sync::Arc<P>,
                ) -> Self {
                    Self {
                        address,
                        provider: std::sync::Arc::clone(&provider),
                        account: None,
                    }
                }

                pub fn with_account(mut self, account: std::sync::Arc<starknet::accounts::SingleOwnerAccount<std::sync::Arc<P>, starknet::signers::LocalWallet>>,
                ) -> Self {
                    self.account = Some(std::sync::Arc::clone(&account));
                    self
                }
            }
        }
    }
}
