#!/bin/sh

set -x
set -e

compile () {
  starknet-compile $1  --output $2
  chown $USER_ID:$GROUP_ID $2
}

# Ugly hack to make contract artifacts compact
sed -i "s/indent=4/separators=(',', ':')/g" /usr/local/lib/python3.7/site-packages/starkware/cairo/lang/compiler/cairo_compile.py

# ./artifacts/event_example.txt
compile "/contracts/EventExample.cairo" "/artifacts/event_example.txt"

# ./artifacts/oz_account.txt
cd /contracts/openzeppelin
compile "/contracts/openzeppelin/openzeppelin/account/Account.cairo" "/artifacts/oz_account.txt"
