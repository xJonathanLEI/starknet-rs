#!/bin/sh

# ./get_block/1_with_transactions.txt
curl -o ./get_block/1_with_transactions.txt "https://alpha4.starknet.io/feeder_gateway/get_block?blockNumber=39232"

# ./get_block/2_with_messages.txt
# (Changed from 39227 to 122387 due to a bug in StarkNet)
curl -o ./get_block/2_with_messages.txt "https://alpha4.starknet.io/feeder_gateway/get_block?blockNumber=122387"

# ./get_block/3_with_events.txt
curl -o ./get_block/3_with_events.txt "https://alpha4.starknet.io/feeder_gateway/get_block?blockNumber=47543"

# ./get_block/4_pending.txt (non-deterministic)
curl -o ./get_block/4_pending.txt "https://alpha4.starknet.io/feeder_gateway/get_block?blockNumber=pending"

# ./get_block/5_with_class_hash_and_actual_fee.txt
curl -o ./get_block/5_with_class_hash_and_actual_fee.txt "https://alpha4.starknet.io/feeder_gateway/get_block?blockNumber=156800"

# ./get_code/1_code.txt
curl -o ./get_code/1_code.txt "https://alpha4.starknet.io/feeder_gateway/get_code?contractAddress=0x05ffd28b3ff2eecd6da0fa64c90e928a9f46f1563976a4fe1770ab48ee43506a"

# ./get_code/2_all_abi_types.txt
curl -o ./get_code/2_all_abi_types.txt "https://alpha4.starknet.io/feeder_gateway/get_code?contractAddress=0x06ef97a90be1c0458f6e7bd1faf05021f2d81211f658155df0c5c97a39eb2d12"

# ./get_transaction/1_invoke.txt
curl -o ./get_transaction/1_invoke.txt "https://alpha4.starknet.io/feeder_gateway/get_transaction?transactionHash=0x10f2462bd8d90ad7242f16c5432f5ca6a53d2846592c6170242e032a5f836a"

# ./get_transaction/2_deploy.txt
curl -o ./get_transaction/2_deploy.txt "https://alpha4.starknet.io/feeder_gateway/get_transaction?transactionHash=0xe624219d7df629bccc9fef9d38b47045a47377938982ad2622580738575f97"

# ./get_transaction/3_not_received.txt
curl -o ./get_transaction/3_not_received.txt "https://alpha4.starknet.io/feeder_gateway/get_transaction?transactionHash=0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"

# ./get_transaction/4_failure.txt
curl -o ./get_transaction/4_failure.txt "https://alpha4.starknet.io/feeder_gateway/get_transaction?transactionHash=0x533d327336f28b973c7f06f7abf15af7f8fd5a1bd4e1991397ae59e49a59885"

# ./get_transaction_receipt/1_accepted.txt
curl -o ./get_transaction_receipt/1_accepted.txt "https://alpha4.starknet.io/feeder_gateway/get_transaction_receipt?transactionHash=0x7cb73f737a8ea0c5c94d7799c2d01a47c81f4cb34287408741264d3f09655da"

# ./get_transaction_receipt/2_not_received.txt
curl -o ./get_transaction_receipt/2_not_received.txt "https://alpha4.starknet.io/feeder_gateway/get_transaction_receipt?transactionHash=0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"

# ./get_transaction_receipt/3_with_events.txt
curl -o ./get_transaction_receipt/3_with_events.txt "https://alpha4.starknet.io/feeder_gateway/get_transaction_receipt?transactionHash=0x688e434a1636c30d0c161f766b99b4bfb143208d859149859941905e94cb022"

# ./get_transaction_receipt/4_failure.txt
curl -o ./get_transaction_receipt/4_failure.txt "https://alpha4.starknet.io/feeder_gateway/get_transaction_receipt?transactionHash=0x533d327336f28b973c7f06f7abf15af7f8fd5a1bd4e1991397ae59e49a59885"

# ./get_transaction_status/1_accepted.txt
curl -o ./get_transaction_status/1_accepted.txt "https://alpha4.starknet.io/feeder_gateway/get_transaction_status?transactionHash=0x7cb73f737a8ea0c5c94d7799c2d01a47c81f4cb34287408741264d3f09655da"

# ./get_transaction_status/2_not_received.txt
curl -o ./get_transaction_status/2_not_received.txt "https://alpha4.starknet.io/feeder_gateway/get_transaction_status?transactionHash=0x7cb73f737a8ea0c5c94d7799c2d01a47c81f4cb34287408741264d3f09655d"

# ./get_transaction_status/3_failure.txt
curl -o ./get_transaction_status/3_failure.txt "https://alpha4.starknet.io/feeder_gateway/get_transaction_status?transactionHash=0x533d327336f28b973c7f06f7abf15af7f8fd5a1bd4e1991397ae59e49a59885"

# ./get_state_update/1_success.txt
curl -o ./get_state_update/1_success.txt "https://alpha4.starknet.io/feeder_gateway/get_state_update?blockNumber=70004"

# ./get_state_update/2_pending_block.txt (non-deterministic)
curl -o ./get_state_update/2_pending_block.txt "https://alpha4.starknet.io/feeder_gateway/get_state_update?blockNumber=pending"

# ./get_full_contract/1_code.txt
curl -o ./get_full_contract/1_code.txt "https://alpha4.starknet.io/feeder_gateway/get_full_contract?contractAddress=0x05ffd28b3ff2eecd6da0fa64c90e928a9f46f1563976a4fe1770ab48ee43506a"

# ./get_full_contract/2_all_abi_types.txt
curl -o ./get_full_contract/2_all_abi_types.txt "https://alpha4.starknet.io/feeder_gateway/get_full_contract?contractAddress=0x06ef97a90be1c0458f6e7bd1faf05021f2d81211f658155df0c5c97a39eb2d12"

# ./get_transaction_trace/1_with_messages.txt
curl -o ./get_transaction_trace/1_with_messages.txt "https://alpha4.starknet.io/feeder_gateway/get_transaction_trace?transactionHash=0x24052dd62bf28d6dfa7056fcc7208b27f7260099572bac42d716bf629f46991"

# ./get_transaction_trace/2_with_events.txt
curl -o ./get_transaction_trace/2_with_events.txt "https://alpha4.starknet.io/feeder_gateway/get_transaction_trace?transactionHash=0x688e434a1636c30d0c161f766b99b4bfb143208d859149859941905e94cb022"

# ./estimate_fee/1_success.txt
curl -o ./estimate_fee/1_success.txt "https://alpha4.starknet.io/feeder_gateway/estimate_fee" \
    -X POST \
    -H "Content-Type: application/json" \
    -d '{
        "contract_address": "0x07394cbe418daa16e42b87ba67372d4ab4a5df0b05c6e554d158458ce245bc10",
        "entry_point_selector": "0x02f0b3c5710379609eb5495f1ecd348cb28167711b73609fe565a72734550354",
        "calldata": [
            "2536338608804621486891098924999890751656158566880912297504415061810375427475",
            "1000000000000000000000",
            "0"
        ],
        "signature": []
    }'
