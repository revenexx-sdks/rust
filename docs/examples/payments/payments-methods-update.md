```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.payments().payments_methods_update(
        "".to_string(),
        Some("".to_string()),
        Some(vec![]),
        Some("".to_string()),
        Some(false),
        Some(0.0),
        Some("".to_string()),
        Some("".to_string()),
        Some("".to_string()),
        Some(serde_json::json!({})),
        Some(0.0),
        Some(serde_json::json!({})),
        Some(0.0),
        Some("".to_string()),
        Some(0),
        Some("".to_string()),
        Some("".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
