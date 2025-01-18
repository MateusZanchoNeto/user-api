use reqwest::Client;
use serde::{Deserialize, Serialize};
#[cfg(test)]
#[derive(Debug, Serialize, Deserialize)]
struct Request {
    name: String,
    email: String,
}

#[cfg(test)]
#[derive(Debug, Serialize, Deserialize)]
struct Response {
    id: i32,
    name: String,
    email: String,
}

#[tokio::test]
async fn test_create_user() {
    let client = Client::new();

    let user = Request {
        name: "John Doe".to_string(),
        email: "john@email.com".to_string(),
    };

    let response = client
        .post("http://localhost:8080/user")
        .header("Content-Type", "application/json")
        .json(&user)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), 201);

    let response_body: Response = response
        .json()
        .await
        .expect("Failed to parse response body.");

    assert_eq!(response_body.name, "John Doe");
    assert_eq!(response_body.email, "john@email.com");
}
