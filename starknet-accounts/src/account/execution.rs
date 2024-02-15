use super::{
    super::NotPreparedError, Account, AccountError, ConnectedAccount, Execution, PreparedExecution,
    RawExecution,
};
use crate::{Call, ExecutionEncoder};
use starknet_core::{
    crypto::compute_hash_on_elements,
    types::{
        BroadcastedInvokeTransaction, BroadcastedInvokeTransactionV1, BroadcastedTransaction,
        FeeEstimate, FieldElement, InvokeTransactionResult, SimulatedTransaction, SimulationFlag,
    },
};
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

impl<'a, A> Execution<'a, A> {
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

    /// Calling this function after manually specifying `nonce` and `max_fee` turns [Execution] into
    /// [PreparedExecution]. Returns `Err` if either field is `None`.
    pub fn prepared(self) -> Result<PreparedExecution<'a, A>, NotPreparedError> {
        let nonce = self.nonce.ok_or(NotPreparedError)?;
        let max_fee = self.max_fee.ok_or(NotPreparedError)?;

        Ok(PreparedExecution {
            account: self.account,
            inner: RawExecution {
                calls: self.calls,
                nonce,
                max_fee,
            },
        })
    }
}

impl<'a, A> Execution<'a, A>
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

    async fn prepare(&self) -> Result<PreparedExecution<'a, A>, AccountError<A::SignError>> {
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

        Ok(PreparedExecution {
            account: self.account,
            inner: RawExecution {
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
        let prepared = PreparedExecution {
            account: self.account,
            inner: RawExecution {
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
                BroadcastedTransaction::Invoke(invoke),
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
        let prepared = PreparedExecution {
            account: self.account,
            inner: RawExecution {
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
                BroadcastedTransaction::Invoke(invoke),
                &flags,
            )
            .await
            .map_err(AccountError::Provider)
    }
}

impl RawExecution {
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

impl<'a, A> PreparedExecution<'a, A>
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

impl<'a, A> PreparedExecution<'a, A>
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
            .add_invoke_transaction(tx_request)
            .await
            .map_err(AccountError::Provider)
    }

    // The `simulate` function is temporarily removed until it's supported in [Provider]
    // TODO: add `simulate` back once transaction simulation in supported

    pub async fn get_invoke_request(
        &self,
        query_only: bool,
    ) -> Result<BroadcastedInvokeTransaction, A::SignError> {
        let signature = self.account.sign_execution(&self.inner, query_only).await?;

        Ok(BroadcastedInvokeTransaction::V1(
            BroadcastedInvokeTransactionV1 {
                max_fee: self.inner.max_fee,
                signature,
                nonce: self.inner.nonce,
                sender_address: self.account.address(),
                calldata: self.account.encode_calls(&self.inner.calls),
                is_query: query_only,
            },
        ))
    }
}
