```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.markets().markets_clone(
        "northwind".to_string(),
        "northwind-b2b".to_string(),
        Some(true),
        Some(true),
        Some(true),
        Some("EUR".to_string()),
        Some("Northwind B2B".to_string()),
        Some("active".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
