use api::solana::get_balance_endpoint;
use actix_web::{test, web, App};

#[actix_web::test]
async fn test_get_balance() {
    let app = test::init_service(
        App::new().route("/balance", web::get().to(get_balance_endpoint)),
    )
    .await;

    let pubkey = "4Nd1m3wvtZ8G5qbf5DqZX3CNB44vHEB8u6SiSHGVekbj"; // Example public key
    let req = test::TestRequest::get()
        .uri(&format!("/balance?pubkey={}", pubkey))
        .to_request();

    let resp = test::call_service(&app, req).await;

    let status = resp.status();
    let body = test::read_body(resp).await;

    println!(
        "Response status: {}, body: {}",
        status,
        String::from_utf8_lossy(&body)
    );

    assert!(status.is_success(), "Expected success, got {}", status);

    let balance: u64 = serde_json::from_slice(&body).expect("Failed to parse response body");
    assert_eq!(balance, 0); // Expected mock balance
}