use crate::schema::trades;
use crate::schema::users;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Trade {
    pub id: i32,
    pub creator_id: i32,
    pub amount: i64,
    pub price: f64,
    pub status: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = trades)]
pub struct NewTrade {
    pub creator_id: i32,
    pub amount: i64,
    pub price: f64,
    pub status: String,
}
