use crate::test_utils::start_test_server;

#[actix_web::test]
async fn test_het_ping() {
    let app = start_test_server().await;
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/ping", &app.address))
        .send()
        .await
        .expect("failed to execute GET /ping request");
    assert!(response.status().is_success())
}

#[actix_web::test]
async fn _test_third_party_api() {
    use wiremock::{
        matchers::{method, path},
        Mock, ResponseTemplate,
    };
    let app = start_test_server().await;
    let client = reqwest::Client::new();

    Mock::given(path("/third_party_api"))
        .and(method("GET"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.test_server)
        .await;
    let url = format!("{}/third_party_api", &app.test_server.uri());
    let response = client.get(&url).send().await.unwrap();

    assert_eq!(200, response.status().as_u16());
}
