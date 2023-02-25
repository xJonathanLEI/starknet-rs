use proc_macro::TokenStream;

mod pedersen;
mod poseidon;

/// Generates the lookup table for Pedersen hash.
#[proc_macro]
pub fn lookup_table(input: TokenStream) -> TokenStream {
    pedersen::lookup_table(input)
}

/// Generates the constants from Poseidon params.
#[proc_macro]
pub fn poseidon_consts(_input: TokenStream) -> TokenStream {
    poseidon::poseidon_consts()
}
