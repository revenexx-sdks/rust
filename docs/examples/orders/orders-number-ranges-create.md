```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.orders().orders_number_ranges_create(
        "order".to_string(),
        Some("".to_string()),
        Some(123),
        Some(serde_json::json!({"owner":"erp-sync"})),
        Some(6),
        Some(10),
        Some("ORD-".to_string()),
        Some(1),
        Some("".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
