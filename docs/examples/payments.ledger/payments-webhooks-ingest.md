```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.payments_ledger().payments_webhooks_ingest(
        "stripe".to_string(),
        Some(serde_json::Value::Null),
        Some(serde_json::json!({})),
        Some(serde_json::Value::Null),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
