use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use censor::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct IncomingReqBody {
    content: Option<String>,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/check")]
async fn check(req_body: web::Json<IncomingReqBody>) -> impl Responder {
    let content = req_body
        .content
        .clone()
        .unwrap_or_else(|| "No content".to_string());

    let censor = Censor::Standard;

    if censor.check(&content) {
        let censored_content = censor.censor(&content);
        return HttpResponse::Ok().body(censored_content);
    }

    HttpResponse::Ok().body(content)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(check).service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
