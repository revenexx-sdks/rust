```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.orders().orders_items_cancel(
        "".to_string(),
        vec![],
        Some("service-desk".to_string()),
        Some("Out of stock, customer agreed".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
