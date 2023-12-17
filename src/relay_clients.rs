use std::{sync::Arc, time::Duration};

use ethers::types::U64;
use tokio::{select, time};

use crate::{bid_manager::BidManager, relay_client::RelayClient};

pub struct RelayClients {
    // All relay clients to read block builder bids from.
    pub clients: Vec<Arc<RelayClient>>,
    // Bid manager to merge and sort bids.
    pub bid_manager: Arc<BidManager>,
}

impl RelayClients {
    pub fn new(relay_uls: Vec<String>) -> Self {
        Self {
            clients: relay_uls
                .into_iter()
                .map(|r| Arc::new(RelayClient::new(r)))
                .collect(),
            bid_manager: Arc::new(BidManager::new()),
        }
    }

    // Polls for builder bids every `poll_interval_secs` second for `poll_for_secs`
    // seconds.
    pub async fn poll_for(&mut self, block_num: U64, poll_interval_secs: u64, poll_for_secs: u64) {
        let poll_interval = Duration::from_secs(poll_interval_secs);
        let mut interval_timer = time::interval(poll_interval);
        let start_time = time::Instant::now();
        let duration = Duration::from_secs(poll_for_secs);

        loop {
            select! {
                _ = interval_timer.tick() => {
                    // Check if the total polling duration has been exceeded
                    if time::Instant::now().duration_since(start_time) > duration {
                        self.bid_manager.clear_all().await;
                        break;
                    }

                    let mut handles = Vec::new();
                    for client in &self.clients {
                        let client = client.clone();
                        let bid_manager = self.bid_manager.clone();
                        let block_num = block_num;

                        let handle = tokio::spawn(async move {
                            if let Some(bid_traces) = client.get_builder_bids(block_num).await {
                                // Add bid traces to the bid manager
                                bid_manager.add_bids(bid_traces).await;
                            }
                        });

                        handles.push(handle);
                    }

                    // Await all handles to ensure all bid traces are inserted before the next interval
                    for handle in handles {
                        let _ = handle.await;
                    }
                }
                // After poll_for_secs has elapsed, exit the loop
                _ = time::sleep(duration) => {
                    self.bid_manager.clear_all().await;
                    break;
                }
            }
        }
    }
}
