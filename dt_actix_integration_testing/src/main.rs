use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let address = format!("{}:{}", "127.0.0.1", "8099");
    let listener = TcpListener::bind(address)?;
    dt_actix_integration_testing::server::start(listener)?.await?;
    Ok(())
}
