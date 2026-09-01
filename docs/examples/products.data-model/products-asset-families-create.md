```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.products_data_model().products_asset_families_create(
        "packshots".to_string(),
        Some(serde_json::json!({"de":"Packshots","en":"Packshots"})),
        Some(serde_json::json!({"allowed_extensions":["jpg","png"],"pattern":"{sku}_{index}","source":"sku"})),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
