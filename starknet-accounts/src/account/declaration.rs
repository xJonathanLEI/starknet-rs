use super::{
    super::NotPreparedError, Account, AccountError, ConnectedAccount, DeclarationV3,
    PreparedDeclarationV3, RawDeclarationV3,
};

use starknet_core::types::{
    BroadcastedDeclareTransactionV3, BroadcastedTransaction, DataAvailabilityMode,
    DeclareTransactionResult, FeeEstimate, Felt, FlattenedSierraClass, ResourceBounds,
    ResourceBoundsMapping, SimulatedTransaction, SimulationFlag, SimulationFlagForEstimateFee,
};
use starknet_crypto::PoseidonHasher;
use starknet_providers::Provider;
use starknet_signers::SignerInteractivityContext;
use std::sync::Arc;

/// Cairo string for "declare"
const PREFIX_DECLARE: Felt = Felt::from_raw([
    191557713328401194,
    18446744073709551615,
    18446744073709551615,
    17542456862011667323,
]);

/// 2 ^ 128 + 3
const QUERY_VERSION_THREE: Felt = Felt::from_raw([
    576460752142432688,
    18446744073709551584,
    17407,
    18446744073700081569,
]);

impl<'a, A> DeclarationV3<'a, A> {
    /// Constructs a new [`DeclarationV3`].
    ///
    /// Users would typically use [`declare_v3`](fn.declare_v3) on an [`Account`] instead of
    /// directly calling this method.
    pub const fn new(
        contract_class: Arc<FlattenedSierraClass>,
        compiled_class_hash: Felt,
        account: &'a A,
    ) -> Self {
        Self {
            account,
            contract_class,
            compiled_class_hash,
            nonce: None,
            l1_gas: None,
            l1_gas_price: None,
            l2_gas: None,
            l2_gas_price: None,
            l1_data_gas: None,
            l1_data_gas_price: None,
            gas_estimate_multiplier: 1.5,
            gas_price_estimate_multiplier: 1.5,
            tip: None,
        }
    }

    /// Returns a new [`DeclarationV3`] with the `nonce`.
    pub fn nonce(self, nonce: Felt) -> Self {
        Self {
            nonce: Some(nonce),
            ..self
        }
    }

    /// Returns a new [`DeclarationV3`] with the `l1_gas`.
    pub fn l1_gas(self, l1_gas: u64) -> Self {
        Self {
            l1_gas: Some(l1_gas),
            ..self
        }
    }

    /// Returns a new [`DeclarationV3`] with the `l1_gas_price`.
    pub fn l1_gas_price(self, l1_gas_price: u128) -> Self {
        Self {
            l1_gas_price: Some(l1_gas_price),
            ..self
        }
    }

    /// Returns a new [`DeclarationV3`] with the `l2_gas`.
    pub fn l2_gas(self, l2_gas: u64) -> Self {
        Self {
            l2_gas: Some(l2_gas),
            ..self
        }
    }

    /// Returns a new [`DeclarationV3`] with the `l2_gas_price`.
    pub fn l2_gas_price(self, l2_gas_price: u128) -> Self {
        Self {
            l2_gas_price: Some(l2_gas_price),
            ..self
        }
    }

    /// Returns a new [`DeclarationV3`] with the `l1_data_gas`.
    pub fn l1_data_gas(self, l1_data_gas: u64) -> Self {
        Self {
            l1_data_gas: Some(l1_data_gas),
            ..self
        }
    }

    /// Returns a new [`DeclarationV3`] with the `l1_data_gas_price`.
    pub fn l1_data_gas_price(self, l1_data_gas_price: u128) -> Self {
        Self {
            l1_data_gas_price: Some(l1_data_gas_price),
            ..self
        }
    }

    /// Returns a new [`DeclarationV3`] with the gas amount estimate multiplier.  The multiplier is
    /// used when the gas amount is not manually specified and must be fetched from a [`Provider`]
    /// instead.
    pub fn gas_estimate_multiplier(self, gas_estimate_multiplier: f64) -> Self {
        Self {
            gas_estimate_multiplier,
            ..self
        }
    }

