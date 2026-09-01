```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.pages_editor().pages_editor_templates_create(
        "".to_string(),
        "Hero with two teasers".to_string(),
        vec![],
        Some("Full-width hero followed by a two-column teaser row.".to_string()),
        Some("content".to_string()),
        Some(true),
        Some("standard".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
