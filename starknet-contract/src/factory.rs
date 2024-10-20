use starknet_accounts::{Account, AccountError, ConnectedAccount, ExecutionV1, ExecutionV3};
use starknet_core::{
    types::{Call, FeeEstimate, Felt, InvokeTransactionResult, SimulatedTransaction},
    utils::{get_udc_deployed_address, UdcUniqueSettings, UdcUniqueness},
};

/// The default UDC address: 0x041a78e741e5af2fec34b695679bc6891742439f7afb8484ecd7766661ad02bf.
const UDC_ADDRESS: Felt = Felt::from_raw([
    121672436446604875,
    9333317513348225193,
    15685625669053253235,
    15144800532519055890,
]);

/// Selector for entrypoint `deployContract`.
const SELECTOR_DEPLOYCONTRACT: Felt = Felt::from_raw([
    469988280392664069,
    1439621915307882061,
    1265649739554438882,
    18249998464715511309,
]);

/// A contract factory that acts as a blueprint for deploying Starknet smart contracts using the
/// Universal Deployer Contract.
#[derive(Debug)]
pub struct ContractFactory<A> {
    class_hash: Felt,
    udc_address: Felt,
    account: A,
}

/// Abstraction over contract deployment via the UDC. This type uses `INVOKE` v1 transactions under
/// the hood, and hence pays transaction fees in ETH. To use v3 transactions for STRK fee payment,
/// use [`DeploymentV3`] instead.
#[must_use]
#[derive(Debug)]
pub struct DeploymentV1<'f, A> {
    factory: &'f ContractFactory<A>,
    constructor_calldata: Vec<Felt>,
    salt: Felt,
    unique: bool,
    // The following fields allow us to mimic an `Execution` API.
    nonce: Option<Felt>,
    max_fee: Option<Felt>,
    fee_estimate_multiplier: f64,
}

/// Abstraction over contract deployment via the UDC. This type uses `INVOKE` v3 transactions under
/// the hood, and hence pays transaction fees in STRK. To use v1 transactions for ETH fee payment,
/// use [`DeploymentV1`] instead.
#[must_use]
#[derive(Debug)]
pub struct DeploymentV3<'f, A> {
    factory: &'f ContractFactory<A>,
    constructor_calldata: Vec<Felt>,
    salt: Felt,
    unique: bool,
    // The following fields allow us to mimic an `Execution` API.
    nonce: Option<Felt>,
    gas: Option<u64>,
    gas_price: Option<u128>,
    gas_estimate_multiplier: f64,
    gas_price_estimate_multiplier: f64,
}

impl<A> ContractFactory<A> {
    /// Constructs a new [`ContractFactory`] from a class hash and an account.
    ///
    /// The [`ContractFactory`] created uses the default address for the Universal Deployer
    /// Contract. To use a custom UDC deployment, use [`new_with_udc`](fn.new_with_udc) instead.
    pub const fn new(class_hash: Felt, account: A) -> Self {
        Self::new_with_udc(class_hash, account, UDC_ADDRESS)
    }

    /// Constructs a new [`ContractFactory`] with a custom Universal Deployer Contract address.
    pub const fn new_with_udc(class_hash: Felt, account: A, udc_address: Felt) -> Self {
        Self {
            class_hash,
            udc_address,
            account,
        }
    }
}

