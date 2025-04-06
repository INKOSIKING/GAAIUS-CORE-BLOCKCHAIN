use chrono::Utc;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

use crate::transactions::{Transaction, calculate_hash, verify_signature};
use crate::wallet::Wallet;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    pub validator: String,
}

#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub transaction_pool: Vec<Transaction>,
    pub balances: HashMap<String, f64>,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut blockchain = Blockchain {
            chain: vec![],
            transaction_pool: vec![],
            balances: HashMap::new(),
        };
        let genesis = blockchain.create_genesis_block();
        blockchain.chain.push(genesis);
        blockchain
    }

    fn create_genesis_block(&self) -> Block {
        Block {
            index: 0,
            timestamp: Utc::now().timestamp(),
            transactions: vec![],
            previous_hash: String::from("0"),
            hash: String::from("GENESIS_HASH"),
            validator: String::from("GENESIS_VALIDATOR"),
        }
    }

    pub fn add_transaction(&mut self, tx: Transaction) -> bool {
        if verify_signature(&tx) && self.get_balance(&tx.sender) >= tx.amount {
            self.transaction_pool.push(tx);
            true
        } else {
            false
        }
    }

    pub fn mine_block(&mut self, validator_wallet: &Wallet) {
        let transactions = self.transaction_pool.clone();
        let previous_hash = self.chain.last().unwrap().hash.clone();
        let index = self.chain.len() as u64;
        let timestamp = Utc::now().timestamp();

        let mut block = Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            hash: String::new(),
            validator: validator_wallet.address.clone(),
        };

        block.hash = Self::calculate_block_hash(&block);
        self.apply_transactions(&block.transactions, &validator_wallet.address);
        self.chain.push(block);
        self.transaction_pool.clear();
    }

    fn apply_transactions(&mut self, transactions: &[Transaction], validator: &str) {
        for tx in transactions {
            *self.balances.entry(tx.sender.clone()).or_insert(0.0) -= tx.amount;
            *self.balances.entry(tx.recipient.clone()).or_insert(0.0) += tx.amount;
        }

        // Validator reward
        let reward = 5.0; // Adjustable based on consensus
        *self.balances.entry(validator.to_string()).or_insert(0.0) += reward;
    }

    pub fn get_balance(&self, address: &str) -> f64 {
        *self.balances.get(address).unwrap_or(&0.0)
    }

    pub fn get_chain(&self) -> &Vec<Block> {
        &self.chain
    }

    pub fn calculate_block_hash(block: &Block) -> String {
        let data = format!(
            "{}{}{:?}{}{}",
            block.index,
            block.timestamp,
            block.transactions,
            block.previous_hash,
            block.validator
        );
        let mut hasher = Sha256::new();
        hasher.update(data);
        hex::encode(hasher.finalize())
    }
}
