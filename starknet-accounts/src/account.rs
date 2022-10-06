use crate::Call;

use async_trait::async_trait;
use starknet_core::types::{
    AddTransactionResult, BlockId, ContractDefinition, FeeEstimate, FieldElement,
};
use std::{error::Error, sync::Arc};

#[derive(Debug)]
pub struct AttachedAccountCall<'a, A> {
    pub calls: Vec<Call>,
    pub nonce: Option<FieldElement>,
    pub max_fee: Option<FieldElement>,
    pub fee_estimate_multiplier: f32,
    pub(crate) account: &'a A,
}

#[derive(Debug)]
pub struct AttachedAccountDeclaration<'a, A> {
    pub compressed_class: Arc<ContractDefinition>,
    pub class_hash: FieldElement,
    pub nonce: Option<FieldElement>,
    pub max_fee: Option<FieldElement>,
    pub fee_estimate_multiplier: f32,
    pub(crate) account: &'a A,
}

pub trait AccountCall {
    fn get_calls(&self) -> &[Call];

    fn get_nonce(&self) -> &Option<FieldElement>;

    fn nonce(self, nonce: FieldElement) -> Self;

    fn get_max_fee(&self) -> &Option<FieldElement>;

    fn max_fee(self, max_fee: FieldElement) -> Self;

    fn get_fee_estimate_multiplier(&self) -> f32;

    fn fee_estimate_multiplier(self, fee_estimate_multiplier: f32) -> Self;
}

pub trait AccountDeclaration {
    fn get_compressed_class(&self) -> Arc<ContractDefinition>;

    fn get_class_hash(&self) -> FieldElement;

    fn get_nonce(&self) -> &Option<FieldElement>;

    fn nonce(self, nonce: FieldElement) -> Self;

    fn get_max_fee(&self) -> &Option<FieldElement>;

    fn max_fee(self, max_fee: FieldElement) -> Self;

    fn get_fee_estimate_multiplier(&self) -> f32;

    fn fee_estimate_multiplier(self, fee_estimate_multiplier: f32) -> Self;
}

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
pub trait Account: Sized {
    type GetNonceError: Error + Send;
    type EstimateFeeError: Error + Send;
    type SendTransactionError: Error + Send;

    fn address(&self) -> FieldElement;

    async fn get_nonce(
        &self,
        block_identifier: BlockId,
    ) -> Result<FieldElement, Self::GetNonceError>;

    fn execute(&self, calls: &[Call]) -> AttachedAccountCall<Self>;

    fn declare(
        &self,
        compressed_class: Arc<ContractDefinition>,
        class_hash: FieldElement,
    ) -> AttachedAccountDeclaration<Self>;

    async fn estimate_fee<C>(&self, call: &C) -> Result<FeeEstimate, Self::EstimateFeeError>
    where
        C: AccountCall + Sync;

    async fn estimate_declare_fee<D>(
        &self,
        declaration: &D,
    ) -> Result<FeeEstimate, Self::EstimateFeeError>
    where
        D: AccountDeclaration + Sync;

    async fn send_transaction<C>(
        &self,
        call: &C,
    ) -> Result<AddTransactionResult, Self::SendTransactionError>
    where
        C: AccountCall + Sync;

    async fn send_declare_transaction<D>(
        &self,
        declaration: &D,
    ) -> Result<AddTransactionResult, Self::SendTransactionError>
    where
        D: AccountDeclaration + Sync;
}

impl<'a, A> AttachedAccountCall<'a, A>
where
    A: Account + Sync,
{
    pub async fn estimate_fee(&self) -> Result<FeeEstimate, A::EstimateFeeError> {
        self.account.estimate_fee(self).await
    }

    pub async fn send(&self) -> Result<AddTransactionResult, A::SendTransactionError> {
        self.account.send_transaction(self).await
    }
}

impl<'a, A> AccountCall for AttachedAccountCall<'a, A> {
    fn get_calls(&self) -> &[Call] {
        &self.calls
    }

    fn get_nonce(&self) -> &Option<FieldElement> {
        &self.nonce
    }

    fn nonce(self, nonce: FieldElement) -> Self {
        Self {
            calls: self.calls,
            nonce: Some(nonce),
            max_fee: self.max_fee,
            fee_estimate_multiplier: self.fee_estimate_multiplier,
            account: self.account,
        }
    }

    fn get_max_fee(&self) -> &Option<FieldElement> {
        &self.max_fee
    }

    fn max_fee(self, max_fee: FieldElement) -> Self {
        Self {
            calls: self.calls,
            nonce: self.nonce,
            max_fee: Some(max_fee),
            fee_estimate_multiplier: self.fee_estimate_multiplier,
            account: self.account,
        }
    }

    fn get_fee_estimate_multiplier(&self) -> f32 {
        self.fee_estimate_multiplier
    }

    fn fee_estimate_multiplier(self, fee_estimate_multiplier: f32) -> Self {
        Self {
            calls: self.calls,
            nonce: self.nonce,
            max_fee: self.max_fee,
            fee_estimate_multiplier,
            account: self.account,
        }
    }
}

impl<'a, A> AttachedAccountDeclaration<'a, A>
where
    A: Account + Sync,
{
    pub async fn estimate_fee(&self) -> Result<FeeEstimate, A::EstimateFeeError> {
        self.account.estimate_declare_fee(self).await
    }

    pub async fn send(&self) -> Result<AddTransactionResult, A::SendTransactionError> {
        self.account.send_declare_transaction(self).await
    }
}

impl<'a, A> AccountDeclaration for AttachedAccountDeclaration<'a, A> {
    fn get_compressed_class(&self) -> Arc<ContractDefinition> {
        self.compressed_class.clone()
    }

    fn get_class_hash(&self) -> FieldElement {
        self.class_hash
    }

    fn get_nonce(&self) -> &Option<FieldElement> {
        &self.nonce
    }

    fn nonce(self, nonce: FieldElement) -> Self {
        Self {
            compressed_class: self.compressed_class,
            class_hash: self.class_hash,
            nonce: Some(nonce),
            max_fee: self.max_fee,
            fee_estimate_multiplier: self.fee_estimate_multiplier,
            account: self.account,
        }
    }

    fn get_max_fee(&self) -> &Option<FieldElement> {
        &self.max_fee
    }

    fn max_fee(self, max_fee: FieldElement) -> Self {
        Self {
            compressed_class: self.compressed_class,
            class_hash: self.class_hash,
            nonce: self.nonce,
            max_fee: Some(max_fee),
            fee_estimate_multiplier: self.fee_estimate_multiplier,
            account: self.account,
        }
    }

    fn get_fee_estimate_multiplier(&self) -> f32 {
        self.fee_estimate_multiplier
    }

    fn fee_estimate_multiplier(self, fee_estimate_multiplier: f32) -> Self {
        Self {
            compressed_class: self.compressed_class,
            class_hash: self.class_hash,
            nonce: self.nonce,
            max_fee: self.max_fee,
            fee_estimate_multiplier,
            account: self.account,
        }
    }
}
