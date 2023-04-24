use super::{
    super::NotPreparedError, Account, AccountError, ConnectedAccount, Execution, PreparedExecution,
    RawExecution,
};
use crate::Call;

use starknet_core::{
    crypto::compute_hash_on_elements,
    types::{
        AccountTransaction, AddTransactionResult, FeeEstimate, FieldElement,
        InvokeFunctionTransactionRequest, TransactionRequest, TransactionSimulationInfo,
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
    pub async fn estimate_fee(
        &self,
    ) -> Result<FeeEstimate, AccountError<A::SignError, <A::Provider as Provider>::Error>> {
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
    ) -> Result<
        TransactionSimulationInfo,
        AccountError<A::SignError, <A::Provider as Provider>::Error>,
    > {
        self.prepare().await?.simulate().await
    }

    pub async fn send(
        &self,
    ) -> Result<AddTransactionResult, AccountError<A::SignError, <A::Provider as Provider>::Error>>
    {
        self.prepare().await?.send().await
    }

    async fn prepare(
        &self,
    ) -> Result<
        PreparedExecution<'a, A>,
        AccountError<A::SignError, <A::Provider as Provider>::Error>,
    > {
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
                ((fee_estimate.overall_fee as f64 * self.fee_estimate_multiplier) as u64).into()
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
    ) -> Result<FeeEstimate, AccountError<A::SignError, <A::Provider as Provider>::Error>> {
        let prepared = PreparedExecution {
            account: self.account,
            inner: RawExecution {
                calls: self.calls.clone(),
                nonce,
                max_fee: FieldElement::ZERO,
            },
        };
        let invoke = prepared
            .get_invoke_request()
            .await
            .map_err(AccountError::Signing)?;

        self.account
            .provider()
            .estimate_fee(
                AccountTransaction::InvokeFunction(invoke),
                self.account.block_id(),
                false,
            )
            .await
            .map_err(AccountError::Provider)
    }
}

impl RawExecution {
    pub fn raw_calldata(&self) -> Vec<FieldElement> {
        let mut concated_calldata: Vec<FieldElement> = vec![];
        let mut execute_calldata: Vec<FieldElement> = vec![self.calls.len().into()];
        for call in self.calls.iter() {
            execute_calldata.push(call.to); // to
            execute_calldata.push(call.selector); // selector
            execute_calldata.push(concated_calldata.len().into()); // data_offset
            execute_calldata.push(call.calldata.len().into()); // data_len

            for item in call.calldata.iter() {
                concated_calldata.push(*item);
            }
        }
        execute_calldata.push(concated_calldata.len().into()); // calldata_len
        for item in concated_calldata.into_iter() {
            execute_calldata.push(item); // calldata
        }

        execute_calldata
    }

    pub fn transaction_hash(&self, chain_id: FieldElement, address: FieldElement) -> FieldElement {
        compute_hash_on_elements(&[
            PREFIX_INVOKE,
            FieldElement::ONE, // version
            address,
            FieldElement::ZERO, // entry_point_selector
            compute_hash_on_elements(&self.raw_calldata()),
            self.max_fee,
            chain_id,
            self.nonce,
        ])
    }
}

impl<'a, A> PreparedExecution<'a, A> {
    pub fn raw_calldata(&self) -> Vec<FieldElement> {
        self.inner.raw_calldata()
    }
}

impl<'a, A> PreparedExecution<'a, A>
where
    A: Account,
{
    /// Locally calculates the hash of the transaction to be sent from this execution given the
    /// parameters.
    pub fn transaction_hash(&self) -> FieldElement {
        self.inner
            .transaction_hash(self.account.chain_id(), self.account.address())
    }
}

impl<'a, A> PreparedExecution<'a, A>
where
    A: ConnectedAccount,
{
    pub async fn send(
        &self,
    ) -> Result<AddTransactionResult, AccountError<A::SignError, <A::Provider as Provider>::Error>>
    {
        let tx_request = self
            .get_invoke_request()
            .await
            .map_err(AccountError::Signing)?;
        self.account
            .provider()
            .add_transaction(TransactionRequest::InvokeFunction(tx_request))
            .await
            .map_err(AccountError::Provider)
    }

    pub async fn simulate(
        &self,
    ) -> Result<
        TransactionSimulationInfo,
        AccountError<A::SignError, <A::Provider as Provider>::Error>,
    > {
        let tx_request = self
            .get_invoke_request()
            .await
            .map_err(AccountError::Signing)?;
        self.account
            .provider()
            .simulate_transaction(
                AccountTransaction::InvokeFunction(tx_request),
                self.account.block_id(),
                false,
            )
            .await
            .map_err(AccountError::Provider)
    }

    pub async fn get_invoke_request(
        &self,
    ) -> Result<InvokeFunctionTransactionRequest, A::SignError> {
        let signature = self.account.sign_execution(&self.inner).await?;

        Ok(InvokeFunctionTransactionRequest {
            sender_address: self.account.address(),
            calldata: self.raw_calldata(),
            signature,
            max_fee: self.inner.max_fee,
            nonce: self.inner.nonce,
        })
    }
}
