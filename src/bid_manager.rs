use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    sync::Arc,
};

use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    RwLock,
};

use crate::types::BidTrace;

// Manages (sort, organize) all bids given by relays
#[derive(Clone)]
pub struct BidManager {
    highest_bid: Arc<RwLock<Option<BidTrace>>>,
    all_bids: Arc<RwLock<BinaryHeap<Reverse<BidTrace>>>>,
    unique_bids: Arc<RwLock<HashSet<BidTrace>>>,
    top_bid_subscribers: Arc<RwLock<Vec<Sender<BidTrace>>>>,
    new_bid_subscribers: Arc<RwLock<Vec<Sender<BidTrace>>>>,
}

impl BidManager {
    pub fn new() -> Self {
        Self {
            highest_bid: Arc::new(RwLock::new(None)),
            all_bids: Arc::new(RwLock::new(BinaryHeap::new())),
            unique_bids: Arc::new(RwLock::new(HashSet::new())),
            top_bid_subscribers: Arc::new(RwLock::new(Vec::new())),
            new_bid_subscribers: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub(crate) async fn add_bids(&self, new_bids: Vec<BidTrace>) {
        let mut all_bids_guard = self.all_bids.write().await;
        let mut highest_bid_guard = self.highest_bid.write().await;
        let mut unique_bids_guard = self.unique_bids.write().await;
        let top_bid_subscribers_guard = self.top_bid_subscribers.read().await;
        let new_bid_subscribers_guard: tokio::sync::RwLockReadGuard<'_, Vec<Sender<BidTrace>>> =
            self.new_bid_subscribers.read().await;

        for bid in new_bids {
            if unique_bids_guard.insert(bid.clone()) {
                all_bids_guard.push(Reverse(bid.clone()));
                for subscriber in &*new_bid_subscribers_guard {
                    let _ = subscriber.send(bid.clone()).await; // Ignore errors
                }
                match highest_bid_guard.as_ref() {
                    Some(highest) if bid.value <= highest.value => (),
                    _ => {
                        *highest_bid_guard = Some(bid.clone());
                        for subscriber in &*top_bid_subscribers_guard {
                            let _ = subscriber.send(bid.clone()).await; // Ignore errors
                        }
                    }
                }
            }
        }
    }

    pub async fn get_highest_bid(&self) -> Option<BidTrace> {
        let all_bids_guard = self.all_bids.read().await;
        all_bids_guard.peek().map(|b| b.0.clone())
    }

    pub async fn clear_all(&self) {
        let mut all_bids_guard = self.all_bids.write().await;
        let mut highest_bid_guard = self.highest_bid.write().await;
        let mut unique_bids_guard = self.unique_bids.write().await;

        all_bids_guard.clear();
        unique_bids_guard.clear();
        *highest_bid_guard = None;
    }

    // Subscribe to new top block bids
    pub async fn subscribe_to_top_bids(&self) -> Receiver<BidTrace> {
        let (tx, rx) = mpsc::channel(100);
        let mut subscribers_guard = self.top_bid_subscribers.write().await;
        subscribers_guard.push(tx);
        rx
    }

    // Subscribe to all new block bids
    pub async fn subscribe_to_all_new_bids(&self) -> Receiver<BidTrace> {
        let (tx, rx) = mpsc::channel(100);
        let mut subscribers_guard = self.new_bid_subscribers.write().await;
        subscribers_guard.push(tx);
        rx
    }
}
