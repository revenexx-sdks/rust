```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.customers_contacts().customers_contacts_create(
        "einkauf@example.com".to_string(),
        Some("Anna".to_string()),
        Some(true),
        Some("Einkaufsleitung".to_string()),
        Some("Berger".to_string()),
        Some("de-DE".to_string()),
        Some(25000.0),
        Some("".to_string()),
        Some("+49 30 5550123".to_string()),
        Some("pending".to_string()),
        Some("buyer".to_string()),
        Some("invited".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
