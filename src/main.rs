use actix_web::{get, guard, post, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;

mod models;
mod services;

use services::config_users;

struct AppState {
    app_name: String,
}

struct AppStateWithCounter {
    counter: Mutex<i32>,
}

#[get("/heyho")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    HttpResponse::Ok().body(format!("Hello {}!", app_name))
}

async fn index(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    format!("Request number: {}", counter)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix-web"),
            }))
            .app_data(counter.clone())
            .configure(config_users)
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .route("/", web::get().to(index))
            .service(web::scope("/app").route("/index.html", web::get().to(manual_hello)))
            .service(
                web::scope("/rust-test")
                    .guard(guard::Host("localhost"))
                    .route(
                        "",
                        web::to(|| async { HttpResponse::Ok().body("www.rust-lang.org") }),
                    ),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
