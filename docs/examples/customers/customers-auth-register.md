```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.customers().customers_auth_register(
        "einkauf@example.com".to_string(),
        "".to_string(),
        Some("Anna".to_string()),
        Some("Berger".to_string()),
        Some("de-DE".to_string()),
        Some("".to_string()),
        Some("Beispiel Industrietechnik GmbH".to_string()),
        Some("https://shop.example.com/account".to_string()),
        Some("DE123456789".to_string()),
        Some("https://shop.example.com/bestaetigen".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
