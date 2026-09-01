```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.messaging().binding_store(
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        Some(true),
        Some(1),
        Some("".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
