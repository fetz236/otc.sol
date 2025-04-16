use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use db::models::Trade;
use db::schema::trades::dsl::{trades, id as trade_id};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTradeData {
    pub amount: i64,
    pub price: f64,
    pub side: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTradeData {
    pub amount: Option<i64>,
    pub price: Option<f64>,
    pub status: Option<String>,
}

pub async fn get_trades(conn: web::Data<db::Pool>) -> impl Responder {
    let mut conn = conn.get().expect("Failed to get DB connection");
    let results = trades.load::<Trade>(&mut conn).expect("Error loading trades");
    HttpResponse::Ok().json(results)
}

pub async fn get_trade(path: web::Path<i32>, conn: web::Data<db::Pool>) -> impl Responder {
    let trade_id = path.into_inner();
    let mut conn = conn.get().expect("Failed to get DB connection");
    
    let trade = trades
        .filter(trade_id.eq(trade_id))
        .first::<Trade>(&mut conn)
        .expect("Error loading trade");

    HttpResponse::Ok().json(trade)
}

pub async fn create_trade(
    trade_data: web::Json<CreateTradeData>,
    conn: web::Data<db::Pool>,
) -> impl Responder {
    let mut conn = conn.get().expect("Failed to get DB connection");

    let new_trade = Trade {
        id: 0, // Will be set by the database
        creator_id: 1, // TODO: Get from auth
        amount: trade_data.amount,
        price: trade_data.price,
        status: "Open".to_string(),
        created_at: chrono::Utc::now().naive_utc(),
    };

    diesel::insert_into(trades)
        .values(&new_trade)
        .execute(&mut conn)
        .expect("Error saving new trade");

    HttpResponse::Created().json(new_trade)
}

pub async fn update_trade(
    path: web::Path<i32>,
    update_data: web::Json<UpdateTradeData>,
    conn: web::Data<db::Pool>,
) -> impl Responder {
    let trade_id = path.into_inner();
    let mut conn = conn.get().expect("Failed to get DB connection");

    let mut update = Vec::new();
    
    if let Some(amount) = update_data.amount {
        update.push(amount.eq(amount));
    }
    if let Some(price) = update_data.price {
        update.push(price.eq(price));
    }
    if let Some(status) = &update_data.status {
        update.push(status.eq(status));
    }

    if !update.is_empty() {
        diesel::update(trades.filter(trade_id.eq(trade_id)))
            .set(&update)
            .execute(&mut conn)
            .expect("Error updating trade");
    }

    let updated_trade = trades
        .filter(trade_id.eq(trade_id))
        .first::<Trade>(&mut conn)
        .expect("Error loading updated trade");

    HttpResponse::Ok().json(updated_trade)
}

pub async fn delete_trade(path: web::Path<i32>, conn: web::Data<db::Pool>) -> impl Responder {
    let trade_id = path.into_inner();
    let mut conn = conn.get().expect("Failed to get DB connection");

    diesel::delete(trades.filter(trade_id.eq(trade_id)))
        .execute(&mut conn)
        .expect("Error deleting trade");

    HttpResponse::Ok().json(serde_json::json!({
        "message": "Trade deleted successfully"
    }))
}

pub async fn close_trade(path: web::Path<i32>, conn: web::Data<db::Pool>) -> impl Responder {
    let trade_id = path.into_inner();
    let mut conn = conn.get().expect("Failed to get DB connection");

    diesel::update(trades.filter(trade_id.eq(trade_id)))
        .set(status.eq("Closed"))
        .execute(&mut conn)
        .expect("Error closing trade");

    let closed_trade = trades
        .filter(trade_id.eq(trade_id))
        .first::<Trade>(&mut conn)
        .expect("Error loading closed trade");

    HttpResponse::Ok().json(closed_trade)
}

