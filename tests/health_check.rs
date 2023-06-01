use newsletter::helpers;
use reqwest;

#[tokio::test]
async fn health_check_works() {
    // spawn test app and reqwest client for sending requests
    let app = helpers::spawn_app().await;
    let client = reqwest::Client::new();

    // format address with http:// (you'd be surprised how many errors this can cause!)
    let url = format!("http://{}/health_check", app.address);

    let res = client
        .get(url)
        .send()
        .await
        .expect("Failed to send request.");

    assert!(res.status().is_success());
}
