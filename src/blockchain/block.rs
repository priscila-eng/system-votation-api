use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest};

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub index: u64,
    pub timestamp: u128,
    pub voter_id: String,
    pub election_id: String,
    pub vote_option_id: String,
    pub previous_hash: String,
    pub hash: String,
}

impl Block {
    pub fn new(index: u64, voter_id: String, election_id: String, vote_option_id: String, previous_hash: String) -> Self {
        let timestamp = current_timestamp();
        let hash = calculate_hash(index, timestamp, &voter_id, &election_id, &vote_option_id, &previous_hash);

        Block {
            index,
            timestamp,
            voter_id,
            election_id,
            vote_option_id,
            previous_hash,
            hash,
        }
    }
}

pub fn current_timestamp() -> u128 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
    since_the_epoch.as_millis()
}

pub fn calculate_hash(index: u64, timestamp: u128, voter_id: &str, election_id: &str, vote_option_id: &str, previous_hash: &str) -> String {
    let data = (index, timestamp, voter_id, election_id, vote_option_id, previous_hash);
    let encoded = bincode::serialize(&data).unwrap();
    let mut hasher = Sha256::new();
    hasher.update(&encoded);
    format!("{:x}", hasher.finalize())
}
