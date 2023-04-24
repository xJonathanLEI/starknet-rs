use super::{
    super::NotPreparedError, Account, AccountError, ConnectedAccount, Declaration,
    LegacyDeclaration, PreparedDeclaration, PreparedLegacyDeclaration, RawDeclaration,
    RawLegacyDeclaration,
};

use starknet_core::{
    crypto::compute_hash_on_elements,
    types::{
        contract::{legacy::LegacyContractClass, ComputeClassHashError, FlattenedSierraClass},
        AccountTransaction, AddTransactionResult, DeclareTransactionRequest,
        DeclareV1TransactionRequest, DeclareV2TransactionRequest, FeeEstimate, FieldElement,
        TransactionRequest,
    },
};
use starknet_providers::Provider;
use std::sync::Arc;

/// Cairo string for "declare"
const PREFIX_DECLARE: FieldElement = FieldElement::from_mont([
    17542456862011667323,
    18446744073709551615,
    18446744073709551615,
    191557713328401194,
]);

impl<'a, A> Declaration<'a, A> {
    pub fn new(
        contract_class: Arc<FlattenedSierraClass>,
        compiled_class_hash: FieldElement,
        account: &'a A,
    ) -> Self {
        Self {
            account,
            contract_class,
            compiled_class_hash,
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

    /// Calling this function after manually specifying `nonce` and `max_fee` turns [Declaration]
    /// into [PreparedDeclaration]. Returns `Err` if either field is `None`.
    pub fn prepared(self) -> Result<PreparedDeclaration<'a, A>, NotPreparedError> {
        let nonce = self.nonce.ok_or(NotPreparedError)?;
        let max_fee = self.max_fee.ok_or(NotPreparedError)?;

        Ok(PreparedDeclaration {
            account: self.account,
            inner: RawDeclaration {
                contract_class: self.contract_class,
                compiled_class_hash: self.compiled_class_hash,
                nonce,
                max_fee,
            },
        })
    }
}

impl<'a, A> Declaration<'a, A>
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

    pub async fn send(
        &self,
    ) -> Result<AddTransactionResult, AccountError<A::SignError, <A::Provider as Provider>::Error>>
    {
        self.prepare().await?.send().await
    }

    async fn prepare(
        &self,
    ) -> Result<
        PreparedDeclaration<'a, A>,
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

        Ok(PreparedDeclaration {
            account: self.account,
            inner: RawDeclaration {
                contract_class: self.contract_class.clone(),
                compiled_class_hash: self.compiled_class_hash,
                nonce,
                max_fee,
            },
        })
    }

    async fn estimate_fee_with_nonce(
        &self,
        nonce: FieldElement,
    ) -> Result<FeeEstimate, AccountError<A::SignError, <A::Provider as Provider>::Error>> {
        let prepared = PreparedDeclaration {
            account: self.account,
            inner: RawDeclaration {
                contract_class: self.contract_class.clone(),
                compiled_class_hash: self.compiled_class_hash,
                nonce,
                max_fee: FieldElement::ZERO,
            },
        };
        let declare = prepared.get_declare_request().await?;

        self.account
            .provider()
            .estimate_fee(
                AccountTransaction::Declare(declare),
                self.account.block_id(),
                false,
            )
            .await
            .map_err(AccountError::Provider)
    }
}

impl<'a, A> LegacyDeclaration<'a, A> {
    pub fn new(contract_class: Arc<LegacyContractClass>, account: &'a A) -> Self {
        Self {
            account,
            contract_class,
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
    /// [LegacyDeclaration] into [PreparedLegacyDeclaration]. Returns `Err` if either field is
    /// `None`.
    pub fn prepared(self) -> Result<PreparedLegacyDeclaration<'a, A>, NotPreparedError> {
        let nonce = self.nonce.ok_or(NotPreparedError)?;
        let max_fee = self.max_fee.ok_or(NotPreparedError)?;

        Ok(PreparedLegacyDeclaration {
            account: self.account,
            inner: RawLegacyDeclaration {
                contract_class: self.contract_class,
                nonce,
                max_fee,
            },
        })
    }
}

impl<'a, A> LegacyDeclaration<'a, A>
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

    pub async fn send(
        &self,
    ) -> Result<AddTransactionResult, AccountError<A::SignError, <A::Provider as Provider>::Error>>
    {
        self.prepare().await?.send().await
    }

    async fn prepare(
        &self,
    ) -> Result<
        PreparedLegacyDeclaration<'a, A>,
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

