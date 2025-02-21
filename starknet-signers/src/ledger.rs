use async_trait::async_trait;
use coins_ledger::{
    common::{APDUData, APDUResponseCodes},
    transports::LedgerAsync,
    APDUAnswer, APDUCommand, Ledger,
};
use crypto_bigint::{ArrayEncoding, U256};
use semver::Version;
use starknet_core::{crypto::Signature, types::Felt};

use crate::{Signer, SignerInteractivityContext, VerifyingKey};

pub use coins_bip32::path::DerivationPath;

/// The Ledger application identifier for app-starknet.
const CLA_STARKNET: u8 = 0x5a;

/// BIP-32 encoding of `2645'`
const EIP_2645_PURPOSE: u32 = 0x80000a55;

const EIP_2645_PATH_LENGTH: usize = 6;

const VERSION_SIZE: usize = 3;
const PUBLIC_KEY_SIZE: usize = 65;
const SIGNATURE_SIZE: usize = 65;

/// Ledger app wrapper that implements the [`Signer`] trait.
#[derive(Debug)]
pub struct LedgerSigner {
    app: LedgerStarknetApp,
    derivation_path: DerivationPath,
}

/// A handle for communicating with the Ledger Starknet app.
#[derive(Debug)]
pub struct LedgerStarknetApp {
    transport: Ledger,
}

/// Errors using the Ledger hardware wallet.
#[derive(Debug, thiserror::Error)]
pub enum LedgerError {
    /// The HD wallet derivation path is malformed or does not conform to EIP-2645.
    #[error("derivation path is empty, not prefixed with m/2645', or is not 6-level long")]
    InvalidDerivationPath,
    /// Error communicating with the Ledger hardware device.
    #[error(transparent)]
    TransportError(coins_ledger::LedgerError),
    /// An unknown response code is returned from the device.
    #[error("unknown response code from Ledger: {0}")]
    UnknownResponseCode(u16),
    /// The response code returned from the device does not indicate success.
    #[error("failed Ledger request: {0}")]
    UnsuccessfulRequest(APDUResponseCodes),
    /// The response has an unexpected size.
    #[error("unexpected response length - expected: {expected}; actual: {actual}")]
    UnexpectedResponseLength {
        /// The expected response size.
        expected: usize,
        /// The actual response size.
        actual: usize,
    },
}

/// The `GetPubKey` Ledger command.
struct GetVersion;

/// The `GetPubKey` Ledger command.
struct GetPubKeyCommand {
    display: bool,
    path: DerivationPath,
}

/// Part 1 of the `SignHash` command for setting path.
struct SignHashCommand1 {
    path: DerivationPath,
}

/// Part 2 of the `SignHash` command for setting hash.
struct SignHashCommand2 {
    hash: [u8; 32],
}

impl LedgerSigner {
    /// Initializes the Starknet Ledger app. Attempts to find and connect to a Ledger device. The
    /// device must be unlocked and have the Starknet app open.
    ///
    /// The `derivation_path` passed in _must_ follow EIP-2645, i.e. having `2645'` as its "purpose"
    /// level as per BIP-44, as the Ledger app does not allow other paths to be used.
    ///
    /// The path _must_ also be 6-level in length. An example path for Starknet would be:
    ///
    /// `m/2645'/1195502025'/1470455285'/0'/0'/0`
    ///
    /// where:
    ///
    /// - `2645'` is the EIP-2645 prefix
    /// - `1195502025'`, decimal for `0x4741e9c9`, is the 31 lowest bits for `sha256(starknet)`
    /// - `1470455285'`, decimal for `0x57a55df5`, is the 31 lowest bits for `sha256(starkli)`
    ///
    /// Currently, the Ledger app only enforces the length and the first level of the path.
    pub async fn new(derivation_path: DerivationPath) -> Result<Self, LedgerError> {
        if !matches!(derivation_path.iter().next(), Some(&EIP_2645_PURPOSE))
            || derivation_path.len() != EIP_2645_PATH_LENGTH
        {
            return Err(LedgerError::InvalidDerivationPath);
        }

        Ok(Self {
            app: LedgerStarknetApp::new().await?,
            derivation_path,
        })
    }
}

