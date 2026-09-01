```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.customers_value_lists().customers_lifecycle_stages_create(
        "".to_string(),
        "Customer".to_string(),
        Some("Has ordered at least once and is being served.".to_string()),
        Some(serde_json::json!({"de":"Hat mindestens einmal bestellt und wird betreut.","en":"Has ordered at least once and is being served."})),
        Some(true),
        Some(serde_json::json!({"de":"Kunde","en":"Customer"})),
        Some(1),
        Some("neutral".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
