use crate::Call;

use async_trait::async_trait;
use auto_impl::auto_impl;
use starknet_core::types::{
    contract::{legacy::LegacyContractClass, CompressProgramError, ComputeClassHashError},
    BlockId, BlockTag, FieldElement, FlattenedSierraClass,
};
use starknet_providers::{Provider, ProviderError};
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
    type SignError: Error + Send + Sync;

    fn address(&self) -> FieldElement;

    fn chain_id(&self) -> FieldElement;

    async fn sign_execution_v1(
        &self,
        execution: &RawExecutionV1,
        query_only: bool,
    ) -> Result<Vec<FieldElement>, Self::SignError>;

    async fn sign_execution_v3(
        &self,
        execution: &RawExecutionV3,
        query_only: bool,
    ) -> Result<Vec<FieldElement>, Self::SignError>;

    async fn sign_declaration_v2(
        &self,
        declaration: &RawDeclarationV2,
        query_only: bool,
    ) -> Result<Vec<FieldElement>, Self::SignError>;

    async fn sign_declaration_v3(
        &self,
        declaration: &RawDeclarationV3,
        query_only: bool,
    ) -> Result<Vec<FieldElement>, Self::SignError>;

    async fn sign_legacy_declaration(
        &self,
        legacy_declaration: &RawLegacyDeclaration,
        query_only: bool,
    ) -> Result<Vec<FieldElement>, Self::SignError>;

    fn execute_v1(&self, calls: Vec<Call>) -> ExecutionV1<Self> {
        ExecutionV1::new(calls, self)
    }

    fn execute_v3(&self, calls: Vec<Call>) -> ExecutionV3<Self> {
        ExecutionV3::new(calls, self)
    }

    #[deprecated = "use version specific variants (`execute_v1` & `execute_v3`) instead"]
    fn execute(&self, calls: Vec<Call>) -> ExecutionV1<Self> {
        self.execute_v1(calls)
    }

    fn declare_v2(
        &self,
        contract_class: Arc<FlattenedSierraClass>,
        compiled_class_hash: FieldElement,
    ) -> DeclarationV2<Self> {
        DeclarationV2::new(contract_class, compiled_class_hash, self)
    }

    fn declare_v3(
        &self,
        contract_class: Arc<FlattenedSierraClass>,
        compiled_class_hash: FieldElement,
    ) -> DeclarationV3<Self> {
        DeclarationV3::new(contract_class, compiled_class_hash, self)
    }

    #[deprecated = "use version specific variants (`declare_v1` & `declare_v3`) instead"]
    fn declare(
        &self,
        contract_class: Arc<FlattenedSierraClass>,
        compiled_class_hash: FieldElement,
    ) -> DeclarationV2<Self> {
        self.declare_v2(contract_class, compiled_class_hash)
    }

    fn declare_legacy(&self, contract_class: Arc<LegacyContractClass>) -> LegacyDeclaration<Self> {
        LegacyDeclaration::new(contract_class, self)
    }
}

#[auto_impl(&, Box, Arc)]
pub trait ExecutionEncoder {
    fn encode_calls(&self, calls: &[Call]) -> Vec<FieldElement>;
}

/// An [Account] implementation that also comes with a [Provider]. Functionalities that require a
/// connection to the sequencer or node are offloaded to this trait to keep the base [Account]
/// clean and flexible.
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
pub trait ConnectedAccount: Account {
    type Provider: Provider + Sync;

    fn provider(&self) -> &Self::Provider;

    /// Block ID to use when checking nonce and estimating fees.
    fn block_id(&self) -> BlockId {
        BlockId::Tag(BlockTag::Latest)
    }

    async fn get_nonce(&self) -> Result<FieldElement, ProviderError> {
        self.provider()
            .get_nonce(self.block_id(), self.address())
            .await
    }
}

/// Abstraction over `INVOKE` transactions from accounts for invoking contracts. This struct uses
/// v1 `INVOKE` transactions under the hood, and hence pays transaction fees in ETH. To use v3
/// transactions for STRK fee payment, use [ExecutionV3] instead.
///
/// This is an intermediate type allowing users to optionally specify `nonce` and/or `max_fee`.
#[must_use]
#[derive(Debug)]
pub struct ExecutionV1<'a, A> {
    account: &'a A,
    calls: Vec<Call>,
    nonce: Option<FieldElement>,
    max_fee: Option<FieldElement>,
    fee_estimate_multiplier: f64,
}

/// Abstraction over `INVOKE` transactions from accounts for invoking contracts. This struct uses
/// v3 `INVOKE` transactions under the hood, and hence pays transaction fees in STRK. To use v1
/// transactions for ETH fee payment, use [ExecutionV1] instead.
///
/// This is an intermediate type allowing users to optionally specify `nonce`, `gas`, and/or
/// `gas_price`.
#[must_use]
#[derive(Debug)]
pub struct ExecutionV3<'a, A> {
    account: &'a A,
    calls: Vec<Call>,
    nonce: Option<FieldElement>,
    gas: Option<u64>,
    gas_price: Option<u128>,
    gas_estimate_multiplier: f64,
    gas_price_estimate_multiplier: f64,
}

