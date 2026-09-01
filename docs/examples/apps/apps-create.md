```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.apps().apps_create(
        "".to_string(),
        "".to_string(),
        "node-18.0".to_string(),
        Some("npm install".to_string()),
        Some(true),
        Some("src/main.js".to_string()),
        Some(vec![]),
        Some(vec!["any"]),
        Some("".to_string()),
        Some(true),
        Some("main".to_string()),
        Some("".to_string()),
        Some("".to_string()),
        Some(true),
        Some("0 3 * * *".to_string()),
        Some(vec![]),
        Some("s-1vcpu-512mb".to_string()),
        Some(1),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
