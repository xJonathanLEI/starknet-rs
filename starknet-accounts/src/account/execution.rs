use super::{
    super::NotPreparedError, Account, AccountError, ConnectedAccount, ExecutionV1, ExecutionV3,
    PreparedExecutionV1, PreparedExecutionV3, RawExecutionV1, RawExecutionV3,
};
use crate::{Call, ExecutionEncoder};

use starknet_core::{
    crypto::compute_hash_on_elements,
    types::{
        BroadcastedInvokeTransaction, BroadcastedInvokeTransactionV1,
        BroadcastedInvokeTransactionV3, BroadcastedTransaction, DataAvailabilityMode, FeeEstimate,
        FieldElement, InvokeTransactionResult, ResourceBounds, ResourceBoundsMapping,
        SimulatedTransaction, SimulationFlag,
    },
};
use starknet_crypto::PoseidonHasher;
use starknet_providers::Provider;

/// Cairo string for "invoke"
const PREFIX_INVOKE: FieldElement = FieldElement::from_mont([
    18443034532770911073,
    18446744073709551615,
    18446744073709551615,
    513398556346534256,
]);

/// 2 ^ 128 + 1
const QUERY_VERSION_ONE: FieldElement = FieldElement::from_mont([
    18446744073700081633,
    17407,
    18446744073709551584,
    576460752142433776,
]);

/// 2 ^ 128 + 3
const QUERY_VERSION_THREE: FieldElement = FieldElement::from_mont([
    18446744073700081569,
    17407,
    18446744073709551584,
    576460752142432688,
]);

impl<'a, A> ExecutionV1<'a, A> {
    pub fn new(calls: Vec<Call>, account: &'a A) -> Self {
        Self {
            account,
            calls,
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

    /// Calling this function after manually specifying `nonce` and `max_fee` turns [ExecutionV1] into
    /// [PreparedExecutionV1]. Returns `Err` if either field is `None`.
    pub fn prepared(self) -> Result<PreparedExecutionV1<'a, A>, NotPreparedError> {
        let nonce = self.nonce.ok_or(NotPreparedError)?;
        let max_fee = self.max_fee.ok_or(NotPreparedError)?;

        Ok(PreparedExecutionV1 {
            account: self.account,
            inner: RawExecutionV1 {
                calls: self.calls,
                nonce,
                max_fee,
            },
        })
    }
}

impl<'a, A> ExecutionV3<'a, A> {
    pub fn new(calls: Vec<Call>, account: &'a A) -> Self {
        Self {
            account,
            calls,
            nonce: None,
            gas: None,
            gas_price: None,
            gas_estimate_multiplier: 1.5,
            gas_price_estimate_multiplier: 1.5,
        }
    }

    pub fn nonce(self, nonce: FieldElement) -> Self {
        Self {
            nonce: Some(nonce),
            ..self
        }
    }

    pub fn gas(self, gas: u64) -> Self {
        Self {
            gas: Some(gas),
            ..self
        }
    }

    pub fn gas_price(self, gas_price: u128) -> Self {
        Self {
            gas_price: Some(gas_price),
            ..self
        }
    }

    pub fn gas_estimate_multiplier(self, gas_estimate_multiplier: f64) -> Self {
        Self {
            gas_estimate_multiplier,
            ..self
        }
    }

    pub fn gas_price_estimate_multiplier(self, gas_price_estimate_multiplier: f64) -> Self {
        Self {
            gas_price_estimate_multiplier,
            ..self
        }
    }

    /// Calling this function after manually specifying `nonce`, `gas` and `gas_price` turns
    /// [ExecutionV3] into [PreparedExecutionV3]. Returns `Err` if any field is `None`.
    pub fn prepared(self) -> Result<PreparedExecutionV3<'a, A>, NotPreparedError> {
        let nonce = self.nonce.ok_or(NotPreparedError)?;
        let gas = self.gas.ok_or(NotPreparedError)?;
        let gas_price = self.gas_price.ok_or(NotPreparedError)?;

        Ok(PreparedExecutionV3 {
            account: self.account,
            inner: RawExecutionV3 {
                calls: self.calls,
                nonce,
                gas,
                gas_price,
            },
        })
    }
}

