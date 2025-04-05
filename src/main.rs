// GAAIUS CORE v3 - Ultra Secure Blockchain Node
// By INKOSIKING - gaaiusmegachain.com

use std::collections::{HashMap, HashSet};
use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest};
use rand::{thread_rng, Rng};
use std::sync::{Arc, Mutex};

type Address = String;

#[derive(Clone)]
struct Transaction {
    from: Address,
    to: Address,
    amount: u64,
    fee: u64,
    nonce: u64,
}

#[derive(Clone)]
struct Block {
    index: u64,
    timestamp: u128,
    transactions: Vec<Transaction>,
    previous_hash: String,
    validator: Address,
    hash: String,
    signature: String,
}

struct Blockchain {
    chain: Vec<Block>,
    pending: Vec<Transaction>,
    balances: HashMap<Address, u64>,
    validators: HashMap<Address, u64>, // stake
    nonces: HashMap<Address, u64>,
    validator_rewards: u64,
}

impl Blockchain {
    fn new() -> Self {
        let mut bc = Blockchain {
            chain: vec![],
            pending: vec![],
            balances: HashMap::new(),
            validators: HashMap::new(),
            nonces: HashMap::new(),
            validator_rewards: 10_000,
        };
        bc.create_genesis_block();
        bc
    }

    fn create_genesis_block(&mut self) {
        let genesis = Block {
            index: 0,
            timestamp: now(),
            transactions: vec![],
            previous_hash: String::from("0"),
            validator: String::from("genesis"),
            hash: String::from("0"),
            signature: String::from("genesis-signature"),
        };
        self.chain.push(genesis);
    }

    fn hash_block(block: &Block) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!(
            "{}{}{:?}{}{}",
            block.index, block.timestamp, block.transactions, block.previous_hash, block.validator
        ));
        format!("{:x}", hasher.finalize())
    }

    fn add_transaction(&mut self, tx: Transaction) -> bool {
        let nonce = self.nonces.get(&tx.from).unwrap_or(&0);
        if &tx.nonce != nonce {
            return false;
        }

        let balance = self.balances.get(&tx.from).unwrap_or(&0);
        if balance < &(tx.amount + tx.fee) {
            return false;
        }

        self.pending.push(tx.clone());
        *self.nonces.entry(tx.from.clone()).or_insert(0) += 1;
        true
    }

    fn select_validator(&self) -> Option<Address> {
        let total_stake: u64 = self.validators.values().sum();
        if total_stake == 0 {
            return None;
        }

        let mut rng = thread_rng();
        let mut point = rng.gen_range(0..total_stake);
        for (addr, stake) in &self.validators {
            if point < *stake {
                return Some(addr.clone());
            }
            point -= stake;
        }
        None
    }

    fn mine_block(&mut self) -> Option<Block> {
        let validator = self.select_validator()?;
        let previous = self.chain.last().unwrap();
        let mut block = Block {
            index: previous.index + 1,
            timestamp: now(),
            transactions: self.pending.clone(),
            previous_hash: previous.hash.clone(),
            validator: validator.clone(),
            hash: String::new(),
            signature: String::new(), // placeholder
        };
        block.hash = Self::hash_block(&block);
        block.signature = format!("sig-{}", &block.hash[..10]);

        self.chain.push(block.clone());
        self.pending.clear();
        *self.balances.entry(validator).or_insert(0) += self.validator_rewards;

        for tx in &block.transactions {
            *self.balances.entry(tx.from.clone()).or_insert(0) -= tx.amount + tx.fee;
            *self.balances.entry(tx.to.clone()).or_insert(0) += tx.amount;
        }

        Some(block)
    }

    fn stake(&mut self, who: &str, amount: u64) {
        *self.validators.entry(who.to_string()).or_insert(0) += amount;
        *self.balances.entry(who.to_string()).or_insert(0) -= amount;
    }

    fn get_balance(&self, who: &str) -> u64 {
        *self.balances.get(who).unwrap_or(&0)
    }
}

fn now() -> u128 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()
}

fn main() {
    let chain = Arc::new(Mutex::new(Blockchain::new()));

    {
        let mut core = chain.lock().unwrap();
        core.balances.insert("inkosiking".into(), 1_000_000);
        core.stake("inkosiking", 500_000);
    }

    {
        let mut core = chain.lock().unwrap();
        core.add_transaction(Transaction {
            from: "inkosiking".into(),
            to: "gaaius".into(),
            amount: 100_000,
            fee: 100,
            nonce: 0,
        });

        if let Some(block) = core.mine_block() {
            println!("Block Mined: #{} - Hash: {}", block.index, block.hash);
        }

        println!("inkosiking balance: {}", core.get_balance("inkosiking"));
        println!("gaaius balance: {}", core.get_balance("gaaius"));
    }
}
