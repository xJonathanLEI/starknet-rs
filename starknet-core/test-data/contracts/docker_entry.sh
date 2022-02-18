#!/bin/sh

set -x
set -e

compile () {
  starknet-compile $1 | jq -c > $2
  chown $USER_ID:$GROUP_ID $2
}

apt-get update
apt-get install -y jq

# ./artifacts/oz_account.txt
compile "/contracts/OzAccount.cairo" "/artifacts/oz_account.txt"

# ./artifacts/event_example.txt
compile "/contracts/EventExample.cairo" "/artifacts/event_example.txt"
