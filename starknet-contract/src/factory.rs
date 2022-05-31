use flate2::{write::GzEncoder, Compression};
use rand::{prelude::StdRng, RngCore, SeedableRng};
use starknet_core::types::{
    AbiEntry, AddTransactionResult, ContractArtifact, ContractDefinition, DeployTransactionRequest,
    EntryPointsByType, FieldElement, TransactionRequest,
};
use starknet_providers::Provider;
use std::io::Write;

pub struct Factory<P>
where
    P: Provider,
{
    compressed_program: Vec<u8>,
    entry_points_by_type: EntryPointsByType,
    abi: Vec<AbiEntry>,
    provider: P,
}

#[derive(Debug, thiserror::Error)]
pub enum FactoryError {
    #[error(transparent)]
    CannotSerializeProgram(serde_json::Error),
    #[error(transparent)]
    CannotCompressProgram(std::io::Error),
}

impl<P: Provider> Factory<P> {
    pub fn new(artifact: ContractArtifact, provider: P) -> Result<Self, FactoryError> {
        let program_json = serde_json::to_string(&artifact.program)
            .map_err(FactoryError::CannotSerializeProgram)?;

        // Use best compression level to optimize for payload size
        let mut gzip_encoder = GzEncoder::new(Vec::new(), Compression::best());
        gzip_encoder
            .write_all(program_json.as_bytes())
            .map_err(FactoryError::CannotCompressProgram)?;

        let compressed_program = gzip_encoder
            .finish()
            .map_err(FactoryError::CannotCompressProgram)?;

        Ok(Self {
            compressed_program,
            entry_points_by_type: artifact.entry_points_by_type,
            abi: artifact.abi,
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
