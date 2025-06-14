use super::NotPreparedError;

use async_trait::async_trait;
use starknet_core::{
    crypto::compute_hash_on_elements,
    types::{
        BlockId, BlockTag, BroadcastedDeployAccountTransactionV3, BroadcastedTransaction,
        DataAvailabilityMode, DeployAccountTransactionResult, FeeEstimate, Felt, NonZeroFelt,
        ResourceBounds, ResourceBoundsMapping, SimulatedTransaction, SimulationFlag,
        SimulationFlagForEstimateFee, StarknetError,
    },
};
use starknet_crypto::PoseidonHasher;
use starknet_providers::{Provider, ProviderError};
use std::error::Error;

pub mod argent;
pub mod open_zeppelin;

/// Cairo string for `deploy_account`
const PREFIX_DEPLOY_ACCOUNT: Felt = Felt::from_raw([
    461298303000467581,
    18446744073709551615,
    18443211694809419988,
    3350261884043292318,
]);

/// 2 ^ 128 + 3
const QUERY_VERSION_THREE: Felt = Felt::from_raw([
    576460752142432688,
    18446744073709551584,
    17407,
    18446744073700081569,
]);

/// Cairo string for `STARKNET_CONTRACT_ADDRESS`
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

/// Abstraction over different ways of deploying account contracts using the `DEPLOY_ACCOUNT`
/// transaction type.
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
pub trait AccountFactory: Sized {
    /// The [`Provider`] type attached to this account factory.
    type Provider: Provider + Sync;
    /// Possible errors for signing transactions.
    type SignError: Error + Send + Sync;

    /// Gets the class hash of the account contract.
    fn class_hash(&self) -> Felt;

    /// Gets the constructor calldata for the deployment transaction.
    fn calldata(&self) -> Vec<Felt>;

    /// Gets the chain ID of the target network.
    fn chain_id(&self) -> Felt;

    /// Gets a reference to the attached [`Provider`] instance.
    fn provider(&self) -> &Self::Provider;

    /// Whether the underlying signer implementation is interactive, such as a hardware wallet.
    /// Implementations should return `true` if the signing operation is very expensive, even if not
    /// strictly "interactive" as in requiring human input.
    ///
    /// This affects how an account factory makes decision on whether to request a real signature
    /// for estimation/simulation purposes.
    fn is_signer_interactive(&self) -> bool;

    /// Block ID to use when estimating fees.
    fn block_id(&self) -> BlockId {
        BlockId::Tag(BlockTag::Latest)
    }

    /// Signs an execution request to authorize an `DEPLOY_ACCOUNT` v3 transaction that pays
    /// transaction fees in `STRK`.
    ///
    /// If `query_only` is `true`, the commitment must be constructed in a way that a real state-
    /// changing transaction cannot be authenticated. This is to prevent replay attacks.
    async fn sign_deployment_v3(
        &self,
        deployment: &RawAccountDeploymentV3,
        query_only: bool,
    ) -> Result<Vec<Felt>, Self::SignError>;

    /// Generates an instance of [`AccountDeploymentV3`] for sending `DEPLOY_ACCOUNT` v3
    /// transactions. Pays transaction fees in `STRK`.
    fn deploy_v3(&self, salt: Felt) -> AccountDeploymentV3<'_, Self> {
        AccountDeploymentV3::new(salt, self)
    }

    /// Generates an instance of [`AccountDeploymentV3`] for sending `DEPLOY_ACCOUNT` v3
    /// transactions. Pays transaction fees in `STRK`.
    #[deprecated = "transaction version used might change unexpectedly; use `deploy_v3` instead"]
    fn deploy(&self, salt: Felt) -> AccountDeploymentV3<'_, Self> {
        self.deploy_v3(salt)
    }
}

