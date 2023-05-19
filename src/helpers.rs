use crate::{config, server};
use std::net::TcpListener;

// spawn instance of app for testing
pub fn spawn_app() -> String {
    let config = config::get_config().expect("Failed to read config from yaml.");

    let listener = TcpListener::bind(format!("127.0.0.1:{}", config.app_port))
        .expect("Failed to bind random port");

    let server = server::run(listener).expect("Failed to bind address");

    // start the server as a background task using tokio::spawn
    // tokio::spawn will immediately return without waiting for task to complete
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", config.app_port)
}
