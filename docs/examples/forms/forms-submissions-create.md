```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.forms().forms_submissions_create(
        serde_json::json!({"company":"Example GmbH","email":"buyer@example.com","message":"Please quote 200 units of ACME-4711-BLK, delivered to Hamburg."}),
        "".to_string(),
        Some("contact".to_string()),
        Some(serde_json::json!({})),
        Some("/contact".to_string()),
        Some("new".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
