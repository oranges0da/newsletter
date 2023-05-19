use newsletter::config;
use newsletter::server;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = config::get_config().expect("Failed to read configuration.");
    let addr = format!("127.0.0.1:{}", config.app_port);

    let listener = TcpListener::bind(addr).expect("Failed to bind to address port.");

    println!("Running on port: {}", config.app_port);

    server::run(listener)?.await
}
