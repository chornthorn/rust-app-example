use actix_web::{web, HttpResponse};

use crate::config::app_config;

pub async fn get_products(app_config: web::Data<app_config::AppConfig>) -> HttpResponse {
    HttpResponse::Ok().json({
        app_config::AppConfig {
            app_name: app_config.app_name.clone(),
            env_mode: app_config.env_mode.clone(),
            db_url: None,
        }
    })
}

pub async fn get_product() -> HttpResponse {
    HttpResponse::Ok().body("Get product by id")
}
