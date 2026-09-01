```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.avatars().avatars_get_screenshot(
        "https://example.com".to_string(),
        Some(serde_json::json!({})),
        Some(1),
        Some(1),
        Some(1.0),
        Some("light".to_string()),
        Some("Mozilla/5.0 (iPhone; CPU iPhone OS 14_0 like Mac OS X) AppleWebKit/605.1.15".to_string()),
        Some(true),
        Some("en-US".to_string()),
        Some("Africa/Abidjan".to_string()),
        Some(9.99),
        Some(9.99),
        Some(9.99),
        Some(true),
        Some(vec![]),
        Some(1),
        Some(1),
        Some(1),
        Some(1),
        Some("jpg".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
