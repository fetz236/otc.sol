use api::handlers::auth_handlers;
use actix_web::{test, web, App};
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use std::env;
use diesel::RunQueryDsl;

#[actix_web::test]
async fn test_register() {
    // Initialize logger
    let _ = env_logger::builder().is_test(true).try_init();

    // Set up test database connection pool
    let database_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    // Clean the database before the test
    {
        let mut conn = pool.get().expect("Failed to get DB connection");
        diesel::sql_query("TRUNCATE users CASCADE")
            .execute(&mut conn)
            .expect("Failed to truncate users table");
    }

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

    if resp.status() != actix_web::http::StatusCode::CREATED {
        let body = test::read_body(resp).await;
        println!("Response body: {}", String::from_utf8_lossy(&body));
        panic!(
            "Expected status 201 CREATED, got {}",
            resp.status()
        );
    }

    assert_eq!(resp.status(), actix_web::http::StatusCode::CREATED);
}

#[actix_web::test]
async fn test_login() {
    // Initialize logger
    let _ = env_logger::builder().is_test(true).try_init();

    // Set up test database connection pool
    let database_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    // Clean the database before the test
    {
        let mut conn = pool.get().expect("Failed to get DB connection");
        diesel::sql_query("TRUNCATE users CASCADE")
            .execute(&mut conn)
            .expect("Failed to truncate users table");
    }

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

    let register_resp = test::call_service(&app, register_req).await;

    // Check registration was successful
    if register_resp.status() != actix_web::http::StatusCode::CREATED {
        let body = test::read_body(register_resp).await;
        println!("Registration response body: {}", String::from_utf8_lossy(&body));
        panic!(
            "Expected status 201 CREATED for registration, got {}",
            register_resp.status()
        );
    }

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

    if resp.status() != actix_web::http::StatusCode::OK {
        let body = test::read_body(resp).await;
        println!("Login response body: {}", String::from_utf8_lossy(&body));
        panic!(
            "Expected status 200 OK, got {}",
            resp.status()
        );
    }

    assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

    let resp_body = test::read_body(resp).await;
    let resp_json: serde_json::Value = serde_json::from_slice(&resp_body).unwrap();
    assert!(resp_json.get("token").is_some());
}
