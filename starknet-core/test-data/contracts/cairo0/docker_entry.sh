#!/bin/sh

set -x
set -e

compile () {
  starknet-compile-deprecated $1 --output "$2.txt" $3
  python3.9 /generate_hashes.py "$2.txt" > "$2.hashes.json"
  chown $USER_ID:$GROUP_ID "$2.txt"
  chown $USER_ID:$GROUP_ID "$2.hashes.json"
}

# Ugly hack to make contract artifacts compact
sed -i "s/indent=4/separators=(',', ':')/g" /usr/local/lib/python3.9/site-packages/starkware/cairo/lang/compiler/cairo_compile.py

# Ugly hack to send the internal hinted class hash to stdout
sed -i "s/return structs\\.DeprecatedCompiledClass/print(f\\\"  \\\\\\\"hinted_class_hash\\\\\\\": \\\\\\\"{hex(compute_deprecated_hinted_class_hash(contract_class=contract_class))}\\\\\\\",\")\\n    return structs\\.DeprecatedCompiledClass/g" /usr/local/lib/python3.9/site-packages/starkware/starknet/core/os/contract_class/deprecated_class_hash.py

# ./artifacts/event_example.txt
compile "/contracts/EventExample.cairo" "/artifacts/event_example"

# ./artifacts/deployable.txt
compile "/contracts/Deployable.cairo" "/artifacts/deployable"

# ./artifacts/oz_account.txt
cd /contracts/openzeppelin/src
compile "/contracts/openzeppelin/src/openzeppelin/account/presets/Account.cairo" "/artifacts/oz_account" --account_contract
