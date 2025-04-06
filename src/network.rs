use std::collections::HashSet;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::blockchain::Blockchain;

pub struct Node {
    pub peers: Arc<Mutex<HashSet<String>>>,
    pub blockchain: Arc<Mutex<Blockchain>>,
}

impl Node {
    pub fn new(blockchain: Arc<Mutex<Blockchain>>) -> Self {
        Node {
            peers: Arc::new(Mutex::new(HashSet::new())),
            blockchain,
        }
    }

    pub fn start(&self, port: u16) {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap();
        println!("[NODE] Listening on port {}", port);

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            let peers = Arc::clone(&self.peers);
            let chain = Arc::clone(&self.blockchain);

            thread::spawn(move || {
                Self::handle_connection(stream, peers, chain);
            });
        }
    }

    fn handle_connection(mut stream: TcpStream, peers: Arc<Mutex<HashSet<String>>>, chain: Arc<Mutex<Blockchain>>) {
        let mut buffer = [0; 1024];
        if let Ok(size) = stream.read(&mut buffer) {
            let data = String::from_utf8_lossy(&buffer[..size]);
            println!("[PEER] Received: {}", data);

            if data.starts_with("NODE:") {
                let address = data.replace("NODE:", "").trim().to_string();
                peers.lock().unwrap().insert(address);
            }

            if data.starts_with("BLOCKCHAIN_REQUEST") {
                let blockchain = chain.lock().unwrap();
                let json = serde_json::to_string(&*blockchain).unwrap();
                stream.write_all(json.as_bytes()).unwrap();
            }
        }
    }

    pub fn broadcast_blockchain(&self) {
        let blockchain = self.blockchain.lock().unwrap();
        let data = serde_json::to_string(&*blockchain).unwrap();

        for peer in self.peers.lock().unwrap().iter() {
            if let Ok(mut stream) = TcpStream::connect(peer) {
                let _ = stream.write_all(format!("BLOCKCHAIN_UPDATE:{}", data).as_bytes());
            }
        }
    }

    pub fn add_peer(&self, address: String) {
        self.peers.lock().unwrap().insert(address.clone());
        if let Ok(mut stream) = TcpStream::connect(address.clone()) {
            let _ = stream.write_all(format!("NODE:{}", address).as_bytes());
        }
    }
}
