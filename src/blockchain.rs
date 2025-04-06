use crate::transactions::Transaction;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: u128,
    pub previous_hash: Vec<u8>,
    pub hash: Vec<u8>,
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(index: u64, previous_hash: Vec<u8>, transactions: Vec<Transaction>) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let mut block = Block {
            index,
            timestamp,
            previous_hash,
            hash: vec![],
            nonce: 0,
            transactions,
        };
        block.mine_block(2); // Adjust difficulty here
        block
    }

    pub fn calculate_hash(&self) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(&self.index.to_be_bytes());
        hasher.update(&self.timestamp.to_be_bytes());
        hasher.update(&self.previous_hash);
        hasher.update(&self.nonce.to_be_bytes());

        for tx in &self.transactions {
            hasher.update(&tx.calculate_hash());
        }

        hasher.finalize().to_vec()
    }

    pub fn mine_block(&mut self, difficulty: usize) {
        let target = vec![0u8; difficulty];
        while &self.hash[..difficulty] != target {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
    pub pending_transactions: Vec<Transaction>,
    pub reward: u64,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut blockchain = Blockchain {
            chain: vec![],
            difficulty: 2,
            pending_transactions: vec![],
            reward: 100,
        };
        blockchain.create_genesis_block();
        blockchain
    }

    pub fn create_genesis_block(&mut self) {
        let genesis = Block::new(0, vec![0; 32], vec![]);
        self.chain.push(genesis);
    }

    pub fn get_latest_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    pub fn add_transaction(&mut self, tx: Transaction) {
        if tx.is_valid() {
            self.pending_transactions.push(tx);
        }
    }

    pub fn mine_pending_transactions(&mut self, miner_address: String) {
        let reward_tx = Transaction::new(
            "GAAIUS_REWARD".to_string(),
            miner_address,
            self.reward,
            0,
            0,
            vec![],
            vec![],
            None,
        );
        self.pending_transactions.push(reward_tx);

        let new_block = Block::new(
            self.chain.len() as u64,
            self.get_latest_block().hash.clone(),
            self.pending_transactions.clone(),
        );

        self.chain.push(new_block);
        self.pending_transactions.clear();
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let prev = &self.chain[i - 1];

            if current.hash != current.calculate_hash() {
                return false;
            }

            if current.previous_hash != prev.hash {
                return false;
            }
        }
        true
    }
}
