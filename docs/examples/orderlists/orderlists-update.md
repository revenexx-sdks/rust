```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.orderlists().orderlists_update(
        "".to_string(),
        Some("shopping".to_string()),
        Some(serde_json::json!({"department":"facility","erp_reference":"REQ-2026-0042"})),
        Some("Weekly office supplies".to_string()),
        Some(true),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
