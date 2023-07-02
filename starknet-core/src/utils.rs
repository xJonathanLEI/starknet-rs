use crate::{crypto::compute_hash_on_elements, types::FieldElement};

use sha3::{Digest, Keccak256};
use starknet_crypto::pedersen_hash;

const DEFAULT_ENTRY_POINT_NAME: &str = "__default__";
const DEFAULT_L1_ENTRY_POINT_NAME: &str = "__l1_default__";

// 2 ** 251 - 256
const ADDR_BOUND: FieldElement = FieldElement::from_mont([
    18446743986131443745,
    160989183,
    18446744073709255680,
    576459263475590224,
]);

// Cairo string of "STARKNET_CONTRACT_ADDRESS"
const CONTRACT_ADDRESS_PREFIX: FieldElement = FieldElement::from_mont([
    3829237882463328880,
    17289941567720117366,
    8635008616843941496,
    533439743893157637,
]);

/// The uniqueness settings for UDC deployments.
#[derive(Debug, Clone)]
pub enum UdcUniqueness {
    NotUnique,
    Unique(UdcUniqueSettings),
}

#[derive(Debug, Clone)]
pub struct UdcUniqueSettings {
    pub deployer_address: FieldElement,
    pub udc_contract_address: FieldElement,
}

mod errors {
    use core::fmt::{Display, Formatter, Result};
    use std::error::Error;

    #[derive(Debug)]
    pub struct NonAsciiNameError;

    #[derive(Debug)]
    pub enum CairoShortStringToFeltError {
        NonAsciiCharacter,
        StringTooLong,
    }

    #[derive(Debug)]
    pub enum ParseCairoShortStringError {
        ValueOutOfRange,
        UnexpectedNullTerminator,
    }

    impl Error for NonAsciiNameError {}

    impl Display for NonAsciiNameError {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "the provided name contains non-ASCII characters")
        }
    }

    impl Error for CairoShortStringToFeltError {}

    impl Display for CairoShortStringToFeltError {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            match self {
                Self::NonAsciiCharacter => {
                    write!(f, "Cairo string can only contain ASCII characters")
                }
                Self::StringTooLong => {
                    write!(f, "short string exceeds maximum length of 31 characters")
                }
            }
        }
    }

    impl Error for ParseCairoShortStringError {}

    impl Display for ParseCairoShortStringError {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            match self {
                Self::ValueOutOfRange => write!(f, "field element value out of range"),
                Self::UnexpectedNullTerminator => write!(f, "unexpected null terminator"),
            }
        }
    }
}
pub use errors::{CairoShortStringToFeltError, NonAsciiNameError, ParseCairoShortStringError};

/// A variant of eth-keccak that computes a value that fits in a Starknet field element.
pub fn starknet_keccak(data: &[u8]) -> FieldElement {
    let mut hasher = Keccak256::new();
    hasher.update(data);
    let mut hash = hasher.finalize();

    // Remove the first 6 bits
    hash[0] &= 0b00000011;

    // Because we know hash is always 32 bytes
    FieldElement::from_bytes_be(unsafe { &*(hash[..].as_ptr() as *const [u8; 32]) }).unwrap()
}

pub fn get_selector_from_name(func_name: &str) -> Result<FieldElement, NonAsciiNameError> {
    if func_name == DEFAULT_ENTRY_POINT_NAME || func_name == DEFAULT_L1_ENTRY_POINT_NAME {
        Ok(FieldElement::ZERO)
    } else {
        let name_bytes = func_name.as_bytes();
        if name_bytes.is_ascii() {
            Ok(starknet_keccak(name_bytes))
        } else {
            Err(NonAsciiNameError)
        }
    }
}

pub fn get_storage_var_address(
    var_name: &str,
    args: &[FieldElement],
) -> Result<FieldElement, NonAsciiNameError> {
    let var_name_bytes = var_name.as_bytes();
    if var_name_bytes.is_ascii() {
        let mut res = starknet_keccak(var_name_bytes);
        for arg in args.iter() {
            res = pedersen_hash(&res, arg);
        }
        Ok(normalize_address(res))
    } else {
        Err(NonAsciiNameError)
    }
}

/// Converts Cairo short string to [FieldElement].
pub fn cairo_short_string_to_felt(str: &str) -> Result<FieldElement, CairoShortStringToFeltError> {
    if !str.is_ascii() {
        return Err(CairoShortStringToFeltError::NonAsciiCharacter);
    }
    if str.len() > 31 {
        return Err(CairoShortStringToFeltError::StringTooLong);
    }

    let ascii_bytes = str.as_bytes();

    let mut buffer = [0u8; 32];
    buffer[(32 - ascii_bytes.len())..].copy_from_slice(ascii_bytes);

    // The conversion will never fail
    Ok(FieldElement::from_bytes_be(&buffer).unwrap())
}

