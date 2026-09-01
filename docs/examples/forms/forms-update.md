```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.forms().forms_update(
        "".to_string(),
        Some(vec![{"$formkit":"text","label":"Company","name":"company","validation":"required"},{"$formkit":"email","label":"Email","name":"email","validation":"required|email"},{"$formkit":"textarea","label":"What do you need a price for?","name":"message","rows":4},{"$el":"p","children":"We answer price requests within one working day."}]),
        Some(serde_json::json!({})),
        Some("Price request".to_string()),
        Some(serde_json::json!({})),
        Some("price-request".to_string()),
        Some("draft".to_string()),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
