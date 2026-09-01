```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.prices().prices_entries_adjust(
        "".to_string(),
        Some(9.99),
        Some(true),
        Some(9.99),
        Some("exact".to_string()),
        Some("BOLT-".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
