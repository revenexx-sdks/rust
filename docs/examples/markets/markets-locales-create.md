```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.markets().markets_locales_create(
        "".to_string(),
        "de-DE".to_string(),
        "DE".to_string(),
        "de".to_string(),
        Some(true),
        Some(0),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
