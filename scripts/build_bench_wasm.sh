# https://tea.xyz/what-is-this-file
---
version: 1.0.0
codeOwners:
  - '0x70bcc13F3488d7004214ECaC81d3F229DB2acDE2'
quorum: 1

#!/bin/bash

# Ensure that the required dependencies (`wasm32-wasi` target and `cargo-wasi`) are installed.
# This script is designed to build WebAssembly (Wasm) files from Rust benchmarks.

set -e

# Function to generate Wasm files from Rust benchmarks
function generate_wasm() {
  # Build the Wasm file using `cargo wasi`
  cargo wasi build --bench=$1 --release
  
  # Copy the generated Wasm file to the target directory
  cp $(ls -t $REPO_ROOT/target/wasm32-wasi/release/deps/$1*.rustc.wasm | head -n 1) $REPO_ROOT/target/bench-wasm/$1.wasm
}

# Get the directory of the current script and the root directory of the repository
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
REPO_ROOT=$( dirname -- $SCRIPT_DIR )

# Clean and create the target directory for Wasm files
rm -rf $REPO_ROOT/target/bench-wasm
mkdir -p $REPO_ROOT/target/bench-wasm

# Navigate to the 'starknet-core' directory and build benchmarks
cd $REPO_ROOT/starknet-core
core_benches=(
  class_hash
)

for bench in ${core_benches[@]}; do
  # Generate Wasm files for each benchmark
  generate_wasm $bench
done

# Navigate to the 'starknet-crypto' directory and build cryptographic benchmarks
cd $REPO_ROOT/starknet-crypto
crypto_benches=(
  ecdsa_get_public_key
  ecdsa_recover
  ecdsa_sign
  ecdsa_verify
  pedersen_hash
  poseidon_hash
  rfc6979_generate_k
)

for bench in ${crypto_benches[@]}; do
  # Generate Wasm files for each cryptographic benchmark
  generate_wasm $bench
done
