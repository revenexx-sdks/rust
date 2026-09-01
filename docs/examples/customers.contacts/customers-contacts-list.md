```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.customers_contacts().customers_contacts_list(
        Some("".to_string()),
        Some("".to_string()),
        Some("einkauf@example.com".to_string()),
        Some("Anna".to_string()),
        Some("Berger".to_string()),
        Some("+49 30 5550123".to_string()),
        Some("Einkaufsleitung".to_string()),
        Some("buyer".to_string()),
        Some("invited".to_string()),
        Some(9.99),
        Some("pending".to_string()),
        Some("2026-01-01T12:00:00Z".to_string()),
        Some("vertrieb@example.com".to_string()),
        Some("Could not be verified as a commercial buyer.".to_string()),
        Some("de-DE".to_string()),
        Some(true),
        Some("".to_string()),
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
