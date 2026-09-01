```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.customers_contacts().customers_contacts_events_create(
        "".to_string(),
        "Called about the annual requirement".to_string(),
        Some("vertrieb@example.com".to_string()),
        Some("note".to_string()),
        Some("Asked for a quote on the annual bolt requirement; call back in week 34.".to_string()),
        Some("2026-01-01T12:00:00Z".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
