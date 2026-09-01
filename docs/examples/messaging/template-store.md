```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.messaging().template_store(
        "".to_string(),
        "".to_string(),
        Some("".to_string()),
        Some("".to_string()),
        Some("".to_string()),
        Some(vec![]),
        Some(true),
        Some("".to_string()),
        Some("".to_string()),
        Some(vec![]),
        Some("transactional".to_string()),
        Some("".to_string()),
        Some(true),
        Some("".to_string()),
        Some("2026-01-01T12:00:00Z".to_string()),
        Some("2026-01-01T12:00:00Z".to_string()),
        Some(vec![]),
        Some(vec![]),
        Some("marketing".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
