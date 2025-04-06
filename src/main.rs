mod blockchain;
mod network;
mod transactions;
mod vm; // Including the vm module for SmartContract usage

use blockchain::Blockchain;
use network::Network;
use transactions::Transaction;
use vm::{SmartContract, VMType}; // Import the SmartContract and VMType

fn main() {
    // Example usage of SmartContract deployment logic
    let contract = SmartContract {
        vm_type: VMType::WASM,
        bytecode: vec![0x00, 0x61, 0x73, 0x6D],
        gas_limit: 500_000,
        sender: "gaaius_sender".to_string(),
        contract_address: "gaaius_contract_001".to_string(),
    };

    match contract.deploy() {
        Ok(msg) => println!("Contract deployed successfully: {}", msg),
        Err(err) => eprintln!("Deployment failed: {}", err),
    }

    // Blockchain initialization and usage
    let blockchain = Blockchain::new();
    blockchain.start();

    // Network logic
    let network = Network::new("localhost:8080");
    network.connect();

    // Example transaction usage
    let transaction = Transaction::new("gaaius_sender", "gaaius_receiver", 100);
    transaction.process();
}
