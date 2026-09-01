```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.carts().carts_list(
        Some("".to_string()),
        Some("Weekly order".to_string()),
        Some("active".to_string()),
        Some("".to_string()),
        Some("a1b2c3d4e5f6".to_string()),
        Some("".to_string()),
        Some("EUR".to_string()),
        Some(true),
        Some(100),
        Some(12.0),
        Some("2026-01-01T12:00:00Z".to_string()),
        Some("2026-01-01T12:00:00Z".to_string()),
        Some("SO-10042".to_string()),
        Some("".to_string()),
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
