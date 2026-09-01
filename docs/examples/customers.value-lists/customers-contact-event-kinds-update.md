```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.customers_value_lists().customers_contact_event_kinds_update(
        "".to_string(),
        Some("Somebody spoke to this person on the phone.".to_string()),
        Some(serde_json::json!({"de":"Es wurde mit dieser Person telefoniert.","en":"Somebody spoke to this person on the phone."})),
        Some(true),
        Some(serde_json::json!({"de":"Telefonat","en":"Phone call"})),
        Some(1),
        Some("Phone call".to_string()),
        Some("neutral".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
