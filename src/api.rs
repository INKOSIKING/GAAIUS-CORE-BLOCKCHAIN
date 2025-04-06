use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

use crate::blockchain::{Block, Blockchain};
use crate::transactions::Transaction;

#[derive(Serialize)]
struct BlockResponse {
    index: usize,
    hash: String,
    previous_hash: String,
    timestamp: u128,
    transactions: Vec<Transaction>,
}

async fn get_blocks(data: web::Data<Arc<Mutex<Blockchain>>>) -> impl Responder {
    let blockchain = data.lock().unwrap();
    let blocks: Vec<BlockResponse> = blockchain.chain.iter().map(|block| BlockResponse {
        index: block.index,
        hash: block.hash.clone(),
        previous_hash: block.previous_hash.clone(),
        timestamp: block.timestamp,
        transactions: block.transactions.clone(),
    }).collect();

    HttpResponse::Ok().json(blocks)
}

#[derive(Deserialize)]
struct TxRequest {
    sender: String,
    recipient: String,
    amount: u64,
}

async fn post_transaction(
    data: web::Data<Arc<Mutex<Blockchain>>>,
    tx: web::Json<TxRequest>,
) -> impl Responder {
    let mut blockchain = data.lock().unwrap();
    let transaction = Transaction {
        sender: tx.sender.clone(),
        recipient: tx.recipient.clone(),
        amount: tx.amount,
    };
    blockchain.add_transaction(transaction);
    HttpResponse::Ok().json("Transaction added")
}

pub async fn start_api(blockchain: Arc<Mutex<Blockchain>>) -> std::io::Result<()> {
    println!("[REST API] Starting server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(blockchain.clone()))
            .route("/blocks", web::get().to(get_blocks))
            .route("/transaction", web::post().to(post_transaction))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
