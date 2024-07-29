use super::NotPreparedError;

use async_trait::async_trait;
use starknet_core::{
    crypto::compute_hash_on_elements,
    types::{
        BlockId, BlockTag, BroadcastedDeployAccountTransaction,
        BroadcastedDeployAccountTransactionV1, BroadcastedDeployAccountTransactionV3,
        BroadcastedTransaction, DataAvailabilityMode, DeployAccountTransactionResult, FeeEstimate,
        Felt, NonZeroFelt, ResourceBounds, ResourceBoundsMapping, SimulatedTransaction,
        SimulationFlag, SimulationFlagForEstimateFee, StarknetError,
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

/// 2 ^ 128 + 1
const QUERY_VERSION_ONE: Felt = Felt::from_raw([
    576460752142433776,
    18446744073709551584,
    17407,
    18446744073700081633,
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

    /// Signs an execution request to authorize an `DEPLOY_ACCOUNT` v1 transaction that pays
    /// transaction fees in `ETH`.
    ///
    /// If `query_only` is `true`, the commitment must be constructed in a way that a real state-
    /// changing transaction cannot be authenticated. This is to prevent replay attacks.
    async fn sign_deployment_v1(
        &self,
        deployment: &RawAccountDeploymentV1,
        query_only: bool,
    ) -> Result<Vec<Felt>, Self::SignError>;

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

    /// Generates an instance of [`AccountDeploymentV1`] for sending `DEPLOY_ACCOUNT` v1
    /// transactions. Pays transaction fees in `ETH`.
    fn deploy_v1(&self, salt: Felt) -> AccountDeploymentV1<'_, Self> {
        AccountDeploymentV1::new(salt, self)
    }

    /// Generates an instance of [`AccountDeploymentV3`] for sending `DEPLOY_ACCOUNT` v3
    /// transactions. Pays transaction fees in `STRK`.
    fn deploy_v3(&self, salt: Felt) -> AccountDeploymentV3<'_, Self> {
        AccountDeploymentV3::new(salt, self)
    }

    /// Generates an instance of [`AccountDeploymentV1`] for sending `DEPLOY_ACCOUNT` v1
    /// transactions. Pays transaction fees in `ETH`.
    #[deprecated = "use version specific variants (`deploy_v1` & `deploy_v3`) instead"]
    fn deploy(&self, salt: Felt) -> AccountDeploymentV1<'_, Self> {
        self.deploy_v1(salt)
    }
}

/// Abstraction over `DEPLOY_ACCOUNT` transactions for account contract deployment. This struct uses
/// v1 `DEPLOY_ACCOUNT` transactions under the hood, and hence pays transaction fees in ETH. To use
/// v3 transactions for STRK fee payment, use [`AccountDeploymentV3`] instead.
///
/// An intermediate type allowing users to optionally specify `nonce` and/or `max_fee`.
#[must_use]
#[derive(Debug)]
pub struct AccountDeploymentV1<'f, F> {
    factory: &'f F,
    salt: Felt,
    // We need to allow setting nonce here as `DeployAccount` transactions may have non-zero nonces
    /// after failed transactions can be included in blocks.
    nonce: Option<Felt>,
    max_fee: Option<Felt>,
    fee_estimate_multiplier: f64,
}

/// Abstraction over `DEPLOY_ACCOUNT` transactions for account contract deployment. This struct uses
/// v3 `DEPLOY_ACCOUNT` transactions under the hood, and hence pays transaction fees in STRK. To use
/// v1 transactions for ETH fee payment, use [`AccountDeploymentV1`] instead.
///
/// This is an intermediate type allowing users to optionally specify `nonce`, `gas`, and/or
/// `gas_price`.
#[must_use]
#[derive(Debug)]
pub struct AccountDeploymentV3<'f, F> {
    factory: &'f F,
    salt: Felt,
    // We need to allow setting nonce here as `DeployAccount` transactions may have non-zero nonces
    /// after failed transactions can be included in blocks.
    nonce: Option<Felt>,
    gas: Option<u64>,
    gas_price: Option<u128>,
    gas_estimate_multiplier: f64,
    gas_price_estimate_multiplier: f64,
}

