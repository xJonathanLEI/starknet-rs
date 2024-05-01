use super::{
    super::NotPreparedError, Account, AccountError, ConnectedAccount, DeclarationV2, DeclarationV3,
    LegacyDeclaration, PreparedDeclarationV2, PreparedDeclarationV3, PreparedLegacyDeclaration,
    RawDeclarationV2, RawDeclarationV3, RawLegacyDeclaration,
};

use starknet_core::{
    crypto::compute_hash_on_elements,
    types::{
        contract::{legacy::LegacyContractClass, ComputeClassHashError},
        BroadcastedDeclareTransaction, BroadcastedDeclareTransactionV1,
        BroadcastedDeclareTransactionV2, BroadcastedDeclareTransactionV3, BroadcastedTransaction,
        DataAvailabilityMode, DeclareTransactionResult, FeeEstimate, FieldElement,
        FlattenedSierraClass, ResourceBounds, ResourceBoundsMapping, SimulatedTransaction,
        SimulationFlag,
    },
};
use starknet_crypto::PoseidonHasher;
use starknet_providers::Provider;
use std::sync::Arc;

/// Cairo string for "declare"
const PREFIX_DECLARE: FieldElement = FieldElement::from_mont([
    17542456862011667323,
    18446744073709551615,
    18446744073709551615,
    191557713328401194,
]);

/// 2 ^ 128 + 1
const QUERY_VERSION_ONE: FieldElement = FieldElement::from_mont([
    18446744073700081633,
    17407,
    18446744073709551584,
    576460752142433776,
]);

/// 2 ^ 128 + 2
const QUERY_VERSION_TWO: FieldElement = FieldElement::from_mont([
    18446744073700081601,
    17407,
    18446744073709551584,
    576460752142433232,
]);

/// 2 ^ 128 + 3
const QUERY_VERSION_THREE: FieldElement = FieldElement::from_mont([
    18446744073700081569,
    17407,
    18446744073709551584,
    576460752142432688,
]);

impl<'a, A> DeclarationV2<'a, A> {
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

    /// Calling this function after manually specifying `nonce` and `max_fee` turns [DeclarationV2]
    /// into [PreparedDeclarationV2]. Returns `Err` if either field is `None`.
    pub fn prepared(self) -> Result<PreparedDeclarationV2<'a, A>, NotPreparedError> {
        let nonce = self.nonce.ok_or(NotPreparedError)?;
        let max_fee = self.max_fee.ok_or(NotPreparedError)?;

        Ok(PreparedDeclarationV2 {
            account: self.account,
            inner: RawDeclarationV2 {
                contract_class: self.contract_class,
                compiled_class_hash: self.compiled_class_hash,
                nonce,
                max_fee,
            },
        })
    }
}

impl<'a, A> DeclarationV2<'a, A>
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

    pub async fn send(&self) -> Result<DeclareTransactionResult, AccountError<A::SignError>> {
        self.prepare().await?.send().await
    }

    async fn prepare(&self) -> Result<PreparedDeclarationV2<'a, A>, AccountError<A::SignError>> {
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

        Ok(PreparedDeclarationV2 {
            account: self.account,
            inner: RawDeclarationV2 {
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
    ) -> Result<FeeEstimate, AccountError<A::SignError>> {
        let prepared = PreparedDeclarationV2 {
            account: self.account,
            inner: RawDeclarationV2 {
                contract_class: self.contract_class.clone(),
                compiled_class_hash: self.compiled_class_hash,
                nonce,
                max_fee: FieldElement::ZERO,
            },
        };
        let declare = prepared.get_declare_request(true).await?;

        self.account
            .provider()
            .estimate_fee_single(
                BroadcastedTransaction::Declare(BroadcastedDeclareTransaction::V2(declare)),
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
        let prepared = PreparedDeclarationV2 {
            account: self.account,
            inner: RawDeclarationV2 {
                contract_class: self.contract_class.clone(),
                compiled_class_hash: self.compiled_class_hash,
                nonce,
                max_fee: self.max_fee.unwrap_or_default(),
            },
        };
        let declare = prepared.get_declare_request(true).await?;

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
                BroadcastedTransaction::Declare(BroadcastedDeclareTransaction::V2(declare)),
                &flags,
            )
            .await
            .map_err(AccountError::Provider)
    }
}

impl<'a, A> DeclarationV3<'a, A> {
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
    /// [DeclarationV3] into [PreparedDeclarationV3]. Returns `Err` if any field is `None`.
    pub fn prepared(self) -> Result<PreparedDeclarationV3<'a, A>, NotPreparedError> {
        let nonce = self.nonce.ok_or(NotPreparedError)?;
        let gas = self.gas.ok_or(NotPreparedError)?;
        let gas_price = self.gas_price.ok_or(NotPreparedError)?;

        Ok(PreparedDeclarationV3 {
            account: self.account,
            inner: RawDeclarationV3 {
                contract_class: self.contract_class,
                compiled_class_hash: self.compiled_class_hash,
                nonce,
                gas,
                gas_price,
            },
        })
    }
}

