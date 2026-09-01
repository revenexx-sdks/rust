```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.products().products_create(
        "ACME-4711-BLK".to_string(),
        Some(serde_json::json!({"channel_locale_specific":{"b2b":{"de_DE":{"description":"Staffelpreise auf Anfrage."}}},"channel_specific":{"b2b":{"minimum_order_quantity":6}},"common":{"colour":"black","manufacturer_aid":"4711-BLK","net_weight":2.4},"locale_specific":{"de_DE":{"description":"Bürstenloser Motor, 2 Akkus im Set.","name":"Akku-Bohrschrauber 18V"},"en_GB":{"name":"18V cordless drill"}}})),
        Some(serde_json::json!({"computed_at":"2026-01-01T12:00:00Z","filled":9,"missing":["net_weight","packaging_unit","safety_datasheet"],"ratio":0.75,"required":12})),
        Some("2026-01-01T12:00:00Z".to_string()),
        Some(true),
        Some("".to_string()),
        Some("".to_string()),
        Some("simple".to_string()),
        Some("".to_string()),
        Some(serde_json::json!({"PRODUCT_SET":{"product_models":[],"products":[{"identifier":"ACME-4711-CASTER","quantity":4}]}})),
        Some("standard".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
