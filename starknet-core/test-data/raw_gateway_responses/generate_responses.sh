#!/bin/sh

set -e

mkdir -p ./get_block/
mkdir -p ./get_class_by_hash/
mkdir -p ./get_state_update/
mkdir -p ./get_transaction/
mkdir -p ./get_transaction_status/

# ./get_block/1_with_transactions.txt
curl -o ./get_block/1_with_transactions.txt "https://alpha-sepolia.starknet.io/feeder_gateway/get_block?blockNumber=100"

# ./get_block/2_with_messages.txt
curl -o ./get_block/2_with_messages.txt "https://alpha-sepolia.starknet.io/feeder_gateway/get_block?blockNumber=25"

# ./get_block/3_with_events.txt
curl -o ./get_block/3_with_events.txt "https://alpha-sepolia.starknet.io/feeder_gateway/get_block?blockNumber=4"

# ./get_block/4_pending.txt (non-deterministic)
curl -o ./get_block/4_pending.txt "https://alpha-sepolia.starknet.io/feeder_gateway/get_block?blockNumber=pending"

# NOTE: block with the same criteria not found in alpha-sepolia yet
# ./get_block/5_with_class_hash_and_actual_fee.txt
curl -o ./get_block/5_with_class_hash_and_actual_fee.txt "https://alpha-sepolia.starknet.io/feeder_gateway/get_block?blockNumber=1"

# ./get_block/6_with_sequencer_address.txt
curl -o ./get_block/6_with_sequencer_address.txt "https://alpha-sepolia.starknet.io/feeder_gateway/get_block?blockNumber=100"

# ./get_block/7_with_declare_tx.txt
curl -o ./get_block/7_with_declare_tx.txt "https://alpha-sepolia.starknet.io/feeder_gateway/get_block?blockNumber=7"

# ./get_block/8_with_starknet_version.txt
curl -o ./get_block/8_with_starknet_version.txt "https://alpha-sepolia.starknet.io/feeder_gateway/get_block?blockNumber=100"

# NOTE: block with the same criteria not found in alpha-sepolia yet
# ./get_block/9_with_messages_without_nonce.txt
curl -o ./get_block/9_with_messages_without_nonce.txt "https://alpha-sepolia.starknet.io/feeder_gateway/get_block?blockNumber=1"

# ./get_block/10_with_l1_handler.txt
curl -o ./get_block/10_with_l1_handler.txt "https://alpha-sepolia.starknet.io/feeder_gateway/get_block?blockNumber=6"

# NOTE: block with the same criteria not found in alpha-sepolia yet
# ./get_block/11_without_execution_resources.txt
curl -o ./get_block/11_without_execution_resources.txt "https://alpha-sepolia.starknet.io/feeder_gateway/get_block?blockNumber=1"

# NOTE: block with the same criteria not found in alpha-sepolia yet
# ./get_block/12_l1_handler_without_nonce.txt
curl -o ./get_block/12_l1_handler_without_nonce.txt "https://alpha-mainnet.starknet.io/feeder_gateway/get_block?blockNumber=1"

# NOTE: block with the same criteria not found in alpha-sepolia yet
# ./get_block/13_without_entry_point.txt
curl -o ./get_block/13_without_entry_point.txt "https://alpha-sepolia.starknet.io/feeder_gateway/get_block?blockNumber=1"

# ./get_block/14_deploy_account.txt
curl -o ./get_block/14_deploy_account.txt "https://alpha-sepolia.starknet.io/feeder_gateway/get_block?blockNumber=7"

# ./get_block/15_declare_v2.txt
curl -o ./get_block/15_declare_v2.txt "https://alpha-sepolia.starknet.io/feeder_gateway/get_block?blockNumber=7"

# NOTE: block with the same criteria not found in alpha-sepolia yet
# ./get_block/16_with_reverted_tx.txt
curl -o ./get_block/16_with_reverted_tx.txt "https://alpha-sepolia.starknet.io/feeder_gateway/get_block?blockNumber=1"

# ./get_transaction/1_invoke.txt
curl -o ./get_transaction/1_invoke.txt "https://alpha-sepolia.starknet.io/feeder_gateway/get_transaction?transactionHash=0xcc475316c44b764c852e4ce721b15afcc8b9c53a5a54c85020f5dee067b8ce"

# NOTE: transaction with the same criteria not found in alpha-sepolia yet
# ./get_transaction/2_deploy.txt
curl -o ./get_transaction/2_deploy.txt "https://alpha-sepolia.starknet.io/feeder_gateway/get_transaction?transactionHash=0x1"

# ./get_transaction/3_not_received.txt
curl -o ./get_transaction/3_not_received.txt "https://alpha-sepolia.starknet.io/feeder_gateway/get_transaction?transactionHash=0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"

# NOTE: transaction with the same criteria not found in alpha-sepolia yet
# ./get_transaction/4_failure.txt
curl -o ./get_transaction/4_failure.txt "https://alpha-sepolia.starknet.io/feeder_gateway/get_transaction?transactionHash=0x1"

# ./get_transaction/5_declare_v1.txt
curl -o ./get_transaction/5_declare_v1.txt "https://alpha-sepolia.starknet.io/feeder_gateway/get_transaction?transactionHash=0x1936a09e5aaee208fc0f7cc826e126d421c3ac9aca2c789605e1e919e399185"

