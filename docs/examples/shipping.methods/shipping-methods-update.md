```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.shipping_methods().shipping_methods_update(
        "".to_string(),
        Some("acme-parcel".to_string()),
        Some("8a4d1c7e-2b93-4f61-b0d2-6c5a9e3f1a44".to_string()),
        Some("express".to_string()),
        Some(vec!["DE","AT","CH"]),
        Some("EUR".to_string()),
        Some("Delivered by the next working day when ordered before the cut-off.".to_string()),
        Some(true),
        Some(1),
        Some(1),
        Some(100.0),
        Some(serde_json::json!({"de":"Expressversand","en":"Express delivery"})),
        Some("volume_litres".to_string()),
        Some("weight".to_string()),
        Some(serde_json::json!({"erp_key":"SHIP-EXPRESS","printer":"label-2"})),
        Some("Express delivery".to_string()),
        Some(1),
        Some(9.9),
        Some("fixed".to_string()),
        Some(31.5),
        Some("reduced".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
