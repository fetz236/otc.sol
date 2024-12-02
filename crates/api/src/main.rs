mod solana;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct BalanceRequest {
    pubkey: String,
}

async fn get_balance_endpoint(
    query: web::Query<BalanceRequest>,
) -> impl Responder {
    match solana::get_balance(&query.pubkey).await {
        Ok(balance) => HttpResponse::Ok().json(balance),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/balance", web::get().to(get_balance_endpoint))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
