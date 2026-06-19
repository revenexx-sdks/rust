```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.avatars().avatars_get_screenshot(
        "".to_string(),
        Some(serde_json::json!({})),
        Some(0),
        Some(0),
        Some(0.0),
        Some("".to_string()),
        Some("".to_string()),
        Some(false),
        Some("".to_string()),
        Some("".to_string()),
        Some(0.0),
        Some(0.0),
        Some(0.0),
        Some(false),
        Some(vec![]),
        Some(0),
        Some(0),
        Some(0),
        Some(0),
        Some("".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
