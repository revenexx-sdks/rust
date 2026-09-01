```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.products_data_model().products_attribute_groups_create(
        "technical_attributes".to_string(),
        Some(serde_json::json!({"de":"Technische Attribute","en":"Technical attributes"})),
        Some(1),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
