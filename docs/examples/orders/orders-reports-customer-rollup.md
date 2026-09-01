```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.orders().orders_reports_customer_rollup(
        Some("2026-01-01T12:00:00Z".to_string()),
        Some("".to_string()),
        Some(vec![]),
        Some(vec![]),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
