use ethers::types::U64;
use reqwest::Client;

use crate::types::BidTrace;

// A single relay client instance.
pub struct RelayClient {
    // Relay Url.
    url: String,
    // Reqwest client to do the requests.
    client: Client,
}

impl RelayClient {
    pub fn new(url: String) -> Self {
        Self {
            url: format!("{}/relay/v1/data/bidtraces/builder_blocks_received", url),
            client: Client::new(),
        }
    }

    pub async fn get_builder_bids(&self, block_num: U64) -> Option<Vec<BidTrace>> {
        let res = match self
            .client
            .get(format!("{}?block_number={}", &self.url, block_num))
            .header("accept", "application/json")
            .send()
            .await
        {
            Ok(response) => response,
            Err(e) => {
                eprintln!("Error getting block bids: {}", e);
                return None;
            }
        };

        let bid_traces = match res.json::<Vec<BidTrace>>().await {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Error decoding bids: {}", e);
                return None;
            }
        };

        Some(bid_traces)
    }
}
