```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.markets().markets_create(
        "northwind".to_string(),
        "Northwind".to_string(),
        Some("EUR".to_string()),
        Some(false),
        Some(serde_json::json!({"de-DE":"Nordwind","en-GB":"Northwind"})),
        Some(0),
        Some("active".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
