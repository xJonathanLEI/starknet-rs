use super::NotPreparedError;

use async_trait::async_trait;
use starknet_core::{
    crypto::compute_hash_on_elements,
    types::{
        AccountTransaction, AddTransactionResult, BlockId, DeployAccountTransactionRequest,
        FeeEstimate, FieldElement, StarknetError, TransactionRequest,
    },
};
use starknet_providers::{Provider, ProviderError};
use std::error::Error;

pub mod argent;
pub mod open_zeppelin;

/// Cairo string for "deploy_account"
const PREFIX_DEPLOY_ACCOUNT: FieldElement = FieldElement::from_mont([
    3350261884043292318,
    18443211694809419988,
    18446744073709551615,
    461298303000467581,
]);

/// Cairo string for "STARKNET_CONTRACT_ADDRESS"
const PREFIX_CONTRACT_ADDRESS: FieldElement = FieldElement::from_mont([
    3829237882463328880,
    17289941567720117366,
    8635008616843941496,
    533439743893157637,
]);

// 2 ** 251 - 256
const ADDR_BOUND: FieldElement = FieldElement::from_mont([
    18446743986131443745,
    160989183,
    18446744073709255680,
    576459263475590224,
]);

/// This trait enables deploying account contracts using the `DeployAccount` transaction type.
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
pub trait AccountFactory: Sized {
    type Provider: Provider;
    type SignError: Error + Send + Sync;

    fn class_hash(&self) -> FieldElement;

    fn calldata(&self) -> Vec<FieldElement>;

    fn chain_id(&self) -> FieldElement;

    fn provider(&self) -> &Self::Provider;

    /// Block ID to use when estimating fees.
    fn block_id(&self) -> BlockId {
        BlockId::Latest
    }

    async fn sign_deployment(
        &self,
        deployment: &RawAccountDeployment,
    ) -> Result<Vec<FieldElement>, Self::SignError>;

    fn deploy(&self, salt: FieldElement) -> AccountDeployment<Self> {
        AccountDeployment::new(salt, self)
    }
}

/// An intermediate type allowing users to optionally specify `nonce` and/or `max_fee`.
#[must_use]
#[derive(Debug)]
pub struct AccountDeployment<'f, F> {
    factory: &'f F,
    salt: FieldElement,
    // We need to allow setting nonce here as `DeployAccount` transactions may have non-zero nonces
    /// after failed transactions can be included in blocks.
    nonce: Option<FieldElement>,
    max_fee: Option<FieldElement>,
    fee_estimate_multiplier: f64,
}

/// [AccountDeployment] but with `nonce` and `max_fee` already determined.
#[derive(Debug, Clone)]
pub struct RawAccountDeployment {
    salt: FieldElement,
    nonce: FieldElement,
    max_fee: FieldElement,
}

/// [RawAccountDeployment] but with a factory associated.
#[derive(Debug)]
pub struct PreparedAccountDeployment<'f, F> {
    factory: &'f F,
    inner: RawAccountDeployment,
}

#[derive(Debug, thiserror::Error)]
pub enum AccountFactoryError<S, P> {
    #[error(transparent)]
    Signing(S),
    #[error(transparent)]
    Provider(ProviderError<P>),
}

impl<'f, F> AccountDeployment<'f, F> {
    pub fn new(salt: FieldElement, factory: &'f F) -> Self {
        Self {
            factory,
            salt,
            nonce: None,
            max_fee: None,
            fee_estimate_multiplier: 1.1,
        }
    }

    pub fn nonce(self, nonce: FieldElement) -> Self {
        Self {
            nonce: Some(nonce),
            ..self
        }
    }

    pub fn max_fee(self, max_fee: FieldElement) -> Self {
        Self {
            max_fee: Some(max_fee),
            ..self
        }
    }

    pub fn fee_estimate_multiplier(self, fee_estimate_multiplier: f64) -> Self {
        Self {
            fee_estimate_multiplier,
            ..self
        }
    }

    /// Calling this function after manually specifying `nonce` and `max_fee` turns
    /// [AccountDeployment] into [PreparedAccountDeployment]. Returns `Err` if either field is
    /// `None`.
    pub fn prepared(self) -> Result<PreparedAccountDeployment<'f, F>, NotPreparedError> {
        let nonce = self.nonce.ok_or(NotPreparedError)?;
        let max_fee = self.max_fee.ok_or(NotPreparedError)?;

        Ok(PreparedAccountDeployment {
            factory: self.factory,
            inner: RawAccountDeployment {
                salt: self.salt,
                nonce,
                max_fee,
            },
        })
    }
}

