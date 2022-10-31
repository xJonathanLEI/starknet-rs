#!/bin/sh

# Deterministically generate contract artifacts

docker run -it --rm \
    -v "$(pwd)/artifacts:/artifacts" \
    -v "$(pwd)/contracts:/contracts:ro" \
    -v "$(pwd)/docker_entry.sh:/entry.sh:ro" \
    -v "$(pwd)/scripts/generate_hashes.py:/generate_hashes.py:ro" \
    --env "USER_ID=$(id -u)" \
    --env "GROUP_ID=$(id -g)" \
    --entrypoint "/entry.sh" \
    shardlabs/cairo-cli:0.10.1
