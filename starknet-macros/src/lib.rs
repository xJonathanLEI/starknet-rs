use proc_macro::TokenStream;
use quote::quote;
use starknet_core::{
    types::FieldElement,
    utils::{cairo_short_string_to_felt, get_selector_from_name},
};
use syn::{
    parse_macro_input, punctuated::Punctuated, token::Comma, Data, DataStruct, Field, Ident, LitStr,
};

#[proc_macro]
pub fn selector(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);

    let str_value = input.value();

    let selector_value = get_selector_from_name(&str_value).expect("invalid selector name");
    let selector_raw = selector_value.into_mont();

    format!(
        "{}::from_mont([{}, {}, {}, {}])",
        field_element_path(),
        selector_raw[0],
        selector_raw[1],
        selector_raw[2],
        selector_raw[3],
    )
    .parse()
    .unwrap()
}

#[proc_macro]
pub fn short_string(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);

    let str_value = input.value();

    let felt_value = cairo_short_string_to_felt(&str_value).expect("invalid Cairo short string");
    let felt_raw = felt_value.into_mont();

    format!(
        "{}::from_mont([{}, {}, {}, {}])",
        field_element_path(),
        felt_raw[0],
        felt_raw[1],
        felt_raw[2],
        felt_raw[3],
    )
    .parse()
    .unwrap()
}

#[proc_macro]
pub fn felt(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);

    let str_value = input.value();

    let felt_value = if str_value.starts_with("0x") {
        FieldElement::from_hex_be(&str_value).expect("invalid FieldElement value")
    } else {
        FieldElement::from_dec_str(&str_value).expect("invalid FieldElement value")
    };

    let felt_raw = felt_value.into_mont();

    format!(
        "{}::from_mont([{}, {}, {}, {}])",
        field_element_path(),
        felt_raw[0],
        felt_raw[1],
        felt_raw[2],
        felt_raw[3],
    )
    .parse()
    .unwrap()
}

#[proc_macro]
pub fn felt_dec(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);

    let str_value = input.value();

    let felt_value = FieldElement::from_dec_str(&str_value).expect("invalid FieldElement value");
    let felt_raw = felt_value.into_mont();

    format!(
        "{}::from_mont([{}, {}, {}, {}])",
        field_element_path(),
        felt_raw[0],
        felt_raw[1],
        felt_raw[2],
        felt_raw[3],
    )
    .parse()
    .unwrap()
}

#[proc_macro]
pub fn felt_hex(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);

    let str_value = input.value();

    let felt_value = FieldElement::from_hex_be(&str_value).expect("invalid FieldElement value");
    let felt_raw = felt_value.into_mont();

    format!(
        "{}::from_mont([{}, {}, {}, {}])",
        field_element_path(),
        felt_raw[0],
        felt_raw[1],
        felt_raw[2],
        felt_raw[3],
    )
    .parse()
    .unwrap()
}

#[cfg(feature = "use_imported_type")]
fn field_element_path() -> &'static str {
    "FieldElement"
}

#[cfg(not(feature = "use_imported_type"))]
fn field_element_path() -> &'static str {
    "::starknet::core::types::FieldElement"
}

#[proc_macro_derive(Decode)]
pub fn decode_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_decode_macro(&ast)
}

fn impl_decode_macro(ast: &syn::DeriveInput) -> TokenStream {
    if let Data::Struct(DataStruct {
        fields: syn::Fields::Named(ref fields),
        ..
    }) = ast.data
    {
        impl_decode_macro_for_struct(&ast.ident, &fields.named)
    } else {
        // TODO: use abort_call_site instead
        panic!("Decode only supports non-tuple structs")
    }
}

fn impl_decode_macro_for_struct(name: &Ident, fields: &Punctuated<Field, Comma>) -> TokenStream {
    // Generate Decode implementation for a struct given its name and fields
    // For a struct:
    // struct Transfer {
    //     from: Address,
    //     to: Address,
    //     amount: Uint,
    // }
    // Generates:
    // impl Decode for Transfer {
    //     fn decode(tokens: &[Token]) -> Self {
    //         Transfer {
    //             from: Address::try_from(&tokens[0]).unwrap(),
    //             to: Address::try_from(&tokens[1]).unwrap(),
    //             amount: Uint::try_from(&tokens[2]).unwrap(),
    //         }
    //     }
    // }

    let fields = fields.iter().enumerate().map(|(i, field)| {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;
        quote!( #field_name: #field_type::try_from(&tokens[#i]).unwrap() )
    });

    let gen = quote! {
        impl Decode for #name {
            fn decode(tokens: &[Token]) -> Self {
                #name { #( #fields ),* }
            }
        }
    };
    gen.into()
}
