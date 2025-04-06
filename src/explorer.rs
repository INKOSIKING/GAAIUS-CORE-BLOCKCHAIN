// src/explorer.rs

use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::blockchain::{Blockchain, Transaction};

#[derive(Serialize)]
struct WalletView {
    address: String,
    balance: u64,
    transactions: Vec<Transaction>,
}

#[derive(Deserialize)]
pub struct QueryParams {
    address: Option<String>,
    tx_id: Option<String>,
    page: Option<usize>,
    limit: Option<usize>,
}

#[get("/explorer")]
pub async fn explorer(
    data: web::Data<Blockchain>,
    query: web::Query<QueryParams>,
) -> impl Responder {
    let blockchain = data.get_ref();
    let mut results = vec![];

    if let Some(addr) = &query.address {
        let txs: Vec<Transaction> = blockchain
            .get_all_transactions()
            .into_iter()
            .filter(|tx| &tx.from == addr || &tx.to == addr)
            .collect();

        let balance = blockchain.get_balance(addr);

        let wallet = WalletView {
            address: addr.clone(),
            balance,
            transactions: txs,
        };

        return HttpResponse::Ok().json(wallet);
    }

    if let Some(tx_id) = &query.tx_id {
        if let Some(tx) = blockchain.get_transaction_by_id(tx_id) {
            return HttpResponse::Ok().json(tx);
        } else {
            return HttpResponse::NotFound().body("Transaction not found");
        }
    }

    // Default case: return paginated transaction list
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);
    let start = (page - 1) * limit;

    results = blockchain
        .get_all_transactions()
        .into_iter()
        .skip(start)
        .take(limit)
        .collect();

    HttpResponse::Ok().json(results)
}
