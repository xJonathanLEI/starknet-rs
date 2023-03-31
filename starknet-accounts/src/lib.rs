mod account;
pub use account::{
    Account, AccountError, ConnectedAccount, Declaration, Execution, LegacyDeclaration,
    PreparedDeclaration, PreparedExecution, PreparedLegacyDeclaration, RawDeclaration,
    RawExecution, RawLegacyDeclaration,
};

mod call;
pub use call::Call;

mod factory;
pub use factory::{
    argent::ArgentAccountFactory, open_zeppelin::OpenZeppelinAccountFactory, AccountDeployment,
    AccountFactory, AccountFactoryError, PreparedAccountDeployment, RawAccountDeployment,
};

pub mod single_owner;
pub use single_owner::SingleOwnerAccount;

#[derive(Debug, thiserror::Error)]
#[error("Not all fields are prepared")]
pub struct NotPreparedError;
