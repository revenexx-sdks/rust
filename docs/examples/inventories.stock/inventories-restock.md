```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.inventories_stock().inventories_restock(
        Some(vec![]),
        Some("main".to_string()),
        Some("SO-2026-000123".to_string()),
        Some("".to_string()),
        Some(1.0),
        Some("Return: wrong size".to_string()),
        Some(true),
        Some("ACME-4711-BLK".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
