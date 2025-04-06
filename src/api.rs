use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

use crate::blockchain::Blockchain;
use crate::transactions::Transaction;

#[derive(Serialize)]
struct ChainResponse {
    chain: Vec<String>,
    length: usize,
}

#[derive(Deserialize)]
struct TxInput {
    sender: String,
    recipient: String,
    amount: u64,
}

async fn get_chain(data: web::Data<Blockchain>) -> impl Responder {
    let chain_data = data.get_chain()
        .into_iter()
        .map(|b| format!("{:?}", b))
        .collect();

    web::Json(ChainResponse {
        chain: chain_data,
        length: data.get_chain().len(),
    })
}

async fn add_transaction(tx: web::Json<TxInput>, data: web::Data<Blockchain>) -> impl Responder {
    let tx = Transaction::new(tx.sender.clone(), tx.recipient.clone(), tx.amount);
    data.add_transaction(tx);
    HttpResponse::Ok().body("Transaction added.")
}

pub async fn run_api(blockchain: Blockchain) -> std::io::Result<()> {
    let data = web::Data::new(blockchain);
    println!("[API] Starting REST API on http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .route("/chain", web::get().to(get_chain))
            .route("/tx", web::post().to(add_transaction))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
