```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.inventories_reservations().inventories_reserve(
        "SO-2026-000123".to_string(),
        Some("2026-01-01T12:00:00Z".to_string()),
        Some(vec![]),
        Some("main".to_string()),
        Some("".to_string()),
        Some(2.0),
        Some(serde_json::json!({})),
        Some("ACME-4711-BLK".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
