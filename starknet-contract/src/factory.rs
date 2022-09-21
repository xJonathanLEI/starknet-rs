use rand::{prelude::StdRng, RngCore, SeedableRng};
use starknet_core::types::{
    contract_artifact::CompressProgramError, AbiEntry, AddTransactionResult, ContractArtifact,
    ContractDefinition, DeployTransactionRequest, EntryPointsByType, FieldElement,
    TransactionRequest,
};
use starknet_providers::Provider;

pub struct Factory<P>
where
    P: Provider,
{
    compressed_program: Vec<u8>,
    entry_points_by_type: EntryPointsByType,
    abi: Vec<AbiEntry>,
    provider: P,
}

impl<P: Provider> Factory<P> {
    pub fn new(artifact: &ContractArtifact, provider: P) -> Result<Self, CompressProgramError> {
        let compressed_program = artifact.program.compress()?;

        Ok(Self {
            compressed_program,
            entry_points_by_type: artifact.entry_points_by_type.clone(),
            abi: artifact.abi.clone(),
            provider,
        })
    }

    pub async fn deploy(
        &self,
        constructor_calldata: Vec<FieldElement>,
        token: Option<String>,
    ) -> Result<AddTransactionResult, P::Error> {
        let mut salt_buffer = [0u8; 32];

        // Generate 31 bytes only here to avoid out of range error
        // TODO: change to cover full range
        let mut rng = StdRng::from_entropy();
        rng.fill_bytes(&mut salt_buffer[1..]);

        self.provider
            .add_transaction(
                TransactionRequest::Deploy(DeployTransactionRequest {
                    contract_address_salt: FieldElement::from_bytes_be(&salt_buffer).unwrap(),
                    contract_definition: ContractDefinition {
                        program: self.compressed_program.clone(),
                        entry_points_by_type: self.entry_points_by_type.clone(),
                        abi: Some(self.abi.clone()),
                    },
                    constructor_calldata,
                }),
                token,
            )
            .await
    }
}
