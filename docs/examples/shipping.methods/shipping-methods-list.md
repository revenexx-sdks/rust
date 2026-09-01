```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.shipping_methods().shipping_methods_list(
        Some(1),
        Some(1),
        Some("position.asc".to_string()),
        Some("express".to_string()),
        Some(true),
        Some("matrix".to_string()),
        Some("8a4d1c7e-2b93-4f61-b0d2-6c5a9e3f1a44".to_string()),
        Some("acme-parcel".to_string()),
        Some("reduced".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
