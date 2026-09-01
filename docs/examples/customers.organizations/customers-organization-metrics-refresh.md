```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.customers_organizations().customers_organization_metrics_refresh(
        Some("2026-01-01T12:00:00Z".to_string()),
        Some("".to_string()),
        Some(vec![]),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
