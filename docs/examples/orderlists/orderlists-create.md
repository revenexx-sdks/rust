```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.orderlists().orderlists_create(
        "Weekly office supplies".to_string(),
        "".to_string(),
        "Jamie Rivera".to_string(),
        Some(vec![]),
        Some("shopping".to_string()),
        Some(serde_json::json!({"department":"facility","erp_reference":"REQ-2026-0042"})),
        Some("".to_string()),
        Some(true),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
