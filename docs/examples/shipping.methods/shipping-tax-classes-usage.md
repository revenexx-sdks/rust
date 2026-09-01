```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.shipping_methods().shipping_tax_classes_usage(
        "reduced".to_string(),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
