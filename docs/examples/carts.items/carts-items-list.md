```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.carts_items().carts_items_list(
        "".to_string(),
        Some("".to_string()),
        Some("product".to_string()),
        Some("".to_string()),
        Some("BOLT-M8-30".to_string()),
        Some("Hex bolt M8".to_string()),
        Some(100.0),
        Some("pcs".to_string()),
        Some(0.12),
        Some("EUR".to_string()),
        Some(19.0),
        Some(12.0),
        Some(0),
        Some("2026-01-01T12:00:00Z".to_string()),
        Some("2026-01-01T12:00:00Z".to_string()),
        Some(1),
        Some(1),
        Some("created_at.desc".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
