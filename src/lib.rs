pub mod blocks_handler;
pub mod events_handler;
pub mod swap_details;
pub mod web3_client;

use dotenv::dotenv;
use std::env;
use web3::{error::Error as Web3Error, transports::WebSocket, Web3};

pub const BLOCK_CONFIRMATIONS: u64 = 5;

pub async fn setup_web3() -> Result<Web3<WebSocket>, Web3Error> {
    dotenv().ok();

    let ws_endpoint = env::var("WEBSOCKET_ENDPOINT").expect("Couldn't load WEBSOCKET_ENDPOINT");
    let transport = WebSocket::new(ws_endpoint.as_str()).await?;
    Ok(Web3::new(transport))
}
