use actix_web::HttpResponse;

pub async fn login_form() -> HttpResponse {
    HttpResponse::Ok().finish()
}

use actix_web::http::header::ContentType;

pub async fn login_form() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(include_str!("login.html"))
}
pub async fn login_form() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(include_str!("login.html"))
}
