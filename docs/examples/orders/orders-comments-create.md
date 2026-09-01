```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.orders().orders_comments_create(
        "".to_string(),
        "Called the customer, delivery agreed for next week.".to_string(),
        Some("service-desk".to_string()),
        Some("internal".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
