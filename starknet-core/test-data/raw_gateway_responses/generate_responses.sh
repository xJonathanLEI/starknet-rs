#!/bin/sh

set -e

mkdir -p ./get_block/
mkdir -p ./get_block_traces/
mkdir -p ./get_class_by_hash/
mkdir -p ./get_state_update/
mkdir -p ./get_transaction/
mkdir -p ./get_transaction_status/
mkdir -p ./get_transaction_trace/

# ./get_block/1_with_transactions.txt
curl -o ./get_block/1_with_transactions.txt "https://external.integration.starknet.io/feeder_gateway/get_block?blockNumber=100"

# ./get_block/2_with_messages.txt
curl -o ./get_block/2_with_messages.txt "https://external.integration.starknet.io/feeder_gateway/get_block?blockNumber=102"

# ./get_block/3_with_events.txt
curl -o ./get_block/3_with_events.txt "https://external.integration.starknet.io/feeder_gateway/get_block?blockNumber=70000"

# ./get_block/4_pending.txt (non-deterministic)
curl -o ./get_block/4_pending.txt "https://external.integration.starknet.io/feeder_gateway/get_block?blockNumber=pending"

# NOTE: block with the same criteria not found in goerli-integration yet
# ./get_block/5_with_class_hash_and_actual_fee.txt
curl -o ./get_block/5_with_class_hash_and_actual_fee.txt "https://external.integration.starknet.io/feeder_gateway/get_block?blockNumber=1"

# ./get_block/6_with_sequencer_address.txt
curl -o ./get_block/6_with_sequencer_address.txt "https://external.integration.starknet.io/feeder_gateway/get_block?blockNumber=200000"

# ./get_block/7_with_declare_tx.txt
curl -o ./get_block/7_with_declare_tx.txt "https://external.integration.starknet.io/feeder_gateway/get_block?blockNumber=209679"

# ./get_block/8_with_starknet_version.txt
curl -o ./get_block/8_with_starknet_version.txt "https://external.integration.starknet.io/feeder_gateway/get_block?blockNumber=200000"

# NOTE: block with the same criteria not found in goerli-integration yet
# ./get_block/9_with_messages_without_nonce.txt
curl -o ./get_block/9_with_messages_without_nonce.txt "https://external.integration.starknet.io/feeder_gateway/get_block?blockNumber=1"

# ./get_block/10_with_l1_handler.txt
curl -o ./get_block/10_with_l1_handler.txt "https://external.integration.starknet.io/feeder_gateway/get_block?blockNumber=77881"

# NOTE: block with the same criteria not found in goerli-integration yet
# ./get_block/11_without_execution_resources.txt
curl -o ./get_block/11_without_execution_resources.txt "https://external.integration.starknet.io/feeder_gateway/get_block?blockNumber=1"

# NOTE: block with the same criteria not found in goerli-integration yet
# ./get_block/12_l1_handler_without_nonce.txt
curl -o ./get_block/12_l1_handler_without_nonce.txt "https://alpha-mainnet.starknet.io/feeder_gateway/get_block?blockNumber=1"

# NOTE: block with the same criteria not found in goerli-integration yet
# ./get_block/13_without_entry_point.txt
curl -o ./get_block/13_without_entry_point.txt "https://external.integration.starknet.io/feeder_gateway/get_block?blockNumber=1"

# ./get_block/14_deploy_account.txt
curl -o ./get_block/14_deploy_account.txt "https://external.integration.starknet.io/feeder_gateway/get_block?blockNumber=228457"

# ./get_block/15_declare_v2.txt
curl -o ./get_block/15_declare_v2.txt "https://external.integration.starknet.io/feeder_gateway/get_block?blockNumber=283364"

# NOTE: block with the same criteria not found in goerli-integration yet
# ./get_block/16_with_reverted_tx.txt
curl -o ./get_block/16_with_reverted_tx.txt "https://external.integration.starknet.io/feeder_gateway/get_block?blockNumber=1"

