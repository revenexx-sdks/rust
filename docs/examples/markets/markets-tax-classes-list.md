```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.markets().markets_tax_classes_list(
        "".to_string(),
        Some("".to_string()),
        Some("standard".to_string()),
        Some("Standard rate".to_string()),
        Some("{"de-DE":"Regelsatz","en-GB":"Standard rate"}".to_string()),
        Some(20.0),
        Some(true),
        Some(0),
        Some("2026-01-01T12:00:00Z".to_string()),
        Some("2026-01-01T12:00:00Z".to_string()),
        Some(50),
        Some(0),
        Some("position.asc".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
