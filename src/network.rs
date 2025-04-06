// src/network.rs

use std::collections::HashSet;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::blockchain::Blockchain;
use crate::transactions::Transaction;

#[derive(Debug, Clone)]
pub struct Peer {
    pub address: String,
}

impl Peer {
    pub fn new(address: String) -> Self {
        Peer { address }
    }
}

pub struct P2PNetwork {
    pub peers: Arc<Mutex<HashSet<String>>>,
    pub blockchain: Arc<Mutex<Blockchain>>,
}

impl P2PNetwork {
    pub fn new(blockchain: Arc<Mutex<Blockchain>>) -> Self {
        P2PNetwork {
            peers: Arc::new(Mutex::new(HashSet::new())),
            blockchain,
        }
    }

    pub fn start_listener(&self, port: u16) {
        let listener = TcpListener::bind(("0.0.0.0", port)).expect("Failed to bind port");
        println!("Listening for peers on port {}", port);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let peers = self.peers.clone();
                    let blockchain = self.blockchain.clone();
                    thread::spawn(move || {
                        P2PNetwork::handle_peer(stream, peers, blockchain);
                    });
                }
                Err(e) => {
                    eprintln!("Failed to accept peer connection: {}", e);
                }
            }
        }
    }

    pub fn connect_to_peer(&self, address: &str) {
        match TcpStream::connect(address) {
            Ok(mut stream) => {
                println!("Connected to peer at {}", address);
                let mut peers = self.peers.lock().unwrap();
                peers.insert(address.to_string());

                let message = b"Hello GAAIUS Peer!";
                stream.write_all(message).expect("Failed to write to peer");
            }
            Err(e) => {
                eprintln!("Failed to connect to peer {}: {}", address, e);
            }
        }
    }

    fn handle_peer(mut stream: TcpStream, peers: Arc<Mutex<HashSet<String>>>, blockchain: Arc<Mutex<Blockchain>>) {
        let mut buffer = [0; 512];
        match stream.read(&mut buffer) {
            Ok(size) => {
                let data = String::from_utf8_lossy(&buffer[..size]);
                println!("Received from peer: {}", data);

                // Future: Handle syncing blocks or mempool transactions here.
            }
            Err(e) => {
                eprintln!("Failed to read from peer: {}", e);
            }
        }

        let peer_address = stream.peer_addr().unwrap().to_string();
        peers.lock().unwrap().insert(peer_address);
    }

    pub fn broadcast_transaction(&self, tx: &Transaction) {
        let peers = self.peers.lock().unwrap();
        for peer in peers.iter() {
            if let Ok(mut stream) = TcpStream::connect(peer) {
                let msg = serde_json::to_string(tx).unwrap();
                let _ = stream.write_all(msg.as_bytes());
            }
        }
    }
}
