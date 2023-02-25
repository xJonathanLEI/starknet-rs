use starknet_core::types::{FieldElement};

#[derive(Debug)]
pub enum ResolvingError {
    ConnectionError(String),
    InvalidContractResult,
    InvalidDomain,
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

// pub async fn domain_to_address(
//     domain: &str,
//     provider: SequencerGatewayProvider,
//     contract_addr: FieldElement,
// ) -> Result<FieldElement, ResolvingError> {
//     if !domain.ends_with(".stark") {
//         return Err(ResolvingError::InvalidDomain);
//     }
//     let encoding_result = encode(&domain[0..domain.len() - 6]);
//     match encoding_result {
//         Ok(encoded) => {
//             match provider
//                 .call_contract(
//                     CallFunction {
//                         contract_address: contract_addr,
//                         entry_point_selector: SELECTOR_D2A,
//                         calldata: vec![FieldElement::ONE, encoded],
//                     },
//                     BlockId::Latest,
//                 )
//                 .await
//             {
//                 Ok(result) => match result.result.first() {
//                     Some(x) => Ok(*x),
//                     None => Err(ResolvingError::InvalidContractResult),
//                 },
//                 Err(cause) => Err(ResolvingError::ConnectionError(cause.to_string())),
//             }
//         }
//         Err(_) => Err(ResolvingError::InvalidDomain),
//     }
// }

// // default values for mainnet
// #[macro_export]
// macro_rules! domain_to_address {
//     ($domain: expr) => {
//         domain_to_address(
//             $domain,
//             SequencerGatewayProvider::starknet_alpha_mainnet(),
//             FieldElement::from_mont([
//                 9876522541644636344,
//                 16204762974907305178,
//                 9525933456780166611,
//                 327799339589885214,
//             ]),
//         )
//     };
// }



// pub async fn address_to_domain(
//     address: FieldElement,
//     provider: SequencerGatewayProvider,
//     contract_addr: FieldElement,
// ) -> Result<String, ResolvingError> {
//     match provider
//         .call_contract(
//             CallFunction {
//                 contract_address: contract_addr,
//                 entry_point_selector: SELECTOR_A2D,
//                 calldata: vec![address],
//             },
//             BlockId::Latest,
//         )
//         .await
//     {
//         Ok(result) => {
//             let mut calldata = result.result.iter();
//             let mut domain = String::new().to_owned();
//             match calldata.next() {
//                 Some(_) => {
//                     calldata.for_each(|value| {
//                         domain.push_str(decode(*value).as_str());
//                         domain.push('.');
//                     });
//                     domain.push_str("stark");
//                     Ok(domain)
//                 }
//                 None => Err(ResolvingError::InvalidContractResult),
//             }
//         }
//         Err(cause) => Err(ResolvingError::ConnectionError(cause.to_string())),
//     }
// }

// default values for mainnet
// #[macro_export]
// macro_rules! address_to_domain {
//     ($address: expr) => {
//         address_to_domain(
//             $address,
//             SequencerGatewayProvider::starknet_alpha_mainnet(),
//             FieldElement::from_mont([
//                 9876522541644636344,
//                 16204762974907305178,
//                 9525933456780166611,
//                 327799339589885214,
//             ]),
//         )
//     };
// }
