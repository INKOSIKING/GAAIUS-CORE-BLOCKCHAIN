// src/main.rs

mod blockchain;
mod transactions;
mod api;
mod cli;
mod explorer;

use crate::blockchain::Blockchain;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let blockchain = Blockchain::new();

    // Start CLI or API
    cli::run(blockchain.clone());
    api::run_server(blockchain).await
}
