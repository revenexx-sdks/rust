```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.carts_items().carts_items_create(
        "".to_string(),
        Some(serde_json::json!({"colour":"RAL 7016","finish":"brushed","length_mm":2400,"mounting":"wall"})),
        Some("EUR".to_string()),
        Some(serde_json::json!({"campaign":"spring-catalogue","locale":"de-DE","source":"storefront"})),
        Some("Hex bolt M8".to_string()),
        Some(1),
        Some("".to_string()),
        Some(9.99),
        Some("BOLT-M8-30".to_string()),
        Some(serde_json::json!({})),
        Some(19.0),
        Some("product".to_string()),
        Some("pcs".to_string()),
        Some(9.99),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
