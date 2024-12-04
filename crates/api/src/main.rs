use api::handlers::{auth_handlers, trades};
use api::solana;
use api::middleware::auth_middleware::AuthMiddleware;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use serde::Deserialize;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: DbPool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/balance", web::get().to(get_balance_endpoint))
            .service(
                web::scope("/auth")
                    .route("/register", web::post().to(auth_handlers::register))
                    .route("/login", web::post().to(auth_handlers::login)),
            )
            .service(
                web::scope("/trades")
                    .wrap(AuthMiddleware)
                    .route("", web::post().to(trades::create_trade)),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn get_balance_endpoint(query: web::Query<BalanceRequest>) -> impl Responder {
    match solana::get_balance(&query.pubkey).await {
        Ok(balance) => HttpResponse::Ok().json(balance),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[derive(Deserialize)]
struct BalanceRequest {
    pubkey: String,
}
