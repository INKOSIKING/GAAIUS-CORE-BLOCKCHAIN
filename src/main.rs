mod blockchain;
mod transactions;
mod network;
mod wallet;
mod cli;

use std::sync::{Arc, Mutex};
use blockchain::Blockchain;
use network::start_p2p_network;
use cli::start_cli;

fn main() {
    // Initialize blockchain in a thread-safe shared pointer
    let blockchain = Arc::new(Mutex::new(Blockchain::new()));

    // Start the P2P network in a background thread
    let blockchain_clone = Arc::clone(&blockchain);
    std::thread::spawn(move || {
        start_p2p_network(blockchain_clone);
    });

    // Start the CLI
    start_cli(blockchain);
}