    /// Returns a new [`DeclarationV3`] with the gas price estimate multiplier.  The multiplier is
    /// used when the gas price is not manually specified and must be fetched from a [`Provider`]
    /// instead.
    pub fn gas_price_estimate_multiplier(self, gas_price_estimate_multiplier: f64) -> Self {
        Self {
            gas_price_estimate_multiplier,
            ..self
        }
    }

    /// Returns a new [`DeclarationV3`] with the `tip`.
    pub fn tip(self, tip: u64) -> Self {
        Self {
            tip: Some(tip),
            ..self
        }
    }

    /// Calling this function after manually specifying all optional fields turns [`DeclarationV3`]
    /// into [`PreparedDeclarationV3`]. Returns `Err` if any field is `None`.
    pub fn prepared(self) -> Result<PreparedDeclarationV3<'a, A>, NotPreparedError> {
        let nonce = self.nonce.ok_or(NotPreparedError)?;
        let l1_gas = self.l1_gas.ok_or(NotPreparedError)?;
        let l1_gas_price = self.l1_gas_price.ok_or(NotPreparedError)?;
        let l2_gas = self.l2_gas.ok_or(NotPreparedError)?;
        let l2_gas_price = self.l2_gas_price.ok_or(NotPreparedError)?;
        let l1_data_gas = self.l1_data_gas.ok_or(NotPreparedError)?;
        let l1_data_gas_price = self.l1_data_gas_price.ok_or(NotPreparedError)?;
        let tip = self.tip.ok_or(NotPreparedError)?;

        Ok(PreparedDeclarationV3 {
            account: self.account,
            inner: RawDeclarationV3 {
                contract_class: self.contract_class,
                compiled_class_hash: self.compiled_class_hash,
                nonce,
                l1_gas,
                l1_gas_price,
                l2_gas,
                l2_gas_price,
                l1_data_gas,
                l1_data_gas_price,
                tip,
            },
        })
    }
}

