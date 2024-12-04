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
