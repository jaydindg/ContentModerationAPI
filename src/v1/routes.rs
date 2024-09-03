use actix_web::{post, web, HttpResponse, Responder};

use censor::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct IncomingReqBody {
    pub content: Option<String>,
}

#[post("/api/v1/check-text")]
pub async fn check_text(req_body: web::Json<IncomingReqBody>) -> impl Responder {
    // Returns Bool
    let content = req_body
        .content
        .clone()
        .unwrap_or_else(|| "No content".to_string());

    let censor = Censor::Standard;

    if censor.check(&content) {
        return HttpResponse::Ok().json(true); // content has profanity, return true
    }

    HttpResponse::Ok().json(false) // content does not have profanity, return false
}

#[post("/api/v1/censor-text")]
pub async fn censor_text(req_body: web::Json<IncomingReqBody>) -> impl Responder {
    // Returns String
    let content = req_body
        .content
        .clone()
        .unwrap_or_else(|| "No content".to_string());

    let censor = Censor::Standard;

    if censor.check(&content) {
        let censored_content = censor.censor(&content);
        return HttpResponse::Ok().body(censored_content); // content has profanity, return censored
    }

    HttpResponse::Ok().body(content) // content does not have profanity, return original
}
