use actix_web::{App, HttpServer, web};
use std::sync::{Arc, Mutex};
use crate::blockchain::Blockchain;
use crate::explorer::{get_blocks, get_block_by_hash, get_wallet_info, get_transaction_by_hash};

pub async fn run_api_server(blockchain: Arc<Mutex<Blockchain>>) -> std::io::Result<()> {
    println!("Starting GAAIUS API Explorer at http://localhost:8000");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(blockchain.clone()))
            .service(get_blocks)
            .service(get_block_by_hash)
            .service(get_wallet_info)
            .service(get_transaction_by_hash)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
