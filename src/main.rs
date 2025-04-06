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
use tokio::task;

#[tokio::main]
async fn main() {
    // Load CLI args
    let cli = CliCommand::parse();

    // Shared blockchain state
    let blockchain = Arc::new(Mutex::new(Blockchain::new()));
    let wallet = Arc::new(Mutex::new(Wallet::new()));

    // Start API server in background
    let blockchain_api = Arc::clone(&blockchain);
    let wallet_api = Arc::clone(&wallet);

    task::spawn(async move {
        api::run_server(blockchain_api, wallet_api).await;
    });

    // Handle CLI commands
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
                println!("Transaction from {} to {} for {} sent.", from, to, amount);
            } else {
                println!("Failed to create transaction. Check balances.");
            }
        }
        None => {
            println!("GAAIUS CORE v4 node is running.");
            println!("API available at http://localhost:3030");
            println!("Use CLI flags to interact (e.g., `
