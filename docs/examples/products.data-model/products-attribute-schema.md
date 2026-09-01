```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.products_data_model().products_attribute_schema(
        Some("".to_string()),
        Some("".to_string()),
        Some("product".to_string()),
        Some("brand".to_string()),
        Some("de_DE".to_string()),
        Some("b2b".to_string()),
        Some("simple".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
