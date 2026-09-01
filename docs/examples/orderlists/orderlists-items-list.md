```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.orderlists().orderlists_items_list(
        "".to_string(),
        Some("".to_string()),
        Some("".to_string()),
        Some("ACME-4711-BLK".to_string()),
        Some("Copy paper A4, 80 g/m², white".to_string()),
        Some("https://cdn.example.com/catalog/acme-4711-blk.jpg".to_string()),
        Some(12.0),
        Some("piece".to_string()),
        Some(3.49),
        Some(19.0),
        Some("CC-100".to_string()),
        Some("{}".to_string()),
        Some("CUST-4711".to_string()),
        Some("office-supplies".to_string()),
        Some("paper".to_string()),
        Some(0),
        Some("{}".to_string()),
        Some("2026-01-01T12:00:00Z".to_string()),
        Some("2026-01-01T12:00:00Z".to_string()),
        Some(50),
        Some(0),
        Some("created_at.desc".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
