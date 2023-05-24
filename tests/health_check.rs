use newsletter::helpers;

#[tokio::test]
async fn health_check_works() {
    let addr = helpers::spawn_app().await; // get full local address of spawned app

    // instatiate a client and send a request to the given address
    let client = reqwest::Client::new();

    let res = client
        .get(format!("{}/health_check", &addr))
        .send()
        .await
        .expect("Failed to send request.");

    assert!(res.status().is_success());
}