#[async_trait]
impl Signer for LedgerSigner {
    type GetPublicKeyError = LedgerError;
    type SignError = LedgerError;

    async fn get_public_key(&self) -> Result<VerifyingKey, Self::GetPublicKeyError> {
        self.app
            .get_public_key(self.derivation_path.clone(), false)
            .await
    }

    async fn sign_hash(&self, hash: &Felt) -> Result<Signature, Self::SignError> {
        self.app.sign_hash(self.derivation_path.clone(), hash).await
    }

    fn is_interactive(&self, _context: SignerInteractivityContext<'_>) -> bool {
        true
    }
}

impl LedgerStarknetApp {
    /// Initializes the Starknet Ledger app. Attempts to find and connect to a Ledger device. The
    /// device must be unlocked and have the Starknet app open.
    pub async fn new() -> Result<Self, LedgerError> {
        let transport = Ledger::init().await?;

        Ok(Self { transport })
    }

    /// Gets the Ledger app version.
    pub async fn get_version(&self) -> Result<Version, LedgerError> {
        let response = self.transport.exchange(&GetVersion.into()).await?;

        let data = get_apdu_data(&response)?;
        if data.len() != VERSION_SIZE {
            return Err(LedgerError::UnexpectedResponseLength {
                expected: VERSION_SIZE,
                actual: data.len(),
            });
        }

        Ok(Version::new(data[0] as u64, data[1] as u64, data[2] as u64))
    }

    /// Gets a public key from the app for a particular derivation path, with optional on-device
    /// confirmation for extra security.
    ///
    /// The derivation path _must_ follow EIP-2645, i.e. having `2645'` as its "purpose" level as
    /// per BIP-44, as the Ledger app does not allow other paths to be used.
    ///
    /// The path _must_ also be 6-level in length. An example path for Starknet would be:
    ///
    /// `m/2645'/1195502025'/1470455285'/0'/0'/0`
    ///
    /// where:
    ///
    /// - `2645'` is the EIP-2645 prefix
    /// - `1195502025'`, decimal for `0x4741e9c9`, is the 31 lowest bits for `sha256(starknet)`
    /// - `1470455285'`, decimal for `0x57a55df5`, is the 31 lowest bits for `sha256(starkli)`
    ///
    /// Currently, the Ledger app only enforces the length and the first level of the path.
    pub async fn get_public_key(
        &self,
        derivation_path: DerivationPath,
        display: bool,
    ) -> Result<VerifyingKey, LedgerError> {
        let response = self
            .transport
            .exchange(
                &GetPubKeyCommand {
                    display,
                    path: derivation_path,
                }
                .into(),
            )
            .await?;

        let data = get_apdu_data(&response)?;
        if data.len() != PUBLIC_KEY_SIZE {
            return Err(LedgerError::UnexpectedResponseLength {
                expected: PUBLIC_KEY_SIZE,
                actual: data.len(),
            });
        }

        // Unwrapping here is safe as length is fixed
        let pubkey_x = Felt::from_bytes_be(&data[1..33].try_into().unwrap());

        Ok(VerifyingKey::from_scalar(pubkey_x))
    }

