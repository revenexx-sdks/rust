```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.payments_ledger().payments_create(
        49.9,
        "invoice".to_string(),
        Some("".to_string()),
        Some("".to_string()),
        Some("DE".to_string()),
        Some("EUR".to_string()),
        Some("checkout-2f9c41".to_string()),
        Some(serde_json::json!({"order_source":"web"})),
        Some("ORD-10042".to_string()),
        Some("https://shop.example.com/checkout/return".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
