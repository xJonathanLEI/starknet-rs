use async_trait::async_trait;
use auto_impl::auto_impl;
use starknet_core::types::{
    contract::ComputeClassHashError, BlockId, BlockTag, Call, Felt, FlattenedSierraClass,
};
use starknet_providers::{Provider, ProviderError};
use starknet_signers::SignerInteractivityContext;
use std::{error::Error, sync::Arc};

mod declaration;
mod execution;

/// The standard Starknet account contract interface. It makes no assumption about the underlying
/// signer or provider. Account implementations that come with an active connection to the network
/// should also implement [ConnectedAccount] for useful functionalities like estimating fees and
/// sending transactions.
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
pub trait Account: ExecutionEncoder + Sized {
    /// Possible errors for signing transactions.
    type SignError: Error + Send + Sync;

    /// Gets the account contract's address.
    fn address(&self) -> Felt;

    /// Gets the chain ID of the network where the account contract was deployed.
    fn chain_id(&self) -> Felt;

    /// Signs an execution request to authorize an `INVOKE` v3 transaction that pays transaction
    /// fees in `STRK`.
    ///
    /// If `query_only` is `true`, the commitment must be constructed in a way that a real state-
    /// changing transaction cannot be authenticated. This is to prevent replay attacks.
    async fn sign_execution_v3(
        &self,
        execution: &RawExecutionV3,
        query_only: bool,
    ) -> Result<Vec<Felt>, Self::SignError>;

    /// Signs an execution request to authorize an `DECLARE` v3 transaction that pays transaction
    /// fees in `STRK` for declaring Cairo 1 classes.
    ///
    /// If `query_only` is `true`, the commitment must be constructed in a way that a real state-
    /// changing transaction cannot be authenticated. This is to prevent replay attacks.
    async fn sign_declaration_v3(
        &self,
        declaration: &RawDeclarationV3,
        query_only: bool,
    ) -> Result<Vec<Felt>, Self::SignError>;

    /// Whether the underlying signer implementation is interactive, such as a hardware wallet.
    /// Implementations should return `true` if the signing operation is very expensive, even if not
    /// strictly "interactive" as in requiring human input.
    ///
    /// This affects how an account makes decision on whether to request a real signature for
    /// estimation/simulation purposes.
    fn is_signer_interactive(&self, context: SignerInteractivityContext<'_>) -> bool;

    /// Generates an instance of [`ExecutionV3`] for sending `INVOKE` v3 transactions. Pays
    /// transaction fees in `STRK`.
    fn execute_v3(&self, calls: Vec<Call>) -> ExecutionV3<'_, Self> {
        ExecutionV3::new(calls, self)
    }

    /// Generates an instance of [`ExecutionV3`] for sending `INVOKE` v3 transactions. Pays
    /// transaction fees in `STRK`.
    #[deprecated = "transaction version used might change unexpectedly; use `execute_v3` instead"]
    fn execute(&self, calls: Vec<Call>) -> ExecutionV3<'_, Self> {
        self.execute_v3(calls)
    }

    /// Generates an instance of [`DeclarationV3`] for sending `DECLARE` v3 transactions. Pays
    /// transaction fees in `STRK`.
    ///
    /// To declare a Sierra (Cairo 1) class, a `compiled_class_hash` must be provided. This can be
    /// obtained by compiling the Sierra class to obtain a CASM class, and then hashing it.
    ///
    /// The compilation of Sierra to CASM can either be done interactively via the
    /// `starknet-sierra-compile` command from the Cairo toolchain, or programmatically through the
    /// Cairo crates.
    ///
    /// Hashing the resulting CASM class is supported in the `starknet-core` crate. It can also be
    /// done interactively via Starkli with its `starkli class-hash` command.
    ///
    /// This method is only used for declaring Sierra (Cairo 1) classes. Declaring legacy (Cairo 0)
    /// classes is no longer supported.
    fn declare_v3(
        &self,
        contract_class: Arc<FlattenedSierraClass>,
        compiled_class_hash: Felt,
    ) -> DeclarationV3<'_, Self> {
        DeclarationV3::new(contract_class, compiled_class_hash, self)
    }

    /// Generates an instance of [`DeclarationV3`] for sending `DECLARE` v3 transactions. Pays
    /// transaction fees in `STRK`.
    ///
    /// To declare a Sierra (Cairo 1) class, a `compiled_class_hash` must be provided. This can be
    /// obtained by compiling the Sierra class to obtain a CASM class, and then hashing it.
    ///
    /// The compilation of Sierra to CASM can either be done interactively via the
    /// `starknet-sierra-compile` command from the Cairo toolchain, or programmatically through the
    /// Cairo crates.
    ///
    /// Hashing the resulting CASM class is supported in the `starknet-core` crate. It can also be
    /// done interactively via Starkli with its `starkli class-hash` command.
    ///
    /// This method is only used for declaring Sierra (Cairo 1) classes. Declaring legacy (Cairo 0)
    /// classes is no longer supported.
    #[deprecated = "transaction version used might change unexpectedly; use `declare_v3` instead"]
    fn declare(
        &self,
        contract_class: Arc<FlattenedSierraClass>,
        compiled_class_hash: Felt,
    ) -> DeclarationV3<'_, Self> {
        self.declare_v3(contract_class, compiled_class_hash)
    }
}

