use actix_web::web;

mod index;
mod hello;
mod auth;

use index::index_service;
use hello::hello_service;
use auth::{
    login,
    verify
};

pub fn service_hub(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(index_service)
            .service(hello_service)
            .service(login)
            .service(verify)
    );
}