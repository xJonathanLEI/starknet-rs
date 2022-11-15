#!/bin/bash

# Build benchmark wasm artifacts with `build_bench_wasm.sh` first

set -e

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
REPO_ROOT=$( dirname -- $SCRIPT_DIR )

RUNTIME="$1"

if [ -z "$RUNTIME" ]; then
  echo "Runtime not specified"
  exit 1
fi

benches=(
  class_hash
  ecdsa_get_public_key
  ecdsa_sign
  ecdsa_verify
  pedersen_hash
  rfc6979_generate_k
)

for bench in ${benches[@]}; do
  $RUNTIME run --dir=. $REPO_ROOT/target/bench-wasm/$bench.wasm -- --bench
done
