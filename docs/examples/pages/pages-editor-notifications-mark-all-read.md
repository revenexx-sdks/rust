```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.pages().pages_editor_notifications_mark_all_read(
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
