```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.messaging().stats_index(
        Some(1),
        Some("2026-01-01".to_string()),
        Some("2026-01-01".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
