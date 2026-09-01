```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.apps().apps_create_vcs_deployment(
        "".to_string(),
        "main".to_string(),
        "branch".to_string(),
        Some(true),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
