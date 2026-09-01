```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.shipping_methods().shipping_tiers_list(
        "".to_string(),
        Some(1),
        Some(1),
        Some("position.asc".to_string()),
        Some(10.0),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
