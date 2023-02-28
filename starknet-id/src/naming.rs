use starknet_core::types::FieldElement;

#[derive(Debug)]
pub enum ResolvingError {
    ConnectionError(String),
    InvalidContractResult,
    InvalidDomain,
    NotSupported,
}

pub const SELECTOR_D2A: FieldElement = FieldElement::from_mont([
    6985039847805449502,
    5414334324946440161,
    14839945581867836860,
    261254206219932239,
]);

pub const MAINNET_CONTRACT: FieldElement = FieldElement::from_mont([
    9876522541644636344,
    16204762974907305178,
    9525933456780166611,
    327799339589885214,
]);

pub const GOERLI_CONTRACT: FieldElement = FieldElement::from_mont([
    3991710935722461676,
    1453192132188820719,
    4558680749370441117,
    452192057203262238,
]);

pub const SELECTOR_A2D: FieldElement = FieldElement::from_mont([
    14453853710431432356,
    1760183467521543892,
    6971481136651747063,
    40507466578104802,
]);
