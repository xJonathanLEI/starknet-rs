//! Library for deploying and using Starknet account contracts.

#![deny(missing_docs)]

mod account;
pub use account::{
    Account, AccountError, ConnectedAccount, DeclarationV2, DeclarationV3, ExecutionEncoder,
    ExecutionV1, ExecutionV3, LegacyDeclaration, PreparedDeclarationV2, PreparedDeclarationV3,
    PreparedExecutionV1, PreparedExecutionV3, PreparedLegacyDeclaration, RawDeclarationV2,
    RawDeclarationV3, RawExecutionV1, RawExecutionV3, RawLegacyDeclaration,
};

mod factory;
pub use factory::{
    argent::ArgentAccountFactory, open_zeppelin::OpenZeppelinAccountFactory, AccountDeploymentV1,
    AccountDeploymentV3, AccountFactory, AccountFactoryError, PreparedAccountDeploymentV1,
    PreparedAccountDeploymentV3, RawAccountDeploymentV1, RawAccountDeploymentV3,
};

/// Module containing types for using an account contract with only one signer.
pub mod single_owner;
pub use single_owner::{ExecutionEncoding, SingleOwnerAccount};

/// Error when calling `prepared()` on a type when not all fields are populated.
#[derive(Debug, thiserror::Error)]
#[error("Not all fields are prepared")]
pub struct NotPreparedError;
