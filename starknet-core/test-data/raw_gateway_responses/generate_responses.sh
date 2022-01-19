#!/bin/sh

# ./get_block/1_with_deploy_tx.txt
curl -o ./get_block/1_with_deploy_tx.txt "https://alpha4.starknet.io/feeder_gateway/get_block?blockNumber=39232"

# ./get_block/2_with_messages.txt
curl -o ./get_block/2_with_messages.txt "https://alpha4.starknet.io/feeder_gateway/get_block?blockNumber=39227"

# ./get_block/3_with_events.txt
curl -o ./get_block/3_with_events.txt "https://alpha4.starknet.io/feeder_gateway/get_block?blockNumber=47543"

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

# ./get_transaction_receipt/1_accepted.txt
curl -o ./get_transaction_receipt/1_accepted.txt "https://alpha4.starknet.io/feeder_gateway/get_transaction_receipt?transactionHash=0x7cb73f737a8ea0c5c94d7799c2d01a47c81f4cb34287408741264d3f09655da"

# ./get_transaction_receipt/2_not_received.txt
curl -o ./get_transaction_receipt/2_not_received.txt "https://alpha4.starknet.io/feeder_gateway/get_transaction_receipt?transactionHash=0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"

# ./get_transaction_receipt/3_with_events.txt
curl -o ./get_transaction_receipt/3_with_events.txt "https://alpha4.starknet.io/feeder_gateway/get_transaction_receipt?transactionHash=0x688e434a1636c30d0c161f766b99b4bfb143208d859149859941905e94cb022"
