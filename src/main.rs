use std::error::Error;

use ethers::prelude::*;
use flashbots_relay_grant::relay_client::RelayClients;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize RelayClients with URLs
    let mut relay_clients = RelayClients::new(vec![
        "https://relay.ultrasound.money".to_string(),
        "https://agnostic-relay.net".to_string(),
        "https://boost-relay.flashbots.net".to_string(),
    ]);
    let mut bid_manager_receiver = relay_clients.bid_manager.subscribe_to_top_bids().await;

    // Spawn a task to handle received messages from the bid manager
    tokio::spawn(async move {
        while let Some(data) = bid_manager_receiver.recv().await {
            println!("New Highest Bid: {}", data);
        }
    });

    // Connect to the WebSocket provider
    let provider = Provider::<Ws>::connect("wss://go.getblock.io/<your_access_token_here>").await?;

    // Subscribe to new blocks
    let mut block_stream = provider.subscribe_blocks().await?;

    // Process new blocks as they come in
    while let Some(block) = block_stream.next().await {
        let block_number = block.number.expect("Block number not found in new block");
        println!("New block: {}", block_number);

        // Poll for each new block
        relay_clients
            .poll_for(block_number + U64::one(), 1, 12)
            .await
    }

    Ok(())
}
