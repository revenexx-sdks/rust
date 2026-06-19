```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.storage().asset_store(
        "".to_string(),
        Some("".to_string()),
        Some("".to_string()),
        Some("".to_string()),
        Some("".to_string()),
        Some(false),
        Some(vec![]),
        Some(false),
        Some("".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
