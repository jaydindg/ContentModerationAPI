use actix_web::{post, web, HttpResponse, Responder};

use censor::*;
use serde::Deserialize;

const NO_CONTENT_STRING: &str = "No content sent to API.";

/// Represents the body of an incoming request.
///
/// This struct is used to deserialize the JSON payload of an incoming request.
/// It contains the following fields:
///
/// * `content`: An optional string containing the general content to be reviewed.
/// * `extra_filters`: An optional vector of strings specifying extra words that the user wants to be censored.
/// * `excludes`: An optional vector of strings specifying words that the user does NOT want to be censored.
#[derive(Deserialize)]
struct IncomingReqBody {
    /// General content being sent over to be reviewed.
    content: Option<String>,

    /// Extra words the user wants to be censored.
    extra_filters: Option<Vec<String>>,

    /// Words the user does NOT want to be censored.
    excludes: Option<Vec<String>>,
}

/// Represents the incoming request query param containing the gawlix for replacing text.
///
/// # Fields
///
/// * `grawlix` - A query param `String` that holds the grawlix content.
#[derive(Deserialize)]
struct GrawlixQueryParams {
    grawlix: String,
}

/// Handles POST requests to the `/api/v1/check-text` endpoint.
///
/// This asynchronous function processes incoming JSON requests containing text content to be reviewed for profanity.
/// It uses the `IncomingReqBody` struct to deserialize the request body and applies additional filters and exclusions
/// specified by the user. The function returns a JSON response indicating whether the content contains profanity.
///
/// # Arguments
///
/// * `req_body`: A JSON payload containing the text content and optional filters and exclusions.
///
/// # Returns
///
/// A JSON response with a boolean value:
/// * `true` if the content contains profanity.
/// * `false` if the content does not contain profanity.
#[post("/api/v1/check-text")]
pub async fn check_text(req_body: web::Json<IncomingReqBody>) -> impl Responder {
    // Returns Bool
    let content = req_body
        .content
        .clone()
        .unwrap_or_else(|| NO_CONTENT_STRING.to_string());

    let extra_filters = req_body.extra_filters.clone().unwrap_or_else(|| vec![]);
    let excludes = req_body.excludes.clone().unwrap_or_else(|| vec![]);

    let mut censor = Censor::Standard;

    for filter in extra_filters {
        // Include extra words to be censored.
        censor += filter.as_str();
    }

    for exclude in excludes {
        // Remove extra words to be censored.
        censor -= exclude.as_str();
    }

    if censor.check(&content) {
        return HttpResponse::Ok().json(true); // content has profanity, return true
    }

    HttpResponse::Ok().json(false) // content does not have profanity, return false
}

/// Handles POST requests to the `/api/v1/censor-text` endpoint.
///
/// This asynchronous function processes the incoming JSON payload, which is deserialized into an `IncomingReqBody` struct.
/// It censors the content based on the provided extra filters and excludes, and returns the censored content if any profanity is detected.
///
/// # Arguments
///
/// * `req_body`: The JSON payload of the incoming request, deserialized into an `IncomingReqBody` struct.
///
/// # Returns
///
/// An `HttpResponse` containing the censored content if profanity is detected, or the original content if no profanity is found.
#[post("/api/v1/censor-text")]
pub async fn censor_text(req_body: web::Json<IncomingReqBody>) -> impl Responder {
    // Returns String
    let content = req_body
        .content
        .clone()
        .unwrap_or_else(|| NO_CONTENT_STRING.to_string());

    let extra_filters = req_body.extra_filters.clone().unwrap_or_else(|| vec![]);
    let excludes = req_body.excludes.clone().unwrap_or_else(|| vec![]);

    let mut censor = Censor::Standard;

    for filter in extra_filters {
        // Include extra words to be censored.
        censor += filter.as_str();
    }

    for exclude in excludes {
        // Remove words to be censored.
        censor -= exclude.as_str();
    }

    if censor.check(&content) {
        let censored_content = censor.censor(&content);
        return HttpResponse::Ok().body(censored_content); // content has profanity, return censored
    }

    HttpResponse::Ok().body(content) // content does not have profanity, return original
}

/// Handles POST requests to the `/api/v1/replace-text` endpoint.
///
/// This asynchronous function processes the incoming JSON payload, which is deserialized into an `IncomingReqBody` struct,
/// and a query parameter for grawlix input. It replaces the censored words in the content with the provided grawlix string.
///
/// # Arguments
///
/// * `req_body`: The JSON payload of the incoming request, deserialized into an `IncomingReqBody` struct.
/// * `grawlix_input`: The query parameter containing the grawlix string to replace censored words.
///
/// # Returns
///
/// An `HttpResponse` containing the content with censored words replaced by the grawlix string if profanity is detected,
/// or the original content if no profanity is found.
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
    let extra_filters = req_body.extra_filters.clone().unwrap_or_else(|| vec![]);
    let excludes = req_body.excludes.clone().unwrap_or_else(|| vec![]);

    let mut censor = Censor::Standard;

    for filter in extra_filters {
        // Include extra words to be censored.
        censor += filter.as_str();
    }

    for exclude in excludes {
        // Remove words to be censored.
        censor -= exclude.as_str();
    }

    if censor.check(&content) {
        let replaced_content = censor.replace(&content, &grawlix_clone);
        return HttpResponse::Ok().body(replaced_content);
    }

    HttpResponse::Ok().body(content)
}
