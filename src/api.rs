// src/api.rs

use actix_web::{App, HttpServer};
use crate::blockchain::Blockchain;
use crate::explorer::explorer;
use crate::cli::health_check;

pub async fn run_server(blockchain: Blockchain) -> std::io::Result<()> {
    let data = actix_web::web::Data::new(blockchain);

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(health_check)
            .service(explorer)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
