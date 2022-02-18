use ark_ff::{
    biginteger::BigInteger256,
    fields::{FftParameters, Fp256Parameters, FpParameters},
};

pub struct FrParameters;

impl Fp256Parameters for FrParameters {}

impl FftParameters for FrParameters {
    type BigInt = BigInteger256;

    const TWO_ADICITY: u32 = 192;

    const TWO_ADIC_ROOT_OF_UNITY: BigInteger256 = BigInteger256::new([
        0x4106bccd64a2bdd8,
        0xaaada25731fe3be9,
        0xa35c5be60505574,
        0x7222e32c47afc26,
    ]);
}

impl FpParameters for FrParameters {
    const MODULUS: BigInteger256 = BigInteger256::new([0x1, 0x0, 0x0, 0x800000000000011]);

    const MODULUS_BITS: u32 = 252;

    const CAPACITY: u32 = Self::MODULUS_BITS - 1;

    const REPR_SHAVE_BITS: u32 = 4;

    const R: BigInteger256 = BigInteger256::new([
        0xffffffffffffffe1,
        0xffffffffffffffff,
        0xffffffffffffffff,
        0x7fffffffffffdf0,
    ]);

    const R2: BigInteger256 = BigInteger256::new([
        0xfffffd737e000401,
        0x1330fffff,
        0xffffffffff6f8000,
        0x7ffd4ab5e008810,
    ]);

    const INV: u64 = 0xffffffffffffffff;

    const GENERATOR: BigInteger256 = BigInteger256::new([
        0xffffffffffffffa1,
        0xffffffffffffffff,
        0xffffffffffffffff,
        0x7fffffffffff9b0,
    ]);

    const MODULUS_MINUS_ONE_DIV_TWO: BigInteger256 =
        BigInteger256::new([0x0, 0x0, 0x8000000000000000, 0x400000000000008]);

    const T: BigInteger256 = BigInteger256::new([0x800000000000011, 0x0, 0x0, 0x0]);

    const T_MINUS_ONE_DIV_TWO: BigInteger256 =
        BigInteger256::new([0x400000000000008, 0x0, 0x0, 0x0]);
}
