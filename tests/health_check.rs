use newsletter::helpers;
use reqwest;

#[tokio::test]
async fn health_check_works() {
    // spawn test app and reqwest client for sending requests
    let app = helpers::spawn_app().await;
    let client = reqwest::Client::new();

    // format address with http:// (you'd be surprised how many errors this can cause!)
    let address = format!("http://{}/health_check", app.address);

    let res = client
        .get(address)
        .send()
        .await
        .expect("Failed to send request.");

    assert!(res.status().is_success());
}
