```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.shipping_carriers().shipping_carriers_list(
        Some(1),
        Some(1),
        Some("position.asc".to_string()),
        Some("acme-parcel".to_string()),
        Some("active".to_string()),
        Some("express".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
