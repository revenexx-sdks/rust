```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.orderlists().orderlists_list(
        Some("".to_string()),
        Some("".to_string()),
        Some("shopping".to_string()),
        Some(50),
        Some(0),
        Some("created_at.desc".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
