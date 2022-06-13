//! src/startup.rs
use crate::routes::home;
// [...]

fn run(/* */) -> Result</* */> {
    // [...]
    let server = HttpServer::new(move || {
        App::new()
            // [...]
            .route("/", web::get().to(home))
            // [...]
    })
    // [...]
}


fn run(/* */) -> Result<Server, std::io::Error> {
    // [...]
    let server = HttpServer::new(move || {
        App::new()
            // [...]
            .route("/login", web::get().to(login_form))
            // [...]
    })
    // [...]
}

fn run(/* */) -> Result</* */> {
    // [...]
    let server = HttpServer::new(move || {
        App::new()
            // [...]
            .route("/login", web::post().to(login))
            // [...]
    })
    // [...]
}