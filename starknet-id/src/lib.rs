mod encoding;
pub use encoding::{decode, encode};

mod naming;
pub use naming::{address_to_domain, domain_to_address, ResolvingError, MAINNET_CONTRACT, GOERLI_CONTRACT};
