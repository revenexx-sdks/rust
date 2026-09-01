```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.products_references().products_reference_entity_records_create(
        "acme_tools".to_string(),
        "".to_string(),
        Some(serde_json::json!({"common":{"country":"DE","founded":1946},"locale_specific":{"de_DE":{"description":"Werkzeughersteller aus Süddeutschland."}}})),
        Some(serde_json::json!({"de":"Acme Tools","en":"Acme Tools"})),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
