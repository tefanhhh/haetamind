use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

#[post("")]
pub async fn create() -> impl Responder {
    HttpResponse::Ok().body("Hello world create!")
}

#[get("")]
async fn find_all() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/{username}")]
pub async fn find_one(username: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("find_one: {}", username))
}

#[put("")]
pub async fn update() -> impl Responder {
    HttpResponse::Ok().body("Hello world update!")
}

#[delete("")]
pub async fn delete() -> impl Responder {
    HttpResponse::Ok().body("Hello world delete!")
}
