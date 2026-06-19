```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.messaging().messaging_list_subscriber_logs(
        "".to_string(),
        Some(vec![]),
        Some(false),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
