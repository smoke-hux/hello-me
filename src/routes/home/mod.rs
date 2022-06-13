//! src/routes/home/mod.rs
use actix_web::HttpResponse;

pub async fn home() -> HttpResponse {
    HttpResponse::Ok().finish()
}
pub async fn home() -> HttpResponse {
    HttpResponse::Ok().body(include_str!("home.html"))
}

use actix_web::http::header::ContentType;

pub async fn home() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(include_str!("home.html"))
}