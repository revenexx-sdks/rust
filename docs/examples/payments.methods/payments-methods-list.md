```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.payments_methods().payments_methods_list(
        Some(1),
        Some(1),
        Some("created_at.desc".to_string()),
        Some("invoice".to_string()),
        Some("self_managed".to_string()),
        Some(true),
        Some("stripe".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