impl<'a, A> DeclarationV3<'a, A>
where
    A: ConnectedAccount + Sync,
{
    /// Estimates transaction fees from a [`Provider`].
    pub async fn estimate_fee(&self) -> Result<FeeEstimate, AccountError<A::SignError>> {
        // Resolves nonce
        let nonce = match self.nonce {
            Some(value) => value,
            None => self
                .account
                .get_nonce()
                .await
                .map_err(AccountError::Provider)?,
        };

        self.estimate_fee_with_nonce(nonce).await
    }

    /// Simulates the transaction from a [`Provider`]. Transaction validation and fee transfer can
    /// be skipped.
    pub async fn simulate(
        &self,
        skip_validate: bool,
        skip_fee_charge: bool,
    ) -> Result<SimulatedTransaction, AccountError<A::SignError>> {
        // Resolves nonce
        let nonce = match self.nonce {
            Some(value) => value,
            None => self
                .account
                .get_nonce()
                .await
                .map_err(AccountError::Provider)?,
        };

        self.simulate_with_nonce(nonce, skip_validate, skip_fee_charge)
            .await
    }

    /// Signs and broadcasts the transaction to the network.
    pub async fn send(&self) -> Result<DeclareTransactionResult, AccountError<A::SignError>> {
        self.prepare().await?.send().await
    }

    async fn prepare(&self) -> Result<PreparedDeclarationV3<'a, A>, AccountError<A::SignError>> {
        // Resolves nonce
        let nonce = match self.nonce {
            Some(value) => value,
            None => self
                .account
                .get_nonce()
                .await
                .map_err(AccountError::Provider)?,
        };

        // Resolves fee settings
        let (
            l1_gas,
            l1_gas_price,
            l2_gas,
            l2_gas_price,
            l1_data_gas,
            l1_data_gas_price,
            full_block,
        ) = match (
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
                None,
            ),
            (Some(l1_gas), _, Some(l2_gas), _, Some(l1_data_gas), _) => {
                // When all `gas` fields are specified, we only need the gas prices in FRI. By
                // specifying all gas values, the user might be trying to avoid a full fee
                // estimation (e.g. flaky dependencies), so it's inappropriate to call
                // `estimate_fee` here.

                let (block_l1_gas_price, block_l2_gas_price, block_l1_data_gas_price, full_block) =
                    if self.tip.is_some() {
                        // No need to estimate tip. Just fetch the lightest-weight block we can get.
                        let block = self
                            .account
                            .provider()
                            .get_block_with_tx_hashes(self.account.block_id())
                            .await
                            .map_err(AccountError::Provider)?;
                        (
                            block.l1_gas_price().price_in_fri,
                            block.l2_gas_price().price_in_fri,
                            block.l1_data_gas_price().price_in_fri,
                            None,
                        )
                    } else {
                        // We only need th block header here but still fetching the full block to be used
                        // for tip estimation below.
                        let block = self
                            .account
                            .provider()
                            .get_block_with_txs(self.account.block_id())
                            .await
                            .map_err(AccountError::Provider)?;
                        (
                            block.l1_gas_price().price_in_fri,
                            block.l2_gas_price().price_in_fri,
                            block.l1_data_gas_price().price_in_fri,
                            Some(block),
                        )
                    };

                let adjusted_l1_gas_price =
                    ((TryInto::<u64>::try_into(block_l1_gas_price)
                        .map_err(|_| AccountError::FeeOutOfRange)? as f64)
                        * self.gas_price_estimate_multiplier) as u128;
                let adjusted_l2_gas_price =
                    ((TryInto::<u64>::try_into(block_l2_gas_price)
                        .map_err(|_| AccountError::FeeOutOfRange)? as f64)
                        * self.gas_price_estimate_multiplier) as u128;
                let adjusted_l1_data_gas_price =
                    ((TryInto::<u64>::try_into(block_l1_data_gas_price)
                        .map_err(|_| AccountError::FeeOutOfRange)? as f64)
                        * self.gas_price_estimate_multiplier) as u128;

                (
                    l1_gas,
                    adjusted_l1_gas_price,
                    l2_gas,
                    adjusted_l2_gas_price,
                    l1_data_gas,
                    adjusted_l1_data_gas_price,
                    full_block,
                )
            }
            // We have to perform fee estimation as long as gas is not specified
            _ => {
                let fee_estimate = self.estimate_fee_with_nonce(nonce).await?;

                (
                    ((fee_estimate.l1_gas_consumed as f64) * self.gas_estimate_multiplier) as u64,
                    ((TryInto::<u64>::try_into(fee_estimate.l1_gas_price)
                        .map_err(|_| AccountError::FeeOutOfRange)? as f64)
                        * self.gas_price_estimate_multiplier) as u128,
                    ((fee_estimate.l2_gas_consumed as f64) * self.gas_estimate_multiplier) as u64,
                    ((TryInto::<u64>::try_into(fee_estimate.l2_gas_price)
                        .map_err(|_| AccountError::FeeOutOfRange)? as f64)
                        * self.gas_price_estimate_multiplier) as u128,
                    ((fee_estimate.l1_data_gas_consumed as f64) * self.gas_estimate_multiplier)
                        as u64,
                    ((TryInto::<u64>::try_into(fee_estimate.l1_data_gas_price)
                        .map_err(|_| AccountError::FeeOutOfRange)? as f64)
                        * self.gas_price_estimate_multiplier) as u128,
                    None,
                )
            }
        };

        let tip = match self.tip {
            Some(tip) => tip,
            None => {
                // Need to estimate tip from median. Maybe a full block has already been fetched?
                let block = match full_block {
                    Some(block) => block,
                    None => self
                        .account
                        .provider()
                        .get_block_with_txs(self.account.block_id())
                        .await
                        .map_err(AccountError::Provider)?,
                };
                block.median_tip()
            }
        };

        Ok(PreparedDeclarationV3 {
            account: self.account,
            inner: RawDeclarationV3 {
                contract_class: self.contract_class.clone(),
                compiled_class_hash: self.compiled_class_hash,
                nonce,
                l1_gas,
                l1_gas_price,
                l2_gas,
                l2_gas_price,
                l1_data_gas,
                l1_data_gas_price,
                tip,
            },
        })
    }

    async fn estimate_fee_with_nonce(
        &self,
        nonce: Felt,
    ) -> Result<FeeEstimate, AccountError<A::SignError>> {
        let skip_signature = self
            .account
            .is_signer_interactive(SignerInteractivityContext::Other);

        let prepared = PreparedDeclarationV3 {
            account: self.account,
            inner: RawDeclarationV3 {
                contract_class: self.contract_class.clone(),
                compiled_class_hash: self.compiled_class_hash,
                nonce,
                l1_gas: 0,
                l1_gas_price: 0,
                l2_gas: 0,
                l2_gas_price: 0,
                l1_data_gas: 0,
                l1_data_gas_price: 0,
                tip: 0,
            },
        };
        let declare = prepared.get_declare_request(true, skip_signature).await?;

        self.account
            .provider()
            .estimate_fee_single(
                BroadcastedTransaction::Declare(declare),
                if skip_signature {
                    // Validation would fail since real signature was not requested
                    vec![SimulationFlagForEstimateFee::SkipValidate]
                } else {
                    // With the correct signature in place, run validation for accurate results
                    vec![]
                },
                self.account.block_id(),
            )
            .await
            .map_err(AccountError::Provider)
    }

    async fn simulate_with_nonce(
        &self,
        nonce: Felt,
        skip_validate: bool,
        skip_fee_charge: bool,
    ) -> Result<SimulatedTransaction, AccountError<A::SignError>> {
        let skip_signature = if self
            .account
            .is_signer_interactive(SignerInteractivityContext::Other)
        {
            // If signer is interactive, we would try to minimize signing requests. However, if the
            // caller has decided to not skip validation, it's best we still request a real
            // signature, as otherwise the simulation would most likely fail.
            skip_validate
        } else {
            // Signing with non-interactive signers is cheap so always request signatures.
            false
        };

        let prepared = PreparedDeclarationV3 {
            account: self.account,
            inner: RawDeclarationV3 {
                contract_class: self.contract_class.clone(),
                compiled_class_hash: self.compiled_class_hash,
                nonce,
                l1_gas: self.l1_gas.unwrap_or_default(),
                l1_gas_price: self.l1_gas_price.unwrap_or_default(),
                l2_gas: self.l2_gas.unwrap_or_default(),
                l2_gas_price: self.l2_gas_price.unwrap_or_default(),
                l1_data_gas: self.l1_data_gas.unwrap_or_default(),
                l1_data_gas_price: self.l1_data_gas_price.unwrap_or_default(),
                tip: self.tip.unwrap_or_default(),
            },
        };
        let declare = prepared.get_declare_request(true, skip_signature).await?;

        let mut flags = vec![];

        if skip_validate {
            flags.push(SimulationFlag::SkipValidate);
        }
        if skip_fee_charge {
            flags.push(SimulationFlag::SkipFeeCharge);
        }

        self.account
            .provider()
            .simulate_transaction(
                self.account.block_id(),
                BroadcastedTransaction::Declare(declare),
                &flags,
            )
            .await
            .map_err(AccountError::Provider)
    }
}

