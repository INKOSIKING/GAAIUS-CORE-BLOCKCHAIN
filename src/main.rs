mod blockchain;
mod network;
mod transactions;
mod vm;
mod api;

use blockchain::Blockchain;
use network::Network;
use transactions::Transaction;
use vm::{SmartContract, VMType};

#[tokio::main]
async fn main() {
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

    let blockchain = Blockchain::new();
    blockchain.start();

    let network = Network::new("localhost:8080");
    network.connect();

    let transaction = Transaction::new("gaaius_sender", "gaaius_receiver", 100);
    transaction.process();

    // Start the REST API
    api::start_api(blockchain).await;
}
