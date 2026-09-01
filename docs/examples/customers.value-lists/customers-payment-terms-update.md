```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.customers_value_lists().customers_payment_terms_update(
        "".to_string(),
        Some("Invoice due 30 days after the delivery note.".to_string()),
        Some(serde_json::json!({"de":"Rechnung 30 Tage nach Lieferschein fällig.","en":"Invoice due 30 days after the delivery note."})),
        Some(true),
        Some(serde_json::json!({"de":"Zahlbar in 30 Tagen","en":"Net 30 days"})),
        Some(1),
        Some("Net 30 days".to_string()),
        Some("neutral".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
