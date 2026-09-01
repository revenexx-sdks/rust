```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.orders().orders_cancel(
        "".to_string(),
        Some("service-desk".to_string()),
        Some("Customer withdrew the order".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