/// Converts [FieldElement] to Cairo short string.
pub fn parse_cairo_short_string(felt: &FieldElement) -> Result<String, ParseCairoShortStringError> {
    if felt == &FieldElement::ZERO {
        return Ok(String::new());
    }

    let be_bytes = felt.to_bytes_be();
    if be_bytes[0] > 0 {
        return Err(ParseCairoShortStringError::ValueOutOfRange);
    }

    let mut buffer = String::with_capacity(31);
    for byte in be_bytes.into_iter() {
        if byte == 0u8 {
            if !buffer.is_empty() {
                return Err(ParseCairoShortStringError::UnexpectedNullTerminator);
            }
        } else {
            buffer.push(byte as char)
        }
    }
    Ok(buffer)
}

/// Computes the target contract address of a "native" contract deployment. Use
/// `get_udc_deployed_address` instead if you want to compute the target address for deployments
/// through the Universal Deployer Contract.
pub fn get_contract_address(
    salt: FieldElement,
    class_hash: FieldElement,
    constructor_calldata: &[FieldElement],
    deployer_address: FieldElement,
) -> FieldElement {
    normalize_address(compute_hash_on_elements(&[
        CONTRACT_ADDRESS_PREFIX,
        deployer_address,
        salt,
        class_hash,
        compute_hash_on_elements(constructor_calldata),
    ]))
}

/// Computes the target contract address for deployments through the Universal Deploy Contract.
pub fn get_udc_deployed_address(
    salt: FieldElement,
    class_hash: FieldElement,
    uniqueness: &UdcUniqueness,
    constructor_calldata: &[FieldElement],
) -> FieldElement {
    match uniqueness {
        UdcUniqueness::NotUnique => {
            get_contract_address(salt, class_hash, constructor_calldata, FieldElement::ZERO)
        }
        UdcUniqueness::Unique(settings) => {
            let unique_salt = pedersen_hash(&settings.deployer_address, &salt);
            get_contract_address(
                unique_salt,
                class_hash,
                constructor_calldata,
                settings.udc_contract_address,
            )
        }
    }
}

