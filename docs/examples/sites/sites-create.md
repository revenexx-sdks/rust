```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.sites().sites_create(
        "node-18.0".to_string(),
        "analog".to_string(),
        "".to_string(),
        "".to_string(),
        Some("static".to_string()),
        Some("npm run build".to_string()),
        Some(true),
        Some("index.html".to_string()),
        Some("npm install".to_string()),
        Some("".to_string()),
        Some(true),
        Some("".to_string()),
        Some("main".to_string()),
        Some("".to_string()),
        Some("".to_string()),
        Some(true),
        Some("s-1vcpu-512mb".to_string()),
        Some(1),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
