use futures::StreamExt;
use uniswap_dai_usd_monitor::blocks_handler::BlocksHandler;
use uniswap_dai_usd_monitor::web3_client::Web3BlocksFetcher;
use uniswap_dai_usd_monitor::{setup_web3, BLOCK_CONFIRMATIONS};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    log::info!("🚀 App launched. Fetching blocks...");

    let web3 = setup_web3().await?;
    let web3_blocks_fetcher = Web3BlocksFetcher { web3: web3.clone() };
    let mut blocks_handler = BlocksHandler::new(BLOCK_CONFIRMATIONS, web3_blocks_fetcher)?;

    let mut block_stream = web3.eth_subscribe().subscribe_new_heads().await?;
    while let Some(Ok(block_header)) = block_stream.next().await {
        blocks_handler
            .handle_block(block_header)
            .await
            .map_err(anyhow::Error::msg)?;
    }

    Ok(())
}
