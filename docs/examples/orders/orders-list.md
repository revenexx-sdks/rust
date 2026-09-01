```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.orders().orders_list(
        Some("".to_string()),
        Some("ORD-000123".to_string()),
        Some("PO-2026-0042".to_string()),
        Some("ERP-4711".to_string()),
        Some("2026-01-01T12:00:00Z".to_string()),
        Some("".to_string()),
        Some("".to_string()),
        Some("".to_string()),
        Some("".to_string()),
        Some("EUR".to_string()),
        Some("pending".to_string()),
        Some("open".to_string()),
        Some("unfulfilled".to_string()),
        Some(true),
        Some("Credit check pending".to_string()),
        Some(3),
        Some(149.7),
        Some(5.9),
        Some(29.56),
        Some(185.16),
        Some("2026-01-01T12:00:00Z".to_string()),
        Some("2026-01-01T12:00:00Z".to_string()),
        Some("2026-01-01T12:00:00Z".to_string()),
        Some("2026-01-01T12:00:00Z".to_string()),
        Some("2026-01-01T12:00:00Z".to_string()),
        Some(50),
        Some(0),
        Some("created_at.desc".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
