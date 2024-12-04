use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use solana_sdk::pubkey::Pubkey;
use solana_client::rpc_client::RpcClient;
use std::str::FromStr;

#[derive(Deserialize)]
pub struct BalanceRequest {
    pub pubkey: String,
}

pub async fn get_balance_endpoint(query: web::Query<BalanceRequest>) -> impl Responder {
    match get_balance(&query.pubkey).await {
        Ok(balance) => HttpResponse::Ok().json(balance),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn get_balance(pubkey_str: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let rpc_url = "https://api.mainnet-beta.solana.com";
    let pubkey = Pubkey::from_str(pubkey_str)?;

    // Run the blocking call in a threadpool
    let balance = web::block(move || {
        let client = RpcClient::new(rpc_url.to_string());
        client.get_balance(&pubkey)
    })
    .await??; // Unwrap Result twice: one from `web::block`, another from `get_balance`

    Ok(balance)
}