pub mod hasher_service;

use actix_web::web;

pub fn config_hasher(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("hasher")
            .service(hasher_service::do_hash)
            .service(hasher_service::do_verify),
    );
}
