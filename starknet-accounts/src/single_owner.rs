use crate::{Account, ConnectedAccount, ExecutionEncoder, RawDeclarationV3, RawExecutionV3};

use async_trait::async_trait;
use starknet_core::types::{contract::ComputeClassHashError, BlockId, BlockTag, Call, Felt};
use starknet_providers::Provider;
use starknet_signers::{Signer, SignerInteractivityContext};

/// A generic [`Account`] implementation for controlling account contracts that only have one signer
/// using ECDSA the STARK curve.
#[derive(Debug, Clone)]
pub struct SingleOwnerAccount<P, S>
where
    P: Provider + Send,
    S: Signer + Send,
{
    provider: P,
    signer: S,
    address: Felt,
    chain_id: Felt,
    block_id: BlockId,
    encoding: ExecutionEncoding,
}

/// Errors signing an execution/declaration request.
#[derive(Debug, thiserror::Error)]
pub enum SignError<S> {
    /// An error encountered by the signer implementation.
    #[error(transparent)]
    Signer(S),
    /// Failure to compute the class hash of the class being declared.
    #[error(transparent)]
    ClassHash(ComputeClassHashError),
}

/// How calldata for the `__execute__` entrypoint is encoded.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ExecutionEncoding {
    /// Encode `__execute__` calldata in Cairo 0 style, where calldata from all calls are concated
    /// and appended at the end.
    Legacy,
    /// Encode `__execute__` calldata in Cairo (1) style, where each call is self-contained.
    New,
}

impl<P, S> SingleOwnerAccount<P, S>
where
    P: Provider + Sync + Send,
    S: Signer + Sync + Send,
{
    /// Create a new account controlled by a single signer.
    ///
    /// ### Parameters
    ///
    /// - `provider`: A `Provider` implementation that provides access to the Starknet network.
    /// - `signer`: A `Signer` implementation that can generate valid signatures for this account.
    /// - `address`: Account contract address.
    /// - `chain_id`: Network chain ID.
    /// - `encoding`: How `__execute__` calldata should be encoded.
    pub const fn new(
        provider: P,
        signer: S,
        address: Felt,
        chain_id: Felt,
        encoding: ExecutionEncoding,
    ) -> Self {
        Self {
            provider,
            signer,
            address,
            chain_id,
            block_id: BlockId::Tag(BlockTag::PreConfirmed),
            encoding,
        }
    }

    /// Sets a new block ID to run queries against.
    pub fn set_block_id(&mut self, block_id: BlockId) -> &Self {
        self.block_id = block_id;
        self
    }
}

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
impl<P, S> Account for SingleOwnerAccount<P, S>
where
    P: Provider + Sync + Send,
    S: Signer + Sync + Send,
{
    type SignError = SignError<S::SignError>;

    fn address(&self) -> Felt {
        self.address
    }

    fn chain_id(&self) -> Felt {
        self.chain_id
    }

    async fn sign_execution_v3(
        &self,
        execution: &RawExecutionV3,
        query_only: bool,
    ) -> Result<Vec<Felt>, Self::SignError> {
        let tx_hash = execution.transaction_hash(self.chain_id, self.address, query_only, self);
        let signature = self
            .signer
            .sign_hash(&tx_hash)
            .await
            .map_err(SignError::Signer)?;

        Ok(vec![signature.r, signature.s])
    }

    async fn sign_declaration_v3(
        &self,
        declaration: &RawDeclarationV3,
        query_only: bool,
    ) -> Result<Vec<Felt>, Self::SignError> {
        let tx_hash = declaration.transaction_hash(self.chain_id, self.address, query_only);
        let signature = self
            .signer
            .sign_hash(&tx_hash)
            .await
            .map_err(SignError::Signer)?;

        Ok(vec![signature.r, signature.s])
    }

    fn is_signer_interactive(&self, context: SignerInteractivityContext<'_>) -> bool {
        self.signer.is_interactive(context)
    }
}

impl<P, S> ExecutionEncoder for SingleOwnerAccount<P, S>
where
    P: Provider + Send,
    S: Signer + Send,
{
    fn encode_calls(&self, calls: &[Call]) -> Vec<Felt> {
        let mut execute_calldata: Vec<Felt> = vec![calls.len().into()];

        match self.encoding {
            ExecutionEncoding::Legacy => {
                let mut concated_calldata: Vec<Felt> = vec![];
                for call in calls {
                    execute_calldata.push(call.to); // to
                    execute_calldata.push(call.selector); // selector
                    execute_calldata.push(concated_calldata.len().into()); // data_offset
                    execute_calldata.push(call.calldata.len().into()); // data_len

                    for item in &call.calldata {
                        concated_calldata.push(*item);
                    }
                }

                execute_calldata.push(concated_calldata.len().into()); // calldata_len
                execute_calldata.extend_from_slice(&concated_calldata);
            }
            ExecutionEncoding::New => {
                for call in calls {
                    execute_calldata.push(call.to); // to
                    execute_calldata.push(call.selector); // selector

                    execute_calldata.push(call.calldata.len().into()); // calldata.len()
                    execute_calldata.extend_from_slice(&call.calldata);
                }
            }
        }

        execute_calldata
    }
}

impl<P, S> ConnectedAccount for SingleOwnerAccount<P, S>
where
    P: Provider + Sync + Send,
    S: Signer + Sync + Send,
{
    type Provider = P;

    fn provider(&self) -> &Self::Provider {
        &self.provider
    }

    fn block_id(&self) -> BlockId {
        self.block_id
    }
}
