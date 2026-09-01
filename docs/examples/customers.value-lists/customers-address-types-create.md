```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.customers_value_lists().customers_address_types_create(
        "".to_string(),
        "Shipping address".to_string(),
        Some("Where the goods go.".to_string()),
        Some(serde_json::json!({"de":"Wohin die Ware geliefert wird.","en":"Where the goods go."})),
        Some(true),
        Some(serde_json::json!({"de":"Lieferadresse","en":"Shipping address"})),
        Some(1),
        Some("neutral".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
