```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.orders().orders_ship(
        "".to_string(),
        Some("DHL".to_string()),
        Some(serde_json::json!({"warehouse":"HAM-1"})),
        Some("DEL-000123".to_string()),
        Some(vec![]),
        Some("2026-01-01T12:00:00Z".to_string()),
        Some("00340434161234567890".to_string()),
        Some("https://example.com/track/00340434161234567890".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
