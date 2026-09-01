```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.products_data_model().products_families_update(
        "".to_string(),
        Some("power_tools".to_string()),
        Some("main_image".to_string()),
        Some("name".to_string()),
        Some(serde_json::json!({"de":"Elektrowerkzeuge","en":"Power tools"})),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
