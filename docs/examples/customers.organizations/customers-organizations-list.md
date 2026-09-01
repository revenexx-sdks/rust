```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.customers_organizations().customers_organizations_list(
        Some("".to_string()),
        Some("Beispiel Industrietechnik GmbH".to_string()),
        Some("DE123456789".to_string()),
        Some("Maschinenbau".to_string()),
        Some("K-10042".to_string()),
        Some("active".to_string()),
        Some("customer".to_string()),
        Some("net_30".to_string()),
        Some(9.99),
        Some("standard".to_string()),
        Some(true),
        Some("".to_string()),
        Some("2026-01-01T12:00:00Z".to_string()),
        Some("2026-01-01T12:00:00Z".to_string()),
        Some(1),
        Some(1),
        Some("created_at.desc".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
