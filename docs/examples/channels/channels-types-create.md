```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.channels().channels_types_create(
        "feed".to_string(),
        "Product feed".to_string(),
        Some("A web shop a human browses.".to_string()),
        Some(serde_json::json!({"de":"Shop","en":"Shop"})),
        Some(true),
        Some(serde_json::json!({"de":"Shop","en":"Shop"})),
        Some(1),
        Some("neutral".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
