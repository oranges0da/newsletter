use newsletter::helpers;
use reqwest;

#[tokio::test]
async fn sub_returns_200_if_valid_form() {
    let app = helpers::spawn_app().await;
    let client = reqwest::Client::new();

    // format address with http:// for readability and reasons
    let address = format!("http://{}/sub", app.address);

    // send valid form data and expect 200 OK response
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let res = client
        .post(address)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, res.status().as_u16());
}
