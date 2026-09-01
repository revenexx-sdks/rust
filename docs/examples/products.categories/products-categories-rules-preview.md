```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.products_categories().products_categories_rules_preview(
        "".to_string(),
        vec![],
        Some("all".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
