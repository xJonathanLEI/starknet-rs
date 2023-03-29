#!/bin/sh

# Deterministically generate contract artifacts

docker run -it --rm \
    -v "$(pwd)/artifacts:/artifacts" \
    -v "$(pwd)/contracts:/contracts:ro" \
    -v "$(pwd)/docker_entry.sh:/entry.sh:ro" \
    --env "USER_ID=$(id -u)" \
    --env "GROUP_ID=$(id -g)" \
    --entrypoint "/entry.sh" \
    starknet/cairo:1.0.0-alpha.3
