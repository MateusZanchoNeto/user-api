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
async fn test_update_user() {
    let client = Client::new();

    let users = client
        .get("http://localhost:8080/users")
        .send()
        .await
        .expect("Failed to execute request.");

    let users = users
        .json::<Vec<User>>()
        .await
        .expect("Failed to parse response body.");

    let user = users[0].clone();

    let response = client
        .get(format!("http://localhost:8080/user/{}", user.id))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), 200);

    let response_user: User = response
        .json()
        .await
        .expect("Failed to parse response body.");

    assert_eq!(response_user.id, user.id);
    assert_eq!(response_user.name, user.name);
    assert_eq!(response_user.email, user.email);
    
    let response = client.put(format!("http://localhost:8080/user/{}", user.id))
        .header("Content-Type", "application/json")
        .json(&User {
            id: user.id,
            name: "Jane Doe".to_string(),
            email: "jane@email.com".to_string(),
        })
        .send()
        .await
        .expect("Failed to execute request.");
    
    assert_eq!(response.status(), 200);
    
    let response_user: User = response
        .json()
        .await
        .expect("Failed to parse response body.");
    
    assert_eq!(response_user.id, user.id);
    assert_eq!(response_user.name, "Jane Doe");
    assert_eq!(response_user.email, "jane@email.com");
}
