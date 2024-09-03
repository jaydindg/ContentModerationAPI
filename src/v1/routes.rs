use actix_web::{post, web, HttpResponse, Responder};

use censor::*;
use serde::Deserialize;

const NO_CONTENT_STRING: &str = "No content sent to API.";

/// Represents the incoming request body containing optional text content.
///
/// # Fields
///
/// * `content` - An optional `String` that holds the text content to be processed.
#[derive(Deserialize)]
struct IncomingReqBody {
    content: Option<String>,
}

#[derive(Deserialize)]
struct GrawlixQueryParams {
    grawlix: String,
}

/// Asynchronously checks if the provided text contains profanity.
///
/// # Arguments
///
/// * `req_body` - A JSON object containing the text to be checked.
///
/// # Returns
///
/// * `HttpResponse` - Returns `true` if the text contains profanity, otherwise `false`.
#[post("/api/v1/check-text")]
pub async fn check_text(req_body: web::Json<IncomingReqBody>) -> impl Responder {
    // Returns Bool
    let content = req_body
        .content
        .clone()
        .unwrap_or_else(|| NO_CONTENT_STRING.to_string());

    let censor = Censor::Standard;

    if censor.check(&content) {
        return HttpResponse::Ok().json(true); // content has profanity, return true
    }

    HttpResponse::Ok().json(false) // content does not have profanity, return false
}

/// Asynchronously censors any profanity in the provided text.
///
/// # Arguments
///
/// * `req_body` - A JSON object containing the text to be censored.
///
/// # Returns
///
/// * `HttpResponse` - Returns the censored text if profanity is found, otherwise returns the original text.
#[post("/api/v1/censor-text")]
pub async fn censor_text(req_body: web::Json<IncomingReqBody>) -> impl Responder {
    // Returns String
    let content = req_body
        .content
        .clone()
        .unwrap_or_else(|| NO_CONTENT_STRING.to_string());

    let censor = Censor::Standard;

    if censor.check(&content) {
        let censored_content = censor.censor(&content);
        return HttpResponse::Ok().body(censored_content); // content has profanity, return censored
    }

    HttpResponse::Ok().body(content) // content does not have profanity, return original
}

/// This function handles POST requests to the `/api/v1/replace-text` endpoint.
/// It takes a JSON body and query parameters, processes the content, and returns
/// a censored version of the text if any inappropriate content is found.
///
/// # Arguments
///
/// * `req_body` - A JSON body containing the content to be processed.
/// * `grawlix_input` - Query parameters containing the grawlix string to replace inappropriate content.
///
/// # Returns
///
/// * `HttpResponse` - Returns an HTTP response with the processed content. If inappropriate content is found,
/// it is replaced with the grawlix string. Otherwise, the original content is returned.
#[post("/api/v1/replace-text")]
pub async fn replace_text(
    req_body: web::Json<IncomingReqBody>,
    grawlix_input: web::Query<GrawlixQueryParams>,
) -> impl Responder {
    //
    let content = req_body
        .content
        .clone()
        .unwrap_or_else(|| NO_CONTENT_STRING.to_string());

    let grawlix_clone = grawlix_input.grawlix.clone();
    let censor = Censor::Standard;

    if censor.check(&content) {
        let replaced_content = censor.replace(&content, &grawlix_clone);
        return HttpResponse::Ok().body(replaced_content);
    }

    HttpResponse::Ok().body(content)
}
