```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.customers().customers_auth_otp(
        "einkauf@example.com".to_string(),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
