```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.orders().orders_return(
        "".to_string(),
        Some(serde_json::json!({"rma_portal_case":"C-2026-0917"})),
        Some(vec![]),
        Some("Damaged on arrival".to_string()),
        Some(true),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
