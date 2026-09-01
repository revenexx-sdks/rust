```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.forms().forms_submissions_prune(
        Some(true),
        Some("contact".to_string()),
        Some(1),
        Some("new".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
