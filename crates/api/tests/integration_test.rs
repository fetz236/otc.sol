use actix_web::{test, App};
use api::get_balance_endpoint;

#[actix_web::test]
async fn test_get_balance() {
    let app =
        test::init_service(App::new().route("/balance", web::get().to(get_balance_endpoint))).await;

    let req = test::TestRequest::get()
        .uri("/balance?pubkey=5oNDL3swdJJF1g9DzJiZ4ynHXgszjAEpUkxVYejchzrY")
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());
}
