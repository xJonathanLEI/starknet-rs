#!/bin/sh

set -x
set -e

hash () {
  python3.9 /generate_hashes.py "$1" > "$1.hashes.json"
  chown $USER_ID:$GROUP_ID "$1.hashes.json"
}

hash "/artifacts/abi_types"

hash "/artifacts/erc20"
