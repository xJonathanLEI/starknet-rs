use crate::{
    Account, Call, ConnectedAccount, ExecutionEncoder, RawDeclaration, RawExecution,
    RawLegacyDeclaration,
};
use async_trait::async_trait;
use starknet_core::types::{contract::ComputeClassHashError, BlockId, BlockTag, FieldElement};
use starknet_providers::Provider;
use starknet_signers::Signer;

#[derive(Debug, Clone)]
pub struct SingleOwnerAccount<P, S>
where
    P: Provider + Send,
    S: Signer + Send,
{
    provider: P,
    signer: S,
    address: FieldElement,
    chain_id: FieldElement,
    block_id: BlockId,
    encoding: ExecutionEncoding,
}

#[derive(Debug, thiserror::Error)]
pub enum SignError<S> {
    #[error(transparent)]
    Signer(S),
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
    /// ### Arguments
    ///
    /// * `provider`: A `Provider` implementation that provides access to the Starknet network.
    /// * `signer`: A `Signer` implementation that can generate valid signatures for this account.
    /// * `address`: Account contract address.
    /// * `chain_id`: Network chain ID.
    /// * `encoding`: How `__execute__` calldata should be encoded.
    pub fn new(
        provider: P,
        signer: S,
        address: FieldElement,
        chain_id: FieldElement,
        encoding: ExecutionEncoding,
    ) -> Self {
        Self {
            provider,
            signer,
            address,
            chain_id,
            block_id: BlockId::Tag(BlockTag::Latest),
            encoding,
        }
    }

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

    fn address(&self) -> FieldElement {
        self.address
    }

    fn chain_id(&self) -> FieldElement {
        self.chain_id
    }

    async fn sign_execution(
        &self,
        execution: &RawExecution,
        query_only: bool,
    ) -> Result<Vec<FieldElement>, Self::SignError> {
        let tx_hash = execution.transaction_hash(self.chain_id, self.address, query_only, self);
        let signature = self
            .signer
            .sign_hash(&tx_hash)
            .await
            .map_err(SignError::Signer)?;

        Ok(vec![signature.r, signature.s])
    }

    async fn sign_declaration(
        &self,
        declaration: &RawDeclaration,
        query_only: bool,
    ) -> Result<Vec<FieldElement>, Self::SignError> {
        let tx_hash = declaration.transaction_hash(self.chain_id, self.address, query_only);
        let signature = self
            .signer
            .sign_hash(&tx_hash)
            .await
            .map_err(SignError::Signer)?;

        Ok(vec![signature.r, signature.s])
    }

    async fn sign_legacy_declaration(
        &self,
        legacy_declaration: &RawLegacyDeclaration,
        query_only: bool,
    ) -> Result<Vec<FieldElement>, Self::SignError> {
        let tx_hash = legacy_declaration
            .transaction_hash(self.chain_id, self.address, query_only)
            .map_err(SignError::ClassHash)?;
        let signature = self
            .signer
            .sign_hash(&tx_hash)
            .await
            .map_err(SignError::Signer)?;

        Ok(vec![signature.r, signature.s])
    }
}

impl<P, S> ExecutionEncoder for SingleOwnerAccount<P, S>
where
    P: Provider + Send,
    S: Signer + Send,
{
    fn encode_calls(&self, calls: &[Call]) -> Vec<FieldElement> {
        let mut execute_calldata: Vec<FieldElement> = vec![calls.len().into()];

        match self.encoding {
            ExecutionEncoding::Legacy => {
                let mut concated_calldata: Vec<FieldElement> = vec![];
                for call in calls.iter() {
                    execute_calldata.push(call.to); // to
                    execute_calldata.push(call.selector); // selector
                    execute_calldata.push(concated_calldata.len().into()); // data_offset
                    execute_calldata.push(call.calldata.len().into()); // data_len

                    for item in call.calldata.iter() {
                        concated_calldata.push(*item);
                    }
                }

                execute_calldata.push(concated_calldata.len().into()); // calldata_len
                execute_calldata.extend_from_slice(&concated_calldata);
            }
            ExecutionEncoding::New => {
                for call in calls.iter() {
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
