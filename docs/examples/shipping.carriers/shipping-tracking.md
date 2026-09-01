```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.shipping_carriers().shipping_tracking(
        "acme-parcel".to_string(),
        Some("DE".to_string()),
        Some("12345".to_string()),
        Some("ACME000000001DE".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
