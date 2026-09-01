```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.products_data_model().products_attributes_update(
        "".to_string(),
        Some("net_weight".to_string()),
        Some(serde_json::json!({"reference_entity":"brand"})),
        Some("brand".to_string()),
        Some("product".to_string()),
        Some("".to_string()),
        Some(true),
        Some(true),
        Some(serde_json::json!({"de":"Nettogewicht","en":"Net weight"})),
        Some(true),
        Some(1),
        Some(true),
        Some("select".to_string()),
        Some(true),
        Some(serde_json::json!({"max_length":64,"min_length":3})),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
