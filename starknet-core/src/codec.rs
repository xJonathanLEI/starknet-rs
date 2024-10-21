use alloc::{boxed::Box, fmt::Formatter, format, string::*, vec::*};
use core::{fmt::Display, mem::MaybeUninit};

use num_traits::ToPrimitive;

use crate::types::{Felt, U256};

pub use starknet_core_derive::{Decode, Encode};

/// Any type where [`Felt`]s can be written into. This would typically be [`Vec<Felt>`], but can
/// also be something like a stateful hasher.
///
/// The trait method is infallible, as the most common use case is to simply write into a `Vec`.
/// Making the method infallible avoids over-engineering. However, if deemed necessary, a future
/// breaking change can make this fallible instead.
pub trait FeltWriter {
    /// Adds a single [Felt] element into the writer.
    fn write(&mut self, felt: Felt);
}

/// Any type that can be serialized into a series of [Felt]s. This trait corresponds to the
/// `serialize` function of the Cairo `Serde` trait.
pub trait Encode {
    /// Converts the type into a list of [`Felt`] and append them into the writer.
    fn encode<W: FeltWriter>(&self, writer: &mut W) -> Result<(), Error>;
}

/// Any type that can be deserialized from a series of [Felt]s. This trait corresponds to the
/// `deserialize` function of the Cairo `Serde` trait.
pub trait Decode<'a>: Sized {
    /// Converts into the type from a list of [`Felt`].
    fn decode<T>(reader: T) -> Result<Self, Error>
    where
        T: IntoIterator<Item = &'a Felt>,
    {
        Self::decode_iter(&mut reader.into_iter())
    }

    /// Converts into the type from an iterator of references to [`Felt`].
    fn decode_iter<T>(iter: &mut T) -> Result<Self, Error>
    where
        T: Iterator<Item = &'a Felt>;
}

/// Error type for any encoding/decoding operations.
///
/// A simple string representation is forced onto all implementations for simplicity. This is
/// because most of the time, a encoding/decoding error indicates a bug that requires human
/// attention to fix anyway; even when handling untrusted data, the program is likely to only be
/// interested in knowing that an error _did_ occur, instead of handling based on cause.
///
/// There might be cases where allocations must be avoided. A feature could be added in the future
/// that turns the `repr` into `()` to address this. Such a feature would be a non-breaking change
/// so there's no need to add it now.
#[derive(Debug)]
pub struct Error {
    repr: Box<str>,
}

impl FeltWriter for Vec<Felt> {
    fn write(&mut self, felt: Felt) {
        self.push(felt);
    }
}

impl Encode for Felt {
    fn encode<W: FeltWriter>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write(*self);
        Ok(())
    }
}

impl Encode for bool {
    fn encode<W: FeltWriter>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write(if *self { Felt::ONE } else { Felt::ZERO });
        Ok(())
    }
}

impl Encode for u8 {
    fn encode<W: FeltWriter>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write((*self).into());
        Ok(())
    }
}

impl Encode for u16 {
    fn encode<W: FeltWriter>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write((*self).into());
        Ok(())
    }
}

impl Encode for u32 {
    fn encode<W: FeltWriter>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write((*self).into());
        Ok(())
    }
}

impl Encode for u64 {
    fn encode<W: FeltWriter>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write((*self).into());
        Ok(())
    }
}

impl Encode for u128 {
    fn encode<W: FeltWriter>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write((*self).into());
        Ok(())
    }
}

impl Encode for U256 {
    fn encode<W: FeltWriter>(&self, writer: &mut W) -> Result<(), Error> {
        self.low().encode(writer)?;
        self.high().encode(writer)?;
        Ok(())
    }
}

impl<T> Encode for Option<T>
where
    T: Encode,
{
    fn encode<W: FeltWriter>(&self, writer: &mut W) -> Result<(), Error> {
        match self {
            Some(inner) => {
                writer.write(Felt::ZERO);
                inner.encode(writer)?;
            }
            None => {
                writer.write(Felt::ONE);
            }
        }

        Ok(())
    }
}

