```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.customers_segments().customers_segment_members_update(
        "".to_string(),
        Some("".to_string()),
        Some("".to_string()),
        Some("manual".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
