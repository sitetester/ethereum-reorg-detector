use crate::{events_handler::EventsHandler, web3_client::BlocksFetcher};
use anyhow::bail;
use log::{debug, info};
use std::collections::BTreeMap;
use web3::types::BlockHeader;

type BlockNumerWithBlockInfo = BTreeMap<u64, BlockHeader>;

pub struct BlocksHandler<T: BlocksFetcher> {
    block_confirmations: u64,
    blocks_fetcher: T,
    previous_blocks: BlockNumerWithBlockInfo,
    starting_block_number: u64,
}

impl<T: BlocksFetcher> BlocksHandler<T> {
    pub fn new(block_confirmations: u64, blocks_fetcher: T) -> Result<Self, anyhow::Error> {
        Ok(Self {
            block_confirmations,
            blocks_fetcher,
            previous_blocks: BTreeMap::new(),
            starting_block_number: 0,
        })
    }

    pub async fn handle_block(&mut self, block_header: BlockHeader) -> Result<(), anyhow::Error> {
        let block_number = block_header.number.unwrap().as_u64();
        info!(
            "current block: {}, hash: {:?}",
            block_number,
            block_header.hash.unwrap()
        );

        // first block (e.g., 21836327)
        if self.starting_block_number == 0 {
            self.starting_block_number = block_number;

            self.previous_blocks.insert(block_number, block_header);
            return Ok(());
        }

        // `?` will cause return early in case of mismatch
        self.match_parent_hash(block_number - 1, &block_header)?;

        let diff = block_number - self.starting_block_number;
        debug!(
            "diff = {} ({} - {})",
            diff, block_number, self.starting_block_number
        );

        // for `diff == 1`, do nothing, as match_parent_hash check already applied
        if diff > 1 {
            let (start, end) = self.get_blocks_range(diff);
            self.match_previous_blocks_hashes(start, end).await?;
        }

        self.previous_blocks.insert(block_number, block_header);

        // cleanup, remove block when we reach N+5 confirmations
        if diff == self.block_confirmations {
            let starting_block = self
                .previous_blocks
                .get(&self.starting_block_number)
                .cloned();
            let starting_block_hash = starting_block.unwrap().hash.unwrap();
            debug!(
                "✅ N+5 condition met. Fetching events for block: {} with hash: {:?}",
                self.starting_block_number, starting_block_hash
            );
            let target_block = self.starting_block_number;
            // but first show relevant events
            let events_handler = EventsHandler::new(self.blocks_fetcher.web3())?;
            let swap_info = events_handler.handle_events(starting_block_hash).await?;
            if swap_info.is_empty() {
                debug!("events not found");
            } else {
                info!("swap info: {:#?}", swap_info);
            }

            self.previous_blocks.remove(&target_block);
            debug!("⛔ block: {} untracked", target_block);
            self.starting_block_number += 1;
            debug!("-----------------------");
        }

        Ok(())
    }

    fn match_parent_hash(
        &mut self,
        previous_block_number: u64,
        block_header: &BlockHeader,
    ) -> Result<(), anyhow::Error> {
        let may_be_previous_block_header =
            self.previous_blocks.get(&previous_block_number).cloned();
        if let Some(previous_block_header) = may_be_previous_block_header {
            let previous_block_hash = previous_block_header.hash.unwrap();
            if block_header.parent_hash != previous_block_hash {
                // (:?) is needed for H256 type to properly display the full hex representation of the hash
                bail!(
                    "parent_hash mismatch. Previous: {:?}, current: {:?}",
                    previous_block_hash,
                    block_header.parent_hash
                );
            }
            debug!(
                "√ parent hash matched for previous block: {}",
                previous_block_number
            );
        }

        Ok(())
    }

    /// Check relevant tests for different inputs
    fn get_blocks_range(&mut self, diff: u64) -> (u64, u64) {
        if diff == 2 {
            return (self.starting_block_number, self.starting_block_number);
        }

        let start = self.starting_block_number;
        let end = self.starting_block_number + (diff - 2);
        (start, end)
    }

