use crate::states::app_config::AppConfig;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;

mod handler;
mod models;
mod router;
mod states;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // load the environment variables from the .env file
    dotenv().ok();

    // read the environment variable
    let app_port: String = env::var("APP_PORT").expect("APP_PORT must be set");
    let env_mode: String = env::var("ENV_MODE").expect("ENV_MODE must be set");
    let app_name: String = env::var("APP_NAME").expect("APP_NAME must be set");
    let db_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    impl AppConfig {
        pub fn new(app_name: String, env_mode: String, db_url: String) -> Self {
            AppConfig {
                app_name,
                env_mode,
                db_url: Some(db_url),
            }
        }
    }

    // create an instance of the AppConfig struct
    let app_config = web::Data::new(AppConfig::new(
        app_name.clone(),
        env_mode.clone(),
        db_url.clone(),
    ));

    // start the HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(app_config.clone())
            .service(web::scope("/api").configure(router::product_router))
    })
    .bind(format!("127.0.0.1:{}", app_port))?
    .run()
    .await
}
