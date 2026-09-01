```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.shipping_value_lists().shipping_weight_units_list(
        Some(1),
        Some(1),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