pub fn normalize_address(address: FieldElement) -> FieldElement {
    address % ADDR_BOUND
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_starknet_keccak() {
        // Generated from `cairo-lang`
        let data = b"execute";
        let expected_hash = FieldElement::from_hex_be(
            "0240060cdb34fcc260f41eac7474ee1d7c80b7e3607daff9ac67c7ea2ebb1c44",
        )
        .unwrap();

        let hash = starknet_keccak(data);

        assert_eq!(hash, expected_hash);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_get_selector_from_name() {
        // Generated from `cairo-lang`
        let func_name = "execute";
        let expected_selector = FieldElement::from_hex_be(
            "0240060cdb34fcc260f41eac7474ee1d7c80b7e3607daff9ac67c7ea2ebb1c44",
        )
        .unwrap();

        let selector = get_selector_from_name(func_name).unwrap();

        assert_eq!(selector, expected_selector);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_get_default_selector() {
        let default_selector = FieldElement::from_hex_be(
            "0000000000000000000000000000000000000000000000000000000000000000",
        )
        .unwrap();

        assert_eq!(
            get_selector_from_name("__default__").unwrap(),
            default_selector
        );
        assert_eq!(
            get_selector_from_name("__l1_default__").unwrap(),
            default_selector
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_get_selector_from_non_ascii_name() {
        let func_name = "🦀";

        match get_selector_from_name(func_name) {
            Err(_) => {}
            _ => panic!("Should throw error on non-ASCII name"),
        };
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_get_storage_var_address() {
        // Generated from `cairo-lang`
        let var_name = "balance";
        let expected_addr = FieldElement::from_hex_be(
            "0x0206f38f7e4f15e87567361213c28f235cccdaa1d7fd34c9db1dfe9489c6a091",
        )
        .unwrap();

        let addr = get_storage_var_address(var_name, &[]).unwrap();

        assert_eq!(addr, expected_addr);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_get_storage_var_address_with_args() {
        // Generated from `cairo-lang`
        let var_name = "balanceOf";
        let expected_addr = FieldElement::from_hex_be(
            "0x07de334d65aa93d9185729b424025918b18892418c85b802775d1f0d2be30a1d",
        )
        .unwrap();

        let addr = get_storage_var_address(var_name, &[1234u64.into()]).unwrap();

        assert_eq!(addr, expected_addr);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_cairo_short_string_to_felt() {
        let data = [
            (
                "abcdefghijklmnopqrstuvwxyz",
                "156490583352162063278528710879425690470022892627113539022649722",
            ),
            (
                "1234567890123456789012345678901",
                "86921973946889608444641514252360676678984087116218318142845213717418291249",
            ),
        ];

        for (str, felt_dec) in data.into_iter() {
            assert_eq!(
                cairo_short_string_to_felt(str).unwrap(),
                FieldElement::from_dec_str(felt_dec).unwrap()
            );
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_cairo_short_string_to_felt_too_long() {
        assert!(matches!(
            cairo_short_string_to_felt("12345678901234567890123456789012"),
            Err(CairoShortStringToFeltError::StringTooLong)
        ));
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_cairo_short_string_to_felt_non_ascii() {
        assert!(matches!(
            cairo_short_string_to_felt("🦀"),
            Err(CairoShortStringToFeltError::NonAsciiCharacter)
        ));
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_parse_cairo_short_string() {
        let data = [
            (
                "abcdefghijklmnopqrstuvwxyz",
                "156490583352162063278528710879425690470022892627113539022649722",
            ),
            (
                "1234567890123456789012345678901",
                "86921973946889608444641514252360676678984087116218318142845213717418291249",
            ),
        ];

        for (str, felt_dec) in data.into_iter() {
            assert_eq!(
                parse_cairo_short_string(&FieldElement::from_dec_str(felt_dec).unwrap()).unwrap(),
                str
            );
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_parse_cairo_short_string_too_long() {
        assert!(matches!(
            parse_cairo_short_string(
                &FieldElement::from_hex_be(
                    "0x0111111111111111111111111111111111111111111111111111111111111111"
                )
                .unwrap()
            ),
            Err(ParseCairoShortStringError::ValueOutOfRange)
        ));
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_parse_cairo_short_string_unexpected_null() {
        assert!(matches!(
            parse_cairo_short_string(
                &FieldElement::from_hex_be(
                    "0x0011111111111111111111111111111111111111111111111111111111110011"
                )
                .unwrap()
            ),
            Err(ParseCairoShortStringError::UnexpectedNullTerminator)
        ));
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_get_contract_address() {
        assert_eq!(
            get_contract_address(
                FieldElement::from_hex_be(
                    "0x0018a7a329d1d85b621350f2b5fc9c64b2e57dfe708525f0aff2c90de1e5b9c8"
                )
                .unwrap(),
                FieldElement::from_hex_be(
                    "0x0750cd490a7cd1572411169eaa8be292325990d33c5d4733655fe6b926985062"
                )
                .unwrap(),
                &[FieldElement::ONE],
                FieldElement::ZERO
            ),
            FieldElement::from_hex_be(
                "0x00da27ef7c3869c3a6cc6a0f7bf07a51c3e590825adba8a51cae27d815839eec"
            )
            .unwrap()
        )
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_udc_address_not_unique() {
        let address = get_udc_deployed_address(
            FieldElement::from_hex_be(
                "0x06df0e9a9842d97ff3f4c6de7494d6e69d0a107a72150f9c53d59515b91ed9cb",
            )
            .unwrap(),
            FieldElement::from_hex_be(
                "0x0562fc1d911530d18a86ea3ef4be50018923898d3c573288c5abb9c2344459ed",
            )
            .unwrap(),
            &UdcUniqueness::NotUnique,
            &[FieldElement::from_hex_be("0x1234").unwrap()],
        );

        assert_eq!(
            FieldElement::from_hex_be(
                "0x0288e5952d2f2f0e897ea0c5401c6e9f584a89eebfb08b5b26f090a8bbf67eb6",
            )
            .unwrap(),
            address
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_udc_address_unique() {
        let address = get_udc_deployed_address(
            FieldElement::from_hex_be(
                "0x01f65976b95bf17ae1cb04afc9fc1eeee26d3e1aaa1f30aa535bf261e4322ab8",
            )
            .unwrap(),
            FieldElement::from_hex_be(
                "0x0562fc1d911530d18a86ea3ef4be50018923898d3c573288c5abb9c2344459ed",
            )
            .unwrap(),
            &UdcUniqueness::Unique(UdcUniqueSettings {
                deployer_address: FieldElement::from_hex_be(
                    "0x00b1461de04c6a1aa3375bdf9b7723a8779c082ffe21311d683a0b15c078b5dc",
                )
                .unwrap(),
                udc_contract_address: FieldElement::from_hex_be(
                    "0x041a78e741e5af2fec34b695679bc6891742439f7afb8484ecd7766661ad02bf",
                )
                .unwrap(),
            }),
            &[FieldElement::from_hex_be("0x1234").unwrap()],
        );

        assert_eq!(
            FieldElement::from_hex_be(
                "0x02406943b25942021f213b047c8765e531dddce3b981722f7aeb2ca137e18dbf",
            )
            .unwrap(),
            address
        );
    }
}