/// Abstraction over `DECLARE` transactions from accounts for invoking contracts. This struct uses
/// v2 `DECLARE` transactions under the hood, and hence pays transaction fees in ETH. To use v3
/// transactions for STRK fee payment, use [DeclarationV3] instead.
///
/// An intermediate type allowing users to optionally specify `nonce` and/or `max_fee`.
#[must_use]
#[derive(Debug)]
pub struct DeclarationV2<'a, A> {
    account: &'a A,
    contract_class: Arc<FlattenedSierraClass>,
    compiled_class_hash: FieldElement,
    nonce: Option<FieldElement>,
    max_fee: Option<FieldElement>,
    fee_estimate_multiplier: f64,
}

/// Abstraction over `DECLARE` transactions from accounts for invoking contracts. This struct uses
/// v3 `DECLARE` transactions under the hood, and hence pays transaction fees in STRK. To use v2
/// transactions for ETH fee payment, use [DeclarationV2] instead.
///
/// This is an intermediate type allowing users to optionally specify `nonce`, `gas`, and/or
/// `gas_price`.
#[must_use]
#[derive(Debug)]
pub struct DeclarationV3<'a, A> {
    account: &'a A,
    contract_class: Arc<FlattenedSierraClass>,
    compiled_class_hash: FieldElement,
    nonce: Option<FieldElement>,
    gas: Option<u64>,
    gas_price: Option<u128>,
    gas_estimate_multiplier: f64,
    gas_price_estimate_multiplier: f64,
}

/// An intermediate type allowing users to optionally specify `nonce` and/or `max_fee`.
#[must_use]
#[derive(Debug)]
pub struct LegacyDeclaration<'a, A> {
    account: &'a A,
    contract_class: Arc<LegacyContractClass>,
    nonce: Option<FieldElement>,
    max_fee: Option<FieldElement>,
    fee_estimate_multiplier: f64,
}

/// [ExecutionV1] but with `nonce` and `max_fee` already determined.
#[derive(Debug)]
pub struct RawExecutionV1 {
    calls: Vec<Call>,
    nonce: FieldElement,
    max_fee: FieldElement,
}

/// [ExecutionV3] but with `nonce`, `gas` and `gas_price` already determined.
#[derive(Debug)]
pub struct RawExecutionV3 {
    calls: Vec<Call>,
    nonce: FieldElement,
    gas: u64,
    gas_price: u128,
}

/// [DeclarationV2] but with `nonce` and `max_fee` already determined.
#[derive(Debug)]
pub struct RawDeclarationV2 {
    contract_class: Arc<FlattenedSierraClass>,
    compiled_class_hash: FieldElement,
    nonce: FieldElement,
    max_fee: FieldElement,
}

/// [DeclarationV3] but with `nonce`, `gas` and `gas_price` already determined.
#[derive(Debug)]
pub struct RawDeclarationV3 {
    contract_class: Arc<FlattenedSierraClass>,
    compiled_class_hash: FieldElement,
    nonce: FieldElement,
    gas: u64,
    gas_price: u128,
}

/// [LegacyDeclaration] but with `nonce` and `max_fee` already determined.
#[derive(Debug)]
pub struct RawLegacyDeclaration {
    contract_class: Arc<LegacyContractClass>,
    nonce: FieldElement,
    max_fee: FieldElement,
}

/// [RawExecutionV1] but with an account associated.
#[derive(Debug)]
pub struct PreparedExecutionV1<'a, A> {
    account: &'a A,
    inner: RawExecutionV1,
}

/// [RawExecutionV3] but with an account associated.
#[derive(Debug)]
pub struct PreparedExecutionV3<'a, A> {
    account: &'a A,
    inner: RawExecutionV3,
}

/// [RawDeclarationV2] but with an account associated.
#[derive(Debug)]
pub struct PreparedDeclarationV2<'a, A> {
    account: &'a A,
    inner: RawDeclarationV2,
}

/// [RawDeclarationV3] but with an account associated.
#[derive(Debug)]
pub struct PreparedDeclarationV3<'a, A> {
    account: &'a A,
    inner: RawDeclarationV3,
}

/// [RawLegacyDeclaration] but with an account associated.
#[derive(Debug)]
pub struct PreparedLegacyDeclaration<'a, A> {
    account: &'a A,
    inner: RawLegacyDeclaration,
}

#[derive(Debug, thiserror::Error)]
pub enum AccountError<S> {
    #[error(transparent)]
    Signing(S),
    #[error(transparent)]
    Provider(ProviderError),
    #[error(transparent)]
    ClassHashCalculation(ComputeClassHashError),
    #[error(transparent)]
    ClassCompression(CompressProgramError),
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

    fn address(&self) -> FieldElement {
        (*self).address()
    }

