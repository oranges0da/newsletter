use newsletter::config;
use newsletter::server;
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = config::get_config().expect("Failed to read configuration.");
    let addr = format!("127.0.0.1:{}", config.app_port);

    let db_pool = PgPool::connect(&config.db_settings.get_database_url())
        .await
        .expect("Failed to connect to database.");
    let listener = TcpListener::bind(addr).expect("Failed to bind to address port.");

    println!("Running on port: {}", config.app_port);

    server::run(listener, db_pool)?.await
}