# ./get_transaction/1_invoke.txt
curl -o ./get_transaction/1_invoke.txt "https://external.integration.starknet.io/feeder_gateway/get_transaction?transactionHash=0x6c26da8c26aa61dc40ed36b9078536f3b5b0532e884a8f0b7488480580bf3c9"

# ./get_transaction/2_deploy.txt
curl -o ./get_transaction/2_deploy.txt "https://external.integration.starknet.io/feeder_gateway/get_transaction?transactionHash=0x2cce6f468d865bf93476c7a96b7ce0ca3d26a6ffbb4ba93a027a67f0d2e2773"

# ./get_transaction/3_not_received.txt
curl -o ./get_transaction/3_not_received.txt "https://external.integration.starknet.io/feeder_gateway/get_transaction?transactionHash=0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"

# NOTE: transaction with the same criteria not found in goerli-integration yet
# ./get_transaction/4_failure.txt
curl -o ./get_transaction/4_failure.txt "https://external.integration.starknet.io/feeder_gateway/get_transaction?transactionHash=0x1"

# ./get_transaction/5_declare_v1.txt
curl -o ./get_transaction/5_declare_v1.txt "https://external.integration.starknet.io/feeder_gateway/get_transaction?transactionHash=0x42d25cc0876a5885be10d59d9d665821ac17965fcb47dd6c0c633dbcc7c4bb6"

# ./get_transaction/6_declare_v2.txt
curl -o ./get_transaction/6_declare_v2.txt "https://external.integration.starknet.io/feeder_gateway/get_transaction?transactionHash=0x722b666ce83ec69c18190aae6149f79e6ad4b9c051b171cc6c309c9e0c28129"

# NOTE: transaction with the same criteria not found in goerli-integration yet
# ./get_transaction/7_reverted.txt
curl -o ./get_transaction/7_reverted.txt "https://external.integration.starknet.io/feeder_gateway/get_transaction?transactionHash=0x1"

# ./get_transaction/8_invoke_v3.txt
curl -o ./get_transaction/8_invoke_v3.txt "https://external.integration.starknet.io/feeder_gateway/get_transaction?transactionHash=0x4eaf2732c8459e8de553f50d6aeb989f4b1173a9ff5579a55e8aea8b01d0a44"

# ./get_transaction/9_declare_v3.txt
curl -o ./get_transaction/9_declare_v3.txt "https://external.integration.starknet.io/feeder_gateway/get_transaction?transactionHash=0x41d1f5206ef58a443e7d3d1ca073171ec25fa75313394318fc83a074a6631c3"

# ./get_transaction/10_deploy_account_v3.txt
curl -o ./get_transaction/10_deploy_account_v3.txt "https://external.integration.starknet.io/feeder_gateway/get_transaction?transactionHash=0x29fd7881f14380842414cdfdd8d6c0b1f2174f8916edcfeb1ede1eb26ac3ef0"

# ./get_transaction_status/1_accepted.txt
curl -o ./get_transaction_status/1_accepted.txt "https://external.integration.starknet.io/feeder_gateway/get_transaction_status?transactionHash=0x6c26da8c26aa61dc40ed36b9078536f3b5b0532e884a8f0b7488480580bf3c9"

# ./get_transaction_status/2_not_received.txt
curl -o ./get_transaction_status/2_not_received.txt "https://external.integration.starknet.io/feeder_gateway/get_transaction_status?transactionHash=0x7cb73f737a8ea0c5c94d7799c2d01a47c81f4cb34287408741264d3f09655d"

# NOTE: transaction with the same criteria not found in goerli-integration yet
# ./get_transaction_status/3_failure.txt
curl -o ./get_transaction_status/3_failure.txt "https://external.integration.starknet.io/feeder_gateway/get_transaction_status?transactionHash=0x1"

