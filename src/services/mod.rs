pub mod user_service;

use actix_web::web;

pub fn config_users(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("users")
            .service(user_service::create)
            .service(user_service::find_all)
            .service(user_service::find_one)
            .service(user_service::update)
            .service(user_service::delete),
    );
}
