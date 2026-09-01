```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.avatars().avatars_get_initials(
        Some("Ada Lovelace".to_string()),
        Some(1),
        Some(1),
        Some("1a73e8".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
