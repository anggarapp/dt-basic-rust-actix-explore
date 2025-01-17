use chrono::Local;
use dt_actix_api::server::{get_connection_pool, Application};
use dt_actix_api::settings::{get_app_mode, DatabaseSettings};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;
use wiremock::MockServer;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
    pub port: u16,
    pub test_server: MockServer,
}

impl TestApp {
    pub async fn payload_for_get(&self, endpoint: &str) -> reqwest::Response {
        reqwest::Client::new()
            .get(&format!("{}/{}", &self.address, endpoint.to_string()))
            .header("Content-Type", "application/json") // Update the content type
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn payload_for_post(&self, body: String, endpoint: &str) -> reqwest::Response {
        reqwest::Client::new()
            .post(&format!("{}/{}", &self.address, endpoint.to_string()))
            .header("Content-Type", "application/json") // Update the content type
            .body(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn payload_for_put(&self, body: String, endpoint: &str) -> reqwest::Response {
        reqwest::Client::new()
            .put(&format!("{}/{}", &self.address, endpoint.to_string()))
            .header("Content-Type", "application/json") // Update the content type
            .body(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn payload_for_delete(&self, body: String, endpoint: &str) -> reqwest::Response {
        reqwest::Client::new()
            .delete(&format!("{}/{}", &self.address, endpoint.to_string()))
            .header("Content-Type", "application/json") // Update the content type
            .body(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }
}

pub async fn start_test_server() -> TestApp {
    let test_server = MockServer::start().await;
    let current_time = Local::now();
    let time_prefix = current_time.format("%Y%m%d%H%M%S").to_string();

    let configuration = {
        let mut cfg = get_app_mode().expect("Failed to read configuration.");

        cfg.database.database_name = format!(
            "{}_{}_{}",
            "dt_actix_api_test".to_string(),
            time_prefix,
            Uuid::new_v4().to_string()
        );
        cfg.application.port = 0;
        cfg.test_client.base_url = test_server.uri();

        cfg
    };

    // create and migrate the database
    configure_database(&configuration.database).await;

    let app = Application::build_app(configuration.clone())
        .await
        .expect("Failed to build application");

    let application_port = app.port();

    let address = format!("http://127.0.0.1:{}", application_port);

    let _ = actix_web::rt::spawn(app.run_app());

    TestApp {
        address,
        db_pool: get_connection_pool(&configuration.database),
        port: application_port,
        test_server,
    }
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect_with(&config.parse_connection_string())
        .await
        .expect("Failed to parse db config");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database");

    let connection_pool = PgPool::connect_with(config.database_connection_string())
        .await
        .expect("Failed to connect to Postgres db");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate database");

    connection_pool
}
