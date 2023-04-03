#!/bin/sh

# ./get_block/1_with_transactions.txt
curl -o ./get_block/1_with_transactions.txt "https://alpha4.starknet.io/feeder_gateway/get_block?blockNumber=39232"

# ./get_block/2_with_messages.txt
# (Changed from 39227 to 122387 due to a bug in Starknet)
curl -o ./get_block/2_with_messages.txt "https://alpha4.starknet.io/feeder_gateway/get_block?blockNumber=122387"

# ./get_block/3_with_events.txt
curl -o ./get_block/3_with_events.txt "https://alpha4.starknet.io/feeder_gateway/get_block?blockNumber=47543"

# ./get_block/4_pending.txt (non-deterministic)
curl -o ./get_block/4_pending.txt "https://alpha4.starknet.io/feeder_gateway/get_block?blockNumber=pending"

# ./get_block/5_with_class_hash_and_actual_fee.txt
curl -o ./get_block/5_with_class_hash_and_actual_fee.txt "https://alpha4.starknet.io/feeder_gateway/get_block?blockNumber=156800"

# ./get_block/6_with_sequencer_address.txt
curl -o ./get_block/6_with_sequencer_address.txt "https://alpha4.starknet.io/feeder_gateway/get_block?blockNumber=183471"

# ./get_block/7_with_declare_tx.txt
curl -o ./get_block/7_with_declare_tx.txt "https://alpha4.starknet.io/feeder_gateway/get_block?blockNumber=232491"

# ./get_block/8_with_starknet_version.txt
curl -o ./get_block/8_with_starknet_version.txt "https://alpha4.starknet.io/feeder_gateway/get_block?blockNumber=273155"

# ./get_block/9_with_messages_without_nonce.txt
curl -o ./get_block/9_with_messages_without_nonce.txt "https://alpha4.starknet.io/feeder_gateway/get_block?blockNumber=1564"

# ./get_block/10_with_l1_handler.txt
curl -o ./get_block/10_with_l1_handler.txt "https://alpha4.starknet.io/feeder_gateway/get_block?blockNumber=322735"

# ./get_block/11_without_execution_resources.txt
curl -o ./get_block/11_without_execution_resources.txt "https://alpha4.starknet.io/feeder_gateway/get_block?blockNumber=324209"

# ./get_block/12_l1_handler_without_nonce.txt
curl -o ./get_block/12_l1_handler_without_nonce.txt "https://alpha-mainnet.starknet.io/feeder_gateway/get_block?blockNumber=192"

# ./get_block/13_without_entry_point.txt
curl -o ./get_block/13_without_entry_point.txt "https://alpha4.starknet.io/feeder_gateway/get_block?blockNumber=375873"

# ./get_block/14_deploy_account.txt
curl -o ./get_block/14_deploy_account.txt "https://alpha4.starknet.io/feeder_gateway/get_block?blockNumber=375919"

# ./get_block/15_declare_v2.txt
curl -o ./get_block/15_declare_v2.txt "https://alpha4.starknet.io/feeder_gateway/get_block?blockNumber=788322"

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

# ./get_transaction/5_declare_v1.txt
curl -o ./get_transaction/5_declare_v1.txt "https://alpha4.starknet.io/feeder_gateway/get_transaction?transactionHash=0x6943a2586593262662f04bfd45e4a144f18c80353a4129b854443b79197c0cc"

# ./get_transaction/6_declare_v2.txt
curl -o ./get_transaction/6_declare_v2.txt "https://alpha4.starknet.io/feeder_gateway/get_transaction?transactionHash=0x4c0955d59cd105eb916d0cf24eb12c01be435fe83d95cd184e32a035c85c1d3"

# ./get_transaction_receipt/1_accepted.txt
curl -o ./get_transaction_receipt/1_accepted.txt "https://alpha4.starknet.io/feeder_gateway/get_transaction_receipt?transactionHash=0x7cb73f737a8ea0c5c94d7799c2d01a47c81f4cb34287408741264d3f09655da"

# ./get_transaction_receipt/2_not_received.txt
curl -o ./get_transaction_receipt/2_not_received.txt "https://alpha4.starknet.io/feeder_gateway/get_transaction_receipt?transactionHash=0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"

# ./get_transaction_receipt/3_with_events.txt
curl -o ./get_transaction_receipt/3_with_events.txt "https://alpha4.starknet.io/feeder_gateway/get_transaction_receipt?transactionHash=0x688e434a1636c30d0c161f766b99b4bfb143208d859149859941905e94cb022"

# ./get_transaction_receipt/4_failure.txt
curl -o ./get_transaction_receipt/4_failure.txt "https://alpha4.starknet.io/feeder_gateway/get_transaction_receipt?transactionHash=0x533d327336f28b973c7f06f7abf15af7f8fd5a1bd4e1991397ae59e49a59885"

# ./get_transaction_receipt/5_declare_v1.txt
curl -o ./get_transaction_receipt/5_declare_v1.txt "https://alpha4.starknet.io/feeder_gateway/get_transaction_receipt?transactionHash=0x6943a2586593262662f04bfd45e4a144f18c80353a4129b854443b79197c0cc"

# ./get_transaction_receipt/6_declare_v2.txt
curl -o ./get_transaction_receipt/6_declare_v2.txt "https://alpha4.starknet.io/feeder_gateway/get_transaction_receipt?transactionHash=0x4c0955d59cd105eb916d0cf24eb12c01be435fe83d95cd184e32a035c85c1d3"

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

# ./get_state_update/3_with_declarations.txt
curl -o ./get_state_update/3_with_declarations.txt "https://alpha4.starknet.io/feeder_gateway/get_state_update?blockNumber=267588"

