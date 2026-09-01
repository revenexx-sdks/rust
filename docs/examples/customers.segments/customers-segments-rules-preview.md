```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.customers_segments().customers_segments_rules_preview(
        "".to_string(),
        vec![],
        Some("all".to_string()),
        Some("organizations".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
