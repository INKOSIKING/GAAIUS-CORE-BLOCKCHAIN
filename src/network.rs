use std::collections::HashSet;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::blockchain::{Block, Blockchain};
use crate::transactions::Transaction;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum NetworkMessage {
    NewTransaction(Transaction),
    NewBlock(Block),
    RequestChain,
    ResponseChain(Vec<Block>),
    RegisterPeer(String),
}

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

    pub fn start(&self, address: &str) {
        let listener = TcpListener::bind(address).expect("Failed to bind to address");
        println!("Node listening on {}", address);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let peers = Arc::clone(&self.peers);
                    let chain = Arc::clone(&self.blockchain);
                    thread::spawn(move || {
                        Node::handle_connection(stream, peers, chain);
                    });
                }
                Err(e) => {
                    eprintln!("Connection error: {}", e);
                }
            }
        }
    }

    pub fn connect_to_peer(&self, address: &str) {
        if self.peers.lock().unwrap().insert(address.to_string()) {
            let msg = serde_json::to_string(&NetworkMessage::RegisterPeer(address.to_string())).unwrap();
            Node::send_message(address, &msg);
        }
    }

    pub fn broadcast(&self, msg: &NetworkMessage) {
        let peers = self.peers.lock().unwrap().clone();
        let serialized
