use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use std::sync::{Arc, Mutex};
use crate::blockchain::Blockchain;

async fn explorer_ui(data: web::Data<Arc<Mutex<Blockchain>>>) -> impl Responder {
    let blockchain = data.lock().unwrap();
    let mut html = String::new();

    html.push_str("<html><head><title>GAAIUS Explorer</title></head><body>");
    html.push_str("<h1>GAAIUS Blockchain Explorer</h1><ul>");

    for block in &blockchain.chain {
        html.push_str(&format!(
            "<li><strong>Block #{}</strong><br>\
             Timestamp: {}<br>\
             Hash: {}<br>\
             Previous Hash: {}<br>\
             Transactions: {}<hr></li>",
            block.index,
            block.timestamp,
            block.hash,
            block.previous_hash,
            block.transactions.len()
        ));
    }

    html.push_str("</ul></body></html>");
    HttpResponse::Ok().body(html)
}

pub async fn start_explorer(blockchain: Arc<Mutex<Blockchain>>) -> std::io::Result<()> {
    println!("[Explorer] Running at http://localhost:8081");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(blockchain.clone()))
            .route("/", web::get().to(explorer_ui))
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}
