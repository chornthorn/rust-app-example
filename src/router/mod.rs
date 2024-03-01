use actix_web::web;

use crate::handler::product_handler;

pub fn product_router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/products")
            .route("", web::get().to(product_handler::get_products))
            .route("/{id}", web::get().to(product_handler::get_product)),
    );
}
