```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.customers_roles().customers_roles_defaults(
        serde_json::json!({}),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
