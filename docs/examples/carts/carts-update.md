```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.carts().carts_update(
        "".to_string(),
        Some("".to_string()),
        Some("EUR".to_string()),
        Some(serde_json::json!({"campaign":"spring-catalogue","locale":"de-DE","source":"storefront"})),
        Some("Weekly order".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