impl<T> Encode for Vec<T>
where
    T: Encode,
{
    fn encode<W: FeltWriter>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write(Felt::from(self.len()));

        for item in self {
            item.encode(writer)?;
        }

        Ok(())
    }
}

impl<T, const N: usize> Encode for [T; N]
where
    T: Encode,
{
    fn encode<W: FeltWriter>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write(Felt::from(N));

        for item in self {
            item.encode(writer)?;
        }

        Ok(())
    }
}

impl<T> Encode for [T]
where
    T: Encode,
{
    fn encode<W: FeltWriter>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write(Felt::from(self.len()));

        for item in self {
            item.encode(writer)?;
        }

        Ok(())
    }
}

impl<'a> Decode<'a> for Felt {
    fn decode_iter<T>(iter: &mut T) -> Result<Self, Error>
    where
        T: Iterator<Item = &'a Self>,
    {
        iter.next().ok_or_else(Error::input_exhausted).cloned()
    }
}

impl<'a> Decode<'a> for bool {
    fn decode_iter<T>(iter: &mut T) -> Result<Self, Error>
    where
        T: Iterator<Item = &'a Felt>,
    {
        let input = iter.next().ok_or_else(Error::input_exhausted)?;
        if input == &Felt::ZERO {
            Ok(false)
        } else if input == &Felt::ONE {
            Ok(true)
        } else {
            Err(Error::value_out_of_range(input, "bool"))
        }
    }
}

impl<'a> Decode<'a> for u8 {
    fn decode_iter<T>(iter: &mut T) -> Result<Self, Error>
    where
        T: Iterator<Item = &'a Felt>,
    {
        let input = iter.next().ok_or_else(Error::input_exhausted)?;
        input
            .to_u8()
            .ok_or_else(|| Error::value_out_of_range(input, "u8"))
    }
}

impl<'a> Decode<'a> for u16 {
    fn decode_iter<T>(iter: &mut T) -> Result<Self, Error>
    where
        T: Iterator<Item = &'a Felt>,
    {
        let input = iter.next().ok_or_else(Error::input_exhausted)?;
        input
            .to_u16()
            .ok_or_else(|| Error::value_out_of_range(input, "u16"))
    }
}

impl<'a> Decode<'a> for u32 {
    fn decode_iter<T>(iter: &mut T) -> Result<Self, Error>
    where
        T: Iterator<Item = &'a Felt>,
    {
        let input = iter.next().ok_or_else(Error::input_exhausted)?;
        input
            .to_u32()
            .ok_or_else(|| Error::value_out_of_range(input, "u32"))
    }
}

impl<'a> Decode<'a> for u64 {
    fn decode_iter<T>(iter: &mut T) -> Result<Self, Error>
    where
        T: Iterator<Item = &'a Felt>,
    {
        let input = iter.into_iter().next().ok_or_else(Error::input_exhausted)?;
        input
            .to_u64()
            .ok_or_else(|| Error::value_out_of_range(input, "u64"))
    }
}

impl<'a> Decode<'a> for u128 {
    fn decode_iter<T>(iter: &mut T) -> Result<Self, Error>
    where
        T: Iterator<Item = &'a Felt>,
    {
        let input = iter.next().ok_or_else(Error::input_exhausted)?;
        input
            .to_u128()
            .ok_or_else(|| Error::value_out_of_range(input, "u128"))
    }
}

impl<'a> Decode<'a> for U256 {
    fn decode_iter<T>(iter: &mut T) -> Result<Self, Error>
    where
        T: Iterator<Item = &'a Felt>,
    {
        let input_low = iter.next().ok_or_else(Error::input_exhausted)?;
        let input_high = iter.next().ok_or_else(Error::input_exhausted)?;

        let input_low = input_low
            .to_u128()
            .ok_or_else(|| Error::value_out_of_range(input_low, "u128"))?;
        let input_high = input_high
            .to_u128()
            .ok_or_else(|| Error::value_out_of_range(input_high, "u128"))?;

        Ok(Self::from_words(input_low, input_high))
    }
}

