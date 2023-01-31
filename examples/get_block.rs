use starknet::{
    core::types::BlockId,
    providers::{Provider, SequencerGatewayProvider},
};

#[tokio::main]
async fn main() {
    let provider = SequencerGatewayProvider::starknet_alpha_goerli();
    let latest_block = provider.get_block(BlockId::Latest).await;
    println!("{latest_block:#?}");
}
