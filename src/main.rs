use actix_web::{get, App, HttpResponse, HttpServer, Responder};
mod v1 {
    pub mod routes;
}

use v1::routes::censor_text;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(censor_text).service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
