use super::NotPreparedError;

use async_trait::async_trait;
use starknet_core::{
    crypto::compute_hash_on_elements,
    types::{
        BlockId, BlockTag, BroadcastedDeployAccountTransaction,
        BroadcastedDeployAccountTransactionV1, BroadcastedTransaction,
        DeployAccountTransactionResult, FeeEstimate, SimulatedTransaction, SimulationFlag,
        StarknetError,
    },
};
use starknet_providers::{Provider, ProviderError};
use starknet_types_core::felt::Felt;
use starknet_types_core::felt::NonZeroFelt;
use std::error::Error;

pub mod argent;
pub mod open_zeppelin;

/// Cairo string for "deploy_account"
const PREFIX_DEPLOY_ACCOUNT: Felt = Felt::from_raw([
    461298303000467581,
    18446744073709551615,
    18443211694809419988,
    3350261884043292318,
]);

/// Cairo string for "STARKNET_CONTRACT_ADDRESS"
const PREFIX_CONTRACT_ADDRESS: Felt = Felt::from_raw([
    533439743893157637,
    8635008616843941496,
    17289941567720117366,
    3829237882463328880,
]);

// 2 ** 251 - 256
const ADDR_BOUND: NonZeroFelt = NonZeroFelt::from_raw([
    576459263475590224,
    18446744073709255680,
    160989183,
    18446743986131443745,
]);

/// This trait enables deploying account contracts using the `DeployAccount` transaction type.
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
pub trait AccountFactory: Sized {
    type Provider: Provider + Sync;
    type SignError: Error + Send + Sync;

    fn class_hash(&self) -> Felt;

    fn calldata(&self) -> Vec<Felt>;

    fn chain_id(&self) -> Felt;

    fn provider(&self) -> &Self::Provider;

    /// Block ID to use when estimating fees.
    fn block_id(&self) -> BlockId {
        BlockId::Tag(BlockTag::Latest)
    }

    async fn sign_deployment(
        &self,
        deployment: &RawAccountDeployment,
    ) -> Result<Vec<Felt>, Self::SignError>;

    fn deploy(&self, salt: Felt) -> AccountDeployment<Self> {
        AccountDeployment::new(salt, self)
    }
}

/// An intermediate type allowing users to optionally specify `nonce` and/or `max_fee`.
#[must_use]
#[derive(Debug)]
pub struct AccountDeployment<'f, F> {
    factory: &'f F,
    salt: Felt,
    // We need to allow setting nonce here as `DeployAccount` transactions may have non-zero nonces
    /// after failed transactions can be included in blocks.
    nonce: Option<Felt>,
    max_fee: Option<Felt>,
    fee_estimate_multiplier: f64,
}

/// [AccountDeployment] but with `nonce` and `max_fee` already determined.
#[derive(Debug, Clone)]
pub struct RawAccountDeployment {
    salt: Felt,
    nonce: Felt,
    max_fee: Felt,
}

/// [RawAccountDeployment] but with a factory associated.
#[derive(Debug)]
pub struct PreparedAccountDeployment<'f, F> {
    factory: &'f F,
    inner: RawAccountDeployment,
}

#[derive(Debug, thiserror::Error)]
pub enum AccountFactoryError<S> {
    #[error(transparent)]
    Signing(S),
    #[error(transparent)]
    Provider(ProviderError),
    #[error("fee calculation overflow")]
    FeeOutOfRange,
}

impl<'f, F> AccountDeployment<'f, F> {
    pub fn new(salt: Felt, factory: &'f F) -> Self {
        Self {
            factory,
            salt,
            nonce: None,
            max_fee: None,
            fee_estimate_multiplier: 1.1,
        }
    }

    pub fn nonce(self, nonce: Felt) -> Self {
        Self {
            nonce: Some(nonce),
            ..self
        }
    }

