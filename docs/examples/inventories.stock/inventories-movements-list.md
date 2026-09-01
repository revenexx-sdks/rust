```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.inventories_stock().inventories_movements_list(
        Some(50),
        Some(0),
        Some("created_at.desc".to_string()),
        Some("".to_string()),
        Some("".to_string()),
        Some("".to_string()),
        Some("ACME-4711-BLK".to_string()),
        Some("inbound".to_string()),
        Some(5.0),
        Some("SO-2026-000123".to_string()),
        Some("Delivery note 4711".to_string()),
        Some("{}".to_string()),
        Some("2026-01-01T12:00:00Z".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
