use newsletter::helpers;

#[tokio::test]
async fn health_check_works() {
    // spawn test app and reqwest client for sending requests
    let app = helpers::spawn_app().await;
    let client = reqwest::Client::new();

    let res = client
        .get(format!("{}/health_check", app.address))
        .send()
        .await
        .expect("Failed to send request.");

    assert!(res.status().is_success());
}