impl<'f, F> AccountDeployment<'f, F>
where
    F: AccountFactory + Sync,
{
    /// Locally calculates the target deployment address.
    pub fn address(&self) -> FieldElement {
        calculate_contract_address(
            self.salt,
            self.factory.class_hash(),
            &self.factory.calldata(),
        )
    }

    pub async fn fetch_nonce(
        &self,
    ) -> Result<FieldElement, ProviderError<<F::Provider as Provider>::Error>> {
        match self
            .factory
            .provider()
            .get_nonce(self.address(), self.factory.block_id())
            .await
        {
            Ok(nonce) => Ok(nonce),
            Err(ProviderError::StarknetError(StarknetError::ContractNotFound)) => {
                Ok(FieldElement::ZERO)
            }
            Err(err) => Err(err),
        }
    }

    pub async fn estimate_fee(
        &self,
    ) -> Result<FeeEstimate, AccountFactoryError<F::SignError, <F::Provider as Provider>::Error>>
    {
        // Resolves nonce
        let nonce = match self.nonce {
            Some(value) => value,
            None => self
                .fetch_nonce()
                .await
                .map_err(AccountFactoryError::Provider)?,
        };

        self.estimate_fee_with_nonce(nonce).await
    }

    pub async fn send(
        &self,
    ) -> Result<
        AddTransactionResult,
        AccountFactoryError<F::SignError, <F::Provider as Provider>::Error>,
    > {
        self.prepare().await?.send().await
    }

    async fn prepare(
        &self,
    ) -> Result<
        PreparedAccountDeployment<'f, F>,
        AccountFactoryError<F::SignError, <F::Provider as Provider>::Error>,
    > {
        // Resolves nonce
        let nonce = match self.nonce {
            Some(value) => value,
            None => self
                .fetch_nonce()
                .await
                .map_err(AccountFactoryError::Provider)?,
        };

        // Resolves max_fee
        let max_fee = match self.max_fee {
            Some(value) => value,
            None => {
                let fee_estimate = self.estimate_fee_with_nonce(nonce).await?;
                ((fee_estimate.overall_fee as f64 * self.fee_estimate_multiplier) as u64).into()
            }
        };

        Ok(PreparedAccountDeployment {
            factory: self.factory,
            inner: RawAccountDeployment {
                salt: self.salt,
                nonce,
                max_fee,
            },
        })
    }

    async fn estimate_fee_with_nonce(
        &self,
        nonce: FieldElement,
    ) -> Result<FeeEstimate, AccountFactoryError<F::SignError, <F::Provider as Provider>::Error>>
    {
        let prepared = PreparedAccountDeployment {
            factory: self.factory,
            inner: RawAccountDeployment {
                salt: self.salt,
                nonce,
                max_fee: FieldElement::ZERO,
            },
        };
        let deploy = prepared
            .get_deploy_request()
            .await
            .map_err(AccountFactoryError::Signing)?;

        self.factory
            .provider()
            .estimate_fee(
                AccountTransaction::DeployAccount(deploy),
                self.factory.block_id(),
                false,
            )
            .await
            .map_err(AccountFactoryError::Provider)
    }
}

impl<'f, F> PreparedAccountDeployment<'f, F> {
    pub fn from_raw(raw_deployment: RawAccountDeployment, factory: &'f F) -> Self {
        Self {
            factory,
            inner: raw_deployment,
        }
    }
}

impl<'f, F> PreparedAccountDeployment<'f, F>
where
    F: AccountFactory,
{
    /// Locally calculates the target deployment address.
    pub fn address(&self) -> FieldElement {
        calculate_contract_address(
            self.inner.salt,
            self.factory.class_hash(),
            &self.factory.calldata(),
        )
    }

    pub fn transaction_hash(&self) -> FieldElement {
        let mut calldata_to_hash = vec![self.factory.class_hash(), self.inner.salt];
        calldata_to_hash.append(&mut self.factory.calldata());

        compute_hash_on_elements(&[
            PREFIX_DEPLOY_ACCOUNT,
            FieldElement::ONE, // version
            self.address(),
            FieldElement::ZERO, // entry_point_selector
            compute_hash_on_elements(&calldata_to_hash),
            self.inner.max_fee,
            self.factory.chain_id(),
            self.inner.nonce,
        ])
    }

    pub async fn send(
        &self,
    ) -> Result<
        AddTransactionResult,
        AccountFactoryError<F::SignError, <F::Provider as Provider>::Error>,
    > {
        let tx_request = self
            .get_deploy_request()
            .await
            .map_err(AccountFactoryError::Signing)?;
        self.factory
            .provider()
            .add_transaction(TransactionRequest::DeployAccount(tx_request))
            .await
            .map_err(AccountFactoryError::Provider)
    }

    async fn get_deploy_request(&self) -> Result<DeployAccountTransactionRequest, F::SignError> {
        let signature = self.factory.sign_deployment(&self.inner).await?;

        Ok(DeployAccountTransactionRequest {
            class_hash: self.factory.class_hash(),
            contract_address_salt: self.inner.salt,
            constructor_calldata: self.factory.calldata(),
            max_fee: self.inner.max_fee,
            signature,
            nonce: self.inner.nonce,
        })
    }
}

fn calculate_contract_address(
    salt: FieldElement,
    class_hash: FieldElement,
    constructor_calldata: &[FieldElement],
) -> FieldElement {
    compute_hash_on_elements(&[
        PREFIX_CONTRACT_ADDRESS,
        FieldElement::ZERO,
        salt,
        class_hash,
        compute_hash_on_elements(constructor_calldata),
    ]) % ADDR_BOUND
}
