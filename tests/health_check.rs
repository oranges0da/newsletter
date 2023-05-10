use newsletter::server::run;
use std::net::TcpListener;

// spawn instance of app for testing
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port(); // retrieve assigned port

    let server = run(listener).expect("Failed to bind address");

    // start the server as a background task using tokio::spawn
    // tokio::spawn will immediately return without waiting for task to complete
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}

#[actix_web::test]
async fn health_check_works() {
    let address = spawn_app(); // spawn instance of server

    // instatiate a client and send a request to the given address
    let client = reqwest::Client::new();

    let res = client
        .get(format!("{}/health_check", address))
        .send()
        .await
        .expect("Failed to send request.");

    assert!(res.status().is_success());
}
