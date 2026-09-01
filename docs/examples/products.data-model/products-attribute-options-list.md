```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.products_data_model().products_attribute_options_list(
        Some(1),
        Some(1),
        Some("created_at.desc".to_string()),
        Some("".to_string()),
        Some("".to_string()),
        Some("stainless_steel".to_string()),
        Some(1),
        Some("{}".to_string()),
        Some("{}".to_string()),
        Some("2026-01-01T12:00:00Z".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
