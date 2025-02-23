use serde_json::{json, Value};
use tokio::fs;
use uniswap_dai_usd_monitor::setup_web3;
use web3::types::BlockHeader;
use web3::Transport;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let web3 = setup_web3().await?;

    let start_block = 21836327;
    let end_block = start_block + 10;
    println!("Blocks range: {} - {}", start_block, end_block);

    let mut headers = Vec::new();
    for block_number in start_block..=end_block {
        println!("Fetching block: {}", block_number);
        let params = vec![json!(format!("0x{:x}", block_number)), json!(false)];
        let response: Value = web3
            .transport()
            .execute("eth_getBlockByNumber", params)
            .await?;

        let header: BlockHeader = serde_json::from_value(response)?;
        headers.push(header);
    }

    fs::create_dir_all("tests/fixtures").await?;
    let json = serde_json::to_string_pretty(&headers)?;
    fs::write("tests/fixtures/block_headers.json", json).await?;

    println!("✅ Generated {} block headers", headers.len());
    Ok(())
}
