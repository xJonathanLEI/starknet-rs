//! Library for deploying and interacting with Starknet contracts.
//!
//! Currently, this crate only provides a single type [`ContractFactory`] for deploying contracts
//! using the Universal Deployer Contract.
//!
//! In the future, features like ABI-based contract binding generation will be added to allow type-
//! safe interaction with Starknet smart contracts.

#![deny(missing_docs)]

mod factory;
pub use factory::{ContractFactory, DeploymentV1, DeploymentV3};
