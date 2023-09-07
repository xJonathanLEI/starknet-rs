use starknet_macros::felt;
use starknet_providers::{Provider, SequencerGatewayProvider};

#[tokio::main]
async fn main() {
    let provider = SequencerGatewayProvider::starknet_alpha_goerli();

    // https://testnet.starkscan.co/tx/0x023cffcb294f338aad5c8351e0a5d49db6625f09e1df6ac5ebc06649bfbd1345#overview
    let hash = felt!("0x023cffcb294f338aad5c8351e0a5d49db6625f09e1df6ac5ebc06649bfbd1345");
    let tx_trace = provider.trace_transaction(hash).await.unwrap();
    println!("{:?}", tx_trace);
    dbg!(tx_trace);

    // https://testnet.starkscan.co/block/0x7991a152b4a8d4e9a2d424808a93ad5ae2f4698a6b8e04d0c043f7e4996aabf
    let hash = felt!("0x7991a152b4a8d4e9a2d424808a93ad5ae2f4698a6b8e04d0c043f7e4996aabf");
    let block_trace = provider.trace_block_transactions(hash).await.unwrap();
    println!("{:?}", block_trace);
    dbg!(block_trace);
}
