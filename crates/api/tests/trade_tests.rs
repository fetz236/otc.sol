use api::handlers::{auth_handlers, trades};
use api::middleware::auth_middleware::AuthMiddleware;
use actix_web::{test, web, App};
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use std::env;
use diesel::RunQueryDsl;
use diesel::prelude::*;
use db::models::{User, Trade};
use db::schema::users::dsl::*;
use db::schema::trades::dsl::*;

// Helper function to set up test app with all routes
fn setup_test_app(pool: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>) -> App {
    test::init_service(
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
                    .route("", web::post().to(trades::create_trade))
                    .route("", web::get().to(trades::get_trades))
                    .route("/{id}", web::get().to(trades::get_trade))
                    .route("/{id}", web::put().to(trades::update_trade))
                    .route("/{id}", web::delete().to(trades::delete_trade))
                    .route("/{id}/close", web::post().to(trades::close_trade)),
            ),
    )
    .await
}

// Helper function to create a test user and get auth token
async fn setup_test_user(app: &App) -> String {
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

    let register_resp = test::call_service(app, register_req).await;
    assert_eq!(register_resp.status(), actix_web::http::StatusCode::CREATED);

    // Login to get token
    let login_payload = serde_json::json!({
        "username": "testuser",
        "password": "password123"
    });

    let login_req = test::TestRequest::post()
        .uri("/auth/login")
        .set_json(&login_payload)
        .to_request();

    let login_resp = test::call_service(app, login_req).await;
    assert_eq!(login_resp.status(), actix_web::http::StatusCode::OK);

    let login_body = test::read_body(login_resp).await;
    let login_json: serde_json::Value = serde_json::from_slice(&login_body).unwrap();
    login_json.get("token").unwrap().as_str().unwrap().to_string()
}

// Helper function to clean up database
fn cleanup_database(pool: &r2d2::Pool<ConnectionManager<PgConnection>>) {
    let mut conn = pool.get().expect("Failed to get DB connection");
    diesel::delete(trades).execute(&mut conn).unwrap_or(0);
    diesel::delete(users).execute(&mut conn).unwrap_or(0);
}

#[actix_web::test]
async fn test_create_trade() {
    let database_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    cleanup_database(&pool);
    let app = setup_test_app(web::Data::new(pool.clone()));
    let token = setup_test_user(&app).await;

    // Create a trade
    let create_trade_payload = serde_json::json!({
        "amount": 1000000,
        "price": 35.5,
        "side": "buy"
    });

    let trade_req = test::TestRequest::post()
        .uri("/trades")
        .set_json(&create_trade_payload)
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let trade_resp = test::call_service(&app, trade_req).await;
    assert_eq!(trade_resp.status(), actix_web::http::StatusCode::CREATED);

    let trade_body = test::read_body(trade_resp).await;
    let trade: Trade = serde_json::from_slice(&trade_body).unwrap();
    assert_eq!(trade.amount, 1000000);
    assert_eq!(trade.price, 35.5);
    assert_eq!(trade.status, "Open");
}

#[actix_web::test]
async fn test_get_trades() {
    let database_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    cleanup_database(&pool);
    let app = setup_test_app(web::Data::new(pool.clone()));
    let token = setup_test_user(&app).await;

    // Create a trade first
    let create_trade_payload = serde_json::json!({
        "amount": 1000000,
        "price": 35.5,
        "side": "buy"
    });

    let trade_req = test::TestRequest::post()
        .uri("/trades")
        .set_json(&create_trade_payload)
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let _ = test::call_service(&app, trade_req).await;

    // Get trades
    let get_trades_req = test::TestRequest::get()
        .uri("/trades")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let get_trades_resp = test::call_service(&app, get_trades_req).await;
    assert_eq!(get_trades_resp.status(), actix_web::http::StatusCode::OK);

    let trades_body = test::read_body(get_trades_resp).await;
    let trades: Vec<Trade> = serde_json::from_slice(&trades_body).unwrap();
    assert_eq!(trades.len(), 1);
    assert_eq!(trades[0].amount, 1000000);
    assert_eq!(trades[0].price, 35.5);
    assert_eq!(trades[0].status, "Open");
}