impl<'a, T> Decode<'a> for Option<T>
where
    T: Decode<'a>,
{
    fn decode_iter<I>(iter: &mut I) -> Result<Self, Error>
    where
        I: Iterator<Item = &'a Felt>,
    {
        let tag = iter.next().ok_or_else(Error::input_exhausted)?;

        if tag == &Felt::ZERO {
            Ok(Some(T::decode_iter(iter)?))
        } else if tag == &Felt::ONE {
            Ok(None)
        } else {
            Err(Error::unknown_enum_tag(tag, "Option<T>"))
        }
    }
}

impl<'a, T> Decode<'a> for Vec<T>
where
    T: Decode<'a>,
{
    fn decode_iter<I>(iter: &mut I) -> Result<Self, Error>
    where
        I: Iterator<Item = &'a Felt>,
    {
        let length = iter.next().ok_or_else(Error::input_exhausted)?;
        let length = length
            .to_usize()
            .ok_or_else(|| Error::value_out_of_range(length, "usize"))?;

        let mut result = Self::with_capacity(length);

        for _ in 0..length {
            result.push(T::decode_iter(iter)?);
        }

        Ok(result)
    }
}

impl<'a, T, const N: usize> Decode<'a> for [T; N]
where
    T: Decode<'a> + Sized,
{
    fn decode_iter<I>(iter: &mut I) -> Result<Self, Error>
    where
        I: Iterator<Item = &'a Felt>,
    {
        let length = iter.next().ok_or_else(Error::input_exhausted)?;
        let length = length
            .to_usize()
            .ok_or_else(|| Error::value_out_of_range(length, "usize"))?;

        if length != N {
            return Err(Error::length_mismatch(N, length));
        }

        let mut result: [MaybeUninit<T>; N] = unsafe { MaybeUninit::uninit().assume_init() };

        for elem in &mut result[..] {
            *elem = MaybeUninit::new(T::decode_iter(iter)?);
        }

        Ok(unsafe { core::mem::transmute_copy::<_, [T; N]>(&result) })
    }
}

impl Error {
    /// Creates an [`Error`] which indicates that the input stream has ended prematurely.
    pub fn input_exhausted() -> Self {
        Self {
            repr: "unexpected end of input stream"
                .to_string()
                .into_boxed_str(),
        }
    }

    /// Creates an [`Error`] which indicates that the length (likely prefix) is different from the
    /// expected value.
    pub fn length_mismatch(expected: usize, actual: usize) -> Self {
        Self {
            repr: format!("expecting length `{}` but got `{}`", expected, actual).into_boxed_str(),
        }
    }

    /// Creates an [`Error`] which indicates that the input value is out of range.
    pub fn value_out_of_range<V>(value: V, type_name: &str) -> Self
    where
        V: Display,
    {
        Self {
            repr: format!("value `{}` is out of range for type `{}`", value, type_name)
                .into_boxed_str(),
        }
    }

    /// Creates an [`Error`] which indicates that the enum tag does not belong to a known variant.
    pub fn unknown_enum_tag<V>(tag: V, type_name: &str) -> Self
    where
        V: Display,
    {
        Self {
            repr: format!("enum tag `{}` is unknown for type `{}`", tag, type_name)
                .into_boxed_str(),
        }
    }

    /// Creates an [`Error`] using a custom error string.
    pub fn custom<T>(content: T) -> Self
    where
        T: Display,
    {
        Self {
            repr: content.to_string().into_boxed_str(),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.repr)
    }
}

#[cfg(test)]
mod tests {
    use core::str::FromStr;