/// An abstraction over different ways to encode [`Vec<Call>`] into [`Vec<Felt>`].
///
/// Standard Cairo 0 and Cairo 1 account contracts encodes calls differently. Custom account
/// contract implementations might also impose arbitrary encoding rules.
#[auto_impl(&, Box, Arc)]
pub trait ExecutionEncoder {
    /// Encodes the list of [`Call`] into a list of [`Felt`] to be used as calldata to the account's
    /// `__execute__` entrypoint.
    fn encode_calls(&self, calls: &[Call]) -> Vec<Felt>;
}

/// An [`Account`] implementation that also comes with a [`Provider`]. Functionalities that require
/// a connection to the sequencer or node are offloaded to this trait to keep the base [`Account`]
/// clean and flexible.
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
pub trait ConnectedAccount: Account {
    /// The [`Provider`] type attached to this account.
    type Provider: Provider + Sync;

    /// Gets a reference to the attached [`Provider`] instance.
    fn provider(&self) -> &Self::Provider;

    /// Gets block ID to use when checking nonce and estimating fees.
    fn block_id(&self) -> BlockId {
        BlockId::Tag(BlockTag::Latest)
    }

    /// Gets the next available nonce to be used.
    async fn get_nonce(&self) -> Result<Felt, ProviderError> {
        self.provider()
            .get_nonce(self.block_id(), self.address())
            .await
    }
}

/// Abstraction over `INVOKE` transactions from accounts for invoking contracts. This struct uses
/// v3 `INVOKE` transactions under the hood, and hence pays transaction fees in STRK.
///
/// This is an intermediate type allowing users to optionally specify `nonce` and transaction fee
/// options.
#[must_use]
#[derive(Debug)]
pub struct ExecutionV3<'a, A> {
    account: &'a A,
    calls: Vec<Call>,
    nonce: Option<Felt>,
    l1_gas: Option<u64>,
    l1_gas_price: Option<u128>,
    l2_gas: Option<u64>,
    l2_gas_price: Option<u128>,
    l1_data_gas: Option<u64>,
    l1_data_gas_price: Option<u128>,
    gas_estimate_multiplier: f64,
    gas_price_estimate_multiplier: f64,
}

/// Abstraction over `DECLARE` transactions from accounts for invoking contracts. This struct uses
/// v3 `DECLARE` transactions under the hood, and hence pays transaction fees in STRK.
///
/// This is an intermediate type allowing users to optionally specify `nonce` and transaction fee
/// options.
#[must_use]
#[derive(Debug)]
pub struct DeclarationV3<'a, A> {
    account: &'a A,
    contract_class: Arc<FlattenedSierraClass>,
    compiled_class_hash: Felt,
    nonce: Option<Felt>,
    l1_gas: Option<u64>,
    l1_gas_price: Option<u128>,
    l2_gas: Option<u64>,
    l2_gas_price: Option<u128>,
    l1_data_gas: Option<u64>,
    l1_data_gas_price: Option<u128>,
    gas_estimate_multiplier: f64,
    gas_price_estimate_multiplier: f64,
}

/// [`ExecutionV3`] but with `nonce` and other transaction fee options already determined.
#[derive(Debug)]
pub struct RawExecutionV3 {
    calls: Vec<Call>,
    nonce: Felt,
    l1_gas: u64,
    l1_gas_price: u128,
    l2_gas: u64,
    l2_gas_price: u128,
    l1_data_gas: u64,
    l1_data_gas_price: u128,
}

/// [`DeclarationV3`] but with `nonce` and other transaction fee options already determined.
#[derive(Debug)]
pub struct RawDeclarationV3 {
    contract_class: Arc<FlattenedSierraClass>,
    compiled_class_hash: Felt,
    nonce: Felt,
    l1_gas: u64,
    l1_gas_price: u128,
    l2_gas: u64,
    l2_gas_price: u128,
    l1_data_gas: u64,
    l1_data_gas_price: u128,
}

/// [`RawExecutionV3`] but with an account associated.
#[derive(Debug)]
pub struct PreparedExecutionV3<'a, A> {
    account: &'a A,
    inner: RawExecutionV3,
}

/// [`RawDeclarationV3`] but with an account associated.
#[derive(Debug)]
pub struct PreparedDeclarationV3<'a, A> {
    account: &'a A,
    inner: RawDeclarationV3,
}

