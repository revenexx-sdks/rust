```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.payments_methods().payments_methods_eligible(
        Some(49.9),
        Some("DE".to_string()),
        Some("EUR".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
