use crate::swap_details::SwapDetails;
use anyhow::{anyhow, Context};
use web3::{
    contract::Contract,
    ethabi,
    ethabi::{Event, Hash},
    transports::WebSocket,
    types::{Log, H160, H256},
    Web3,
};

pub struct EventsHandler {
    web3: Web3<WebSocket>,
    contract_address: H160,
    swap_event: Event,
    swap_event_signature: Hash,
}

impl EventsHandler {
    pub fn new(web3: Web3<WebSocket>) -> Result<Self, anyhow::Error> {
        let contract_address = H160::from_slice(
            &hex::decode("5777d92f208679db4b9778590fa3cab3ac9e2168")
                .map_err(|e| anyhow!("Failed to decode hex: {}", e))?[..],
        );

        let contract = Contract::from_json(
            web3.eth(),
            contract_address,
            include_bytes!("contracts/uniswap_pool_abi.json"),
        )
        .map_err(|e| anyhow!("Failed to create contract: {}", e))?;

        let swap_event = contract
            .abi()
            .events_by_name("Swap")?
            .first()
            .unwrap()
            .clone();
        let swap_event_signature = swap_event.signature();

        Ok(Self {
            web3,
            contract_address,
            swap_event,
            swap_event_signature,
        })
    }

    pub async fn fetch_swap_logs(&self, block_hash: H256) -> Result<Vec<Log>, anyhow::Error> {
        let filter = web3::types::FilterBuilder::default()
            .block_hash(block_hash)
            .address(vec![self.contract_address])
            .topics(Some(vec![self.swap_event_signature]), None, None, None)
            .build();

        let swap_logs = self.web3.eth().logs(filter).await?;
        Ok(swap_logs)
    }

    pub fn parse_logs(&self, raw_logs: Vec<Log>) -> Result<Vec<ethabi::Log>, anyhow::Error> {
        let mut parsed_logs = vec![];
        for log in raw_logs {
            let parsed_log = self.swap_event.parse_log(ethabi::RawLog {
                topics: log.topics,
                data: log.data.0,
            })?;

            parsed_logs.push(parsed_log);
        }
        Ok(parsed_logs)
    }

    pub async fn to_swap_details(
        &self,
        parsed_logs: Vec<ethabi::Log>,
    ) -> Result<Vec<SwapDetails>, anyhow::Error> {
        let mut swap_details = vec![];
        for parsed_log in parsed_logs {
            swap_details.push(SwapDetails::from_parsed_log(parsed_log)?);
        }

        Ok(swap_details)
    }

    pub async fn handle_events(&self, block_hash: H256) -> Result<Vec<SwapDetails>, anyhow::Error> {
        let mut handled_events = vec![];

        let raw_logs = self.fetch_swap_logs(block_hash).await?;
        if raw_logs.is_empty() {
            return Ok(handled_events);
        }

        let parsed_logs = self.parse_logs(raw_logs)?;
        handled_events = self
            .to_swap_details(parsed_logs)
            .await
            .context("Could not convert to swap info")?;

        Ok(handled_events)
    }
}
