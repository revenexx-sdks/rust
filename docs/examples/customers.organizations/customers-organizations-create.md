```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.customers_organizations().customers_organizations_create(
        "Beispiel Industrietechnik GmbH".to_string(),
        Some("Maschinenbau".to_string()),
        Some(5000.0),
        Some("K-10042".to_string()),
        Some(true),
        Some("customer".to_string()),
        Some("net_30".to_string()),
        Some("standard".to_string()),
        Some(serde_json::json!({"account_manager":"sales-north","delivery_tour":"tuesday","self_pickup":true})),
        Some("active".to_string()),
        Some("DE123456789".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
