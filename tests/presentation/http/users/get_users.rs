use reqwest::Client;
use serde::{Deserialize, Serialize};

#[cfg(test)]
#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: i32,
    name: String,
    email: String,
}

#[tokio::test]
async fn test_get_users() {
    let client = Client::new();

    let users = client
        .get("http://localhost:8080/users")
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(users.status(), 200);

    let users: Vec<User> = users.json().await.expect("Failed to parse response body.");

    assert!(!users.is_empty());
}
