```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.shipping_methods().shipping_rates(
        Some("2026-01-01T12:00:00Z".to_string()),
        Some(serde_json::json!({"volume_litres":48})),
        Some("DE".to_string()),
        Some("EUR".to_string()),
        Some("3f2b6d10-7c41-4c0a-9a35-2f5b8e0d9c11".to_string()),
        Some(129.9),
        Some(129.9),
        Some(109.16),
        Some(3.0),
        Some(12.5),
        Some("kg".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