/// Errors using Starknet accounts.
#[derive(Debug, thiserror::Error)]
pub enum AccountError<S> {
    /// An error is encountered when signing a request.
    #[error(transparent)]
    Signing(S),
    /// An error is encountered with communicating with the network.
    #[error(transparent)]
    Provider(ProviderError),
    /// Unable to calculate the class hash for declaration.
    #[error(transparent)]
    ClassHashCalculation(ComputeClassHashError),
    /// Transaction fee calculation overflow.
    #[error("fee calculation overflow")]
    FeeOutOfRange,
}

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
impl<A> Account for &A
where
    A: Account + Sync,
{
    type SignError = A::SignError;

    fn address(&self) -> Felt {
        (*self).address()
    }

    fn chain_id(&self) -> Felt {
        (*self).chain_id()
    }

    async fn sign_execution_v3(
        &self,
        execution: &RawExecutionV3,
        query_only: bool,
    ) -> Result<Vec<Felt>, Self::SignError> {
        (*self).sign_execution_v3(execution, query_only).await
    }

    async fn sign_declaration_v3(
        &self,
        declaration: &RawDeclarationV3,
        query_only: bool,
    ) -> Result<Vec<Felt>, Self::SignError> {
        (*self).sign_declaration_v3(declaration, query_only).await
    }

    fn is_signer_interactive(&self, context: SignerInteractivityContext<'_>) -> bool {
        (*self).is_signer_interactive(context)
    }
}

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
impl<A> Account for Box<A>
where
    A: Account + Sync + Send,
{
    type SignError = A::SignError;

    fn address(&self) -> Felt {
        self.as_ref().address()
    }

    fn chain_id(&self) -> Felt {
        self.as_ref().chain_id()
    }

    async fn sign_execution_v3(
        &self,
        execution: &RawExecutionV3,
        query_only: bool,
    ) -> Result<Vec<Felt>, Self::SignError> {
        self.as_ref().sign_execution_v3(execution, query_only).await
    }

    async fn sign_declaration_v3(
        &self,
        declaration: &RawDeclarationV3,
        query_only: bool,
    ) -> Result<Vec<Felt>, Self::SignError> {
        self.as_ref()
            .sign_declaration_v3(declaration, query_only)
            .await
    }

    fn is_signer_interactive(&self, context: SignerInteractivityContext<'_>) -> bool {
        self.as_ref().is_signer_interactive(context)
    }
}

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
impl<A> Account for Arc<A>
where
    A: Account + Sync + Send,
{
    type SignError = A::SignError;

    fn address(&self) -> Felt {
        self.as_ref().address()
    }

    fn chain_id(&self) -> Felt {
        self.as_ref().chain_id()
    }

    async fn sign_execution_v3(
        &self,
        execution: &RawExecutionV3,
        query_only: bool,
    ) -> Result<Vec<Felt>, Self::SignError> {
        self.as_ref().sign_execution_v3(execution, query_only).await
    }

    async fn sign_declaration_v3(
        &self,
        declaration: &RawDeclarationV3,
        query_only: bool,
    ) -> Result<Vec<Felt>, Self::SignError> {
        self.as_ref()
            .sign_declaration_v3(declaration, query_only)
            .await
    }

    fn is_signer_interactive(&self, context: SignerInteractivityContext<'_>) -> bool {
        self.as_ref().is_signer_interactive(context)
    }
}

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
impl<A> ConnectedAccount for &A
where
    A: ConnectedAccount + Sync,
{
    type Provider = A::Provider;

    fn provider(&self) -> &Self::Provider {
        (*self).provider()
    }

    fn block_id(&self) -> BlockId {
        (*self).block_id()
    }

    async fn get_nonce(&self) -> Result<Felt, ProviderError> {
        (*self).get_nonce().await
    }
}

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
impl<A> ConnectedAccount for Box<A>
where
    A: ConnectedAccount + Sync + Send,
{
    type Provider = A::Provider;

    fn provider(&self) -> &Self::Provider {
        self.as_ref().provider()
    }

    fn block_id(&self) -> BlockId {
        self.as_ref().block_id()
    }

    async fn get_nonce(&self) -> Result<Felt, ProviderError> {
        self.as_ref().get_nonce().await
    }
}

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
impl<A> ConnectedAccount for Arc<A>
where
    A: ConnectedAccount + Sync + Send,
{
    type Provider = A::Provider;

    fn provider(&self) -> &Self::Provider {
        self.as_ref().provider()
    }

    fn block_id(&self) -> BlockId {
        self.as_ref().block_id()
    }

    async fn get_nonce(&self) -> Result<Felt, ProviderError> {
        self.as_ref().get_nonce().await
    }
}
