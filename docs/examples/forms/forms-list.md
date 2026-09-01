```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.forms().forms_list(
        Some("".to_string()),
        Some("Contact request".to_string()),
        Some("contact".to_string()),
        Some("draft".to_string()),
        Some("2026-01-31T09:15:00Z".to_string()),
        Some("2026-01-31T09:15:00Z".to_string()),
        Some(50),
        Some(0),
        Some("created_at.desc".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
