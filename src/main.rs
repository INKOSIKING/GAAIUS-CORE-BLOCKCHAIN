mod blockchain;
mod transaction;
mod network;
mod api;
mod cli;
mod explorer; // <-- NEW: Added explorer

use blockchain::Blockchain;
use std::sync::{Arc, Mutex};
use crate::api::start_api;
use crate::cli::run_cli;
use crate::explorer::start_explorer; // <-- NEW
use tokio;

#[tokio::main]
async fn main() {
    // Shared blockchain instance
    let blockchain = Arc::new(Mutex::new(Blockchain::new()));

    // CLI commands (interactive or args)
    run_cli(blockchain.clone());

    // REST API server
    tokio::spawn(start_api(blockchain.clone()));

    // Blockchain Explorer web UI
    tokio::spawn(start_explorer(blockchain.clone()));

    // Keep the core node running
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(3600)).await;
    }
}