        Ok(PreparedLegacyDeclaration {
            account: self.account,
            inner: RawLegacyDeclaration {
                contract_class: self.contract_class.clone(),
                nonce,
                max_fee,
            },
        })
    }

    async fn estimate_fee_with_nonce(
        &self,
        nonce: FieldElement,
    ) -> Result<FeeEstimate, AccountError<A::SignError, <A::Provider as Provider>::Error>> {
        let prepared = PreparedLegacyDeclaration {
            account: self.account,
            inner: RawLegacyDeclaration {
                contract_class: self.contract_class.clone(),
                nonce,
                max_fee: FieldElement::ZERO,
            },
        };
        let declare = prepared.get_declare_request().await?;

        self.account
            .provider()
            .estimate_fee(
                AccountTransaction::Declare(declare),
                self.account.block_id(),
                false,
            )
            .await
            .map_err(AccountError::Provider)
    }
}

impl RawDeclaration {
    pub fn transaction_hash(&self, chain_id: FieldElement, address: FieldElement) -> FieldElement {
        compute_hash_on_elements(&[
            PREFIX_DECLARE,
            FieldElement::TWO, // version
            address,
            FieldElement::ZERO, // entry_point_selector
            compute_hash_on_elements(&[self.contract_class.class_hash()]),
            self.max_fee,
            chain_id,
            self.nonce,
            self.compiled_class_hash,
        ])
    }
}

impl RawLegacyDeclaration {
    pub fn transaction_hash(
        &self,
        chain_id: FieldElement,
        address: FieldElement,
    ) -> Result<FieldElement, ComputeClassHashError> {
        Ok(compute_hash_on_elements(&[
            PREFIX_DECLARE,
            FieldElement::ONE, // version
            address,
            FieldElement::ZERO, // entry_point_selector
            compute_hash_on_elements(&[self.contract_class.class_hash()?]),
            self.max_fee,
            chain_id,
            self.nonce,
        ]))
    }
}

impl<'a, A> PreparedDeclaration<'a, A>
where
    A: Account,
{
    /// Locally calculates the hash of the transaction to be sent from this declaration given the
    /// parameters.
    pub fn transaction_hash(&self) -> FieldElement {
        self.inner
            .transaction_hash(self.account.chain_id(), self.account.address())
    }
}

impl<'a, A> PreparedDeclaration<'a, A>
where
    A: ConnectedAccount,
{
    pub async fn send(
        &self,
    ) -> Result<AddTransactionResult, AccountError<A::SignError, <A::Provider as Provider>::Error>>
    {
        let tx_request = self.get_declare_request().await?;
        self.account
            .provider()
            .add_transaction(TransactionRequest::Declare(tx_request))
            .await
            .map_err(AccountError::Provider)
    }

    pub async fn get_declare_request(
        &self,
    ) -> Result<
        DeclareTransactionRequest,
        AccountError<A::SignError, <A::Provider as Provider>::Error>,
    > {
        let signature = self
            .account
            .sign_declaration(&self.inner)
            .await
            .map_err(AccountError::Signing)?;

        let compressed_class = self.inner.contract_class.compress().unwrap();

        Ok(DeclareTransactionRequest::V2(DeclareV2TransactionRequest {
            contract_class: Arc::new(compressed_class),
            compiled_class_hash: self.inner.compiled_class_hash,
            sender_address: self.account.address(),
            max_fee: self.inner.max_fee,
            signature,
            nonce: self.inner.nonce,
        }))
    }
}

impl<'a, A> PreparedLegacyDeclaration<'a, A>
where
    A: Account,
{
    /// Locally calculates the hash of the transaction to be sent from this declaration given the
    /// parameters.
    pub fn transaction_hash(&self) -> Result<FieldElement, ComputeClassHashError> {
        self.inner
            .transaction_hash(self.account.chain_id(), self.account.address())
    }
}

impl<'a, A> PreparedLegacyDeclaration<'a, A>
where
    A: ConnectedAccount,
{
    pub async fn send(
        &self,
    ) -> Result<AddTransactionResult, AccountError<A::SignError, <A::Provider as Provider>::Error>>
    {
        let tx_request = self.get_declare_request().await?;
        self.account
            .provider()
            .add_transaction(TransactionRequest::Declare(tx_request))
            .await
            .map_err(AccountError::Provider)
    }

    pub async fn get_declare_request(
        &self,
    ) -> Result<
        DeclareTransactionRequest,
        AccountError<A::SignError, <A::Provider as Provider>::Error>,
    > {
        let signature = self
            .account
            .sign_legacy_declaration(&self.inner)
            .await
            .map_err(AccountError::Signing)?;

        let compressed_class = self.inner.contract_class.compress().unwrap();

        Ok(DeclareTransactionRequest::V1(DeclareV1TransactionRequest {
            contract_class: Arc::new(compressed_class),
            sender_address: self.account.address(),
            max_fee: self.inner.max_fee,
            signature,
            nonce: self.inner.nonce,
        }))
    }
}
