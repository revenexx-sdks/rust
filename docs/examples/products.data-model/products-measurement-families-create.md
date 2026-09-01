```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.products_data_model().products_measurement_families_create(
        "weight".to_string(),
        "kilogram".to_string(),
        Some(serde_json::json!({"de":"Gewicht","en":"Weight"})),
        Some(serde_json::json!([{"code":"kilogram","convert_factor":1,"symbol":"kg"},{"code":"gram","convert_factor":0.001,"symbol":"g"}])),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
