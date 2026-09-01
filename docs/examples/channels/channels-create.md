```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.channels().channels_create(
        "shop".to_string(),
        "Shop".to_string(),
        Some(true),
        Some(serde_json::json!({"de":"Shop","en":"Shop"})),
        Some(1),
        Some("active".to_string()),
        Some("storefront".to_string()),
        Some("inherit".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