impl<A> ContractFactory<A>
where
    A: Account,
{
    /// Generates an instance of [`DeploymentV1`] for sending `INVOKE` v1 transactions for the
    /// contract deployment. Pays transaction fees in `ETH`.
    pub const fn deploy_v1(
        &self,
        constructor_calldata: Vec<Felt>,
        salt: Felt,
        unique: bool,
    ) -> DeploymentV1<'_, A> {
        DeploymentV1 {
            factory: self,
            constructor_calldata,
            salt,
            unique,
            nonce: None,
            max_fee: None,
            fee_estimate_multiplier: 1.1,
        }
    }

    /// Generates an instance of [`DeploymentV3`] for sending `INVOKE` v3 transactions for the
    /// contract deployment. Pays transaction fees in `STRK`.
    pub const fn deploy_v3(
        &self,
        constructor_calldata: Vec<Felt>,
        salt: Felt,
        unique: bool,
    ) -> DeploymentV3<'_, A> {
        DeploymentV3 {
            factory: self,
            constructor_calldata,
            salt,
            unique,
            nonce: None,
            gas: None,
            gas_price: None,
            gas_estimate_multiplier: 1.5,
            gas_price_estimate_multiplier: 1.5,
        }
    }

    /// Generates an instance of [`DeploymentV1`] for sending `INVOKE` v1 transactions for the
    /// contract deployment. Pays transaction fees in `ETH`.
    #[deprecated = "use version specific variants (`deploy_v1` & `deploy_v3`) instead"]
    pub const fn deploy(
        &self,
        constructor_calldata: Vec<Felt>,
        salt: Felt,
        unique: bool,
    ) -> DeploymentV1<'_, A> {
        self.deploy_v1(constructor_calldata, salt, unique)
    }
}

impl<A> DeploymentV1<'_, A> {
    /// Returns a new [`DeploymentV1`] with the `nonce`.
    pub fn nonce(self, nonce: Felt) -> Self {
        Self {
            nonce: Some(nonce),
            ..self
        }
    }

    /// Returns a new [`DeploymentV1`] with the `max_fee`.
    pub fn max_fee(self, max_fee: Felt) -> Self {
        Self {
            max_fee: Some(max_fee),
            ..self
        }
    }

    /// Returns a new [`DeploymentV1`] with the fee estimate multiplier. The multiplier is used
    /// when transaction fee is not manually specified and must be fetched from a
    /// [`Provider`](starknet_providers::Provider) instead.
    pub fn fee_estimate_multiplier(self, fee_estimate_multiplier: f64) -> Self {
        Self {
            fee_estimate_multiplier,
            ..self
        }
    }
}

impl<A> DeploymentV3<'_, A> {
    /// Returns a new [`DeploymentV3`] with the `nonce`.
    pub fn nonce(self, nonce: Felt) -> Self {
        Self {
            nonce: Some(nonce),
            ..self
        }
    }

    /// Returns a new [`DeploymentV3`] with the `gas`.
    pub fn gas(self, gas: u64) -> Self {
        Self {
            gas: Some(gas),
            ..self
        }
    }

    /// Returns a new [`DeploymentV3`] with the `gas_price`.
    pub fn gas_price(self, gas_price: u128) -> Self {
        Self {
            gas_price: Some(gas_price),
            ..self
        }
    }

    /// Returns a new [`DeploymentV3`] with the gas amount estimate multiplier.  The multiplier is
    /// used when the gas amount is not manually specified and must be fetched from a
    /// [`Provider`](starknet_providers::Provider) instead.
    pub fn gas_estimate_multiplier(self, gas_estimate_multiplier: f64) -> Self {
        Self {
            gas_estimate_multiplier,
            ..self
        }
    }

    /// Returns a new [`DeploymentV3`] with the gas price estimate multiplier.  The multiplier is
    /// used when the gas price is not manually specified and must be fetched from a
    /// [`Provider`](starknet_providers::Provider) instead.
    pub fn gas_price_estimate_multiplier(self, gas_price_estimate_multiplier: f64) -> Self {
        Self {
            gas_price_estimate_multiplier,
            ..self
        }
    }
}

impl<A> DeploymentV1<'_, A>
where
    A: Account,
{
    /// Calculate the resulting contract address without sending a transaction.
    pub fn deployed_address(&self) -> Felt {
        get_udc_deployed_address(
            self.salt,
            self.factory.class_hash,
            &if self.unique {
                UdcUniqueness::Unique(UdcUniqueSettings {
                    deployer_address: self.factory.account.address(),
                    udc_contract_address: self.factory.udc_address,
                })
            } else {
                UdcUniqueness::NotUnique
            },
            &self.constructor_calldata,
        )
    }
}

