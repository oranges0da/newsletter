use newsletter::config;
use newsletter::server::run;
use std::net::TcpListener;

// spawn instance of app for testing
fn spawn_app() -> String {
    let config = config::get_config().expect("Failed to read config from yaml.");

    let listener = TcpListener::bind(format!("127.0.0.1:{}", config.app_port))
        .expect("Failed to bind random port");

    let server = run(listener).expect("Failed to bind address");

    // start the server as a background task using tokio::spawn
    // tokio::spawn will immediately return without waiting for task to complete
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", config.app_port)
}

#[actix_web::test]
async fn health_check_works() {
    let addr = spawn_app(); // get full local address of spawned app

    // instatiate a client and send a request to the given address
    let client = reqwest::Client::new();

    let res = client
        .get(format!("{}/health_check", addr))
        .send()
        .await
        .expect("Failed to send request.");

    assert!(res.status().is_success());
}
