//! Handlers
//! 

use actix_web::{post, web, HttpResponse, Responder};

use crate::payloads::RequestPayload;

/// Sample handler to process a request payload and convert into response payload
/// handling errors returned.
/// Other data can be used here but must be passed into App in main.rs
#[post("/microservice/path")]
pub async fn handle_post_request(
    body : web::Json<RequestPayload>,
) -> impl Responder {
    let request = body.into_inner();
    match request.process() {
        Ok(r) => HttpResponse::Ok().json(r),
        Err(e) => HttpResponse::BadRequest().json(e),
    }
    
}