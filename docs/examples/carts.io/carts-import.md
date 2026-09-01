```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.carts_io().carts_import(
        Some("".to_string()),
        Some("sku,name,quantity,unit_price
BOLT-M8-30,Hex bolt M8,100,0.12
NUT-M8,Hex nut M8,100,0.04
".to_string()),
        Some("Weekly order".to_string()),
        Some(serde_json::json!({"cart":{"currency":"EUR","name":"Weekly order"},"items":[{"name":"Hex bolt M8","quantity":100,"sku":"BOLT-M8-30","unit_price":0.12}]})),
        Some("".to_string()),
        Some("a1b2c3d4e5f6".to_string()),
        Some("".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
