use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::net::{TcpListener, TcpStream};
use std::io::{BufRead, BufReader, Write};
use crate::blockchain::{Blockchain, Block};
use crate::transactions::Transaction;

#[derive(Clone)]
pub struct Node {
    pub address: String,
    pub peers: Arc<Mutex<HashSet<String>>>,
    pub blockchain: Arc<Mutex<Blockchain>>,
}

impl Node {
    pub fn new(address: String) -> Self {
        Node {
            address,
            peers: Arc::new(Mutex::new(HashSet::new())),
            blockchain: Arc::new(Mutex::new(Blockchain::new())),
        }
    }

    pub fn start(&self) {
        let listener = TcpListener::bind(&self.address).expect("Failed to bind listener");
        println!("Node running at {}", self.address);

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            let blockchain = Arc::clone(&self.blockchain);
            std::thread::spawn(move || {
                Node::handle_connection(stream, blockchain);
            });
        }
    }

    fn handle_connection(stream: TcpStream, blockchain: Arc<Mutex<Blockchain>>) {
        let reader = BufReader::new(&stream);
        for line in reader.lines() {
            if let Ok(request) = line {
                if request == "latest_block" {
                    let chain = blockchain.lock().unwrap();
                    let latest = chain.latest_block();
                    let response = serde_json::to_string(latest).unwrap();
                    let _ = stream.try_clone().unwrap().write_all(response.as_bytes());
                }
            }
        }
    }

    pub fn broadcast_new_block(&self, block: &Block) {
        let peers = self.peers.lock().unwrap();
        let message = serde_json::to_string(block).unwrap();

        for peer in peers.iter() {
            if let Ok(mut stream) = TcpStream::connect(peer) {
                let _ = stream.write_all(message.as_bytes());
            }
        }
    }

    pub fn add_peer(&self, peer: String) {
        self.peers.lock().unwrap().insert(peer);
    }

    pub fn submit_transaction(&self, tx: Transaction) {
        let mut chain = self.blockchain.lock().unwrap();
        let nonce = 0; // Placeholder: integrate PoS or mining logic
        chain.add_block(vec![tx], nonce);
    }
}
