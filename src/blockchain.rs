use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use crate::transactions::Transaction;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Block {
    pub index: u64,
    pub timestamp: u128,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub nonce: u64,
    pub hash: String,
}

impl Block {
    pub fn new(index: u64, timestamp: u128, transactions: Vec<Transaction>, previous_hash: String, nonce: u64) -> Self {
        let mut block = Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            nonce,
            hash: String::new(),
        };
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let data = format!(
            "{}{}{:?}{}{}",
            self.index,
            self.timestamp,
            self.transactions,
            self.previous_hash,
            self.nonce
        );
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block::new(0, 0, vec![], "0".to_string(), 0);
        Blockchain {
            chain: vec![genesis_block],
        }
    }

    pub fn latest_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>, nonce: u64) {
        let block = Block::new(
            (self.chain.len()) as u64,
            chrono::Utc::now().timestamp_millis() as u128,
            transactions,
            self.latest_block().hash.clone(),
            nonce,
        );
        self.chain.push(block);
    }

    pub fn is_valid_chain(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];
            if current.hash != current.calculate_hash() {
                return false;
            }
            if current.previous_hash != previous.hash {
                return false;
            }
        }
        true
    }
}
