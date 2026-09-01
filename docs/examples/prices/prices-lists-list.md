```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.prices().prices_lists_list(
        Some("".to_string()),
        Some("standard".to_string()),
        Some("Standard prices".to_string()),
        Some("The list every buyer falls back to.".to_string()),
        Some("EUR".to_string()),
        Some("active".to_string()),
        Some(1),
        Some(true),
        Some("net".to_string()),
        Some(true),
        Some(true),
        Some("".to_string()),
        Some("".to_string()),
        Some("".to_string()),
        Some("2026-01-01T12:00:00Z".to_string()),
        Some("2026-01-01T12:00:00Z".to_string()),
        Some("2026-01-01T12:00:00Z".to_string()),
        Some("2026-01-01T12:00:00Z".to_string()),
        Some(1),
        Some(1),
        Some("created_at.desc".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