/// [`AccountDeploymentV1`] but with `nonce` and `max_fee` already determined.
#[derive(Debug, Clone)]
pub struct RawAccountDeploymentV1 {
    salt: Felt,
    nonce: Felt,
    max_fee: Felt,
}

/// [`AccountDeploymentV3`] but with `nonce`, `gas` and `gas_price` already determined.
#[derive(Debug, Clone)]
pub struct RawAccountDeploymentV3 {
    salt: Felt,
    nonce: Felt,
    gas: u64,
    gas_price: u128,
}

/// [`RawAccountDeploymentV1`] but with a factory associated.
#[derive(Debug)]
pub struct PreparedAccountDeploymentV1<'f, F> {
    factory: &'f F,
    inner: RawAccountDeploymentV1,
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

impl<'f, F> AccountDeploymentV1<'f, F> {
    /// Constructs a new [`AccountDeploymentV1`].
    ///
    /// Users would typically use [`deploy_v1`](fn.deploy_v1) on an [`AccountFactory`] instead of
    /// directly calling this method.
    pub const fn new(salt: Felt, factory: &'f F) -> Self {
        Self {
            factory,
            salt,
            nonce: None,
            max_fee: None,
            fee_estimate_multiplier: 1.1,
        }
    }

    /// Returns a new [`AccountDeploymentV1`] with the `nonce`.
    pub const fn nonce(self, nonce: Felt) -> Self {
        Self {
            nonce: Some(nonce),
            ..self
        }
    }

    /// Returns a new [`AccountDeploymentV1`] with the `max_fee`.
    pub const fn max_fee(self, max_fee: Felt) -> Self {
        Self {
            max_fee: Some(max_fee),
            ..self
        }
    }

    /// Returns a new [`AccountDeploymentV1`] with the fee estimate multiplier. The multiplier is
    /// used when transaction fee is not manually specified and must be fetched from a [`Provider`]
    /// instead.
    pub const fn fee_estimate_multiplier(self, fee_estimate_multiplier: f64) -> Self {
        Self {
            fee_estimate_multiplier,
            ..self
        }
    }

    /// Calling this function after manually specifying `nonce` and `max_fee` turns
    /// [`AccountDeploymentV1`] into [`PreparedAccountDeploymentV1`]. Returns `Err` if either field is
    /// `None`.
    pub fn prepared(self) -> Result<PreparedAccountDeploymentV1<'f, F>, NotPreparedError> {
        let nonce = self.nonce.ok_or(NotPreparedError)?;
        let max_fee = self.max_fee.ok_or(NotPreparedError)?;

        Ok(PreparedAccountDeploymentV1 {
            factory: self.factory,
            inner: RawAccountDeploymentV1 {
                salt: self.salt,
                nonce,
                max_fee,
            },
        })
    }
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
            gas: None,
            gas_price: None,
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

    /// Returns a new [`AccountDeploymentV3`] with the `gas`.
    pub const fn gas(self, gas: u64) -> Self {
        Self {
            gas: Some(gas),
            ..self
        }
    }

    /// Returns a new [`AccountDeploymentV3`] with the `gas_price`.
    pub const fn gas_price(self, gas_price: u128) -> Self {
        Self {
            gas_price: Some(gas_price),
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
        let gas = self.gas.ok_or(NotPreparedError)?;
        let gas_price = self.gas_price.ok_or(NotPreparedError)?;

        Ok(PreparedAccountDeploymentV3 {
            factory: self.factory,
            inner: RawAccountDeploymentV3 {
                salt: self.salt,
                nonce,
                gas,
                gas_price,
            },
        })
    }
}

