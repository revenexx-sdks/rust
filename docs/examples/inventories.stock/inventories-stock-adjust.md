```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.inventories_stock().inventories_stock_adjust(
        "".to_string(),
        -3.0,
        Some("Stocktake 2026-03, two units damaged".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
