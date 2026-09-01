```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.storage().sync_rule_store(
        "".to_string(),
        "/uploads".to_string(),
        Some(true),
        Some(vec![]),
        Some("0 3 * * *".to_string()),
        Some("".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
