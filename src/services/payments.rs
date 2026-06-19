use std::collections::HashMap;

use serde_json::Value;

use crate::client::Client;
use crate::error::Error;
use crate::input_file::InputFile;

/// Payments service
pub struct Payments {
    client: Client,
}

impl Payments {
    pub fn new(client: Client) -> Self {
        Payments { client }
    }
    pub async fn payments_list(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/payments".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        if api_response.body.is_empty() {
            Ok(serde_json::Value::Null)
        } else {
            Ok(serde_json::from_slice(&api_response.body)?)
        }
    }
    pub async fn payments_create(&self, amount: f64, method_code: String, cart_id: Option<String>, contact_id: Option<String>, country: Option<String>, currency: Option<String>, idempotency_key: Option<String>, metadata: Option<serde_json::Value>, order_ref: Option<String>, return_url: Option<String>) -> Result<crate::models::Payment, Error> {
        let api_path = "/v1/payments".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("amount".to_string(), serde_json::to_value(&amount)?);
        api_params.insert("method_code".to_string(), serde_json::to_value(&method_code)?);
        if let Some(value) = &cart_id {
            api_params.insert("cart_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &contact_id {
            api_params.insert("contact_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &country {
            api_params.insert("country".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &currency {
            api_params.insert("currency".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &idempotency_key {
            api_params.insert("idempotency_key".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &metadata {
            api_params.insert("metadata".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &order_ref {
            api_params.insert("order_ref".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &return_url {
            api_params.insert("return_url".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn payments_methods_list(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/payments/methods".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        if api_response.body.is_empty() {
            Ok(serde_json::Value::Null)
        } else {
            Ok(serde_json::from_slice(&api_response.body)?)
        }
    }
    pub async fn payments_methods_create(&self, code: String, name: String, countries: Option<Vec<String>>, description: Option<String>, enabled: Option<bool>, fee_amount: Option<f64>, fee_currency: Option<String>, fee_type: Option<String>, kind: Option<String>, labels: Option<serde_json::Value>, max_order_value: Option<f64>, metadata: Option<serde_json::Value>, min_order_value: Option<f64>, position: Option<i64>, provider: Option<String>, provider_method: Option<String>) -> Result<crate::models::PaymentMethod, Error> {
        let api_path = "/v1/payments/methods".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("code".to_string(), serde_json::to_value(&code)?);
        api_params.insert("name".to_string(), serde_json::to_value(&name)?);
        if let Some(value) = &countries {
            api_params.insert("countries".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &description {
            api_params.insert("description".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &enabled {
            api_params.insert("enabled".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &fee_amount {
            api_params.insert("fee_amount".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &fee_currency {
            api_params.insert("fee_currency".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &fee_type {
            api_params.insert("fee_type".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &kind {
            api_params.insert("kind".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &max_order_value {
            api_params.insert("max_order_value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &metadata {
            api_params.insert("metadata".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &min_order_value {
            api_params.insert("min_order_value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &provider {
            api_params.insert("provider".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &provider_method {
            api_params.insert("provider_method".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn payments_methods_defaults(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/payments/methods/defaults".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        if api_response.body.is_empty() {
            Ok(serde_json::Value::Null)
        } else {
            Ok(serde_json::from_slice(&api_response.body)?)
        }
    }
    pub async fn payments_methods_eligible(&self, amount: Option<f64>, country: Option<String>, currency: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/payments/methods/eligible".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &amount {
            api_params.insert("amount".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &country {
            api_params.insert("country".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &currency {
            api_params.insert("currency".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        if api_response.body.is_empty() {
            Ok(serde_json::Value::Null)
        } else {
            Ok(serde_json::from_slice(&api_response.body)?)
        }
    }
    pub async fn payments_methods_delete(&self, id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/payments/methods/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("DELETE", &api_path, api_headers, api_params)
            .await?;

        if api_response.body.is_empty() {
            Ok(serde_json::Value::Null)
        } else {
            Ok(serde_json::from_slice(&api_response.body)?)
        }
    }
    pub async fn payments_methods_get(&self, id: String) -> Result<crate::models::PaymentMethod, Error> {
        let api_path = "/v1/payments/methods/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn payments_methods_update(&self, id: String, code: Option<String>, countries: Option<Vec<String>>, description: Option<String>, enabled: Option<bool>, fee_amount: Option<f64>, fee_currency: Option<String>, fee_type: Option<String>, kind: Option<String>, labels: Option<serde_json::Value>, max_order_value: Option<f64>, metadata: Option<serde_json::Value>, min_order_value: Option<f64>, name: Option<String>, position: Option<i64>, provider: Option<String>, provider_method: Option<String>) -> Result<crate::models::PaymentMethod, Error> {
        let api_path = "/v1/payments/methods/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &code {
            api_params.insert("code".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &countries {
            api_params.insert("countries".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &description {
            api_params.insert("description".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &enabled {
            api_params.insert("enabled".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &fee_amount {
            api_params.insert("fee_amount".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &fee_currency {
            api_params.insert("fee_currency".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &fee_type {
            api_params.insert("fee_type".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &kind {
            api_params.insert("kind".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &max_order_value {
            api_params.insert("max_order_value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &metadata {
            api_params.insert("metadata".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &min_order_value {
            api_params.insert("min_order_value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &name {
            api_params.insert("name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &provider {
            api_params.insert("provider".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &provider_method {
            api_params.insert("provider_method".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn payments_providers_list(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/payments/providers".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        if api_response.body.is_empty() {
            Ok(serde_json::Value::Null)
        } else {
            Ok(serde_json::from_slice(&api_response.body)?)
        }
    }
    pub async fn payments_providers_create(&self, provider: String, credentials: Option<serde_json::Value>, enabled: Option<bool>, name: Option<String>, options: Option<serde_json::Value>, test_mode: Option<bool>, webhook_secret: Option<String>) -> Result<crate::models::PaymentProvider, Error> {
        let api_path = "/v1/payments/providers".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("provider".to_string(), serde_json::to_value(&provider)?);
        if let Some(value) = &credentials {
            api_params.insert("credentials".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &enabled {
            api_params.insert("enabled".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &name {
            api_params.insert("name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &options {
            api_params.insert("options".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &test_mode {
            api_params.insert("test_mode".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &webhook_secret {
            api_params.insert("webhook_secret".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn payments_providers_catalog(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/payments/providers/catalog".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        if api_response.body.is_empty() {
            Ok(serde_json::Value::Null)
        } else {
            Ok(serde_json::from_slice(&api_response.body)?)
        }
    }
    pub async fn payments_providers_delete(&self, id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/payments/providers/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("DELETE", &api_path, api_headers, api_params)
            .await?;

        if api_response.body.is_empty() {
            Ok(serde_json::Value::Null)
        } else {
            Ok(serde_json::from_slice(&api_response.body)?)
        }
    }
    pub async fn payments_providers_get(&self, id: String) -> Result<crate::models::PaymentProvider, Error> {
        let api_path = "/v1/payments/providers/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn payments_providers_update(&self, id: String, credentials: Option<serde_json::Value>, enabled: Option<bool>, name: Option<String>, options: Option<serde_json::Value>, provider: Option<String>, test_mode: Option<bool>, webhook_secret: Option<String>) -> Result<crate::models::PaymentProvider, Error> {
        let api_path = "/v1/payments/providers/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &credentials {
            api_params.insert("credentials".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &enabled {
            api_params.insert("enabled".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &name {
            api_params.insert("name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &options {
            api_params.insert("options".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &provider {
            api_params.insert("provider".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &test_mode {
            api_params.insert("test_mode".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &webhook_secret {
            api_params.insert("webhook_secret".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Consumes the dispatch envelope from webhooks.revenexx.com: normalizes the
    /// provider callback (stripe payment intents + a generic shape), resolves the
    /// payment by psp_payment_id or order_ref and moves the ledger. Facts only
    /// move forward — provider retries and redeliveries are idempotent no-ops;
    /// unverified envelopes are refused.
    pub async fn payments_webhooks_ingest(&self, provider: String, data: serde_json::Value) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/payments/webhooks/{provider}".replace("{provider}", &provider.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("provider".to_string(), serde_json::to_value(&provider)?);
        api_params.insert("data".to_string(), serde_json::to_value(&data)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        if api_response.body.is_empty() {
            Ok(serde_json::Value::Null)
        } else {
            Ok(serde_json::from_slice(&api_response.body)?)
        }
    }
    pub async fn payments_get(&self, id: String) -> Result<crate::models::Payment, Error> {
        let api_path = "/v1/payments/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn payments_cancel(&self, id: String) -> Result<crate::models::Payment, Error> {
        let api_path = "/v1/payments/{id}/cancel".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn payments_capture(&self, id: String) -> Result<crate::models::Payment, Error> {
        let api_path = "/v1/payments/{id}/capture".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn payments_confirm(&self, id: String) -> Result<crate::models::Payment, Error> {
        let api_path = "/v1/payments/{id}/confirm".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn payments_refund(&self, id: String) -> Result<crate::models::Payment, Error> {
        let api_path = "/v1/payments/{id}/refund".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
}
