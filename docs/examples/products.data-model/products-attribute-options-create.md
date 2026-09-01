```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.products_data_model().products_attribute_options_create(
        "".to_string(),
        "stainless_steel".to_string(),
        Some(serde_json::json!({"de":"Edelstahl","en":"Stainless steel"})),
        Some(1),
        Some(serde_json::json!({"hex":"#c0c0c0"})),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
