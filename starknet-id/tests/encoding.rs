use starknet_core::types::FieldElement;
use starknet_id::{decode, encode};

#[tokio::test]
async fn encoding_works() {
    let encoded = encode("thomas").unwrap();
    assert_eq!(encoded, FieldElement::from_dec_str("1426911989").unwrap());

    let encoded = encode("").unwrap();
    assert_eq!(encoded, FieldElement::from_dec_str("0").unwrap());

    let encoded = encode("ben").unwrap();
    assert_eq!(encoded, FieldElement::from_dec_str("18925").unwrap());

    let encoded = encode("这来").unwrap();
    assert_eq!(encoded, FieldElement::from_dec_str("8625").unwrap());

    let encoded = encode("efghijk来aa").unwrap();
    assert_eq!(
        encoded,
        FieldElement::from_dec_str("12234603501699554").unwrap()
    );

    let encoded = encode("a").unwrap();
    assert_eq!(encoded, FieldElement::from_dec_str("37").unwrap());

    let encoded = encode("/''[;[");

    assert!(encoded.is_err());
}

#[tokio::test]
async fn decoding_works() {
    let felt = FieldElement::from_dec_str("1426911989").unwrap();
    assert_eq!("thomas", decode(felt));

    let felt = FieldElement::from_dec_str("0").unwrap();
    assert_eq!("", decode(felt));

    let felt = FieldElement::from_dec_str("18925").unwrap();
    assert_eq!("ben", decode(felt));

    let felt = FieldElement::from_dec_str("8625").unwrap();
    assert_eq!("这来", decode(felt));

    let felt = FieldElement::from_dec_str("12234603501699554").unwrap();
    assert_eq!("efghijk来aa", decode(felt));

    let felt = FieldElement::from_dec_str("37").unwrap();
    assert_eq!("a", decode(felt));
}
