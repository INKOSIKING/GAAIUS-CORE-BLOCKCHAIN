mod blockchain;
mod cli;
mod miner;
mod network;
mod transactions;
mod wallet;
mod api;
mod explorer;

use crate::cli::CliCommand;
use crate::blockchain::Blockchain;
use crate::wallet::Wallet;
use clap::Parser;
use std::sync::{Arc, Mutex};
use tokio::{signal, task};

#[tokio::main]
async fn main() {
    let cli = CliCommand::parse();

    // Shared blockchain and wallet states
    let blockchain = Arc::new(Mutex::new(Blockchain::new()));
    let wallet = Arc::new(Mutex::new(Wallet::new()));

    // Start REST API in the background
    let api_blockchain = Arc::clone(&blockchain);
    let api_wallet = Arc::clone(&wallet);

    let api_handle = task::spawn(async move {
        api::run_server(api_blockchain, api_wallet).await;
    });

    // Handle CLI commands (async-safe)
    match cli.command {
        Some(cli::Commands::CreateWallet) => {
            let new_wallet = wallet.lock().unwrap().create_wallet();
            println!("New wallet created: {}", new_wallet);
        }
        Some(cli::Commands::ShowBalance { address }) => {
            let balance = blockchain.lock().unwrap().get_balance(&address);
            println!("Balance for {}: {}", address, balance);
        }
        Some(cli::Commands::Mine { address }) => {
            let mut bc = blockchain.lock().unwrap();
            let reward_tx = bc.create_reward_transaction(address.clone());
            bc.mine_block(vec![reward_tx]);
            println!("Block mined and reward sent to {}", address);
        }
        Some(cli::Commands::Send { from, to, amount }) => {
            let mut bc = blockchain.lock().unwrap();
            if let Some(tx) = bc.create_transaction(&from, &to, amount) {
                bc.add_transaction(tx);
                println!("Transaction sent: {} => {} [{}]", from, to, amount);
            } else {
                println!("Transaction failed. Check sender balance or inputs.");
            }
        }
        None => {
            println!("\nGAAIUS CORE Node v4 is running.");
            println!("Explorer:   http://localhost:3030/explorer");
            println!("API:        http://localhost:3030/api");
            println!("Press Ctrl+C to shut down the node.\n");

            // Wait for shutdown signal (graceful exit)
            signal::ctrl_c().await.unwrap();
            println!("\nShutting down GAAIUS CORE node...");
        }
    }

    // Optional: wait for background API to cleanly shut down
    let _ = api_handle.await;
}
