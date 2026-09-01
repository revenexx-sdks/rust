```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.messaging().suppression_index(
        Some("".to_string()),
        Some("all".to_string()),
        Some("hard_bounce".to_string()),
        Some("".to_string()),
        Some(1),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
