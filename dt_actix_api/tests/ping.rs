use dt_actix_api::server::{get_connection_pool, Application};
use dt_actix_api::settings::get_app_mode;
use sqlx::PgPool;

pub struct MyApp {
    pub address: String,
    pub db_pool: PgPool,
}

async fn start_server() -> MyApp {
    let configuration = get_app_mode().expect("Failed to read configuration.");

    let app = Application::build_app(configuration.clone())
        .await
        .expect("Failed to build application");

    let address = format!("http://127.0.0.1:8099");

    let _ = actix_web::rt::spawn(app.run_app());

    MyApp {
        address,
        db_pool: get_connection_pool(&configuration.database),
    }
}
#[actix_web::test]
async fn test_het_ping() {
    let app = start_server().await;
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/ping", &app.address))
        .send()
        .await
        .expect("failed to execute GET /ping request");
    assert!(response.status().is_success())
}
