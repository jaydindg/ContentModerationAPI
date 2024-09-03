use actix_web::{get, App, HttpResponse, HttpServer, Responder};
mod v1 {
    pub mod routes;
}

use v1::routes;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(routes::censor_text)
            .service(routes::check_text)
            .service(routes::replace_text)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