    fn chain_id(&self) -> FieldElement {
        (*self).chain_id()
    }

    async fn sign_execution_v1(
        &self,
        execution: &RawExecutionV1,
        query_only: bool,
    ) -> Result<Vec<FieldElement>, Self::SignError> {
        (*self).sign_execution_v1(execution, query_only).await
    }

    async fn sign_execution_v3(
        &self,
        execution: &RawExecutionV3,
        query_only: bool,
    ) -> Result<Vec<FieldElement>, Self::SignError> {
        (*self).sign_execution_v3(execution, query_only).await
    }

    async fn sign_declaration_v2(
        &self,
        declaration: &RawDeclarationV2,
        query_only: bool,
    ) -> Result<Vec<FieldElement>, Self::SignError> {
        (*self).sign_declaration_v2(declaration, query_only).await
    }

    async fn sign_declaration_v3(
        &self,
        declaration: &RawDeclarationV3,
        query_only: bool,
    ) -> Result<Vec<FieldElement>, Self::SignError> {
        (*self).sign_declaration_v3(declaration, query_only).await
    }

    async fn sign_legacy_declaration(
        &self,
        legacy_declaration: &RawLegacyDeclaration,
        query_only: bool,
    ) -> Result<Vec<FieldElement>, Self::SignError> {
        (*self)
            .sign_legacy_declaration(legacy_declaration, query_only)
            .await
    }
}

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
impl<A> Account for Box<A>
where
    A: Account + Sync + Send,
{
    type SignError = A::SignError;

    fn address(&self) -> FieldElement {
        self.as_ref().address()
    }

    fn chain_id(&self) -> FieldElement {
        self.as_ref().chain_id()
    }

    async fn sign_execution_v1(
        &self,
        execution: &RawExecutionV1,
        query_only: bool,
    ) -> Result<Vec<FieldElement>, Self::SignError> {
        self.as_ref().sign_execution_v1(execution, query_only).await
    }

    async fn sign_execution_v3(
        &self,
        execution: &RawExecutionV3,
        query_only: bool,
    ) -> Result<Vec<FieldElement>, Self::SignError> {
        self.as_ref().sign_execution_v3(execution, query_only).await
    }

    async fn sign_declaration_v2(
        &self,
        declaration: &RawDeclarationV2,
        query_only: bool,
    ) -> Result<Vec<FieldElement>, Self::SignError> {
        self.as_ref()
            .sign_declaration_v2(declaration, query_only)
            .await
    }

    async fn sign_declaration_v3(
        &self,
        declaration: &RawDeclarationV3,
        query_only: bool,
    ) -> Result<Vec<FieldElement>, Self::SignError> {
        self.as_ref()
            .sign_declaration_v3(declaration, query_only)
            .await
    }

    async fn sign_legacy_declaration(
        &self,
        legacy_declaration: &RawLegacyDeclaration,
        query_only: bool,
    ) -> Result<Vec<FieldElement>, Self::SignError> {
        self.as_ref()
            .sign_legacy_declaration(legacy_declaration, query_only)
            .await
    }
}

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
impl<A> Account for Arc<A>
where
    A: Account + Sync + Send,
{
    type SignError = A::SignError;

    fn address(&self) -> FieldElement {
        self.as_ref().address()
    }

    fn chain_id(&self) -> FieldElement {
        self.as_ref().chain_id()
    }

    async fn sign_execution_v1(
        &self,
        execution: &RawExecutionV1,
        query_only: bool,
    ) -> Result<Vec<FieldElement>, Self::SignError> {
        self.as_ref().sign_execution_v1(execution, query_only).await
    }

    async fn sign_execution_v3(
        &self,
        execution: &RawExecutionV3,
        query_only: bool,
    ) -> Result<Vec<FieldElement>, Self::SignError> {
        self.as_ref().sign_execution_v3(execution, query_only).await
    }

    async fn sign_declaration_v2(
        &self,
        declaration: &RawDeclarationV2,
        query_only: bool,
    ) -> Result<Vec<FieldElement>, Self::SignError> {
        self.as_ref()
            .sign_declaration_v2(declaration, query_only)
            .await
    }

    async fn sign_declaration_v3(
        &self,
        declaration: &RawDeclarationV3,
        query_only: bool,
    ) -> Result<Vec<FieldElement>, Self::SignError> {
        self.as_ref()
            .sign_declaration_v3(declaration, query_only)
            .await
    }

    async fn sign_legacy_declaration(
        &self,
        legacy_declaration: &RawLegacyDeclaration,
        query_only: bool,
    ) -> Result<Vec<FieldElement>, Self::SignError> {
        self.as_ref()
            .sign_legacy_declaration(legacy_declaration, query_only)
            .await
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

    async fn get_nonce(&self) -> Result<FieldElement, ProviderError> {
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

    async fn get_nonce(&self) -> Result<FieldElement, ProviderError> {
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

    async fn get_nonce(&self) -> Result<FieldElement, ProviderError> {
        self.as_ref().get_nonce().await
    }
}
