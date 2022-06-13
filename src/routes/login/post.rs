use actix_web::HttpResponse;

pub async fn login() -> HttpResponse {
    HttpResponse::Ok().finish()
}

// redirect on success
/* 
a redirect response requires two elements 
a redirect status 
a location header , set to the URL  we want to redirect to

all the redirect codes are in the range of the 300
*/
pub async fn login() -> HttpResponse {
    HttpResponse::SeeOther()
        .insert_header((LOCATION, "/"))
        .finish()
}

//processing the data 

/*we can parse it out of the incoming request using actix-web extractor and a struct that implements
serde::Deserialize:*/

use actix_web::web;
use secrecy::Sectret;

#[derive(serde::Deserialize)]
pub struct FormData {
    username: String,
    password: Secret<String>,
}

pub async fn login(_form: web::Form<FormData>) -> HttpResponse {
    
}
