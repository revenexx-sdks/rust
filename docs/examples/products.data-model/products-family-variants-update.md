```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.products_data_model().products_family_variants_update(
        "".to_string(),
        Some(serde_json::json!(["colour","size"])),
        Some("clothing_by_colour_size".to_string()),
        Some("".to_string()),
        Some(serde_json::json!({"de":"Nach Farbe und Größe","en":"By colour and size"})),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
