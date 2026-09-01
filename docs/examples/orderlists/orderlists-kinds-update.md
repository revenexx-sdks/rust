```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.orderlists().orderlists_kinds_update(
        "".to_string(),
        Some("Chemicals ordered against a standing lab protocol.".to_string()),
        Some(serde_json::json!({"de":"Chemikalien, die nach einem festen Laborprotokoll bestellt werden.","en":"Chemicals ordered against a standing lab protocol."})),
        Some(true),
        Some(serde_json::json!({"de":"Reagenzienliste","en":"Reagent list"})),
        Some(2),
        Some("Reagent list".to_string()),
        Some("neutral".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
