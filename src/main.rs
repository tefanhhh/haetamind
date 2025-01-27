mod services;

use actix_web::{App, HttpServer};

use services::config_hasher;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new().configure(config_hasher))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
