```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.orders().orders_payment_status_update(
        "".to_string(),
        "open".to_string(),
        Some("pay_000000000001".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
