```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.prices().prices_entries_ladder(
        "".to_string(),
        9.99,
        Some(9.99),
        Some("".to_string()),
        Some(vec![1,10,50]),
        Some(true),
        Some("exact".to_string()),
        Some("BOLT-M8-30".to_string()),
        Some("pcs".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
