```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.inventories_stock().inventories_stock_create(
        "".to_string(),
        Some(serde_json::json!({"backorder":true})),
        Some("".to_string()),
        Some(10.0),
        Some("ACME-4711-BLK".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
