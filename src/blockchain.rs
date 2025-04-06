use crate::transactions::{Transaction, TransactionType};
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use std::collections::{HashMap, HashSet};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub nonce: u64,
    pub hash: String,
}

#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub pending_transactions: Vec<Transaction>,
    pub balances: HashMap<String, u64>,
    pub contracts: HashMap<String, String>, // address -> code
    pub contract_state: HashMap<String, String>, // simple state key-value
    pub minted_tokens: HashSet<String>, // unique minted tokens
}

impl Blockchain {
    pub fn new() -> Self {
        let mut blockchain = Blockchain {
            chain: vec![],
            pending_transactions: vec![],
            balances: HashMap::new(),
            contracts: HashMap::new(),
            contract_state: HashMap::new(),
            minted_tokens: HashSet::new(),
        };
        blockchain.create_genesis_block();
        blockchain
    }

    fn create_genesis_block(&mut self) {
        let genesis = Block {
            index: 0,
            timestamp: 0,
            transactions: vec![],
            previous_hash: "0".repeat(64),
            nonce: 0,
            hash: "genesis".to_string(),
        };
        self.chain.push(genesis);
    }

    pub fn add_transaction(&mut self, tx: Transaction) {
        if tx.is_valid() {
            self.pending_transactions.push(tx);
        }
    }

    pub fn mine_block(&mut self, miner: &str) {
        let last_block = self.chain.last().unwrap();
        let mut new_block = Block {
            index: last_block.index + 1,
            timestamp: chrono::Utc::now().timestamp() as u64,
            transactions: self.pending_transactions.clone(),
            previous_hash: last_block.hash.clone(),
            nonce: 0,
            hash: String::new(),
        };

        new_block.hash = self.calculate_hash(&new_block);
        self.apply_transactions(&new_block.transactions);
        self.chain.push(new_block);
        self.pending_transactions.clear();
        *self.balances.entry(miner.to_string()).or_insert(0) += 100;
    }

    fn calculate_hash(&self, block: &Block) -> String {
        let mut hasher = Sha256::new();
        let data = format!(
            "{}{}{:?}{}{}",
            block.index,
            block.timestamp,
            block.transactions,
            block.previous_hash,
            block.nonce
        );
        hasher.update(data.as_bytes());
        hex::encode(hasher.finalize())
    }

    fn apply_transactions(&mut self, txs: &[Transaction]) {
        for tx in txs {
            match tx.tx_type {
                TransactionType::Transfer => {
                    if self.balances.get(&tx.from).unwrap_or(&0) >= &tx.amount {
                        *self.balances.entry(tx.from.clone()).or_insert(0) -= tx.amount;
                        *self.balances.entry(tx.to.clone()).or_insert(0) += tx.amount;
                    }
                }
                TransactionType::Stake => {
                    *self.balances.entry(tx.from.clone()).or_insert(0) -= tx.amount;
                    // placeholder for staking logic
                }
                TransactionType::DeployContract => {
                    self.contracts.insert(tx.to.clone(), tx.payload.clone().unwrap_or_default());
                }
                TransactionType::CallContract => {
                    let contract_output = format!("Executed contract {} by {}", tx.to, tx.from);
                    self.contract_state.insert(tx.to.clone(), contract_output);
                }
                TransactionType::MintToken => {
                    if !self.minted_tokens.contains(&tx.to) {
                        self.minted_tokens.insert(tx.to.clone());
                        *self.balances.entry(tx.to.clone()).or_insert(0) += tx.amount;
                    }
                }
            }
        }
    }

    pub fn get_balance(&self, address: &str) -> u64 {
        *self.balances.get(address).unwrap_or(&0)
    }

    pub fn get_contract(&self, address: &str) -> Option<&String> {
        self.contracts.get(address)
    }

    pub fn get_state(&self, address: &str) -> Option<&String> {
        self.contract_state.get(address)
    }
}
