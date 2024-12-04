use actix_web::{web, HttpResponse};
use bcrypt::{hash, verify, DEFAULT_COST};
use db::models::{NewUser, User};
use diesel::prelude::*;
use serde::Deserialize;
use crate::auth::create_token;
use db::schema::users::dsl::users as users_table;
use db::schema::users::username;
use db::schema::users::dsl::users;

#[derive(Deserialize)]
pub struct RegisterData {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub async fn register(
    data: web::Json<RegisterData>,
    pool: web::Data<db::Pool>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");

    let password_hash = hash(&data.password, DEFAULT_COST)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Password hashing error"))?;

    let new_user = NewUser {
        username: data.username.clone(),
        email: data.email.clone(),
        password_hash,
    };

    diesel::insert_into(users_table)
        .values(&new_user)
        .execute(&mut conn)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Database insertion error"))?;

    Ok(HttpResponse::Created().finish())
}

#[derive(Deserialize)]
pub struct LoginData {
    pub username: String,
    pub password: String,
}

pub async fn login(
    data: web::Json<LoginData>,
    pool: web::Data<db::Pool>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");

    let user = users
        .filter(username.eq(&data.username))
        .first::<User>(&mut conn)
        .optional()
        .map_err(|_| actix_web::error::ErrorInternalServerError("Database Error"))?;

    if let Some(user) = user {
        let is_valid = verify(&data.password, &user.password_hash)
            .map_err(|_| actix_web::error::ErrorInternalServerError("Database Error"))?;

        if is_valid {
            let token = create_token(&user.id.to_string())
                .map_err(|_| actix_web::error::ErrorInternalServerError("Database Error"))?;

            return Ok(HttpResponse::Ok().json(serde_json::json!({ "token": token })));
        }
    }

    Ok(HttpResponse::Unauthorized().finish())
}
