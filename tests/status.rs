use reqwest::Client;

#[tokio::test]
async fn status() {
    assert!(api_is_running().await);
}

async fn api_is_running() -> bool {
    let client = Client::new();
    client
        .get("http://localhost:8080/status")
        .send()
        .await
        .is_ok()
}
