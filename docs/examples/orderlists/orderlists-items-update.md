```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.orderlists().orderlists_items_update(
        "".to_string(),
        "".to_string(),
        Some("office-supplies".to_string()),
        Some("CC-100".to_string()),
        Some("CUST-4711".to_string()),
        Some("https://cdn.example.com/catalog/acme-4711-blk.jpg".to_string()),
        Some(serde_json::json!({"erp_line_ref":"4711-01"})),
        Some("Copy paper A4, 80 g/m², white".to_string()),
        Some(0),
        Some(vec!["Deliver to bay 3","Engraving: Team A"]),
        Some(3.49),
        Some("".to_string()),
        Some(12.0),
        Some("ACME-4711-BLK".to_string()),
        Some("paper".to_string()),
        Some(19.0),
        Some("piece".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