# NOTE: transaction with the same criteria not found in goerli-integration yet
# ./get_transaction_status/4_reverted.txt
curl -o ./get_transaction_status/4_reverted.txt "https://external.integration.starknet.io/feeder_gateway/get_transaction_status?transactionHash=0x1"

# ./get_state_update/1_success.txt
curl -o ./get_state_update/1_success.txt "https://external.integration.starknet.io/feeder_gateway/get_state_update?blockNumber=70004"

# ./get_state_update/2_pending_block.txt (non-deterministic)
curl -o ./get_state_update/2_pending_block.txt "https://external.integration.starknet.io/feeder_gateway/get_state_update?blockNumber=pending"

# ./get_state_update/3_with_declarations.txt
curl -o ./get_state_update/3_with_declarations.txt "https://external.integration.starknet.io/feeder_gateway/get_state_update?blockNumber=283364"

# ./get_state_update/4_with_nonce_changes.txt
curl -o ./get_state_update/4_with_nonce_changes.txt "https://external.integration.starknet.io/feeder_gateway/get_state_update?blockNumber=306608"

# ./get_state_update/5_with_declare_v2.txt
curl -o ./get_state_update/5_with_declare_v2.txt "https://external.integration.starknet.io/feeder_gateway/get_state_update?blockNumber=283364"

# ./get_state_update/6_with_replaced_classes.txt
# NOTE: block with the same criteria not found in goerli-integration yet
curl -o ./get_state_update/6_with_replaced_classes.txt "https://external.integration.starknet.io/feeder_gateway/get_state_update?blockNumber=1"

# ./get_transaction_trace/1_with_messages.txt
curl -o ./get_transaction_trace/1_with_messages.txt "https://external.integration.starknet.io/feeder_gateway/get_transaction_trace?transactionHash=0xbf7dc15a05a3a20af67cede6d6b01dc51527a66c76d62e65e3786f6859658d"

# ./get_transaction_trace/2_with_events.txt
curl -o ./get_transaction_trace/2_with_events.txt "https://external.integration.starknet.io/feeder_gateway/get_transaction_trace?transactionHash=0x7d0d0dcc9e8d18a70c7ee767132be57fef01182c5924adeb5796fcbc8bee967"

# ./get_transaction_trace/3_with_call_type.txt
curl -o ./get_transaction_trace/3_with_call_type.txt "https://external.integration.starknet.io/feeder_gateway/get_transaction_trace?transactionHash=0x4eaf2732c8459e8de553f50d6aeb989f4b1173a9ff5579a55e8aea8b01d0a44"

# NOTE: transaction with the same criteria not found in goerli-integration yet
# ./get_transaction_trace/4_with_validation.txt
curl -o ./get_transaction_trace/4_with_validation.txt "https://external.integration.starknet.io/feeder_gateway/get_transaction_trace?transactionHash=0x1"

# ./get_class_by_hash/1_cairo_0.txt
curl -o ./get_class_by_hash/1_cairo_0.txt "https://external.integration.starknet.io/feeder_gateway/get_class_by_hash?classHash=0x025ec026985a3bf9d0cc1fe17326b245dfdc3ff89b8fde106542a3ea56c5a918"

# ./get_class_by_hash/2_not_declared.txt
curl -o ./get_class_by_hash/2_not_declared.txt "https://external.integration.starknet.io/feeder_gateway/get_class_by_hash?classHash=0x111111111111111111111111"

# ./get_class_by_hash/3_cairo_1.txt
curl -o ./get_class_by_hash/3_cairo_1.txt "https://external.integration.starknet.io/feeder_gateway/get_class_by_hash?classHash=0x01a736d6ed154502257f02b1ccdf4d9d1089f80811cd6acad48e6b6a9d1f2003"

# ./get_block_traces/1_success.txt
curl -o ./get_block_traces/1_success.txt "https://external.integration.starknet.io/feeder_gateway/get_block_traces?blockNumber=267588"
