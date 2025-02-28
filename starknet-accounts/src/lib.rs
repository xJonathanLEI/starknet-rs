//! Library for deploying and using Starknet account contracts.

#![deny(missing_docs)]

mod account;
pub use account::{
    Account, AccountError, ConnectedAccount, DeclarationV3, ExecutionEncoder, ExecutionV3,
    PreparedDeclarationV3, PreparedExecutionV3, RawDeclarationV3, RawExecutionV3,
};

mod factory;
pub use factory::{
    argent::ArgentAccountFactory, open_zeppelin::OpenZeppelinAccountFactory, AccountDeploymentV3,
    AccountFactory, AccountFactoryError, PreparedAccountDeploymentV3, RawAccountDeploymentV3,
};

/// Module containing types for using an account contract with only one signer.
pub mod single_owner;
pub use single_owner::{ExecutionEncoding, SingleOwnerAccount};

/// Error when calling `prepared()` on a type when not all fields are populated.
#[derive(Debug, thiserror::Error)]
#[error("Not all fields are prepared")]
pub struct NotPreparedError;
