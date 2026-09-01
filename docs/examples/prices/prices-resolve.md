```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.prices().prices_resolve(
        vec![],
        Some("2026-03-15T09:00:00Z".to_string()),
        Some("".to_string()),
        Some("".to_string()),
        Some("EUR".to_string()),
        Some("".to_string()),
        Some("".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