impl<'a, A> ExecutionV1<'a, A>
where
    A: ConnectedAccount + Sync,
{
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

    pub async fn send(&self) -> Result<InvokeTransactionResult, AccountError<A::SignError>> {
        self.prepare().await?.send().await
    }

    async fn prepare(&self) -> Result<PreparedExecutionV1<'a, A>, AccountError<A::SignError>> {
        // Resolves nonce
        let nonce = match self.nonce {
            Some(value) => value,
            None => self
                .account
                .get_nonce()
                .await
                .map_err(AccountError::Provider)?,
        };

        // Resolves max_fee
        let max_fee = match self.max_fee {
            Some(value) => value,
            None => {
                let fee_estimate = self.estimate_fee_with_nonce(nonce).await?;
                ((((TryInto::<u64>::try_into(fee_estimate.overall_fee)
                    .map_err(|_| AccountError::FeeOutOfRange)?) as f64)
                    * self.fee_estimate_multiplier) as u64)
                    .into()
            }
        };

        Ok(PreparedExecutionV1 {
            account: self.account,
            inner: RawExecutionV1 {
                calls: self.calls.clone(),
                nonce,
                max_fee,
            },
        })
    }

    async fn estimate_fee_with_nonce(
        &self,
        nonce: FieldElement,
    ) -> Result<FeeEstimate, AccountError<A::SignError>> {
        let prepared = PreparedExecutionV1 {
            account: self.account,
            inner: RawExecutionV1 {
                calls: self.calls.clone(),
                nonce,
                max_fee: FieldElement::ZERO,
            },
        };
        let invoke = prepared
            .get_invoke_request(true)
            .await
            .map_err(AccountError::Signing)?;

        self.account
            .provider()
            .estimate_fee_single(
                BroadcastedTransaction::Invoke(BroadcastedInvokeTransaction::V1(invoke)),
                [],
                self.account.block_id(),
            )
            .await
            .map_err(AccountError::Provider)
    }

    async fn simulate_with_nonce(
        &self,
        nonce: FieldElement,
        skip_validate: bool,
        skip_fee_charge: bool,
    ) -> Result<SimulatedTransaction, AccountError<A::SignError>> {
        let prepared = PreparedExecutionV1 {
            account: self.account,
            inner: RawExecutionV1 {
                calls: self.calls.clone(),
                nonce,
                max_fee: self.max_fee.unwrap_or_default(),
            },
        };
        let invoke = prepared
            .get_invoke_request(true)
            .await
            .map_err(AccountError::Signing)?;

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
                BroadcastedTransaction::Invoke(BroadcastedInvokeTransaction::V1(invoke)),
                &flags,
            )
            .await
            .map_err(AccountError::Provider)
    }
}

