```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.carts_io().carts_io_profiles_update(
        "".to_string(),
        Some("insert".to_string()),
        Some("import".to_string()),
        Some("carts".to_string()),
        Some("json".to_string()),
        Some(true),
        Some(serde_json::json!({})),
        Some("cart-export-csv".to_string()),
        Some(serde_json::json!({})),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
