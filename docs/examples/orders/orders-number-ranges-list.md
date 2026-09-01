```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.orders().orders_number_ranges_list(
        Some("".to_string()),
        Some("order".to_string()),
        Some("ORD-".to_string()),
        Some("".to_string()),
        Some(6),
        Some(123),
        Some(1),
        Some(10),
        Some("".to_string()),
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
