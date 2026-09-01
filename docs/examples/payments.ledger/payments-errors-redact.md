```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.payments_ledger().payments_errors_redact(
        Some(true),
        Some(500),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
