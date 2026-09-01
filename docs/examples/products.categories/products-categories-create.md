```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.products_categories().products_categories_create(
        "cordless_drills".to_string(),
        Some(serde_json::json!({"de":"Akku-Bohrschrauber","en":"Cordless drills"})),
        Some("".to_string()),
        Some("tools/power_tools/cordless_drills".to_string()),
        Some(1),
        Some("all".to_string()),
        Some(serde_json::json!({"conditions":[{"field":"attribute:brand","operator":"in","value":["acme","globex"]},{"field":"enabled","operator":"eq","value":true}]})),
        Some("2026-01-01T12:00:00Z".to_string()),
        Some(serde_json::json!({"hero_asset":"packshots/cordless_drills_hero","seo_title":"Cordless drills"})),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
