#!/bin/sh

set -e

# Deterministically generate contract artifacts

docker run -it --rm \
    -v "$(pwd)/artifacts:/artifacts" \
    -v "$(pwd)/contracts:/contracts:ro" \
    -v "$(pwd)/docker_entry_compile.sh:/entry.sh:ro" \
    --env "USER_ID=$(id -u)" \
    --env "GROUP_ID=$(id -g)" \
    --entrypoint "/entry.sh" \
    starknet/cairo:1.0.0-alpha.6


docker run -it --rm \
    -v "$(pwd)/artifacts:/artifacts" \
    -v "$(pwd)/docker_entry_compiled_hashes.sh:/entry.sh:ro" \
    -v "$(pwd)/scripts/generate_compiled_hashes.py:/generate_hashes.py:ro" \
    --env "USER_ID=$(id -u)" \
    --env "GROUP_ID=$(id -g)" \
    --entrypoint "/entry.sh" \
    shardlabs/cairo-cli:0.11.0.1
