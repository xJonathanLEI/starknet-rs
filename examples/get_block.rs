use starknet::{
    core::types::{BlockId, BlockTag},
    providers::{Provider, SequencerGatewayProvider},
};

#[tokio::main]
async fn main() {
    let provider = SequencerGatewayProvider::starknet_alpha_goerli();
    let latest_block = provider
        .get_block_with_tx_hashes(BlockId::Tag(BlockTag::Latest))
        .await;
    println!("{latest_block:#?}");
}
