```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.inventories_reservations().inventories_reservations_get(
        "".to_string(),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
