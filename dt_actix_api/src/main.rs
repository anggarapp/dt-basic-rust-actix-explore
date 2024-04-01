use dt_actix_api::server::Application;
use dt_actix_api::settings::get_app_mode;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_setting = get_app_mode().expect("Failed to read configuration file");
    let app = Application::build_app(app_setting).await?;
    app.run_app().await?;
    Ok(())
}
