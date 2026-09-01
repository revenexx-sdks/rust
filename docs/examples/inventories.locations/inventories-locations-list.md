```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.inventories_locations().inventories_locations_list(
        Some(50),
        Some(0),
        Some("created_at.desc".to_string()),
        Some("".to_string()),
        Some("main".to_string()),
        Some("Main warehouse".to_string()),
        Some("{}".to_string()),
        Some("warehouse".to_string()),
        Some(0),
        Some(true),
        Some("{}".to_string()),
        Some("{}".to_string()),
        Some("2026-01-01T12:00:00Z".to_string()),
        Some("2026-01-01T12:00:00Z".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