impl RawDeclarationV3 {
    /// Calculates transaction hash given `chain_id`, `address`, and `query_only`.
    pub fn transaction_hash(&self, chain_id: Felt, address: Felt, query_only: bool) -> Felt {
        let mut hasher = PoseidonHasher::new();

        hasher.update(PREFIX_DECLARE);
        hasher.update(if query_only {
            QUERY_VERSION_THREE
        } else {
            Felt::THREE
        });
        hasher.update(address);

        hasher.update({
            let mut fee_hasher = PoseidonHasher::new();

            fee_hasher.update(self.tip.into());

            let mut resource_buffer = [
                0, 0, b'L', b'1', b'_', b'G', b'A', b'S', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ];
            resource_buffer[8..(8 + 8)].copy_from_slice(&self.l1_gas.to_be_bytes());
            resource_buffer[(8 + 8)..].copy_from_slice(&self.l1_gas_price.to_be_bytes());
            fee_hasher.update(Felt::from_bytes_be(&resource_buffer));

            let mut resource_buffer = [
                0, 0, b'L', b'2', b'_', b'G', b'A', b'S', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ];
            resource_buffer[8..(8 + 8)].copy_from_slice(&self.l2_gas.to_be_bytes());
            resource_buffer[(8 + 8)..].copy_from_slice(&self.l2_gas_price.to_be_bytes());
            fee_hasher.update(Felt::from_bytes_be(&resource_buffer));

            let mut resource_buffer = [
                0, b'L', b'1', b'_', b'D', b'A', b'T', b'A', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ];
            resource_buffer[8..(8 + 8)].copy_from_slice(&self.l1_data_gas.to_be_bytes());
            resource_buffer[(8 + 8)..].copy_from_slice(&self.l1_data_gas_price.to_be_bytes());
            fee_hasher.update(Felt::from_bytes_be(&resource_buffer));

            fee_hasher.finalize()
        });

        // Hard-coded empty `paymaster_data`
        hasher.update(PoseidonHasher::new().finalize());

        hasher.update(chain_id);
        hasher.update(self.nonce);

        // Hard-coded L1 DA mode for nonce and fee
        hasher.update(Felt::ZERO);

        // Hard-coded empty `account_deployment_data`
        hasher.update(PoseidonHasher::new().finalize());

        hasher.update(self.contract_class.class_hash());
        hasher.update(self.compiled_class_hash);

        hasher.finalize()
    }