impl<A> DeploymentV3<'_, A>
where
    A: Account,
{
    /// Calculate the resulting contract address without sending a transaction.
    pub fn deployed_address(&self) -> Felt {
        get_udc_deployed_address(
            self.salt,
            self.factory.class_hash,
            &if self.unique {
                UdcUniqueness::Unique(UdcUniqueSettings {
                    deployer_address: self.factory.account.address(),
                    udc_contract_address: self.factory.udc_address,
                })
            } else {
                UdcUniqueness::NotUnique
            },
            &self.constructor_calldata,
        )
    }
}

impl<'f, A> DeploymentV1<'f, A>
where
    A: ConnectedAccount + Sync,
{
    /// Estimates transaction fees from a [`Provider`](starknet_providers::Provider).
    pub async fn estimate_fee(&self) -> Result<FeeEstimate, AccountError<A::SignError>> {
        let execution: ExecutionV1<'_, A> = self.into();
        execution.estimate_fee().await
    }

    /// Simulates the transaction from a [`Provider`](starknet_providers::Provider). Transaction
    /// validation and fee transfer can be skipped.
    pub async fn simulate(
        &self,
        skip_validate: bool,
        skip_fee_charge: bool,
    ) -> Result<SimulatedTransaction, AccountError<A::SignError>> {
        let execution: ExecutionV1<'_, A> = self.into();
        execution.simulate(skip_validate, skip_fee_charge).await
    }

    /// Signs and broadcasts the transaction to the network.
    pub async fn send(&self) -> Result<InvokeTransactionResult, AccountError<A::SignError>> {
        let execution: ExecutionV1<'_, A> = self.into();
        execution.send().await
    }
}

impl<'f, A> DeploymentV3<'f, A>
where
    A: ConnectedAccount + Sync,
{
    /// Estimates transaction fees from a [`Provider`](starknet_providers::Provider).
    pub async fn estimate_fee(&self) -> Result<FeeEstimate, AccountError<A::SignError>> {
        let execution: ExecutionV3<'_, A> = self.into();
        execution.estimate_fee().await
    }

    /// Simulates the transaction from a [`Provider`](starknet_providers::Provider). Transaction
    /// validation and fee transfer can be skipped.
    pub async fn simulate(
        &self,
        skip_validate: bool,
        skip_fee_charge: bool,
    ) -> Result<SimulatedTransaction, AccountError<A::SignError>> {
        let execution: ExecutionV3<'_, A> = self.into();
        execution.simulate(skip_validate, skip_fee_charge).await
    }

    /// Signs and broadcasts the transaction to the network.
    pub async fn send(&self) -> Result<InvokeTransactionResult, AccountError<A::SignError>> {
        let execution: ExecutionV3<'_, A> = self.into();
        execution.send().await
    }
}

impl<'f, A> From<&DeploymentV1<'f, A>> for ExecutionV1<'f, A> {
    fn from(value: &DeploymentV1<'f, A>) -> Self {
        let mut calldata = vec![
            value.factory.class_hash,
            value.salt,
            if value.unique { Felt::ONE } else { Felt::ZERO },
            value.constructor_calldata.len().into(),
        ];
        calldata.extend_from_slice(&value.constructor_calldata);

        let execution = Self::new(
            vec![Call {
                to: value.factory.udc_address,
                selector: SELECTOR_DEPLOYCONTRACT,
                calldata,
            }],
            &value.factory.account,
        );

        let execution = if let Some(nonce) = value.nonce {
            execution.nonce(nonce)
        } else {
            execution
        };

        let execution = if let Some(max_fee) = value.max_fee {
            execution.max_fee(max_fee)
        } else {
            execution
        };

        execution.fee_estimate_multiplier(value.fee_estimate_multiplier)
    }
}

