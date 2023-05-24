use crate::routes::{health_check, sub};
use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

// main entry point for main to start instance of app
pub fn run(listener: TcpListener, pool: PgPool) -> Result<Server, std::io::Error> {
    // wrap pgconnection in a web::Data extractor to make it clonable
    let conn_pool = web::Data::new(pool);

    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check::health_check))
            .route("/sub", web::post().to(sub::subscribe))
            .app_data(conn_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