# ./get_transaction/6_declare_v2.txt
curl -o ./get_transaction/6_declare_v2.txt "https://alpha-sepolia.starknet.io/feeder_gateway/get_transaction?transactionHash=0x4cacc2bbdd5ec77b20e908f311ab27d6495b69761e929bb24ba02632716944"

# NOTE: transaction with the same criteria not found in alpha-sepolia yet
# ./get_transaction/7_reverted.txt
curl -o ./get_transaction/7_reverted.txt "https://alpha-sepolia.starknet.io/feeder_gateway/get_transaction?transactionHash=0x1"

# ./get_transaction/8_invoke_v3.txt
curl -o ./get_transaction/8_invoke_v3.txt "https://alpha-sepolia.starknet.io/feeder_gateway/get_transaction?transactionHash=0x45cbde9a600beb9beb77a54f16842ba2871a8ead541dd2803c9397e1f097ed4"

# ./get_transaction/9_declare_v3.txt
curl -o ./get_transaction/9_declare_v3.txt "https://alpha-sepolia.starknet.io/feeder_gateway/get_transaction?transactionHash=0x54270d103c875a613e013d1fd555edcff2085feca9d7b4532243a8257fd5cf3"

# ./get_transaction/10_deploy_account_v3.txt
curl -o ./get_transaction/10_deploy_account_v3.txt "https://alpha-sepolia.starknet.io/feeder_gateway/get_transaction?transactionHash=0x11c67fb3a9a623b3190c9ac41ebf7f5dd421f2583344c498a30a7280c660f01"

# ./get_transaction_status/1_accepted.txt
curl -o ./get_transaction_status/1_accepted.txt "https://alpha-sepolia.starknet.io/feeder_gateway/get_transaction_status?transactionHash=0xcc475316c44b764c852e4ce721b15afcc8b9c53a5a54c85020f5dee067b8ce"

# ./get_transaction_status/2_not_received.txt
curl -o ./get_transaction_status/2_not_received.txt "https://alpha-sepolia.starknet.io/feeder_gateway/get_transaction_status?transactionHash=0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"

# NOTE: transaction with the same criteria not found in alpha-sepolia yet
# ./get_transaction_status/3_failure.txt
curl -o ./get_transaction_status/3_failure.txt "https://alpha-sepolia.starknet.io/feeder_gateway/get_transaction_status?transactionHash=0x1"

# NOTE: transaction with the same criteria not found in alpha-sepolia yet
# ./get_transaction_status/4_reverted.txt
curl -o ./get_transaction_status/4_reverted.txt "https://alpha-sepolia.starknet.io/feeder_gateway/get_transaction_status?transactionHash=0x1"

# ./get_state_update/1_success.txt
curl -o ./get_state_update/1_success.txt "https://alpha-sepolia.starknet.io/feeder_gateway/get_state_update?blockNumber=100"

# ./get_state_update/2_pending_block.txt (non-deterministic)
curl -o ./get_state_update/2_pending_block.txt "https://alpha-sepolia.starknet.io/feeder_gateway/get_state_update?blockNumber=pending"

# ./get_state_update/3_with_declarations.txt
curl -o ./get_state_update/3_with_declarations.txt "https://alpha-sepolia.starknet.io/feeder_gateway/get_state_update?blockNumber=7"

# ./get_state_update/4_with_nonce_changes.txt
curl -o ./get_state_update/4_with_nonce_changes.txt "https://alpha-sepolia.starknet.io/feeder_gateway/get_state_update?blockNumber=7"

# ./get_state_update/5_with_declare_v2.txt
curl -o ./get_state_update/5_with_declare_v2.txt "https://alpha-sepolia.starknet.io/feeder_gateway/get_state_update?blockNumber=7"

# ./get_state_update/6_with_replaced_classes.txt
# NOTE: block with the same criteria not found in alpha-sepolia yet
curl -o ./get_state_update/6_with_replaced_classes.txt "https://alpha-sepolia.starknet.io/feeder_gateway/get_state_update?blockNumber=1"

# ./get_state_update/7_with_deployed_contracts.txt
curl -o ./get_state_update/7_with_deployed_contracts.txt "https://alpha-sepolia.starknet.io/feeder_gateway/get_state_update?blockNumber=7"

# ./get_class_by_hash/1_cairo_0.txt
curl -o ./get_class_by_hash/1_cairo_0.txt "https://alpha-sepolia.starknet.io/feeder_gateway/get_class_by_hash?classHash=0x025ec026985a3bf9d0cc1fe17326b245dfdc3ff89b8fde106542a3ea56c5a918"

# ./get_class_by_hash/2_not_declared.txt
curl -o ./get_class_by_hash/2_not_declared.txt "https://alpha-sepolia.starknet.io/feeder_gateway/get_class_by_hash?classHash=0x111111111111111111111111"

# ./get_class_by_hash/3_cairo_1.txt
curl -o ./get_class_by_hash/3_cairo_1.txt "https://alpha-sepolia.starknet.io/feeder_gateway/get_class_by_hash?classHash=0x01a736d6ed154502257f02b1ccdf4d9d1089f80811cd6acad48e6b6a9d1f2003"
