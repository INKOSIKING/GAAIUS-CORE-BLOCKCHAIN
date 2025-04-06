use crate::blockchain::{Blockchain, Block};
use crate::transactions::Transaction;

pub struct Explorer<'a> {
    chain: &'a Blockchain,
}

impl<'a> Explorer<'a> {
    pub fn new(chain: &'a Blockchain) -> Self {
        Explorer { chain }
    }

    pub fn latest_block(&self) -> Option<&Block> {
        self.chain.get_chain().last()
    }

    pub fn all_blocks(&self) -> Vec<&Block> {
        self.chain.get_chain().iter().collect()
    }

    pub fn all_transactions(&self) -> Vec<&Transaction> {
        self.chain.get_chain().iter()
            .flat_map(|block| block.transactions.iter())
            .collect()
    }

    pub fn find_transactions_by_address(&self, address: &str) -> Vec<&Transaction> {
        self.all_transactions()
            .into_iter()
            .filter(|tx| tx.sender == address || tx.recipient == address)
            .collect()
    }

    pub fn print_summary(&self) {
        println!("--- GAAIUS EXPLORER ---");
        println!("Total Blocks: {}", self.chain.get_chain().len());
        println!("Total Transactions: {}", self.all_transactions().len());

        if let Some(latest) = self.latest_block() {
            println!("Latest Block Hash: {}", latest.hash);
            println!("Transactions in Latest Block: {}", latest.transactions.len());
        }

        println!("------------------------");
    }
}