/// Abstraction over `DEPLOY_ACCOUNT` transactions for account contract deployment. This struct uses
/// v3 `DEPLOY_ACCOUNT` transactions under the hood, and hence pays transaction fees in STRK.
///
/// This is an intermediate type allowing users to optionally specify `nonce` and transaction fee
/// options.
#[must_use]
#[derive(Debug)]
pub struct AccountDeploymentV3<'f, F> {
    factory: &'f F,
    salt: Felt,
    // We need to allow setting nonce here as `DeployAccount` transactions may have non-zero nonces
    /// after failed transactions can be included in blocks.
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

/// [`AccountDeploymentV3`] but with `nonce` and other transaction fee options already determined.
#[derive(Debug, Clone)]
pub struct RawAccountDeploymentV3 {
    salt: Felt,
    nonce: Felt,
    l1_gas: u64,
    l1_gas_price: u128,
    l2_gas: u64,
    l2_gas_price: u128,
    l1_data_gas: u64,
    l1_data_gas_price: u128,
}

/// [`RawAccountDeploymentV3`] but with a factory associated.
#[derive(Debug)]
pub struct PreparedAccountDeploymentV3<'f, F> {
    factory: &'f F,
    inner: RawAccountDeploymentV3,
}

/// Errors using Starknet account factories.
#[derive(Debug, thiserror::Error)]
pub enum AccountFactoryError<S> {
    /// An error is encountered when signing a request.
    #[error(transparent)]
    Signing(S),
    /// An error is encountered with communicating with the network.
    #[error(transparent)]
    Provider(ProviderError),
    /// Transaction fee calculation overflow.
    #[error("fee calculation overflow")]
    FeeOutOfRange,
}

impl<'f, F> AccountDeploymentV3<'f, F> {
    /// Constructs a new [`AccountDeploymentV3`].
    ///
    /// Users would typically use [`deploy_v3`](fn.deploy_v3) on an [`AccountFactory`] instead of
    /// directly calling this method.
    pub const fn new(salt: Felt, factory: &'f F) -> Self {
        Self {
            factory,
            salt,
            nonce: None,
            l1_gas: None,
            l1_gas_price: None,
            l2_gas: None,
            l2_gas_price: None,
            l1_data_gas: None,
            l1_data_gas_price: None,
            gas_estimate_multiplier: 1.5,
            gas_price_estimate_multiplier: 1.5,
        }
    }

    /// Returns a new [`AccountDeploymentV3`] with the `nonce`.
    pub const fn nonce(self, nonce: Felt) -> Self {
        Self {
            nonce: Some(nonce),
            ..self
        }
    }

    /// Returns a new [`AccountDeploymentV3`] with the `l1_gas`.
    pub const fn l1_gas(self, l1_gas: u64) -> Self {
        Self {
            l1_gas: Some(l1_gas),
            ..self
        }
    }

    /// Returns a new [`AccountDeploymentV3`] with the `l1_gas_price`.
    pub const fn l1_gas_price(self, l1_gas_price: u128) -> Self {
        Self {
            l1_gas_price: Some(l1_gas_price),
            ..self
        }
    }

    /// Returns a new [`AccountDeploymentV3`] with the `l2_gas`.
    pub const fn l2_gas(self, l2_gas: u64) -> Self {
        Self {
            l2_gas: Some(l2_gas),
            ..self
        }
    }

    /// Returns a new [`AccountDeploymentV3`] with the `l2_gas_price`.
    pub const fn l2_gas_price(self, l2_gas_price: u128) -> Self {
        Self {
            l2_gas_price: Some(l2_gas_price),
            ..self
        }
    }

    /// Returns a new [`AccountDeploymentV3`] with the `l1_data_gas`.
    pub const fn l1_data_gas(self, l1_data_gas: u64) -> Self {
        Self {
            l1_data_gas: Some(l1_data_gas),
            ..self
        }
    }

    /// Returns a new [`AccountDeploymentV3`] with the `l1_data_gas_price`.
    pub const fn l1_data_gas_price(self, l1_data_gas_price: u128) -> Self {
        Self {
            l1_data_gas_price: Some(l1_data_gas_price),
            ..self
        }
    }