#[actix_web::test]
async fn test_get_trade() {
    let database_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    cleanup_database(&pool);
    let app = setup_test_app(web::Data::new(pool.clone()));
    let token = setup_test_user(&app).await;

    // Create a trade first
    let create_trade_payload = serde_json::json!({
        "amount": 1000000,
        "price": 35.5,
        "side": "buy"
    });

    let trade_req = test::TestRequest::post()
        .uri("/trades")
        .set_json(&create_trade_payload)
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let _ = test::call_service(&app, trade_req).await;

    // Get the trade ID
    let mut conn = pool.get().expect("Failed to get DB connection");
    let trade = trades.first::<Trade>(&mut conn).unwrap();

    // Get specific trade
    let get_trade_req = test::TestRequest::get()
        .uri(&format!("/trades/{}", trade.id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let get_trade_resp = test::call_service(&app, get_trade_req).await;
    assert_eq!(get_trade_resp.status(), actix_web::http::StatusCode::OK);

    let trade_body = test::read_body(get_trade_resp).await;
    let trade: Trade = serde_json::from_slice(&trade_body).unwrap();
    assert_eq!(trade.amount, 1000000);
    assert_eq!(trade.price, 35.5);
    assert_eq!(trade.status, "Open");
}

#[actix_web::test]
async fn test_update_trade() {
    let database_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    cleanup_database(&pool);
    let app = setup_test_app(web::Data::new(pool.clone()));
    let token = setup_test_user(&app).await;

    // Create a trade first
    let create_trade_payload = serde_json::json!({
        "amount": 1000000,
        "price": 35.5,
        "side": "buy"
    });

    let trade_req = test::TestRequest::post()
        .uri("/trades")
        .set_json(&create_trade_payload)
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let _ = test::call_service(&app, trade_req).await;

    // Get the trade ID
    let mut conn = pool.get().expect("Failed to get DB connection");
    let trade = trades.first::<Trade>(&mut conn).unwrap();

    // Update trade
    let update_trade_payload = serde_json::json!({
        "amount": 2000000,
        "price": 40.0
    });

    let update_trade_req = test::TestRequest::put()
        .uri(&format!("/trades/{}", trade.id))
        .set_json(&update_trade_payload)
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let update_trade_resp = test::call_service(&app, update_trade_req).await;
    assert_eq!(update_trade_resp.status(), actix_web::http::StatusCode::OK);

    // Verify update
    let get_trade_req = test::TestRequest::get()
        .uri(&format!("/trades/{}", trade.id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let get_trade_resp = test::call_service(&app, get_trade_req).await;
    let trade_body = test::read_body(get_trade_resp).await;
    let updated_trade: Trade = serde_json::from_slice(&trade_body).unwrap();
    assert_eq!(updated_trade.amount, 2000000);
    assert_eq!(updated_trade.price, 40.0);
}

#[actix_web::test]
async fn test_delete_trade() {
    let database_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    cleanup_database(&pool);
    let app = setup_test_app(web::Data::new(pool.clone()));
    let token = setup_test_user(&app).await;

    // Create a trade first
    let create_trade_payload = serde_json::json!({
        "amount": 1000000,
        "price": 35.5,
        "side": "buy"
    });

    let trade_req = test::TestRequest::post()
        .uri("/trades")
        .set_json(&create_trade_payload)
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let _ = test::call_service(&app, trade_req).await;

    // Get the trade ID
    let mut conn = pool.get().expect("Failed to get DB connection");
    let trade = trades.first::<Trade>(&mut conn).unwrap();

    // Delete trade
    let delete_trade_req = test::TestRequest::delete()
        .uri(&format!("/trades/{}", trade.id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let delete_trade_resp = test::call_service(&app, delete_trade_req).await;
    assert_eq!(delete_trade_resp.status(), actix_web::http::StatusCode::OK);

    // Verify deletion
    let get_trade_req = test::TestRequest::get()
        .uri(&format!("/trades/{}", trade.id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let get_trade_resp = test::call_service(&app, get_trade_req).await;
    assert_eq!(get_trade_resp.status(), actix_web::http::StatusCode::NOT_FOUND);
}

#[actix_web::test]
async fn test_close_trade() {
    let database_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    cleanup_database(&pool);
    let app = setup_test_app(web::Data::new(pool.clone()));
    let token = setup_test_user(&app).await;

    // Create a trade first
    let create_trade_payload = serde_json::json!({
        "amount": 1000000,
        "price": 35.5,
        "side": "buy"
    });

    let trade_req = test::TestRequest::post()
        .uri("/trades")
        .set_json(&create_trade_payload)
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let _ = test::call_service(&app, trade_req).await;

    // Get the trade ID
    let mut conn = pool.get().expect("Failed to get DB connection");
    let trade = trades.first::<Trade>(&mut conn).unwrap();

    // Close trade
    let close_trade_req = test::TestRequest::post()
        .uri(&format!("/trades/{}/close", trade.id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let close_trade_resp = test::call_service(&app, close_trade_req).await;
    assert_eq!(close_trade_resp.status(), actix_web::http::StatusCode::OK);

    // Verify trade is closed
    let get_trade_req = test::TestRequest::get()
        .uri(&format!("/trades/{}", trade.id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let get_trade_resp = test::call_service(&app, get_trade_req).await;
    let trade_body = test::read_body(get_trade_resp).await;
    let closed_trade: Trade = serde_json::from_slice(&trade_body).unwrap();
    assert_eq!(closed_trade.status, "Closed");
}
    