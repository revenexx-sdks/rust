```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.customers_contacts().customers_contact_events_list(
        Some("".to_string()),
        Some("".to_string()),
        Some("".to_string()),
        Some("call".to_string()),
        Some("activity.call".to_string()),
        Some("Called about the annual requirement".to_string()),
        Some("vertrieb@example.com".to_string()),
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
