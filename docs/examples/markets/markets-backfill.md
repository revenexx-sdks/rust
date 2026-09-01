```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.markets().markets_backfill(
        "northwind".to_string(),
        "northwind".to_string(),
        Some(true),
        Some(true),
        Some(true),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
