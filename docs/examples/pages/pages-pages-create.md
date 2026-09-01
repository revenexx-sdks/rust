```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.pages().pages_pages_create(
        "About us".to_string(),
        Some("standard".to_string()),
        Some(serde_json::json!({})),
        Some(serde_json::json!({})),
        Some("about-us".to_string()),
        Some("de".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