    /// Gets a reference to the flattened Sierra (Cairo 1) class being declared.
    pub fn contract_class(&self) -> &FlattenedSierraClass {
        &self.contract_class
    }

    /// Gets the CASM class hash corresponding to the Sierra class being declared.
    pub const fn compiled_class_hash(&self) -> Felt {
        self.compiled_class_hash
    }

    /// Gets the `nonce` of the declaration request.
    pub const fn nonce(&self) -> Felt {
        self.nonce
    }

    /// Gets the `l1_gas` of the declaration request.
    pub const fn l1_gas(&self) -> u64 {
        self.l1_gas
    }

    /// Gets the `l1_gas_price` of the declaration request.
    pub const fn l1_gas_price(&self) -> u128 {
        self.l1_gas_price
    }

    /// Gets the `l2_gas` of the declaration request.
    pub const fn l2_gas(&self) -> u64 {
        self.l2_gas
    }

    /// Gets the `l2_gas_price` of the declaration request.
    pub const fn l2_gas_price(&self) -> u128 {
        self.l2_gas_price
    }

    /// Gets the `l1_data_gas` of the declaration request.
    pub const fn l1_data_gas(&self) -> u64 {
        self.l1_data_gas
    }

    /// Gets the `l1_data_gas_price` of the declaration request.
    pub const fn l1_data_gas_price(&self) -> u128 {
        self.l1_data_gas_price
    }
}

impl<A> PreparedDeclarationV3<'_, A>
where
    A: Account,
{
    /// Locally calculates the hash of the transaction to be sent from this declaration given the
    /// parameters.
    pub fn transaction_hash(&self, query_only: bool) -> Felt {
        self.inner
            .transaction_hash(self.account.chain_id(), self.account.address(), query_only)
    }
}

impl<A> PreparedDeclarationV3<'_, A>
where
    A: ConnectedAccount,
{
    /// Signs and broadcasts the transaction to the network.
    pub async fn send(&self) -> Result<DeclareTransactionResult, AccountError<A::SignError>> {
        let tx_request = self.get_declare_request(false, false).await?;
        self.account
            .provider()
            .add_declare_transaction(tx_request)
            .await
            .map_err(AccountError::Provider)
    }

    async fn get_declare_request(
        &self,
        query_only: bool,
        skip_signature: bool,
    ) -> Result<BroadcastedDeclareTransactionV3, AccountError<A::SignError>> {
        Ok(BroadcastedDeclareTransactionV3 {
            sender_address: self.account.address(),
            compiled_class_hash: self.inner.compiled_class_hash,
            signature: if skip_signature {
                vec![]
            } else {
                self.account
                    .sign_declaration_v3(&self.inner, query_only)
                    .await
                    .map_err(AccountError::Signing)?
            },
            nonce: self.inner.nonce,
            contract_class: self.inner.contract_class.clone(),
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
            tip: self.inner.tip,
            // Hard-coded empty `paymaster_data`
            paymaster_data: vec![],
            // Hard-coded empty `account_deployment_data`
            account_deployment_data: vec![],
            // Hard-coded L1 DA mode for nonce and fee
            nonce_data_availability_mode: DataAvailabilityMode::L1,
            fee_data_availability_mode: DataAvailabilityMode::L1,
            is_query: query_only,
        })
    }
}
