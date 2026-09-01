```rust
use revenexx::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()        .set_endpoint("https://api.revenexx.com")        .set_api_key_auth("<API_KEY>")        ;

    let response = client.orders().orders_update(
        "".to_string(),
        Some(serde_json::json!({"city":"Berlin","company":"Beispiel Industrietechnik GmbH","country":"DE","name":"Anna Berger","street":"Musterstraße 12","zip":"10115"})),
        Some(serde_json::json!({"company":"Beispiel Industrietechnik GmbH","customer_number":"K-10042","email":"anna.berger@example.com","name":"Anna Berger"})),
        Some("PO-2026-0042".to_string()),
        Some(serde_json::json!({"erp_batch":"2026-W32"})),
        Some(serde_json::json!({"city":"Berlin","company":"Beispiel Industrietechnik GmbH","country":"DE","name":"Anna Berger","street":"Musterstraße 12","zip":"10115"})),
        Some(serde_json::json!({"campaign":"spring-catalogue","source":"webshop"})),
    ).await?;

    println!("{:?}", response);
    Ok(())
}
```