impl<'f, F> AccountDeploymentV1<'f, F>
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
    ) -> Result<PreparedAccountDeploymentV1<'f, F>, AccountFactoryError<F::SignError>> {
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

        Ok(PreparedAccountDeploymentV1 {
            factory: self.factory,
            inner: RawAccountDeploymentV1 {
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
        let skip_signature = self.factory.is_signer_interactive();

        let prepared = PreparedAccountDeploymentV1 {
            factory: self.factory,
            inner: RawAccountDeploymentV1 {
                salt: self.salt,
                nonce,
                max_fee: Felt::ZERO,
            },
        };
        let deploy = prepared
            .get_deploy_request(true, skip_signature)
            .await
            .map_err(AccountFactoryError::Signing)?;

        self.factory
            .provider()
            .estimate_fee_single(
                BroadcastedTransaction::DeployAccount(BroadcastedDeployAccountTransaction::V1(
                    deploy,
                )),
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

        let prepared = PreparedAccountDeploymentV1 {
            factory: self.factory,
            inner: RawAccountDeploymentV1 {
                salt: self.salt,
                nonce,
                max_fee: self.max_fee.unwrap_or_default(),
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
                BroadcastedTransaction::DeployAccount(BroadcastedDeployAccountTransaction::V1(
                    deploy,
                )),
                &flags,
            )
            .await
            .map_err(AccountFactoryError::Provider)
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
        let (gas, gas_price) = match (self.gas, self.gas_price) {
            (Some(gas), Some(gas_price)) => (gas, gas_price),
            (Some(gas), _) => {
                // When `gas` is specified, we only need the L1 gas price in FRI. By specifying a
                // a `gas` value, the user might be trying to avoid a full fee estimation (e.g.
                // flaky dependencies), so it's in appropriate to call `estimate_fee` here.

                // This is the lightest-weight block we can get
                let block_l1_gas_price = self
                    .factory
                    .provider()
                    .get_block_with_tx_hashes(self.factory.block_id())
                    .await
                    .map_err(AccountFactoryError::Provider)?
                    .l1_gas_price()
                    .price_in_fri;

                let block_l1_gas_price_bytes = block_l1_gas_price.to_bytes_le();
                if block_l1_gas_price_bytes.iter().skip(8).any(|&x| x != 0) {
                    return Err(AccountFactoryError::FeeOutOfRange);
                }
                let block_l1_gas_price =
                    u64::from_le_bytes(block_l1_gas_price_bytes[..8].try_into().unwrap());

                let gas_price =
                    ((block_l1_gas_price as f64) * self.gas_price_estimate_multiplier) as u128;

                (gas, gas_price)
            }
            // We have to perform fee estimation as long as gas is not specified
            _ => {
                let fee_estimate = self.estimate_fee_with_nonce(nonce).await?;

                let gas = match self.gas {
                    Some(gas) => gas,
                    None => {
                        let overall_fee_bytes = fee_estimate.overall_fee.to_bytes_le();
                        if overall_fee_bytes.iter().skip(8).any(|&x| x != 0) {
                            return Err(AccountFactoryError::FeeOutOfRange);
                        }
                        let overall_fee =
                            u64::from_le_bytes(overall_fee_bytes[..8].try_into().unwrap());

                        let gas_price_bytes = fee_estimate.gas_price.to_bytes_le();
                        if gas_price_bytes.iter().skip(8).any(|&x| x != 0) {
                            return Err(AccountFactoryError::FeeOutOfRange);
                        }
                        let gas_price =
                            u64::from_le_bytes(gas_price_bytes[..8].try_into().unwrap());

                        ((((overall_fee + gas_price - 1) / gas_price) as f64)
                            * self.gas_estimate_multiplier) as u64
                    }
                };

                let gas_price = match self.gas_price {
                    Some(gas_price) => gas_price,
                    None => {
                        let gas_price_bytes = fee_estimate.gas_price.to_bytes_le();
                        if gas_price_bytes.iter().skip(8).any(|&x| x != 0) {
                            return Err(AccountFactoryError::FeeOutOfRange);
                        }
                        let gas_price =
                            u64::from_le_bytes(gas_price_bytes[..8].try_into().unwrap());

                        ((gas_price as f64) * self.gas_price_estimate_multiplier) as u128
                    }
                };

                (gas, gas_price)
            }
        };

        Ok(PreparedAccountDeploymentV3 {
            factory: self.factory,
            inner: RawAccountDeploymentV3 {
                salt: self.salt,
                nonce,
                gas,
                gas_price,
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
                gas: 0,
                gas_price: 0,
            },
        };
        let deploy = prepared
            .get_deploy_request(true, skip_signature)
            .await
            .map_err(AccountFactoryError::Signing)?;

        self.factory
            .provider()
            .estimate_fee_single(
                BroadcastedTransaction::DeployAccount(BroadcastedDeployAccountTransaction::V3(
                    deploy,
                )),
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
                gas: self.gas.unwrap_or_default(),
                gas_price: self.gas_price.unwrap_or_default(),
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
                BroadcastedTransaction::DeployAccount(BroadcastedDeployAccountTransaction::V3(
                    deploy,
                )),
                &flags,
            )
            .await
            .map_err(AccountFactoryError::Provider)
    }
}

impl RawAccountDeploymentV1 {
    /// Gets the `salt` of the deployment request.
    pub const fn salt(&self) -> Felt {
        self.salt
    }

    /// Gets the `nonce` of the deployment request.
    pub const fn nonce(&self) -> Felt {
        self.nonce
    }

    /// Gets the `max_fee` of the deployment request.
    pub const fn max_fee(&self) -> Felt {
        self.max_fee
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

    /// Gets the `gas` of the deployment request.
    pub const fn gas(&self) -> u64 {
        self.gas
    }

    /// Gets the `gas_price` of the deployment request.
    pub const fn gas_price(&self) -> u128 {
        self.gas_price
    }
}

impl<'f, F> PreparedAccountDeploymentV1<'f, F> {
    /// Constructs [`PreparedAccountDeploymentV1`] by attaching a factory to
    /// [`RawAccountDeploymentV1`].
    pub const fn from_raw(raw_deployment: RawAccountDeploymentV1, factory: &'f F) -> Self {
        Self {
            factory,
            inner: raw_deployment,
        }
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

impl<'f, F> PreparedAccountDeploymentV1<'f, F>
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
        let mut calldata_to_hash = vec![self.factory.class_hash(), self.inner.salt];
        calldata_to_hash.append(&mut self.factory.calldata());

        compute_hash_on_elements(&[
            PREFIX_DEPLOY_ACCOUNT,
            if query_only {
                QUERY_VERSION_ONE
            } else {
                Felt::ONE
            }, // version
            self.address(),
            Felt::ZERO, // entry_point_selector
            compute_hash_on_elements(&calldata_to_hash),
            self.inner.max_fee,
            self.factory.chain_id(),
            self.inner.nonce,
        ])
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
            .add_deploy_account_transaction(BroadcastedDeployAccountTransaction::V1(tx_request))
            .await
            .map_err(AccountFactoryError::Provider)
    }

    async fn get_deploy_request(
        &self,
        query_only: bool,
        skip_signature: bool,
    ) -> Result<BroadcastedDeployAccountTransactionV1, F::SignError> {
        Ok(BroadcastedDeployAccountTransactionV1 {
            max_fee: self.inner.max_fee,
            signature: if skip_signature {
                vec![]
            } else {
                self.factory
                    .sign_deployment_v1(&self.inner, query_only)
                    .await?
            },
            nonce: self.inner.nonce,
            contract_address_salt: self.inner.salt,
            constructor_calldata: self.factory.calldata(),
            class_hash: self.factory.class_hash(),
            is_query: query_only,
        })
    }
}

impl<'f, F> PreparedAccountDeploymentV3<'f, F>
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
            resource_buffer[8..(8 + 8)].copy_from_slice(&self.inner.gas.to_be_bytes());
            resource_buffer[(8 + 8)..].copy_from_slice(&self.inner.gas_price.to_be_bytes());
            fee_hasher.update(Felt::from_bytes_be(&resource_buffer));

            // L2 resources are hard-coded to 0
            let resource_buffer = [
                0, 0, b'L', b'2', b'_', b'G', b'A', b'S', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ];
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
            .add_deploy_account_transaction(BroadcastedDeployAccountTransaction::V3(tx_request))
            .await
            .map_err(AccountFactoryError::Provider)
    }

    async fn get_deploy_request(
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
                    max_amount: self.inner.gas,
                    max_price_per_unit: self.inner.gas_price,
                },
                // L2 resources are hard-coded to 0
                l2_gas: ResourceBounds {
                    max_amount: 0,
                    max_price_per_unit: 0,
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
