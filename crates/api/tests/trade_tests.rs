use api::handlers::{auth_handlers, trades};
use api::middleware::auth_middleware::AuthMiddleware;
use actix_web::{test, web, App};
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use std::env;
use diesel::RunQueryDsl;
use diesel::prelude::*;
use db::models::User;
use db::schema::users::dsl::*;
use db::schema::trades::dsl::*;

#[actix_web::test]
async fn test_create_trade() {
    // Set up test database connection pool
    let database_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    // Clean up the database before testing
    let mut conn = pool.get().expect("Failed to get DB connection");
    diesel::delete(trades).execute(&mut conn).unwrap_or(0);
    diesel::delete(users).execute(&mut conn).unwrap_or(0);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::scope("/auth")
                    .route("/register", web::post().to(auth_handlers::register))
                    .route("/login", web::post().to(auth_handlers::login)),
            )
            .service(
                web::scope("/trades")
                    .wrap(AuthMiddleware)
                    .route("", web::post().to(trades::create_trade)),
            ),
    )
    .await;

    // Register a test user
    let register_payload = serde_json::json!({
        "username": "testuser",
        "email": "test@example.com",
        "password": "password123"
    });

    let register_req = test::TestRequest::post()
        .uri("/auth/register")
        .set_json(&register_payload)
        .to_request();

    let register_resp = test::call_service(&app, register_req).await;
    assert_eq!(register_resp.status(), actix_web::http::StatusCode::CREATED);

    // Get the user ID from the database
    let mut conn = pool.get().expect("Failed to get DB connection");
    let _user = users
        .filter(username.eq("testuser"))
        .first::<User>(&mut conn)
        .expect("Failed to get user");

    let login_payload = serde_json::json!({
        "username": "testuser",
        "password": "password123"
    });

    let login_req = test::TestRequest::post()
        .uri("/auth/login")
        .set_json(&login_payload)
        .to_request();

    let login_resp = test::call_service(&app, login_req).await;
    assert_eq!(login_resp.status(), actix_web::http::StatusCode::OK);

    let login_body = test::read_body(login_resp).await;
    let login_json: serde_json::Value = serde_json::from_slice(&login_body).unwrap();
    let token = login_json.get("token").unwrap().as_str().unwrap();

    // Create a trade
    let create_trade_payload = serde_json::json!({
        "amount": 1000000,
        "price": 35.5
    });

    let trade_req = test::TestRequest::post()
        .uri("/trades")
        .set_json(&create_trade_payload)
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let trade_resp = test::call_service(&app, trade_req).await;
    assert_eq!(trade_resp.status(), actix_web::http::StatusCode::CREATED);
}