    /// Requests a signature for a **raw hash** with a certain derivation path. Currently the Ledger
    /// app only supports blind signing raw hashes.
    ///
    /// The derivation path _must_ follow EIP-2645, i.e. having `2645'` as its "purpose" level as
    /// per BIP-44, as the Ledger app does not allow other paths to be used.
    ///
    /// The path _must_ also be 6-level in length. An example path for Starknet would be:
    ///
    /// `m/2645'/1195502025'/1470455285'/0'/0'/0`
    ///
    /// where:
    ///
    /// - `2645'` is the EIP-2645 prefix
    /// - `1195502025'`, decimal for `0x4741e9c9`, is the 31 lowest bits for `sha256(starknet)`
    /// - `1470455285'`, decimal for `0x57a55df5`, is the 31 lowest bits for `sha256(starkli)`
    ///
    /// Currently, the Ledger app only enforces the length and the first level of the path.
    pub async fn sign_hash(
        &self,
        derivation_path: DerivationPath,
        hash: &Felt,
    ) -> Result<Signature, LedgerError> {
        get_apdu_data(
            &self
                .transport
                .exchange(
                    &SignHashCommand1 {
                        path: derivation_path,
                    }
                    .into(),
                )
                .await?,
        )?;

        // The Ledger app prior to version 2.0.0 expects the input to be left shifted by 4 bits:
        let app_version = self.get_version().await?;
        let adjusted_bytes: [u8; 32] = if app_version < Version::new(2, 0, 0) {
            (U256::from_be_slice(&hash.to_bytes_be()) << 4)
                .to_be_byte_array()
                .into()
        } else {
            hash.to_bytes_be()
        };

        let response = self
            .transport
            .exchange(
                &SignHashCommand2 {
                    hash: adjusted_bytes,
                }
                .into(),
            )
            .await?;

        let data = get_apdu_data(&response)?;

        if data.len() != SIGNATURE_SIZE + 1 || data[0] != SIGNATURE_SIZE as u8 {
            return Err(LedgerError::UnexpectedResponseLength {
                expected: SIGNATURE_SIZE,
                actual: data.len(),
            });
        }

        // Unwrapping here is safe as length is fixed
        let r = Felt::from_bytes_be(&data[1..33].try_into().unwrap());
        let s = Felt::from_bytes_be(&data[33..65].try_into().unwrap());

        let signature = Signature { r, s };

        Ok(signature)
    }
}

impl From<coins_ledger::LedgerError> for LedgerError {
    fn from(value: coins_ledger::LedgerError) -> Self {
        Self::TransportError(value)
    }
}

impl From<GetVersion> for APDUCommand {
    fn from(_value: GetVersion) -> Self {
        Self {
            cla: CLA_STARKNET,
            ins: 0x00,
            p1: 0x00,
            p2: 0x00,
            data: APDUData::new(&[]),
            response_len: None,
        }
    }
}

impl From<GetPubKeyCommand> for APDUCommand {
    fn from(value: GetPubKeyCommand) -> Self {
        let path = value
            .path
            .iter()
            .flat_map(|level| level.to_be_bytes())
            .collect::<Vec<_>>();

        Self {
            cla: CLA_STARKNET,
            ins: 0x01,
            p1: if value.display { 0x01 } else { 0x00 },
            p2: 0x00,
            data: APDUData::new(&path),
            response_len: None,
        }
    }
}

impl From<SignHashCommand1> for APDUCommand {
    fn from(value: SignHashCommand1) -> Self {
        let path = value
            .path
            .iter()
            .flat_map(|level| level.to_be_bytes())
            .collect::<Vec<_>>();

        Self {
            cla: CLA_STARKNET,
            ins: 0x02,
            p1: 0x00,
            p2: 0x00,
            data: APDUData::new(&path),
            response_len: None,
        }
    }
}

impl From<SignHashCommand2> for APDUCommand {
    fn from(value: SignHashCommand2) -> Self {
        Self {
            cla: CLA_STARKNET,
            ins: 0x02,
            p1: 0x01,
            p2: 0x00,
            data: APDUData::new(&value.hash),
            response_len: None,
        }
    }
}

fn get_apdu_data(answer: &APDUAnswer) -> Result<&[u8], LedgerError> {
    let ret_code = answer.retcode();

    match TryInto::<APDUResponseCodes>::try_into(ret_code) {
        Ok(status) => {
            if status.is_success() {
                // Unwrapping here as we've already checked success
                Ok(answer.data().unwrap())
            } else {
                Err(LedgerError::UnsuccessfulRequest(status))
            }
        }
        Err(_) => Err(LedgerError::UnknownResponseCode(ret_code)),
    }
}
