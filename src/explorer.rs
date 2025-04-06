use actix_web::{get, web, Responder, HttpResponse};
use serde::Serialize;
use std::sync::{Arc, Mutex};
use crate::blockchain::{Blockchain, Block};

#[derive(Serialize)]
pub struct WalletInfo {
    pub address: String,
    pub balance: u64,
    pub tx_count: usize,
}

#[get("/blocks")]
pub async fn get_blocks(data: web::Data<Arc<Mutex<Blockchain>>>) -> impl Responder {
    let blockchain = data.lock().unwrap();
    HttpResponse::Ok().json(&blockchain.chain)
}

#[get("/block/{hash}")]
pub async fn get_block_by_hash(
    path: web::Path<String>,
    data: web::Data<Arc<Mutex<Blockchain>>>,
) -> impl Responder {
    let hash = path.into_inner();
    let blockchain = data.lock().unwrap();
    if let Some(block) = blockchain.chain.iter().find(|b| b.hash == hash) {
        HttpResponse::Ok().json(block)
    } else {
        HttpResponse::NotFound().body("Block not found")
    }
}

#[get("/wallet/{address}")]
pub async fn get_wallet_info(
    path: web::Path<String>,
    data: web::Data<Arc<Mutex<Blockchain>>>,
) -> impl Responder {
    let address = path.into_inner();
    let blockchain = data.lock().unwrap();

    let balance = blockchain.get_balance(&address);
    let tx_count = blockchain
        .chain
        .iter()
        .flat_map(|b| b.transactions.iter())
        .filter(|tx| tx.sender == address || tx.receiver == address)
        .count();

    let info = WalletInfo {
        address,
        balance,
        tx_count,
    };

    HttpResponse::Ok().json(info)
}

#[get("/transaction/{hash}")]
pub async fn get_transaction_by_hash(
    path: web::Path<String>,
    data: web::Data<Arc<Mutex<Blockchain>>>,
) -> impl Responder {
    let hash = path.into_inner();
    let blockchain = data.lock().unwrap();

    for block in &blockchain.chain {
        for tx in &block.transactions {
            if tx.hash == hash {
                return HttpResponse::Ok().json(tx);
            }
        }
    }

    HttpResponse::NotFound().body("Transaction not found")
}
