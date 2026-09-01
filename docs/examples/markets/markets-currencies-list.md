```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.markets().markets_currencies_list(
        "".to_string(),
        Some("".to_string()),
        Some("EUR".to_string()),
        Some(true),
        Some(0),
        Some("2026-01-01T12:00:00Z".to_string()),
        Some(50),
        Some(0),
        Some("position.asc".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
