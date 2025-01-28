pub mod hasher_service;
pub mod image_service;

use actix_web::web;

pub fn config_hasher(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("hasher")
            .service(hasher_service::do_hash)
            .service(hasher_service::do_verify),
    );
}

pub fn config_image(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("image").service(image_service::save_files));
}
