use anyhow::Context;
use async_trait::async_trait;
use web3::{
    transports::WebSocket,
    types::{BlockId, H256},
    Web3,
};

#[async_trait]
#[cfg_attr(test, mockall::automock)]
pub trait BlocksFetcher {
    async fn get_block_hash(&self, block_number: u64) -> Result<H256, anyhow::Error>;
    fn web3(&self) -> Web3<WebSocket>;
}

#[derive(Clone)]
pub struct Web3BlocksFetcher {
    pub web3: Web3<WebSocket>,
}

#[async_trait]
impl BlocksFetcher for Web3BlocksFetcher {
    async fn get_block_hash(&self, block_number: u64) -> Result<H256, anyhow::Error> {
        let block_id = BlockId::Number(block_number.into());
        let block = self
            .web3
            .eth()
            .block(block_id)
            .await
            .context("Failed to fetch block")?;
        let block = block.unwrap();
        let hash = block.hash.unwrap();
        Ok(hash)
    }

    fn web3(&self) -> Web3<WebSocket> {
        self.web3.clone()
    }
}
