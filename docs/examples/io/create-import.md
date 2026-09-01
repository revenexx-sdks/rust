```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.io().create_import(
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        Some("csv".to_string()),
        Some(vec![]),
        Some(1),
        Some("upsert".to_string()),
        Some("".to_string()),
        Some("live".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
