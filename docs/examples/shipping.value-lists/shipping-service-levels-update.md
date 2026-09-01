```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.shipping_value_lists().shipping_service_levels_update(
        "".to_string(),
        Some("When to pick this service level.".to_string()),
        Some(serde_json::json!({"de":"Wann diese Option zu wählen ist.","en":"When to pick this service level."})),
        Some(true),
        Some(serde_json::json!({"de":"Night courier","en":"Night courier"})),
        Some(1),
        Some("Night courier".to_string()),
        Some("neutral".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
