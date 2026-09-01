```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.markets().markets_tax_classes_update(
        "".to_string(),
        "".to_string(),
        Some("standard".to_string()),
        Some(true),
        Some(serde_json::json!({"de-DE":"Regelsatz","en-GB":"Standard rate"})),
        Some("Standard rate".to_string()),
        Some(0),
        Some(20.0),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