impl<'a, A> ExecutionV3<'a, A>
where
    A: ConnectedAccount + Sync,
{
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

    pub async fn send(&self) -> Result<InvokeTransactionResult, AccountError<A::SignError>> {
        self.prepare().await?.send().await
    }

    async fn prepare(&self) -> Result<PreparedExecutionV3<'a, A>, AccountError<A::SignError>> {
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
        let (gas, gas_price) = match (self.gas, self.gas_price) {
            (Some(gas), Some(gas_price)) => (gas, gas_price),
            // We have to perform fee estimation as long as it's not fully specified
            _ => {
                let fee_estimate = self.estimate_fee_with_nonce(nonce).await?;

                let gas = match self.gas {
                    Some(gas) => gas,
                    None => {
                        (((TryInto::<u64>::try_into(fee_estimate.gas_consumed)
                            .map_err(|_| AccountError::FeeOutOfRange)?)
                            as f64)
                            * self.gas_estimate_multiplier) as u64
                    }
                };

                let gas_price = match self.gas_price {
                    Some(gas_price) => gas_price,
                    None => {
                        (((TryInto::<u64>::try_into(fee_estimate.gas_price)
                            .map_err(|_| AccountError::FeeOutOfRange)?)
                            as f64)
                            * self.gas_price_estimate_multiplier) as u128
                    }
                };

                (gas, gas_price)
            }
        };

        Ok(PreparedExecutionV3 {
            account: self.account,
            inner: RawExecutionV3 {
                calls: self.calls.clone(),
                nonce,
                gas,
                gas_price,
            },
        })
    }

    async fn estimate_fee_with_nonce(
        &self,
        nonce: FieldElement,
    ) -> Result<FeeEstimate, AccountError<A::SignError>> {
        let prepared = PreparedExecutionV3 {
            account: self.account,
            inner: RawExecutionV3 {
                calls: self.calls.clone(),
                nonce,
                gas: 0,
                gas_price: 0,
            },
        };
        let invoke = prepared
            .get_invoke_request(true)
            .await
            .map_err(AccountError::Signing)?;

        self.account
            .provider()
            .estimate_fee_single(
                BroadcastedTransaction::Invoke(BroadcastedInvokeTransaction::V3(invoke)),
                [],
                self.account.block_id(),
            )
            .await
            .map_err(AccountError::Provider)
    }

    async fn simulate_with_nonce(
        &self,
        nonce: FieldElement,
        skip_validate: bool,
        skip_fee_charge: bool,
    ) -> Result<SimulatedTransaction, AccountError<A::SignError>> {
        let prepared = PreparedExecutionV3 {
            account: self.account,
            inner: RawExecutionV3 {
                calls: self.calls.clone(),
                nonce,
                gas: self.gas.unwrap_or_default(),
                gas_price: self.gas_price.unwrap_or_default(),
            },
        };
        let invoke = prepared
            .get_invoke_request(true)
            .await
            .map_err(AccountError::Signing)?;

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
                BroadcastedTransaction::Invoke(BroadcastedInvokeTransaction::V3(invoke)),
                &flags,
            )
            .await
            .map_err(AccountError::Provider)
    }
}

impl RawExecutionV1 {
    pub fn transaction_hash<E>(
        &self,
        chain_id: FieldElement,
        address: FieldElement,
        query_only: bool,
        encoder: E,
    ) -> FieldElement
    where
        E: ExecutionEncoder,
    {
        compute_hash_on_elements(&[
            PREFIX_INVOKE,
            if query_only {
                QUERY_VERSION_ONE
            } else {
                FieldElement::ONE
            }, // version
            address,
            FieldElement::ZERO, // entry_point_selector
            compute_hash_on_elements(&encoder.encode_calls(&self.calls)),
            self.max_fee,
            chain_id,
            self.nonce,
        ])
    }

    pub fn calls(&self) -> &[Call] {
        &self.calls
    }

    pub fn nonce(&self) -> FieldElement {
        self.nonce
    }

    pub fn max_fee(&self) -> FieldElement {
        self.max_fee
    }
}

impl RawExecutionV3 {
    pub fn transaction_hash<E>(
        &self,
        chain_id: FieldElement,
        address: FieldElement,
        query_only: bool,
        encoder: E,
    ) -> FieldElement
    where
        E: ExecutionEncoder,
    {
        let mut hasher = PoseidonHasher::new();

        hasher.update(PREFIX_INVOKE);
        hasher.update(if query_only {
            QUERY_VERSION_THREE
        } else {
            FieldElement::THREE
        });
        hasher.update(address);

        hasher.update({
            let mut fee_hasher = PoseidonHasher::new();

            // Tip: fee market has not been been activated yet so it's hard-coded to be 0
            fee_hasher.update(FieldElement::ZERO);

            let mut resource_buffer = [
                0, 0, b'L', b'1', b'_', b'G', b'A', b'S', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ];
            resource_buffer[8..(8 + 8)].copy_from_slice(&self.gas.to_be_bytes());
            resource_buffer[(8 + 8)..].copy_from_slice(&self.gas_price.to_be_bytes());
            fee_hasher.update(FieldElement::from_bytes_be(&resource_buffer).unwrap());

            // L2 resources are hard-coded to 0
            let resource_buffer = [
                0, 0, b'L', b'2', b'_', b'G', b'A', b'S', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ];
            fee_hasher.update(FieldElement::from_bytes_be(&resource_buffer).unwrap());

            fee_hasher.finalize()
        });

        // Hard-coded empty `paymaster_data`
        hasher.update(PoseidonHasher::new().finalize());

        hasher.update(chain_id);
        hasher.update(self.nonce);

        // Hard-coded L1 DA mode for nonce and fee
        hasher.update(FieldElement::ZERO);

        // Hard-coded empty `account_deployment_data`
        hasher.update(PoseidonHasher::new().finalize());

        hasher.update({
            let mut calldata_hasher = PoseidonHasher::new();

            encoder
                .encode_calls(&self.calls)
                .into_iter()
                .for_each(|element| calldata_hasher.update(element));

            calldata_hasher.finalize()
        });

        hasher.finalize()
    }

