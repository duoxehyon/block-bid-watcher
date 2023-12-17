use std::{
    fmt,
    hash::{Hash, Hasher},
};

use ethers::types::{Address, U256};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct BidTrace {
    #[serde(deserialize_with = "deserialize_u256_from_string")]
    pub slot: U256,
    pub parent_hash: String,
    pub block_hash: String,
    pub builder_pubkey: String,
    pub proposer_pubkey: String,
    pub proposer_fee_recipient: Address,
    #[serde(deserialize_with = "deserialize_u256_from_string")]
    pub gas_limit: U256,
    #[serde(deserialize_with = "deserialize_u256_from_string")]
    pub gas_used: U256,
    #[serde(deserialize_with = "deserialize_u256_from_string")]
    pub value: U256,
    #[serde(deserialize_with = "deserialize_u256_from_string")]
    pub block_number: U256,
    #[serde(deserialize_with = "deserialize_u256_from_string")]
    pub num_tx: U256,
    #[serde(deserialize_with = "deserialize_u256_from_string")]
    pub timestamp: U256,
    #[serde(deserialize_with = "deserialize_u256_from_string")]
    pub timestamp_ms: U256,
}

fn deserialize_u256_from_string<'de, D>(deserializer: D) -> Result<U256, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    U256::from_dec_str(&s).map_err(serde::de::Error::custom)
}

impl fmt::Display for BidTrace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BidTrace {{ block_number: {}, builder_pubkey: {}, value: {} , num_tx: {}}}",
            self.block_number, self.builder_pubkey, self.value, self.num_tx
        )
    }
}

impl Hash for BidTrace {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
        self.builder_pubkey.hash(state);
    }
}

impl Ord for BidTrace {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

impl PartialOrd for BidTrace {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
