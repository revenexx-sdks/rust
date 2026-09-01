```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.customers_segments().customers_segments_create(
        "key_accounts".to_string(),
        Some(serde_json::json!({"de":"Großkunden","en":"Key accounts"})),
        Some(1),
        Some("all".to_string()),
        Some(serde_json::json!({})),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
