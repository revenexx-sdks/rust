```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.prices().prices_lists_update(
        "".to_string(),
        Some("".to_string()),
        Some("dealer-de".to_string()),
        Some("".to_string()),
        Some("EUR".to_string()),
        Some("Contract prices for authorised dealers.".to_string()),
        Some(true),
        Some(serde_json::json!({"de":"Händlerpreise","en":"Dealer prices"})),
        Some(serde_json::json!({"erp_price_group":"A1","source_system":"erp"})),
        Some("Dealer prices".to_string()),
        Some("".to_string()),
        Some(1),
        Some(true),
        Some("active".to_string()),
        Some("net".to_string()),
        Some(true),
        Some("2026-01-01T00:00:00Z".to_string()),
        Some("2026-12-31T23:59:59Z".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
