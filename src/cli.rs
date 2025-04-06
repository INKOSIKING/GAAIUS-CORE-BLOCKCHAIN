use std::env;
use std::io;
use std::sync::{Arc, Mutex};

use crate::blockchain::Blockchain;
use crate::transactions::Transaction;
use crate::network::{start_p2p_network, broadcast_transaction};
use crate::wallet::Wallet;

pub fn start_cli(blockchain: Arc<Mutex<Blockchain>>) {
    println!("=== GAAIUS CORE v4 CLI ===");
    println!("Available commands:");
    println!(" - send <from> <to> <amount>");
    println!(" - balance <address>");
    println!(" - chain");
    println!(" - mine");
    println!(" - exit");

    let wallet = Wallet::new(); // Optional: Replace with actual loaded wallet

    loop {
        let mut input = String::new();
        print!("> ");
        let _ = io::Write::flush(&mut io::stdout());
        io::stdin().read_line(&mut input).unwrap();

        let args: Vec<&str> = input.trim().split_whitespace().collect();
        if args.is_empty() {
            continue;
        }

        match args[0] {
            "send" if args.len() == 4 => {
                let from = args[1];
                let to = args[2];
                let amount: u64 = args[3].parse().unwrap_or(0);

                let tx = Transaction::new(from.to_string(), to.to_string(), amount);
