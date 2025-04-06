use std::io::{self, Write};
use std::sync::{Arc, Mutex};

use crate::blockchain::Blockchain;
use crate::network::{Node, NetworkMessage};
use crate::transactions::Transaction;

pub fn start_cli(blockchain: Arc<Mutex<Blockchain>>, node: Arc<Node>) {
    loop {
        print!("\nGAAIUS CORE > ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "mine" => {
                let mut chain = blockchain.lock().unwrap();
                chain.mine_pending_transactions("gaaius-system".to_string());
                println!("Block mined!");
                let block = chain.chain.last().unwrap().clone();
                node.broadcast(&NetworkMessage::NewBlock(block));
            }
            "chain" => {
                let chain = blockchain.lock().unwrap();
                for block in &chain.chain {
                    println!("{:#?}", block);
                }
            }
            "tx" => {
                if parts.len() < 4 {
                    println!("Usage: tx <sender> <recipient> <amount>");
                    continue;
                }

                let tx = Transaction {
                    sender: parts[1].to_string(),
                    recipient: parts[2].to_string(),
                    amount: parts[3].parse().unwrap_or(0),
                };

                blockchain.lock().unwrap().add_transaction(tx.clone());
                node.broadcast(&NetworkMessage::NewTransaction(tx));
                println!("Transaction added.");
            }
            "peers" => {
                let peers = node.peers.lock().unwrap();
                println!("Connected peers:");
                for peer in peers.iter() {
                    println!("- {}", peer);
                }
            }
            "connect" => {
                if parts.len() < 2 {
                    println!("Usage: connect <ip:port>");
                    continue;
                }
                node.connect_to_peer(parts[1]);
                println!("Connected to peer {}", parts[1]);
            }
            "start" => {
                if parts.len() < 2 {
                    println!("Usage: start <ip:port>");
                    continue;
                }

                let node_clone = Arc::clone(&node);
                std::thread::spawn(move || {
                    node_clone.start(parts[1]);
                });

                println!("Node started on {}", parts[1]);
            }
            "exit" => break,
            _ => println!("Commands: mine, tx, chain, connect, start, peers, exit"),
        }
    }
}
