```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.messaging().messaging_update_push(
        "".to_string(),
        Some("".to_string()),
        Some(0),
        Some("".to_string()),
        Some("".to_string()),
        Some(false),
        Some(false),
        Some(serde_json::json!({})),
        Some(false),
        Some("".to_string()),
        Some("".to_string()),
        Some("".to_string()),
        Some("".to_string()),
        Some("".to_string()),
        Some("".to_string()),
        Some(vec![]),
        Some("".to_string()),
        Some(vec![]),
        Some(vec![]),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