    /// Compares the hashes of freshly fetched blocks
    /// # Arguments
    /// * start - starting block num
    /// * end - ending block num
    async fn match_previous_blocks_hashes(
        &mut self,
        start: u64,
        end: u64,
    ) -> Result<(), anyhow::Error> {
        debug!("start: {}, end: {}", start, end);
        for block_num in start..=end {
            let new_hash = self.blocks_fetcher.get_block_hash(block_num).await?;
            let stored_block = self.previous_blocks.get(&block_num).cloned().unwrap();
            let previous_hash = stored_block.hash.unwrap();
            if new_hash != previous_hash {
                // {:?} will show FULL hash in error message, otherwise, something like `0x69d5…cc0b`
                bail!(
                    "Reorg detected at block {}. Previous hash: {:?}, New hash: {:?}",
                    block_num,
                    previous_hash,
                    new_hash
                );
            }
            debug!("new hash matched for block: {}", block_num);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        setup_web3,
        web3_client::{MockBlocksFetcher, Web3BlocksFetcher},
        BLOCK_CONFIRMATIONS,
    };
    use mockall::predicate::eq;
    use std::str::FromStr;
    use tokio::fs;
    use web3::types::H256;

    /// Check README.md on how to load fixtures
    async fn load_fixtures() -> Vec<BlockHeader> {
        let json = fs::read_to_string("tests/fixtures/block_headers.json")
            .await
            .expect("Failed to read fixtures file");
        serde_json::from_str(&json).expect("Failed to parse blocks")
    }

    async fn get_blocks_handler() -> BlocksHandler<Web3BlocksFetcher> {
        let web3 = setup_web3().await.unwrap();
        let blocks_fetcher = Web3BlocksFetcher { web3 };
        BlocksHandler::new(BLOCK_CONFIRMATIONS, blocks_fetcher).unwrap()
    }

    #[tokio::test]
    async fn test_get_blocks_range() {
        let mut blocks_handler = get_blocks_handler().await;

        // let's say, we started from block 21836327 & currently on 21836329, so diff = 2
        // in this case, it should return (21836327, 21836327), since the check for previous block (21836328) was
        // already performed via `match_parent_hash(..)`
        blocks_handler.starting_block_number = 21836327;
        assert_eq!(
            blocks_handler.get_blocks_range(2),
            (21836327_u64, 21836327_u64)
        );

        // next we are at 21836330, it should return (21836327, 21836328), as the check for previous block (21836329) was
        // already performed via `match_parent_hash(..)`
        assert_eq!(
            blocks_handler.get_blocks_range(3),
            (21836327_u64, 21836328_u64)
        );

        // for 21836331, diff = 4
        assert_eq!(
            blocks_handler.get_blocks_range(4),
            (21836327_u64, 21836329_u64)
        );

        // for 21836332, diff = 5
        assert_eq!(
            blocks_handler.get_blocks_range(5),
            (21836327_u64, 21836330_u64)
        );
    }

    #[tokio::test]
    async fn test_verify_can_process_first_5_blocks() {
        let headers = load_fixtures().await;

        // here, we are going with real blocks_fetcher
        let mut blocks_handler = get_blocks_handler().await;
        for header in headers.iter().take(5) {
            blocks_handler.handle_block(header.clone()).await.unwrap(); // `unwrap` will cause failure
        }
    }

    #[tokio::test]
    async fn test_verify_reorg_detected_for_parent_hash_mismatch() {
        let headers = load_fixtures().await;
        let mut blocks_handler = get_blocks_handler().await;

        blocks_handler
            .handle_block(headers[0].clone())
            .await
            .unwrap();

        blocks_handler
            .handle_block(headers[1].clone())
            .await
            .unwrap();

        // let's modify parent_hash for 3rd block
        let mut block_header = headers[2].clone();
        let original =
            H256::from_str("0xeb59f5c0d9f10f0fffc82ba6acd8904e9ef168e646fb351b5248be587492a7ff")
                .unwrap();
        let modified =
            H256::from_str("0xeb60f5c0d9f10f0fffc82ba6acd8904e9ef168e646fb351b5248be587492a7ff")
                .unwrap(); // `0xeb59` -> `0xeb60`
        block_header.parent_hash = modified;
        let result = blocks_handler.handle_block(block_header).await;
        // (:?) is needed for H256 type to properly display the full hex representation of the hash
        let msg = format!(
            "parent_hash mismatch. Previous: {:?}, current: {:?}",
            original, modified
        );
        assert_eq!(result.unwrap_err().to_string(), msg.as_str());
    }

    #[tokio::test]
    async fn test_verify_reorg_detected_for_first_block_when_handling_third_block() {
        let headers = load_fixtures().await;

        let first_block_hash =
            H256::from_str("0x69d59118ca03248ba8be5933d54a9a58a98a668e5ecb6c6149ef80d544e2cc0b")
                .unwrap();
        // `0x69` -> `0x70`
        let modified_hash =
            H256::from_str("0x70d59118ca03248ba8be5933d54a9a58a98a668e5ecb6c6149ef80d544e2cc0b")
                .unwrap();

        let mut mock_fetcher = MockBlocksFetcher::new();
        let block_number = headers[0].number.unwrap().as_u64();
        // Set up mock for the first block check
        mock_fetcher
            .expect_get_block_hash()
            .with(eq(block_number))
            .times(1) // since it will be called only once
            .returning(
                // outer closure
                move |_|
                    // Pin and Box the Future (Pin<Box<dyn Future<...>>>)
                    Box::pin(
                        // `async` will create a future & resolve to Future<Output = Result<H256, anyhow::Error>>
                        async move {
                            // Wrapped in Result::Ok
                            Ok(modified_hash)
                        }
                    ),
            );

        let mut blocks_handler = BlocksHandler::new(BLOCK_CONFIRMATIONS, mock_fetcher).unwrap();
        blocks_handler
            .handle_block(headers[0].clone())
            .await
            .unwrap();
        blocks_handler
            .handle_block(headers[1].clone())
            .await
            .unwrap();

        let result = blocks_handler.handle_block(headers[2].clone()).await;
        let err_string = result.unwrap_err().to_string();
        let msg = format!(
            "Reorg detected at block {}. Previous hash: {:?}, New hash: {:?}",
            block_number, first_block_hash, modified_hash,
        );
        assert_eq!(err_string, msg);
    }

    #[tokio::test]
    async fn test_verify_reorg_detected_for_second_block_when_handling_fourth_block() {
        let headers = load_fixtures().await;

        let first_block_number = headers[0].number.unwrap().as_u64();
        let first_block_hash = headers[0].clone().hash.unwrap();

        let second_block_number = headers[1].number.unwrap().as_u64();
        let second_block_hash =
            H256::from_str("0xeb59f5c0d9f10f0fffc82ba6acd8904e9ef168e646fb351b5248be587492a7ff")
                .unwrap();
        // `eb59` -> `eb60`
        let second_block_modified_hash =
            H256::from_str("0xeb60f5c0d9f10f0fffc82ba6acd8904e9ef168e646fb351b5248be587492a7ff")
                .unwrap();

        let mut mock_fetcher = MockBlocksFetcher::new();
        // for first block, we return original hash (since current test is targeted against 2nd block)
        mock_fetcher
            .expect_get_block_hash()
            .with(eq(first_block_number))
            .times(2) // will be called multiple times (for 21836329 & 21836330)
            .returning(move |_| Box::pin(async move { Ok(first_block_hash) }));

        // here, we return CHANGED hash
        mock_fetcher
            .expect_get_block_hash()
            .with(eq(second_block_number))
            .times(1) // will be called only ONCE for fourth block (21836330)
            .returning(move |_| Box::pin(async move { Ok(second_block_modified_hash) }));

        let mut blocks_handler = BlocksHandler::new(BLOCK_CONFIRMATIONS, mock_fetcher).unwrap();
        blocks_handler
            .handle_block(headers[0].clone())
            .await
            .unwrap();
        blocks_handler
            .handle_block(headers[1].clone())
            .await
            .unwrap();
        blocks_handler
            .handle_block(headers[2].clone())
            .await
            .unwrap();

        let result = blocks_handler.handle_block(headers[3].clone()).await;
        let err_string = result.unwrap_err().to_string();

        let msg = format!(
            "Reorg detected at block {}. Previous hash: {:?}, New hash: {:?}",
            second_block_number, second_block_hash, second_block_modified_hash
        );
        assert_eq!(err_string, msg);
    }
}
