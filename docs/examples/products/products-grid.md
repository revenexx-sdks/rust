```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.products().products_grid(
        Some(1),
        Some(1),
        Some("created_at.desc".to_string()),
        Some("cordless drill".to_string()),
        Some("simple".to_string()),
        Some(true),
        Some("".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
