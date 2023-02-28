use actix_web::{web, HttpResponse, Responder, Result};
use serde::Serialize;

#[derive(Serialize)]
struct Channel {
    id: String,
    name: String,
}

pub async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("application/json")
        .body(r#"{"message":"Hello world again!"}"#)
}

pub async fn detail(id: web::Path<String>) -> Result<impl Responder> {
    let channel = Channel {
        id: id.to_string(),
        name: "test".to_string(),
    };

    Ok(web::Json(channel))
}
