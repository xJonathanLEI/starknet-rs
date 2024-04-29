mod account;
pub use account::{
    Account, AccountError, ConnectedAccount, DeclarationV2, DeclarationV3, ExecutionEncoder,
    ExecutionV1, ExecutionV3, LegacyDeclaration, PreparedDeclarationV2, PreparedDeclarationV3,
    PreparedExecutionV1, PreparedExecutionV3, PreparedLegacyDeclaration, RawDeclarationV2,
    RawDeclarationV3, RawExecutionV1, RawExecutionV3, RawLegacyDeclaration,
};

mod call;
pub use call::Call;

mod factory;
pub use factory::{
    argent::ArgentAccountFactory, open_zeppelin::OpenZeppelinAccountFactory, AccountDeploymentV1,
    AccountDeploymentV3, AccountFactory, AccountFactoryError, PreparedAccountDeploymentV1,
    PreparedAccountDeploymentV3, RawAccountDeploymentV1, RawAccountDeploymentV3,
};

pub mod single_owner;
pub use single_owner::{ExecutionEncoding, SingleOwnerAccount};

#[derive(Debug, thiserror::Error)]
#[error("Not all fields are prepared")]
pub struct NotPreparedError;
