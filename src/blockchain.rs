use chrono::Utc;
use sha2::{Digest, Sha256};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
    pub transactions: Vec<String>, // Replace String with your actual Transaction struct
}

impl Block {
    pub fn new(index: u64, previous_hash: String, transactions: Vec<String>) -> Self {
        let timestamp = Utc::now().timestamp();
        let mut nonce = 0;
        let mut hash = Self::calculate_hash(index, timestamp, &previous_hash, &transactions, nonce);
        
        // Simple PoS-like simulation: increment nonce until hash starts with "0000"
        while !hash.starts_with("0000") {
            nonce += 1;
            hash = Self::calculate_hash(index, timestamp, &previous_hash, &transactions, nonce);
        }

        Block {
            index,
            timestamp,
            previous_hash,
            hash,
            nonce,
            transactions,
        }
    }

    pub fn calculate_hash(index: u64, timestamp: i64, previous_hash: &str, transactions: &[String], nonce: u64) -> String {
        let data = format!(
            "{}{}{}{:?}{}",
            index, timestamp, previous_hash, transactions, nonce
        );
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }
}
