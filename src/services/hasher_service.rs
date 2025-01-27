use actix_web::{post, web, HttpResponse, Responder};
use bcrypt::{hash, verify, DEFAULT_COST};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct HashRequest {
    value: String,
}

#[derive(Deserialize)]
pub struct VerifyRequest {
    value: String,
    hash: String,
}

#[post("")]
pub async fn do_hash(body: web::Json<HashRequest>) -> impl Responder {
    match hash(body.value.as_str(), DEFAULT_COST) {
        Ok(hash) => HttpResponse::Ok().body(hash.to_string()),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("verify")]
pub async fn do_verify(body: web::Json<VerifyRequest>) -> impl Responder {
    match verify(body.value.as_bytes(), body.hash.as_str()) {
        Ok(_) => HttpResponse::Ok().body("true"),
        Err(_) => HttpResponse::Ok().body("false"),
    }
}
