```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.shipping_carriers().shipping_carriers_create(
        "acme-parcel".to_string(),
        "Acme Parcel".to_string(),
        Some(vec!["DE","AT","CH"]),
        Some("16:00".to_string()),
        Some(1),
        Some(1),
        Some(1),
        Some(serde_json::json!({"de":"Acme Paketdienst","en":"Acme Parcel"})),
        Some(serde_json::json!({"contract":"ACME-2026","customer_number":"4711"})),
        Some(1),
        Some("express".to_string()),
        Some("active".to_string()),
        Some("https://track.example.com/parcels/{tracking_code}".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
