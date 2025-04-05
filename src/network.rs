use std::collections::HashSet;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Clone)]
pub struct Network {
    pub peers: Arc<Mutex<HashSet<String>>>,
}

impl Network {
    pub fn new() -> Self {
        Self {
            peers: Arc::new(Mutex::new(HashSet::new())),
        }
    }

    pub fn start(&self, port: u16) {
        let listener = TcpListener::bind(("0.0.0.0", port)).expect("Failed to bind");
        let peers_clone = self.peers.clone();

        thread::spawn(move || {
            for stream in listener.incoming() {
                match stream {
                    Ok(mut stream) => {
                        let mut buffer = [0; 512];
                        if let Ok(size) = stream.read(&mut buffer) {
                            let message = String::from_utf8_lossy(&buffer[..size]);
                            println!("Received message: {}", message);
                        }
                    }
                    Err(e) => eprintln!("Connection failed: {}", e),
                }
            }
        });

        println!("Listening for peers on port {}", port);
    }

    pub fn broadcast(&self, message: &str) {
        let peers = self.peers.lock().unwrap();
        for peer in peers.iter() {
            if let Ok(mut stream) = TcpStream::connect(peer) {
                let _ = stream.write(message.as_bytes());
            }
        }
    }

    pub fn add_peer(&self, addr: String) {
        let mut peers = self.peers.lock().unwrap();
        peers.insert(addr);
    }

    pub fn list_peers(&self) -> Vec<String> {
        let peers = self.peers.lock().unwrap();
        peers.iter().cloned().collect()
    }
}
