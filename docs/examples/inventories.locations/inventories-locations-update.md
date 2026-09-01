```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.inventories_locations().inventories_locations_update(
        "".to_string(),
        Some(serde_json::json!({"city":"Nuremberg","country":"DE","postal_code":"90402","street":"Industriering 4"})),
        Some("main".to_string()),
        Some(true),
        Some(serde_json::json!({"de":"Hauptlager","en":"Main warehouse"})),
        Some(serde_json::json!({"erp_site":"1000"})),
        Some("Main warehouse".to_string()),
        Some(0),
        Some("warehouse".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
