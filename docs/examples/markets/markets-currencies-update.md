```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.markets().markets_currencies_update(
        "".to_string(),
        "".to_string(),
        Some("".to_string()),
        Some(false),
        Some(0),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
