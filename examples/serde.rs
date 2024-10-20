use starknet::{
    core::{
        codec::{Decode, Encode},
        types::Felt,
    },
    macros::felt,
};

#[derive(Debug, Eq, PartialEq, Encode, Decode)]
struct CairoType {
    a: Felt,
    b: Option<u32>,
    c: bool,
}

fn main() {
    let instance = CairoType {
        a: felt!("123456789"),
        b: Some(100),
        c: false,
    };

    let mut serialized = vec![];
    instance.encode(&mut serialized).unwrap();

    assert_eq!(
        serialized,
        [felt!("123456789"), felt!("0"), felt!("100"), felt!("0")]
    );

    let restored = CairoType::decode(&serialized).unwrap();

    assert_eq!(instance, restored);
}