    pub fn max_fee(self, max_fee: Felt) -> Self {
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
    pub fn address(&self) -> Felt {
        calculate_contract_address(
            self.salt,
            self.factory.class_hash(),
            &self.factory.calldata(),
        )
    }

    pub async fn fetch_nonce(&self) -> Result<Felt, ProviderError> {
        match self
            .factory
            .provider()
            .get_nonce(self.factory.block_id(), self.address())
            .await
        {
            Ok(nonce) => Ok(nonce),
            Err(ProviderError::StarknetError(StarknetError::ContractNotFound)) => Ok(Felt::ZERO),
            Err(err) => Err(err),
        }
    }

    pub async fn estimate_fee(&self) -> Result<FeeEstimate, AccountFactoryError<F::SignError>> {
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

    pub async fn simulate(
        &self,
        skip_validate: bool,
        skip_fee_charge: bool,
    ) -> Result<SimulatedTransaction, AccountFactoryError<F::SignError>> {
        // Resolves nonce
        let nonce = match self.nonce {
            Some(value) => value,
            None => self
                .fetch_nonce()
                .await
                .map_err(AccountFactoryError::Provider)?,
        };

        self.simulate_with_nonce(nonce, skip_validate, skip_fee_charge)
            .await
    }

    pub async fn send(
        &self,
    ) -> Result<DeployAccountTransactionResult, AccountFactoryError<F::SignError>> {
        self.prepare().await?.send().await
    }

    async fn prepare(
        &self,
    ) -> Result<PreparedAccountDeployment<'f, F>, AccountFactoryError<F::SignError>> {
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
                // TODO: remove this when a proper u64 conversion is implemented for `Felt`
                // Obtain the fee estimate
                let fee_estimate = self.estimate_fee_with_nonce(nonce).await?;
                // Convert the overall fee to little-endian bytes
                let overall_fee_bytes = fee_estimate.overall_fee.to_bytes_le();

                // Check if the remaining bytes after the first 8 are all zeros
                if overall_fee_bytes.iter().skip(8).any(|&x| x != 0) {
                    return Err(AccountFactoryError::FeeOutOfRange);
                }

                // Convert the first 8 bytes to u64
                let overall_fee_u64 =
                    u64::from_le_bytes(overall_fee_bytes[..8].try_into().unwrap());

                // Perform necessary operations on overall_fee_u64 and convert to f64 then to u64
                (((overall_fee_u64 as f64) * self.fee_estimate_multiplier) as u64).into()
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
        nonce: Felt,
    ) -> Result<FeeEstimate, AccountFactoryError<F::SignError>> {
        let prepared = PreparedAccountDeployment {
            factory: self.factory,
            inner: RawAccountDeployment {
                salt: self.salt,
                nonce,
                max_fee: Felt::ZERO,
            },
        };
        let deploy = prepared
            .get_deploy_request()
            .await
            .map_err(AccountFactoryError::Signing)?;

        self.factory
            .provider()
            .estimate_fee_single(
                BroadcastedTransaction::DeployAccount(deploy),
                [],
                self.factory.block_id(),
            )
            .await
            .map_err(AccountFactoryError::Provider)
    }

    async fn simulate_with_nonce(
        &self,
        nonce: Felt,
        skip_validate: bool,
        skip_fee_charge: bool,
    ) -> Result<SimulatedTransaction, AccountFactoryError<F::SignError>> {
        let prepared = PreparedAccountDeployment {
            factory: self.factory,
            inner: RawAccountDeployment {
                salt: self.salt,
                nonce,
                max_fee: self.max_fee.unwrap_or_default(),
            },
        };
        let deploy = prepared
            .get_deploy_request()
            .await
            .map_err(AccountFactoryError::Signing)?;

        let mut flags = vec![];

        if skip_validate {
            flags.push(SimulationFlag::SkipValidate);
        }
        if skip_fee_charge {
            flags.push(SimulationFlag::SkipFeeCharge);
        }

        self.factory
            .provider()
            .simulate_transaction(
                self.factory.block_id(),
                BroadcastedTransaction::DeployAccount(deploy),
                &flags,
            )
            .await
            .map_err(AccountFactoryError::Provider)
    }
}

impl RawAccountDeployment {
    pub fn salt(&self) -> Felt {
        self.salt
    }

    pub fn nonce(&self) -> Felt {
        self.nonce
    }

    pub fn max_fee(&self) -> Felt {
        self.max_fee
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
    pub fn address(&self) -> Felt {
        calculate_contract_address(
            self.inner.salt,
            self.factory.class_hash(),
            &self.factory.calldata(),
        )
    }

    pub fn transaction_hash(&self) -> Felt {
        let mut calldata_to_hash = vec![self.factory.class_hash(), self.inner.salt];
        calldata_to_hash.append(&mut self.factory.calldata());

        compute_hash_on_elements(&[
            PREFIX_DEPLOY_ACCOUNT,
            Felt::ONE, // version
            self.address(),
            Felt::ZERO, // entry_point_selector
            compute_hash_on_elements(&calldata_to_hash),
            self.inner.max_fee,
            self.factory.chain_id(),
            self.inner.nonce,
        ])
    }

    pub async fn send(
        &self,
    ) -> Result<DeployAccountTransactionResult, AccountFactoryError<F::SignError>> {
        let tx_request = self
            .get_deploy_request()
            .await
            .map_err(AccountFactoryError::Signing)?;
        self.factory
            .provider()
            .add_deploy_account_transaction(tx_request)
            .await
            .map_err(AccountFactoryError::Provider)
    }

    async fn get_deploy_request(
        &self,
    ) -> Result<BroadcastedDeployAccountTransaction, F::SignError> {
        let signature = self.factory.sign_deployment(&self.inner).await?;

        Ok(BroadcastedDeployAccountTransaction::V1(
            BroadcastedDeployAccountTransactionV1 {
                max_fee: self.inner.max_fee,
                signature,
                nonce: self.inner.nonce,
                contract_address_salt: self.inner.salt,
                constructor_calldata: self.factory.calldata(),
                class_hash: self.factory.class_hash(),
                // TODO: make use of query version tx for estimating fees
                is_query: false,
            },
        ))
    }
}

fn calculate_contract_address(salt: Felt, class_hash: Felt, constructor_calldata: &[Felt]) -> Felt {
    compute_hash_on_elements(&[
        PREFIX_CONTRACT_ADDRESS,
        Felt::ZERO,
        salt,
        class_hash,
        compute_hash_on_elements(constructor_calldata),
    ])
    .mod_floor(&ADDR_BOUND)
}
