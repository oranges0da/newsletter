use newsletter::helpers;

#[tokio::test]
async fn sub_returns_200_if_valid_form() {
    let app = helpers::spawn_app().await;
    let client = reqwest::Client::new();

    // send valid form data and expect 200 OK response
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/sub", app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
}
