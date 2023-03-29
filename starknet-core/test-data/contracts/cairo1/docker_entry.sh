#!/bin/sh

set -x
set -e

compile () {
  starknet-compile $1 "${2}_sierra.txt"
  starknet-sierra-compile "${2}_sierra.txt" "${2}_compiled.txt"
  chown $USER_ID:$GROUP_ID "${2}_sierra.txt"
  chown $USER_ID:$GROUP_ID "${2}_compiled.txt"
}

compile "/contracts/abi_types.cairo" "/artifacts/abi_types"

compile "/contracts/erc20.cairo" "/artifacts/erc20"
