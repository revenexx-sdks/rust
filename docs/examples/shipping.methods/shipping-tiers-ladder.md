```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.shipping_methods().shipping_tiers_ladder(
        "".to_string(),
        4.9,
        5.0,
        30.0,
        Some(0.0),
        Some(true),
        Some(2.0),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
