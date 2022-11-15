#!/bin/bash

# Make sure `wasm32-wasi` target and `cargo-wasi` are installed

set -e

function generate_wasm() {
  cargo wasi build --bench=$1 --release
  cp $(ls -t $REPO_ROOT/target/wasm32-wasi/release/deps/$1*.rustc.wasm | head -n 1) $REPO_ROOT/target/bench-wasm/$1.wasm
}

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
REPO_ROOT=$( dirname -- $SCRIPT_DIR )

rm -rf $REPO_ROOT/target/bench-wasm
mkdir -p $REPO_ROOT/target/bench-wasm

cd $REPO_ROOT/starknet-core
benches=(
  class_hash
)

for bench in ${benches[@]}; do
  generate_wasm $bench
  generate_wasm $bench
done

cd $REPO_ROOT/starknet-crypto
benches=(
  ecdsa_get_public_key
  ecdsa_sign
  ecdsa_verify
  pedersen_hash
  rfc6979_generate_k
)

for bench in ${benches[@]}; do
  generate_wasm $bench
done
