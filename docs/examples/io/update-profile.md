```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.io().update_profile(
        "".to_string(),
        "".to_string(),
        "import".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        Some("upsert".to_string()),
        Some(serde_json::json!({})),
        Some(vec![]),
        Some(serde_json::json!({})),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
