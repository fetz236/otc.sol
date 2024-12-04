use api::handlers::auth_handlers;
use db;
use actix_web::{test, web, App};
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use std::env;

#[actix_web::test]
async fn test_register() {
    // Set up test database connection pool
    let database_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/auth/register", web::post().to(auth_handlers::register)),
    )
    .await;

    let payload = serde_json::json!({
        "username": "testuser",
        "email": "test@example.com",
        "password": "password123"
    });

    let req = test::TestRequest::post()
        .uri("/auth/register")
        .set_json(&payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::CREATED);
}

#[actix_web::test]
async fn test_login() {
    // Set up test database connection pool
    let database_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/auth/register", web::post().to(auth_handlers::register))
            .route("/auth/login", web::post().to(auth_handlers::login)),
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

    let _ = test::call_service(&app, register_req).await;

    // Attempt to log in
    let login_payload = serde_json::json!({
        "username": "testuser",
        "password": "password123"
    });

    let login_req = test::TestRequest::post()
        .uri("/auth/login")
        .set_json(&login_payload)
        .to_request();

    let resp = test::call_service(&app, login_req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

    let resp_body = test::read_body(resp).await;
    let resp_json: serde_json::Value = serde_json::from_slice(&resp_body).unwrap();
    assert!(resp_json.get("token").is_some());
}