    /// Returns a new [`AccountDeploymentV3`] with the gas amount estimate multiplier.  The
    /// multiplier is used when the gas amount is not manually specified and must be fetched from a
    /// [`Provider`] instead.
    pub const fn gas_estimate_multiplier(self, gas_estimate_multiplier: f64) -> Self {
        Self {
            gas_estimate_multiplier,
            ..self
        }
    }

    /// Returns a new [`AccountDeploymentV3`] with the gas price estimate multiplier.  The
    /// multiplier is used when the gas price is not manually specified and must be fetched from a
    /// [`Provider`] instead.
    pub const fn gas_price_estimate_multiplier(self, gas_price_estimate_multiplier: f64) -> Self {
        Self {
            gas_price_estimate_multiplier,
            ..self
        }
    }

    /// Calling this function after manually specifying `nonce` and `max_fee` turns
    /// [`AccountDeploymentV3`] into [`PreparedAccountDeploymentV3`]. Returns `Err` if either field is
    /// `None`.
    pub fn prepared(self) -> Result<PreparedAccountDeploymentV3<'f, F>, NotPreparedError> {
        let nonce = self.nonce.ok_or(NotPreparedError)?;
        let l1_gas = self.l1_gas.ok_or(NotPreparedError)?;
        let l1_gas_price = self.l1_gas_price.ok_or(NotPreparedError)?;
        let l2_gas = self.l2_gas.ok_or(NotPreparedError)?;
        let l2_gas_price = self.l2_gas_price.ok_or(NotPreparedError)?;
        let l1_data_gas = self.l1_data_gas.ok_or(NotPreparedError)?;
        let l1_data_gas_price = self.l1_data_gas_price.ok_or(NotPreparedError)?;

        Ok(PreparedAccountDeploymentV3 {
            factory: self.factory,
            inner: RawAccountDeploymentV3 {
                salt: self.salt,
                nonce,
                l1_gas,
                l1_gas_price,
                l2_gas,
                l2_gas_price,
                l1_data_gas,
                l1_data_gas_price,
            },
        })
    }
}