    use super::*;

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_encode_felt() {
        let mut serialized = Vec::<Felt>::new();
        Felt::from_str("99999999999999999999999999")
            .unwrap()
            .encode(&mut serialized)
            .unwrap();
        assert_eq!(
            serialized,
            vec![Felt::from_str("99999999999999999999999999").unwrap()]
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_encode_bool() {
        let mut serialized = Vec::<Felt>::new();
        true.encode(&mut serialized).unwrap();
        assert_eq!(serialized, vec![Felt::from_str("1").unwrap()]);

        let mut serialized = Vec::<Felt>::new();
        false.encode(&mut serialized).unwrap();
        assert_eq!(serialized, vec![Felt::from_str("0").unwrap()]);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_encode_u8() {
        let mut serialized = Vec::<Felt>::new();
        123u8.encode(&mut serialized).unwrap();
        assert_eq!(serialized, vec![Felt::from_str("123").unwrap()]);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_encode_u16() {
        let mut serialized = Vec::<Felt>::new();
        12345u16.encode(&mut serialized).unwrap();
        assert_eq!(serialized, vec![Felt::from_str("12345").unwrap()]);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_encode_u32() {
        let mut serialized = Vec::<Felt>::new();
        1234567890u32.encode(&mut serialized).unwrap();
        assert_eq!(serialized, vec![Felt::from_str("1234567890").unwrap()]);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_encode_u64() {
        let mut serialized = Vec::<Felt>::new();
        12345678900000000000u64.encode(&mut serialized).unwrap();
        assert_eq!(
            serialized,
            vec![Felt::from_str("12345678900000000000").unwrap()]
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_encode_u128() {
        let mut serialized = Vec::<Felt>::new();
        123456789000000000000000000000u128
            .encode(&mut serialized)
            .unwrap();
        assert_eq!(
            serialized,
            vec![Felt::from_str("123456789000000000000000000000").unwrap()]
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_encode_u256() {
        let mut serialized = Vec::<Felt>::new();
        U256::from_words(12345, 67890)
            .encode(&mut serialized)
            .unwrap();
        assert_eq!(
            serialized,
            vec![
                Felt::from_str("12345").unwrap(),
                Felt::from_str("67890").unwrap()
            ]
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_encode_option() {
        let mut serialized = Vec::<Felt>::new();
        Some(10u32).encode(&mut serialized).unwrap();
        assert_eq!(
            serialized,
            vec![Felt::from_str("0").unwrap(), Felt::from_str("10").unwrap()]
        );

        serialized.clear();
        Option::<u32>::None.encode(&mut serialized).unwrap();
        assert_eq!(serialized, vec![Felt::from_str("1").unwrap()]);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_encode_vec() {
        let mut serialized = Vec::<Felt>::new();
        vec![Some(10u32), None].encode(&mut serialized).unwrap();
        assert_eq!(
            serialized,
            vec![
                Felt::from_str("2").unwrap(),
                Felt::from_str("0").unwrap(),
                Felt::from_str("10").unwrap(),
                Felt::from_str("1").unwrap()
            ]
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_encode_array() {
        let mut serialized = Vec::<Felt>::new();
        <[Option<u32>; 2]>::encode(&[Some(10u32), None], &mut serialized).unwrap();
        assert_eq!(
            serialized,
            vec![
                Felt::from_str("2").unwrap(),
                Felt::from_str("0").unwrap(),
                Felt::from_str("10").unwrap(),
                Felt::from_str("1").unwrap()
            ]
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_encode_slice() {
        let mut serialized = Vec::<Felt>::new();
        <[Option<u32>]>::encode(&[Some(10u32), None], &mut serialized).unwrap();
        assert_eq!(
            serialized,
            vec![
                Felt::from_str("2").unwrap(),
                Felt::from_str("0").unwrap(),
                Felt::from_str("10").unwrap(),
                Felt::from_str("1").unwrap()
            ]
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_derive_encode_struct_named() {
        #[derive(Encode)]
        #[starknet(core = "crate")]
        struct CairoType {
            a: Felt,
            b: U256,
            c: bool,
        }

        let mut serialized = Vec::<Felt>::new();
        CairoType {
            a: Felt::from_str("12345").unwrap(),
            b: U256::from_words(12, 34),
            c: true,
        }
        .encode(&mut serialized)
        .unwrap();
        assert_eq!(
            serialized,
            vec![
                Felt::from_str("12345").unwrap(),
                Felt::from_str("12").unwrap(),
                Felt::from_str("34").unwrap(),
                Felt::from_str("1").unwrap(),
            ]
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_derive_encode_struct_tuple() {
        #[derive(Encode)]
        #[starknet(core = "crate")]
        struct CairoType(Felt, U256, bool);

        let mut serialized = Vec::<Felt>::new();
        CairoType(
            Felt::from_str("12345").unwrap(),
            U256::from_words(12, 34),
            true,
        )
        .encode(&mut serialized)
        .unwrap();
        assert_eq!(
            serialized,
            vec![
                Felt::from_str("12345").unwrap(),
                Felt::from_str("12").unwrap(),
                Felt::from_str("34").unwrap(),
                Felt::from_str("1").unwrap(),
            ]
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_derive_encode_enum() {
        #[derive(Encode)]
        #[starknet(core = "crate")]
        enum CairoType {
            A,
            B(bool),
            C(Option<U256>, u8),
            D { a: u64, b: bool },
        }

        let mut serialized = Vec::<Felt>::new();
        CairoType::A.encode(&mut serialized).unwrap();
        assert_eq!(serialized, vec![Felt::from_str("0").unwrap()]);

        serialized.clear();
        CairoType::B(true).encode(&mut serialized).unwrap();
        assert_eq!(
            serialized,
            vec![Felt::from_str("1").unwrap(), Felt::from_str("1").unwrap()]
        );

        serialized.clear();
        CairoType::C(Some(U256::from_words(12, 23)), 4)
            .encode(&mut serialized)
            .unwrap();
        assert_eq!(
            serialized,
            vec![
                Felt::from_str("2").unwrap(),
                Felt::from_str("0").unwrap(),
                Felt::from_str("12").unwrap(),
                Felt::from_str("23").unwrap(),
                Felt::from_str("4").unwrap(),
            ]
        );

        serialized.clear();
        CairoType::C(None, 8).encode(&mut serialized).unwrap();
        assert_eq!(
            serialized,
            vec![
                Felt::from_str("2").unwrap(),
                Felt::from_str("1").unwrap(),
                Felt::from_str("8").unwrap(),
            ]
        );

        serialized.clear();
        CairoType::D { a: 100, b: false }
            .encode(&mut serialized)
            .unwrap();
        assert_eq!(
            serialized,
            vec![
                Felt::from_str("3").unwrap(),
                Felt::from_str("100").unwrap(),
                Felt::from_str("0").unwrap()
            ]
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_decode_felt() {
        assert_eq!(
            Felt::from_str("99999999999999999999999999").unwrap(),
            Felt::decode(&[Felt::from_str("99999999999999999999999999").unwrap()]).unwrap()
        );
    }

    #[allow(clippy::bool_assert_comparison)]
    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_decode_bool() {
        assert_eq!(true, bool::decode(&[Felt::from_str("1").unwrap()]).unwrap());

        assert_eq!(
            false,
            bool::decode(&[Felt::from_str("0").unwrap()]).unwrap()
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_decode_u8() {
        assert_eq!(
            123u8,
            u8::decode(&[Felt::from_str("123").unwrap()]).unwrap()
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_decode_u16() {
        assert_eq!(
            12345u16,
            u16::decode(&[Felt::from_str("12345").unwrap()]).unwrap()
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_decode_u32() {
        assert_eq!(
            1234567890u32,
            u32::decode(&[Felt::from_str("1234567890").unwrap()]).unwrap()
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_decode_u64() {
        assert_eq!(
            12345678900000000000u64,
            u64::decode(&[Felt::from_str("12345678900000000000").unwrap()]).unwrap()
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_decode_u128() {
        assert_eq!(
            123456789000000000000000000000u128,
            u128::decode(&[Felt::from_str("123456789000000000000000000000").unwrap()]).unwrap()
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_decode_u256() {
        assert_eq!(
            U256::from_words(12345, 67890),
            U256::decode(&[
                Felt::from_str("12345").unwrap(),
                Felt::from_str("67890").unwrap()
            ])
            .unwrap()
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_decode_option() {
        assert_eq!(
            Some(10u32),
            Option::<u32>::decode(&[Felt::from_str("0").unwrap(), Felt::from_str("10").unwrap()])
                .unwrap()
        );

        assert_eq!(
            Option::<u32>::None,
            Option::<u32>::decode(&[Felt::from_str("1").unwrap()]).unwrap()
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_decode_vec() {
        assert_eq!(
            vec![Some(10u32), None],
            Vec::<Option::<u32>>::decode(&[
                Felt::from_str("2").unwrap(),
                Felt::from_str("0").unwrap(),
                Felt::from_str("10").unwrap(),
                Felt::from_str("1").unwrap()
            ])
            .unwrap()
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_decode_array() {
        assert_eq!(
            [Some(10u32), None],
            <[Option<u32>; 2]>::decode(&[
                Felt::from_str("2").unwrap(),
                Felt::from_str("0").unwrap(),
                Felt::from_str("10").unwrap(),
                Felt::from_str("1").unwrap()
            ])
            .unwrap()
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_derive_decode_struct_named() {
        #[derive(Debug, PartialEq, Eq, Decode)]
        #[starknet(core = "crate")]
        struct CairoType {
            a: Felt,
            b: U256,
            c: bool,
        }

        assert_eq!(
            CairoType {
                a: Felt::from_str("12345").unwrap(),
                b: U256::from_words(12, 34),
                c: true,
            },
            CairoType::decode(&[
                Felt::from_str("12345").unwrap(),
                Felt::from_str("12").unwrap(),
                Felt::from_str("34").unwrap(),
                Felt::from_str("1").unwrap(),
            ])
            .unwrap()
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_derive_decode_struct_tuple() {
        #[derive(Debug, PartialEq, Eq, Decode)]
        #[starknet(core = "crate")]
        struct CairoType(Felt, U256, bool);

        assert_eq!(
            CairoType(
                Felt::from_str("12345").unwrap(),
                U256::from_words(12, 34),
                true,
            ),
            CairoType::decode(&[
                Felt::from_str("12345").unwrap(),
                Felt::from_str("12").unwrap(),
                Felt::from_str("34").unwrap(),
                Felt::from_str("1").unwrap(),
            ])
            .unwrap()
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_derive_decode_enum() {
        #[derive(Debug, PartialEq, Eq, Decode)]
        #[starknet(core = "crate")]
        enum CairoType {
            A,
            B(bool),
            C(Option<U256>, u8),
            D { a: u64, b: bool },
        }

        assert_eq!(
            CairoType::A,
            CairoType::decode(&[Felt::from_str("0").unwrap()]).unwrap()
        );

        assert_eq!(
            CairoType::B(true),
            CairoType::decode(&[Felt::from_str("1").unwrap(), Felt::from_str("1").unwrap()])
                .unwrap()
        );

        assert_eq!(
            CairoType::C(Some(U256::from_words(12, 23)), 4),
            CairoType::decode(&[
                Felt::from_str("2").unwrap(),
                Felt::from_str("0").unwrap(),
                Felt::from_str("12").unwrap(),
                Felt::from_str("23").unwrap(),
                Felt::from_str("4").unwrap(),
            ])
            .unwrap()
        );

        assert_eq!(
            CairoType::C(None, 8),
            CairoType::decode(&[
                Felt::from_str("2").unwrap(),
                Felt::from_str("1").unwrap(),
                Felt::from_str("8").unwrap(),
            ])
            .unwrap()
        );

        assert_eq!(
            CairoType::D { a: 100, b: false },
            CairoType::decode(&[
                Felt::from_str("3").unwrap(),
                Felt::from_str("100").unwrap(),
                Felt::from_str("0").unwrap()
            ])
            .unwrap()
        );
    }
}