# ./get_state_update/4_with_nonce_changes.txt
curl -o ./get_state_update/4_with_nonce_changes.txt "https://alpha4.starknet.io/feeder_gateway/get_state_update?blockNumber=326608"

# ./get_state_update/5_with_declare_v2.txt
curl -o ./get_state_update/5_with_declare_v2.txt "https://alpha4.starknet.io/feeder_gateway/get_state_update?blockNumber=788322"

# ./get_state_update/6_with_replaced_classes.txt
curl -o ./get_state_update/6_with_replaced_classes.txt "https://alpha4.starknet.io/feeder_gateway/get_state_update?blockNumber=788504"

# ./get_full_contract/1_cairo_0.txt
curl -o ./get_full_contract/1_cairo_0.txt "https://alpha4.starknet.io/feeder_gateway/get_full_contract?contractAddress=0x06ef97a90be1c0458f6e7bd1faf05021f2d81211f658155df0c5c97a39eb2d12"

# ./get_full_contract/2_cairo_1.txt
curl -o ./get_full_contract/2_cairo_1.txt "https://alpha4.starknet.io/feeder_gateway/get_full_contract?contractAddress=0x4806a00281e11427558c2e3f6d6c036dbf9aa72a0cd6cf1037737a41357fe46"

# ./get_transaction_trace/1_with_messages.txt
curl -o ./get_transaction_trace/1_with_messages.txt "https://alpha4.starknet.io/feeder_gateway/get_transaction_trace?transactionHash=0x24052dd62bf28d6dfa7056fcc7208b27f7260099572bac42d716bf629f46991"

# ./get_transaction_trace/2_with_events.txt
curl -o ./get_transaction_trace/2_with_events.txt "https://alpha4.starknet.io/feeder_gateway/get_transaction_trace?transactionHash=0x688e434a1636c30d0c161f766b99b4bfb143208d859149859941905e94cb022"

# ./get_transaction_trace/3_with_call_type.txt
curl -o ./get_transaction_trace/3_with_call_type.txt "https://alpha4.starknet.io/feeder_gateway/get_transaction_trace?transactionHash=0x4afe0ec31686c150ec8ca315823ae239e7150db43352fb65c9feb423ef23516"

# ./get_transaction_trace/4_with_validation.txt
curl -o ./get_transaction_trace/4_with_validation.txt "https://alpha4.starknet.io/feeder_gateway/get_transaction_trace?transactionHash=0x0115e5f3388f4aecefc359bce5512e8d5b39e20485ac27c9ed7d92a4b9ce1f1b"

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

# ./get_storage_at/1_empty.txt
curl -o ./get_storage_at/1_empty.txt "https://alpha4.starknet.io/feeder_gateway/get_storage_at?contractAddress=0x1&key=2"

# ./get_class_by_hash/1_cairo_0.txt
curl -o ./get_class_by_hash/1_cairo_0.txt "https://alpha4.starknet.io/feeder_gateway/get_class_by_hash?classHash=0x051ad5a4a7944af79eb7bb7993f7f801c19ce0faaf855e6a17549b100c698122"

# ./get_class_by_hash/2_not_declared.txt
curl -o ./get_class_by_hash/2_not_declared.txt "https://alpha4.starknet.io/feeder_gateway/get_class_by_hash?classHash=0x111111111111111111111111"

# ./get_class_by_hash/3_cairo_1.txt
curl -o ./get_class_by_hash/3_cairo_1.txt "https://alpha4.starknet.io/feeder_gateway/get_class_by_hash?classHash=0x571f4a4a5caae490577d67870c431fa560d72038e4c7a270d91aee1112d55d0"

# ./get_block_traces/1_success.txt
curl -o ./get_block_traces/1_success.txt "https://alpha4.starknet.io/feeder_gateway/get_block_traces?blockNumber=267588"

# ./estimate_fee_bulk/1_success.txt
curl -o ./estimate_fee_bulk/1_success.txt "https://alpha4.starknet.io/feeder_gateway/estimate_fee_bulk" \
    -X POST \
    -H "Content-Type: application/json" \
    -d '[{
        "type": "INVOKE_FUNCTION",
        "version": "0x1",
        "contract_address": "0x5b5e9f6f6fb7d2647d81a8b2c2b99cbc9cc9d03d705576d7061812324dca5c0",
        "calldata": [
            "1",
            "3267429884791031784129188059026496191501564961518175231747906707757621165072",
            "1329909728320632088402217562277154056711815095720684343816173432540100887380",
            "0",
            "3",
            "3",
            "2582978326697182094925044915479529632446801760547577461724830811224889140672",
            "1000000000000000000000",
            "0"
        ],
        "signature": [
            "605417791026644483670811513828340231819682850475940872862750374884434792160",
            "3227162751686940146996647969343636789208985440255179192147422777151505011910"
        ],
        "max_fee": "0x0",
        "nonce": "0x0"
    }, {
        "type": "INVOKE_FUNCTION",
        "version": "0x1",
        "contract_address": "0x5b5e9f6f6fb7d2647d81a8b2c2b99cbc9cc9d03d705576d7061812324dca5c0",
        "calldata": [
            "1",
            "3267429884791031784129188059026496191501564961518175231747906707757621165072",
            "1329909728320632088402217562277154056711815095720684343816173432540100887380",
            "0",
            "3",
            "3",
            "2582978326697182094925044915479529632446801760547577461724830811224889140672",
            "2000000000000000000000",
            "0"
        ],
        "signature": [
            "2454731969569471949423549779477272094056061808345298145925675439909833863557",
            "724612237028642548263407980387909582237336146127278825566903814475468042134"
        ],
        "max_fee": "0x0",
        "nonce": "0x1"
    }]'