impl<'f, A> From<&DeploymentV3<'f, A>> for ExecutionV3<'f, A> {
    fn from(value: &DeploymentV3<'f, A>) -> Self {
        let mut calldata = vec![
            value.factory.class_hash,
            value.salt,
            if value.unique { Felt::ONE } else { Felt::ZERO },
            value.constructor_calldata.len().into(),
        ];
        calldata.extend_from_slice(&value.constructor_calldata);

        let execution = Self::new(
            vec![Call {
                to: value.factory.udc_address,
                selector: SELECTOR_DEPLOYCONTRACT,
                calldata,
            }],
            &value.factory.account,
        );

        let execution = if let Some(nonce) = value.nonce {
            execution.nonce(nonce)
        } else {
            execution
        };

        let execution = if let Some(gas) = value.gas {
            execution.gas(gas)
        } else {
            execution
        };

        let execution = if let Some(gas_price) = value.gas_price {
            execution.gas_price(gas_price)
        } else {
            execution
        };

        let execution = execution.gas_estimate_multiplier(value.gas_estimate_multiplier);

        execution.gas_price_estimate_multiplier(value.gas_price_estimate_multiplier)
    }
}
#[cfg(test)]
mod tests {
    use starknet_accounts::{ExecutionEncoding, SingleOwnerAccount};
    use starknet_core::chain_id;
    use starknet_providers::SequencerGatewayProvider;
    use starknet_signers::{LocalWallet, SigningKey};

    use super::*;

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_deployed_address_unique() {
        let factory = ContractFactory::new(
            Felt::from_hex("0x2bfd9564754d9b4a326da62b2f22b8fea7bbeffd62da4fcaea986c323b7aeb")
                .unwrap(),
            SingleOwnerAccount::new(
                SequencerGatewayProvider::starknet_alpha_sepolia(),
                LocalWallet::from_signing_key(SigningKey::from_random()),
                Felt::from_hex("0xb1461de04c6a1aa3375bdf9b7723a8779c082ffe21311d683a0b15c078b5dc")
                    .unwrap(),
                chain_id::SEPOLIA,
                ExecutionEncoding::Legacy,
            ),
        );

        let unique_address_v1 = factory
            .deploy_v1(
                vec![Felt::from_hex("0x1234").unwrap()],
                Felt::from_hex("0x3456").unwrap(),
                true,
            )
            .deployed_address();
        let unique_address_v3 = factory
            .deploy_v3(
                vec![Felt::from_hex("0x1234").unwrap()],
                Felt::from_hex("0x3456").unwrap(),
                true,
            )
            .deployed_address();

        let not_unique_address_v1 = factory
            .deploy_v1(
                vec![Felt::from_hex("0x1234").unwrap()],
                Felt::from_hex("0x3456").unwrap(),
                false,
            )
            .deployed_address();
        let not_unique_address_v3 = factory
            .deploy_v3(
                vec![Felt::from_hex("0x1234").unwrap()],
                Felt::from_hex("0x3456").unwrap(),
                false,
            )
            .deployed_address();

        assert_eq!(
            unique_address_v1,
            Felt::from_hex("0x36e05bcd41191387bc2f04ed9cad4776a75df3b748b0246a5d217a988474181")
                .unwrap()
        );
        assert_eq!(
            unique_address_v3,
            Felt::from_hex("0x36e05bcd41191387bc2f04ed9cad4776a75df3b748b0246a5d217a988474181")
                .unwrap()
        );

        assert_eq!(
            not_unique_address_v1,
            Felt::from_hex("0x3a320b6aa0b451b22fba90b5d75b943932649137c09a86a5cf4853031be70c1")
                .unwrap()
        );
        assert_eq!(
            not_unique_address_v3,
            Felt::from_hex("0x3a320b6aa0b451b22fba90b5d75b943932649137c09a86a5cf4853031be70c1")
                .unwrap()
        );
    }
}
