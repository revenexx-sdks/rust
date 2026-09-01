```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.products_data_model().products_association_types_update(
        "".to_string(),
        Some("cross_sell".to_string()),
        Some(true),
        Some(true),
        Some(serde_json::json!({"de":"Querverkauf","en":"Cross-sell"})),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
