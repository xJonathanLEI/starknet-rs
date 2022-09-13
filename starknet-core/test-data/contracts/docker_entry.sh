#!/bin/sh

set -x
set -e

compile () {
  starknet-compile $1  --output $2 $3
  chown $USER_ID:$GROUP_ID $2
}

# Ugly hack to make contract artifacts compact
sed -i "s/indent=4/separators=(',', ':')/g" /usr/local/lib/python3.9/site-packages/starkware/cairo/lang/compiler/cairo_compile.py

# ./artifacts/event_example.txt
compile "/contracts/EventExample.cairo" "/artifacts/event_example.txt"

# ./artifacts/oz_account.txt
cd /contracts/openzeppelin/src
compile "/contracts/openzeppelin/src/openzeppelin/account/presets/Account.cairo" "/artifacts/oz_account.txt" --account_contract
