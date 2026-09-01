```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.payments_ledger().payments_list(
        Some(1),
        Some(1),
        Some("created_at.desc".to_string()),
        Some("".to_string()),
        Some("".to_string()),
        Some("created".to_string()),
        Some("ORD-10042".to_string()),
        Some("invoice".to_string()),
        Some("self_managed".to_string()),
        Some("stripe".to_string()),
        Some("none".to_string()),
        Some("checkout-2f9c41".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
