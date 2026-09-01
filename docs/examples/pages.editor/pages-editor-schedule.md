```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.pages_editor().pages_editor_schedule(
        "".to_string(),
        "2026-01-01T12:00:00Z".to_string(),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
