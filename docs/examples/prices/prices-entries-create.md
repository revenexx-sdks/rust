```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.prices().prices_entries_create(
        "".to_string(),
        Some(serde_json::json!({"imported_batch":"2026-02-14","source_system":"erp"})),
        Some("standard".to_string()),
        Some("".to_string()),
        Some(9.99),
        Some("BOLT-M8-30".to_string()),
        Some("pcs".to_string()),
        Some(9.99),
        Some("2026-03-01T00:00:00Z".to_string()),
        Some("2026-03-31T23:59:59Z".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
