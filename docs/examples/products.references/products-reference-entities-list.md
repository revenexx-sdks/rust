```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.products_references().products_reference_entities_list(
        Some(1),
        Some(1),
        Some("created_at.desc".to_string()),
        Some("".to_string()),
        Some("brand".to_string()),
        Some("{}".to_string()),
        Some("reference-entities/brand.svg".to_string()),
        Some("2026-01-01T12:00:00Z".to_string()),
        Some("2026-01-01T12:00:00Z".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