impl<'f, F> AccountDeploymentV3<'f, F>
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

    /// Fetches the next available nonce from a [`Provider`]. In most cases this would be `0` but
    /// it can also be non-zero if previous reverted deployment attempts exist.
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

    /// Estimates transaction fees from a [`Provider`].
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

    /// Simulates the transaction from a [`Provider`]. Transaction validation and fee transfer can
    /// be skipped.
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

    /// Signs and broadcasts the transaction to the network.
    pub async fn send(
        &self,
    ) -> Result<DeployAccountTransactionResult, AccountFactoryError<F::SignError>> {
        self.prepare().await?.send().await
    }

    async fn prepare(
        &self,
    ) -> Result<PreparedAccountDeploymentV3<'f, F>, AccountFactoryError<F::SignError>> {
        // Resolves nonce
        let nonce = match self.nonce {
            Some(value) => value,
            None => self
                .fetch_nonce()
                .await
                .map_err(AccountFactoryError::Provider)?,
        };

        // Resolves fee settings
        let (l1_gas, l1_gas_price, l2_gas, l2_gas_price, l1_data_gas, l1_data_gas_price) = match (
            self.l1_gas,
            self.l1_gas_price,
            self.l2_gas,
            self.l2_gas_price,
            self.l1_data_gas,
            self.l1_data_gas_price,
        ) {
            (
                Some(l1_gas),
                Some(l1_gas_price),
                Some(l2_gas),
                Some(l2_gas_price),
                Some(l1_data_gas),
                Some(l1_data_gas_price),
            ) => (
                l1_gas,
                l1_gas_price,
                l2_gas,
                l2_gas_price,
                l1_data_gas,
                l1_data_gas_price,
            ),
            (Some(l1_gas), _, Some(l2_gas), _, Some(l1_data_gas), _) => {
                // When all `gas` fields are specified, we only need the gas prices in FRI. By
                // specifying all gas values, the user might be trying to avoid a full fee
                // estimation (e.g. flaky dependencies), so it's inappropriate to call
                // `estimate_fee` here.

                // This is the lightest-weight block we can get
                let block = self
                    .factory
                    .provider()
                    .get_block_with_tx_hashes(self.factory.block_id())
                    .await
                    .map_err(AccountFactoryError::Provider)?;
                let block_l1_gas_price = block.l1_gas_price().price_in_fri;
                let block_l2_gas_price = block.l2_gas_price().price_in_fri;
                let block_l1_data_gas_price = block.l1_data_gas_price().price_in_fri;

                let adjusted_l1_gas_price = ((TryInto::<u64>::try_into(block_l1_gas_price)
                    .map_err(|_| AccountFactoryError::FeeOutOfRange)?
                    as f64)
                    * self.gas_price_estimate_multiplier)
                    as u128;
                let adjusted_l2_gas_price = ((TryInto::<u64>::try_into(block_l2_gas_price)
                    .map_err(|_| AccountFactoryError::FeeOutOfRange)?
                    as f64)
                    * self.gas_price_estimate_multiplier)
                    as u128;
                let adjusted_l1_data_gas_price = ((TryInto::<u64>::try_into(block_l1_data_gas_price)
                    .map_err(|_| AccountFactoryError::FeeOutOfRange)?
                    as f64)
                    * self.gas_price_estimate_multiplier)
                    as u128;

                (
                    l1_gas,
                    adjusted_l1_gas_price,
                    l2_gas,
                    adjusted_l2_gas_price,
                    l1_data_gas,
                    adjusted_l1_data_gas_price,
                )
            }
            // We have to perform fee estimation as long as gas is not specified
            _ => {
                let fee_estimate = self.estimate_fee_with_nonce(nonce).await?;

                (
                    ((fee_estimate.l1_gas_consumed as f64) * self.gas_estimate_multiplier) as u64,
                    ((TryInto::<u64>::try_into(fee_estimate.l1_gas_price)
                        .map_err(|_| AccountFactoryError::FeeOutOfRange)?
                        as f64)
                        * self.gas_price_estimate_multiplier) as u128,
                    ((fee_estimate.l2_gas_consumed as f64) * self.gas_estimate_multiplier) as u64,
                    ((TryInto::<u64>::try_into(fee_estimate.l2_gas_price)
                        .map_err(|_| AccountFactoryError::FeeOutOfRange)?
                        as f64)
                        * self.gas_price_estimate_multiplier) as u128,
                    ((fee_estimate.l1_data_gas_consumed as f64) * self.gas_estimate_multiplier)
                        as u64,
                    ((TryInto::<u64>::try_into(fee_estimate.l1_data_gas_price)
                        .map_err(|_| AccountFactoryError::FeeOutOfRange)?
                        as f64)
                        * self.gas_price_estimate_multiplier) as u128,
                )
            }
        };

        Ok(PreparedAccountDeploymentV3 {
            factory: self.factory,
            inner: RawAccountDeploymentV3 {
                salt: self.salt,
                nonce,
                l1_gas,
                l1_gas_price,
                l2_gas,
                l2_gas_price,
                l1_data_gas,
                l1_data_gas_price,
            },
        })
    }

    async fn estimate_fee_with_nonce(
        &self,
        nonce: Felt,
    ) -> Result<FeeEstimate, AccountFactoryError<F::SignError>> {
        let skip_signature = self.factory.is_signer_interactive();

        let prepared = PreparedAccountDeploymentV3 {
            factory: self.factory,
            inner: RawAccountDeploymentV3 {
                salt: self.salt,
                nonce,
                l1_gas: 0,
                l1_gas_price: 0,
                l2_gas: 0,
                l2_gas_price: 0,
                l1_data_gas: 0,
                l1_data_gas_price: 0,
            },
        };
        let deploy = prepared
            .get_deploy_request(true, skip_signature)
            .await
            .map_err(AccountFactoryError::Signing)?;

        self.factory
            .provider()
            .estimate_fee_single(
                BroadcastedTransaction::DeployAccount(deploy),
                if skip_signature {
                    // Validation would fail since real signature was not requested
                    vec![SimulationFlagForEstimateFee::SkipValidate]
                } else {
                    // With the correct signature in place, run validation for accurate results
                    vec![]
                },
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
        let skip_signature = if self.factory.is_signer_interactive() {
            // If signer is interactive, we would try to minimize signing requests. However, if the
            // caller has decided to not skip validation, it's best we still request a real
            // signature, as otherwise the simulation would most likely fail.
            skip_validate
        } else {
            // Signing with non-interactive signers is cheap so always request signatures.
            false
        };

        let prepared = PreparedAccountDeploymentV3 {
            factory: self.factory,
            inner: RawAccountDeploymentV3 {
                salt: self.salt,
                nonce,
                l1_gas: self.l1_gas.unwrap_or_default(),
                l1_gas_price: self.l1_gas_price.unwrap_or_default(),
                l2_gas: self.l2_gas.unwrap_or_default(),
                l2_gas_price: self.l2_gas_price.unwrap_or_default(),
                l1_data_gas: self.l1_data_gas.unwrap_or_default(),
                l1_data_gas_price: self.l1_data_gas_price.unwrap_or_default(),
            },
        };
        let deploy = prepared
            .get_deploy_request(true, skip_signature)
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

impl RawAccountDeploymentV3 {
    /// Gets the `salt` of the deployment request.
    pub const fn salt(&self) -> Felt {
        self.salt
    }

    /// Gets the `nonce` of the deployment request.
    pub const fn nonce(&self) -> Felt {
        self.nonce
    }

    /// Gets the `l1_gas` of the deployment request.
    pub const fn l1_gas(&self) -> u64 {
        self.l1_gas
    }

    /// Gets the `l1_gas_price` of the deployment request.
    pub const fn l1_gas_price(&self) -> u128 {
        self.l1_gas_price
    }

    /// Gets the `l2_gas` of the deployment request.
    pub const fn l2_gas(&self) -> u64 {
        self.l2_gas
    }

    /// Gets the `l2_gas_price` of the deployment request.
    pub const fn l2_gas_price(&self) -> u128 {
        self.l2_gas_price
    }

    /// Gets the `l1_data_gas` of the deployment request.
    pub const fn l1_data_gas(&self) -> u64 {
        self.l1_data_gas
    }

    /// Gets the `l1_data_gas_price` of the deployment request.
    pub const fn l1_data_gas_price(&self) -> u128 {
        self.l1_data_gas_price
    }
}

impl<'f, F> PreparedAccountDeploymentV3<'f, F> {
    /// Constructs [`PreparedAccountDeploymentV3`] by attaching a factory to
    /// [`RawAccountDeploymentV3`].
    pub const fn from_raw(raw_deployment: RawAccountDeploymentV3, factory: &'f F) -> Self {
        Self {
            factory,
            inner: raw_deployment,
        }
    }
}

impl<F> PreparedAccountDeploymentV3<'_, F>
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

    /// Calculates transaction hash given `query_only`.
    pub fn transaction_hash(&self, query_only: bool) -> Felt {
        let mut hasher = PoseidonHasher::new();

        hasher.update(PREFIX_DEPLOY_ACCOUNT);
        hasher.update(if query_only {
            QUERY_VERSION_THREE
        } else {
            Felt::THREE
        });
        hasher.update(self.address());

        hasher.update({
            let mut fee_hasher = PoseidonHasher::new();

            // Tip: fee market has not been been activated yet so it's hard-coded to be 0
            fee_hasher.update(Felt::ZERO);

            let mut resource_buffer = [
                0, 0, b'L', b'1', b'_', b'G', b'A', b'S', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ];
            resource_buffer[8..(8 + 8)].copy_from_slice(&self.inner.l1_gas.to_be_bytes());
            resource_buffer[(8 + 8)..].copy_from_slice(&self.inner.l1_gas_price.to_be_bytes());
            fee_hasher.update(Felt::from_bytes_be(&resource_buffer));

            let mut resource_buffer = [
                0, 0, b'L', b'2', b'_', b'G', b'A', b'S', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ];
            resource_buffer[8..(8 + 8)].copy_from_slice(&self.inner.l2_gas.to_be_bytes());
            resource_buffer[(8 + 8)..].copy_from_slice(&self.inner.l2_gas_price.to_be_bytes());
            fee_hasher.update(Felt::from_bytes_be(&resource_buffer));

            let mut resource_buffer = [
                0, b'L', b'1', b'_', b'D', b'A', b'T', b'A', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ];
            resource_buffer[8..(8 + 8)].copy_from_slice(&self.inner.l1_data_gas.to_be_bytes());
            resource_buffer[(8 + 8)..].copy_from_slice(&self.inner.l1_data_gas_price.to_be_bytes());
            fee_hasher.update(Felt::from_bytes_be(&resource_buffer));

            fee_hasher.finalize()
        });

        // Hard-coded empty `paymaster_data`
        hasher.update(PoseidonHasher::new().finalize());

        hasher.update(self.factory.chain_id());
        hasher.update(self.inner.nonce);

        // Hard-coded L1 DA mode for nonce and fee
        hasher.update(Felt::ZERO);

        hasher.update({
            let mut calldata_hasher = PoseidonHasher::new();

            self.factory
                .calldata()
                .into_iter()
                .for_each(|element| calldata_hasher.update(element));

            calldata_hasher.finalize()
        });

        hasher.update(self.factory.class_hash());
        hasher.update(self.inner.salt);

        hasher.finalize()
    }

    /// Signs and broadcasts the transaction to the network.
    pub async fn send(
        &self,
    ) -> Result<DeployAccountTransactionResult, AccountFactoryError<F::SignError>> {
        let tx_request = self
            .get_deploy_request(false, false)
            .await
            .map_err(AccountFactoryError::Signing)?;
        self.factory
            .provider()
            .add_deploy_account_transaction(tx_request)
            .await
            .map_err(AccountFactoryError::Provider)
    }

    pub async fn get_deploy_request(
        &self,
        query_only: bool,
        skip_signature: bool,
    ) -> Result<BroadcastedDeployAccountTransactionV3, F::SignError> {
        Ok(BroadcastedDeployAccountTransactionV3 {
            signature: if skip_signature {
                vec![]
            } else {
                self.factory
                    .sign_deployment_v3(&self.inner, query_only)
                    .await?
            },
            nonce: self.inner.nonce,
            contract_address_salt: self.inner.salt,
            constructor_calldata: self.factory.calldata(),
            class_hash: self.factory.class_hash(),
            resource_bounds: ResourceBoundsMapping {
                l1_gas: ResourceBounds {
                    max_amount: self.inner.l1_gas,
                    max_price_per_unit: self.inner.l1_gas_price,
                },
                l1_data_gas: ResourceBounds {
                    max_amount: self.inner.l1_data_gas,
                    max_price_per_unit: self.inner.l1_data_gas_price,
                },
                l2_gas: ResourceBounds {
                    max_amount: self.inner.l2_gas,
                    max_price_per_unit: self.inner.l2_gas_price,
                },
            },
            // Fee market has not been been activated yet so it's hard-coded to be 0
            tip: 0,
            // Hard-coded empty `paymaster_data`
            paymaster_data: vec![],
            // Hard-coded L1 DA mode for nonce and fee
            nonce_data_availability_mode: DataAvailabilityMode::L1,
            fee_data_availability_mode: DataAvailabilityMode::L1,
            is_query: query_only,
        })
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
