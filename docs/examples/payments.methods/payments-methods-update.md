```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.payments_methods().payments_methods_update(
        "".to_string(),
        Some("invoice".to_string()),
        Some(vec!["DE","AT"]),
        Some("Pay within 14 days of the invoice date.".to_string()),
        Some(true),
        Some(2.5),
        Some("EUR".to_string()),
        Some("none".to_string()),
        Some("self_managed".to_string()),
        Some(serde_json::json!({"de":"Rechnung","en":"Invoice"})),
        Some(2500.0),
        Some(serde_json::json!({"erp_payment_key":"ZTRM01"})),
        Some(10.0),
        Some("Invoice".to_string()),
        Some(0),
        Some("stripe".to_string()),
        Some("card".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