    pub fn calls(&self) -> &[Call] {
        &self.calls
    }

    pub fn nonce(&self) -> FieldElement {
        self.nonce
    }

    pub fn gas(&self) -> u64 {
        self.gas
    }

    pub fn gas_price(&self) -> u128 {
        self.gas_price
    }
}

impl<'a, A> PreparedExecutionV1<'a, A>
where
    A: Account,
{
    /// Locally calculates the hash of the transaction to be sent from this execution given the
    /// parameters.
    pub fn transaction_hash(&self, query_only: bool) -> FieldElement {
        self.inner.transaction_hash(
            self.account.chain_id(),
            self.account.address(),
            query_only,
            self.account,
        )
    }
}

impl<'a, A> PreparedExecutionV3<'a, A>
where
    A: Account,
{
    /// Locally calculates the hash of the transaction to be sent from this execution given the
    /// parameters.
    pub fn transaction_hash(&self, query_only: bool) -> FieldElement {
        self.inner.transaction_hash(
            self.account.chain_id(),
            self.account.address(),
            query_only,
            self.account,
        )
    }
}

impl<'a, A> PreparedExecutionV1<'a, A>
where
    A: ConnectedAccount,
{
    pub async fn send(&self) -> Result<InvokeTransactionResult, AccountError<A::SignError>> {
        let tx_request = self
            .get_invoke_request(false)
            .await
            .map_err(AccountError::Signing)?;
        self.account
            .provider()
            .add_invoke_transaction(BroadcastedInvokeTransaction::V1(tx_request))
            .await
            .map_err(AccountError::Provider)
    }

    // The `simulate` function is temporarily removed until it's supported in [Provider]
    // TODO: add `simulate` back once transaction simulation in supported

    pub async fn get_invoke_request(
        &self,
        query_only: bool,
    ) -> Result<BroadcastedInvokeTransactionV1, A::SignError> {
        let signature = self
            .account
            .sign_execution_v1(&self.inner, query_only)
            .await?;

        Ok(BroadcastedInvokeTransactionV1 {
            max_fee: self.inner.max_fee,
            signature,
            nonce: self.inner.nonce,
            sender_address: self.account.address(),
            calldata: self.account.encode_calls(&self.inner.calls),
            is_query: query_only,
        })
    }
}

impl<'a, A> PreparedExecutionV3<'a, A>
where
    A: ConnectedAccount,
{
    pub async fn send(&self) -> Result<InvokeTransactionResult, AccountError<A::SignError>> {
        let tx_request = self
            .get_invoke_request(false)
            .await
            .map_err(AccountError::Signing)?;
        self.account
            .provider()
            .add_invoke_transaction(BroadcastedInvokeTransaction::V3(tx_request))
            .await
            .map_err(AccountError::Provider)
    }

    // The `simulate` function is temporarily removed until it's supported in [Provider]
    // TODO: add `simulate` back once transaction simulation in supported

    pub async fn get_invoke_request(
        &self,
        query_only: bool,
    ) -> Result<BroadcastedInvokeTransactionV3, A::SignError> {
        let signature = self
            .account
            .sign_execution_v3(&self.inner, query_only)
            .await?;

        Ok(BroadcastedInvokeTransactionV3 {
            sender_address: self.account.address(),
            calldata: self.account.encode_calls(&self.inner.calls),
            signature,
            nonce: self.inner.nonce,
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
            // Hard-coded empty `account_deployment_data`
            account_deployment_data: vec![],
            // Hard-coded L1 DA mode for nonce and fee
            nonce_data_availability_mode: DataAvailabilityMode::L1,
            fee_data_availability_mode: DataAvailabilityMode::L1,
            is_query: query_only,
        })
    }
}
