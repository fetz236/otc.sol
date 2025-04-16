use crate::auth::Claims;
use actix_web::{web, HttpResponse};
use db::models::NewTrade;
use db::schema::trades::dsl::*;
use diesel::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateTradeData {
    pub amount: i64,
    pub price: f64,
}

pub async fn create_trade(
    data: web::Json<CreateTradeData>,
    pool: web::Data<db::Pool>,
    claims: Claims,
) -> Result<HttpResponse, actix_web::Error> {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");

    let new_trade = NewTrade {
        creator_id: claims.sub.parse::<i32>().unwrap(),
        amount: data.amount,
        price: data.price,
        status: "Open".to_string(),
    };

    diesel::insert_into(trades)
        .values(&new_trade)
        .execute(&mut conn)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Database Error"))?;

    Ok(HttpResponse::Created().finish())
}


pub async fn get_trades(
    pool: web::Data<db::Pool>,
    claims: Claims,
) -> Result<HttpResponse, actix_web::Error> {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");

    let trades = trades
        .filter(creator_id.eq(claims.sub.parse::<i32>().unwrap()))
        .load::<Trade>(&mut conn)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Database Error"))?;

    Ok(HttpResponse::Ok().json(trades))
}

pub async fn get_trade(
    pool: web::Data<db::Pool>,
    claims: Claims,
) -> Result<HttpResponse, actix_web::Error> {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");

    let trade = trades
        .filter(creator_id.eq(claims.sub.parse::<i32>().unwrap()))
        .filter(id.eq(id.parse::<i32>().unwrap()))
        .first::<Trade>(&mut conn)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Database Error"))?;

    Ok(HttpResponse::Ok().json(trade))
}

pub async fn update_trade(
    pool: web::Data<db::Pool>,
    claims: Claims,
) -> Result<HttpResponse, actix_web::Error> {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");

    let trade = get_trade(pool, claims).await?;

    diesel::update(trades.find(trade.id))
        .set(&data)
        .execute(&mut conn)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Database Error"))?;

    Ok(HttpResponse::Ok().finish())
}

pub async fn delete_trade(
    pool: web::Data<db::Pool>,
    claims: Claims,
) -> Result<HttpResponse, actix_web::Error> {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");

    let trade = get_trade(pool, claims).await?;

    diesel::delete(trades.find(trade.id))
        .execute(&mut conn)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Database Error"))?;

    Ok(HttpResponse::Ok().finish())
}

pub async fn close_trade(
    pool: web::Data<db::Pool>,
    claims: Claims,
) -> Result<HttpResponse, actix_web::Error> {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");

    let trade = get_trade(pool, claims).await?;

    diesel::update(trades.find(trade.id))
        .set(&data)
        .execute(&mut conn)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Database Error"))?;

    Ok(HttpResponse::Ok().finish())
}

