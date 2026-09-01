```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.products_assets().products_assets_list(
        Some(1),
        Some(1),
        Some("created_at.desc".to_string()),
        Some("".to_string()),
        Some("".to_string()),
        Some("acme-4711-blk_packshot_1".to_string()),
        Some("storage".to_string()),
        Some("ast_01J8ZQ0000000000000000".to_string()),
        Some("packshots/acme-4711-blk_1.jpg".to_string()),
        Some("https://cdn.example.com/packshots/acme-4711-blk_1.jpg".to_string()),
        Some("{}".to_string()),
        Some("2026-01-01T12:00:00Z".to_string()),
        Some("2026-01-01T12:00:00Z".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
