use actix_web::{dev::Server, web, App, HttpResponse, HttpServer};
use std::net::TcpListener;

// main entry point for main to start instance of server
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .listen(listener)?
        .run();

    Ok(server)
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().body("Health check working!")
}