impl<'a, A> DeclarationV3<'a, A>
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
        let (gas, gas_price) = match (self.gas, self.gas_price) {
            (Some(gas), Some(gas_price)) => (gas, gas_price),
            (Some(gas), _) => {
                // When `gas` is specified, we only need the L1 gas price in FRI. By specifying a
                // a `gas` value, the user might be trying to avoid a full fee estimation (e.g.
                // flaky dependencies), so it's in appropriate to call `estimate_fee` here.

                // This is the lightest-weight block we can get
                let block_l1_gas_price = self
                    .account
                    .provider()
                    .get_block_with_tx_hashes(self.account.block_id())
                    .await
                    .map_err(AccountError::Provider)?
                    .l1_gas_price()
                    .price_in_fri;

                let gas_price = (((TryInto::<u64>::try_into(block_l1_gas_price)
                    .map_err(|_| AccountError::FeeOutOfRange)?)
                    as f64)
                    * self.gas_price_estimate_multiplier) as u128;

                (gas, gas_price)
            }
            // We have to perform fee estimation as long as gas is not specified
            _ => {
                let fee_estimate = self.estimate_fee_with_nonce(nonce).await?;

                let gas = match self.gas {
                    Some(gas) => gas,
                    None => {
                        (((TryInto::<u64>::try_into(
                            (fee_estimate.overall_fee + fee_estimate.gas_price - FieldElement::ONE)
                                .floor_div(fee_estimate.gas_price),
                        )
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

        Ok(PreparedDeclarationV3 {
            account: self.account,
            inner: RawDeclarationV3 {
                contract_class: self.contract_class.clone(),
                compiled_class_hash: self.compiled_class_hash,
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
        let prepared = PreparedDeclarationV3 {
            account: self.account,
            inner: RawDeclarationV3 {
                contract_class: self.contract_class.clone(),
                compiled_class_hash: self.compiled_class_hash,
                nonce,
                gas: 0,
                gas_price: 0,
            },
        };
        let declare = prepared.get_declare_request(true).await?;

        self.account
            .provider()
            .estimate_fee_single(
                BroadcastedTransaction::Declare(BroadcastedDeclareTransaction::V3(declare)),
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
        let prepared = PreparedDeclarationV3 {
            account: self.account,
            inner: RawDeclarationV3 {
                contract_class: self.contract_class.clone(),
                compiled_class_hash: self.compiled_class_hash,
                nonce,
                gas: self.gas.unwrap_or_default(),
                gas_price: self.gas_price.unwrap_or_default(),
            },
        };
        let declare = prepared.get_declare_request(true).await?;

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
                BroadcastedTransaction::Declare(BroadcastedDeclareTransaction::V3(declare)),
                &flags,
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

    pub async fn send(&self) -> Result<DeclareTransactionResult, AccountError<A::SignError>> {
        self.prepare().await?.send().await
    }

    async fn prepare(
        &self,
    ) -> Result<PreparedLegacyDeclaration<'a, A>, AccountError<A::SignError>> {
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
    ) -> Result<FeeEstimate, AccountError<A::SignError>> {
        let prepared = PreparedLegacyDeclaration {
            account: self.account,
            inner: RawLegacyDeclaration {
                contract_class: self.contract_class.clone(),
                nonce,
                max_fee: FieldElement::ZERO,
            },
        };
        let declare = prepared.get_declare_request(true).await?;

        self.account
            .provider()
            .estimate_fee_single(
                BroadcastedTransaction::Declare(BroadcastedDeclareTransaction::V1(declare)),
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
        let prepared = PreparedLegacyDeclaration {
            account: self.account,
            inner: RawLegacyDeclaration {
                contract_class: self.contract_class.clone(),
                nonce,
                max_fee: self.max_fee.unwrap_or_default(),
            },
        };
        let declare = prepared.get_declare_request(true).await?;

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
                BroadcastedTransaction::Declare(BroadcastedDeclareTransaction::V1(declare)),
                &flags,
            )
            .await
            .map_err(AccountError::Provider)
    }
}

impl RawDeclarationV2 {
    pub fn transaction_hash(
        &self,
        chain_id: FieldElement,
        address: FieldElement,
        query_only: bool,
    ) -> FieldElement {
        compute_hash_on_elements(&[
            PREFIX_DECLARE,
            if query_only {
                QUERY_VERSION_TWO
            } else {
                FieldElement::TWO
            }, // version
            address,
            FieldElement::ZERO, // entry_point_selector
            compute_hash_on_elements(&[self.contract_class.class_hash()]),
            self.max_fee,
            chain_id,
            self.nonce,
            self.compiled_class_hash,
        ])
    }

    pub fn contract_class(&self) -> &FlattenedSierraClass {
        &self.contract_class
    }

    pub fn compiled_class_hash(&self) -> FieldElement {
        self.compiled_class_hash
    }

    pub fn nonce(&self) -> FieldElement {
        self.nonce
    }

    pub fn max_fee(&self) -> FieldElement {
        self.max_fee
    }
}

impl RawDeclarationV3 {
    pub fn transaction_hash(
        &self,
        chain_id: FieldElement,
        address: FieldElement,
        query_only: bool,
    ) -> FieldElement {
        let mut hasher = PoseidonHasher::new();

        hasher.update(PREFIX_DECLARE);
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

        hasher.update(self.contract_class.class_hash());
        hasher.update(self.compiled_class_hash);

        hasher.finalize()
    }

    pub fn contract_class(&self) -> &FlattenedSierraClass {
        &self.contract_class
    }

    pub fn compiled_class_hash(&self) -> FieldElement {
        self.compiled_class_hash
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

impl RawLegacyDeclaration {
    pub fn transaction_hash(
        &self,
        chain_id: FieldElement,
        address: FieldElement,
        query_only: bool,
    ) -> Result<FieldElement, ComputeClassHashError> {
        Ok(compute_hash_on_elements(&[
            PREFIX_DECLARE,
            if query_only {
                QUERY_VERSION_ONE
            } else {
                FieldElement::ONE
            }, // version
            address,
            FieldElement::ZERO, // entry_point_selector
            compute_hash_on_elements(&[self.contract_class.class_hash()?]),
            self.max_fee,
            chain_id,
            self.nonce,
        ]))
    }

    pub fn contract_class(&self) -> &LegacyContractClass {
        &self.contract_class
    }

    pub fn nonce(&self) -> FieldElement {
        self.nonce
    }

    pub fn max_fee(&self) -> FieldElement {
        self.max_fee
    }
}

impl<'a, A> PreparedDeclarationV2<'a, A>
where
    A: Account,
{
    /// Locally calculates the hash of the transaction to be sent from this declaration given the
    /// parameters.
    pub fn transaction_hash(&self, query_only: bool) -> FieldElement {
        self.inner
            .transaction_hash(self.account.chain_id(), self.account.address(), query_only)
    }
}

impl<'a, A> PreparedDeclarationV2<'a, A>
where
    A: ConnectedAccount,
{
    pub async fn send(&self) -> Result<DeclareTransactionResult, AccountError<A::SignError>> {
        let tx_request = self.get_declare_request(false).await?;
        self.account
            .provider()
            .add_declare_transaction(BroadcastedDeclareTransaction::V2(tx_request))
            .await
            .map_err(AccountError::Provider)
    }

    pub async fn get_declare_request(
        &self,
        query_only: bool,
    ) -> Result<BroadcastedDeclareTransactionV2, AccountError<A::SignError>> {
        let signature = self
            .account
            .sign_declaration_v2(&self.inner, query_only)
            .await
            .map_err(AccountError::Signing)?;

        Ok(BroadcastedDeclareTransactionV2 {
            max_fee: self.inner.max_fee,
            signature,
            nonce: self.inner.nonce,
            contract_class: self.inner.contract_class.clone(),
            compiled_class_hash: self.inner.compiled_class_hash,
            sender_address: self.account.address(),
            is_query: query_only,
        })
    }
}

impl<'a, A> PreparedDeclarationV3<'a, A>
where
    A: Account,
{
    /// Locally calculates the hash of the transaction to be sent from this declaration given the
    /// parameters.
    pub fn transaction_hash(&self, query_only: bool) -> FieldElement {
        self.inner
            .transaction_hash(self.account.chain_id(), self.account.address(), query_only)
    }
}

impl<'a, A> PreparedDeclarationV3<'a, A>
where
    A: ConnectedAccount,
{
    pub async fn send(&self) -> Result<DeclareTransactionResult, AccountError<A::SignError>> {
        let tx_request = self.get_declare_request(false).await?;
        self.account
            .provider()
            .add_declare_transaction(BroadcastedDeclareTransaction::V3(tx_request))
            .await
            .map_err(AccountError::Provider)
    }

    pub async fn get_declare_request(
        &self,
        query_only: bool,
    ) -> Result<BroadcastedDeclareTransactionV3, AccountError<A::SignError>> {
        let signature = self
            .account
            .sign_declaration_v3(&self.inner, query_only)
            .await
            .map_err(AccountError::Signing)?;

        Ok(BroadcastedDeclareTransactionV3 {
            sender_address: self.account.address(),
            compiled_class_hash: self.inner.compiled_class_hash,
            signature,
            nonce: self.inner.nonce,
            contract_class: self.inner.contract_class.clone(),
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

impl<'a, A> PreparedLegacyDeclaration<'a, A>
where
    A: Account,
{
    /// Locally calculates the hash of the transaction to be sent from this declaration given the
    /// parameters.
    pub fn transaction_hash(
        &self,
        query_only: bool,
    ) -> Result<FieldElement, ComputeClassHashError> {
        self.inner
            .transaction_hash(self.account.chain_id(), self.account.address(), query_only)
    }
}

impl<'a, A> PreparedLegacyDeclaration<'a, A>
where
    A: ConnectedAccount,
{
    pub async fn send(&self) -> Result<DeclareTransactionResult, AccountError<A::SignError>> {
        let tx_request = self.get_declare_request(false).await?;
        self.account
            .provider()
            .add_declare_transaction(BroadcastedDeclareTransaction::V1(tx_request))
            .await
            .map_err(AccountError::Provider)
    }

    pub async fn get_declare_request(
        &self,
        query_only: bool,
    ) -> Result<BroadcastedDeclareTransactionV1, AccountError<A::SignError>> {
        let signature = self
            .account
            .sign_legacy_declaration(&self.inner, query_only)
            .await
            .map_err(AccountError::Signing)?;

        let compressed_class = self.inner.contract_class.compress().unwrap();

        Ok(BroadcastedDeclareTransactionV1 {
            max_fee: self.inner.max_fee,
            signature,
            nonce: self.inner.nonce,
            contract_class: Arc::new(compressed_class),
            sender_address: self.account.address(),
            is_query: query_only,
        })
    }
}
