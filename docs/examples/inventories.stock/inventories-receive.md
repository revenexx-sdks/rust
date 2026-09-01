```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.inventories_stock().inventories_receive(
        Some(vec![]),
        Some("main".to_string()),
        Some("".to_string()),
        Some(12.0),
        Some("Delivery note 4711".to_string()),
        Some("ACME-4711-BLK".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
