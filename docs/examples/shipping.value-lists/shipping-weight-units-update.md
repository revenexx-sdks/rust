```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.shipping_value_lists().shipping_weight_units_update(
        "".to_string(),
        Some("When to pick this weight unit.".to_string()),
        Some(serde_json::json!({"de":"Wann diese Option zu wählen ist.","en":"When to pick this weight unit."})),
        Some(1000.0),
        Some(true),
        Some(serde_json::json!({"de":"Tonne","en":"Tonne"})),
        Some(1),
        Some("Tonne".to_string()),
        Some("neutral".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
