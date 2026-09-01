```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.payments_providers().payments_providers_create(
        "stripe".to_string(),
        Some(serde_json::json!({})),
        Some(true),
        Some("Stripe".to_string()),
        Some(serde_json::json!({"capture_method":"automatic","logo_url":"https://apps.example.com/payments/logos/stripe","three_ds":false})),
        Some(true),
        Some("".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
