use std::collections::HashMap;

use serde_json::Value;

use crate::client::Client;
use crate::error::Error;
use crate::input_file::InputFile;

/// Shipping service
pub struct Shipping {
    client: Client,
}

impl Shipping {
    pub fn new(client: Client) -> Self {
        Shipping { client }
    }
    pub async fn shipping_methods_list(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/shipping/methods".to_string();

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
    pub async fn shipping_methods_create(&self, code: String, name: String, carrier: Option<String>, countries: Option<Vec<String>>, currency: Option<String>, description: Option<String>, enabled: Option<bool>, eta_days_max: Option<i64>, eta_days_min: Option<i64>, free_above: Option<f64>, labels: Option<serde_json::Value>, matrix_attribute: Option<String>, matrix_basis: Option<String>, metadata: Option<serde_json::Value>, position: Option<i64>, price: Option<f64>, pricing_type: Option<String>) -> Result<crate::models::ShippingMethod, Error> {
        let api_path = "/v1/shipping/methods".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("code".to_string(), serde_json::to_value(&code)?);
        api_params.insert("name".to_string(), serde_json::to_value(&name)?);
        if let Some(value) = &carrier {
            api_params.insert("carrier".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &countries {
            api_params.insert("countries".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &currency {
            api_params.insert("currency".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &description {
            api_params.insert("description".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &enabled {
            api_params.insert("enabled".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &eta_days_max {
            api_params.insert("eta_days_max".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &eta_days_min {
            api_params.insert("eta_days_min".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &free_above {
            api_params.insert("free_above".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &matrix_attribute {
            api_params.insert("matrix_attribute".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &matrix_basis {
            api_params.insert("matrix_basis".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &metadata {
            api_params.insert("metadata".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &price {
            api_params.insert("price".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &pricing_type {
            api_params.insert("pricing_type".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn shipping_methods_defaults(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/shipping/methods/defaults".to_string();

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
    pub async fn shipping_methods_delete(&self, id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/shipping/methods/{id}".replace("{id}", &id.to_string());

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
    pub async fn shipping_methods_get(&self, id: String) -> Result<crate::models::ShippingMethod, Error> {
        let api_path = "/v1/shipping/methods/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn shipping_methods_update(&self, id: String, carrier: Option<String>, code: Option<String>, countries: Option<Vec<String>>, currency: Option<String>, description: Option<String>, enabled: Option<bool>, eta_days_max: Option<i64>, eta_days_min: Option<i64>, free_above: Option<f64>, labels: Option<serde_json::Value>, matrix_attribute: Option<String>, matrix_basis: Option<String>, metadata: Option<serde_json::Value>, name: Option<String>, position: Option<i64>, price: Option<f64>, pricing_type: Option<String>) -> Result<crate::models::ShippingMethod, Error> {
        let api_path = "/v1/shipping/methods/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &carrier {
            api_params.insert("carrier".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &code {
            api_params.insert("code".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &countries {
            api_params.insert("countries".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &currency {
            api_params.insert("currency".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &description {
            api_params.insert("description".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &enabled {
            api_params.insert("enabled".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &eta_days_max {
            api_params.insert("eta_days_max".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &eta_days_min {
            api_params.insert("eta_days_min".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &free_above {
            api_params.insert("free_above".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &matrix_attribute {
            api_params.insert("matrix_attribute".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &matrix_basis {
            api_params.insert("matrix_basis".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &metadata {
            api_params.insert("metadata".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &name {
            api_params.insert("name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &price {
            api_params.insert("price".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &pricing_type {
            api_params.insert("pricing_type".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn shipping_tiers_list(&self, method_id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/shipping/methods/{method_id}/tiers".replace("{methodId}", &method_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("method_id".to_string(), serde_json::to_value(&method_id)?);

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
    pub async fn shipping_tiers_create(&self, method_id: String, from_value: Option<f64>, position: Option<i64>, price: Option<f64>) -> Result<crate::models::ShippingRateTier, Error> {
        let api_path = "/v1/shipping/methods/{method_id}/tiers".replace("{methodId}", &method_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("method_id".to_string(), serde_json::to_value(&method_id)?);
        if let Some(value) = &from_value {
            api_params.insert("from_value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &price {
            api_params.insert("price".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn shipping_tiers_replace(&self, method_id: String, tiers: Vec<crate::models::ShippingRateTierReplaceItem>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/shipping/methods/{method_id}/tiers".replace("{methodId}", &method_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("method_id".to_string(), serde_json::to_value(&method_id)?);
        api_params.insert("tiers".to_string(), serde_json::to_value(&tiers)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        if api_response.body.is_empty() {
            Ok(serde_json::Value::Null)
        } else {
            Ok(serde_json::from_slice(&api_response.body)?)
        }
    }
    pub async fn shipping_tiers_delete(&self, method_id: String, id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/shipping/methods/{method_id}/tiers/{id}".replace("{methodId}", &method_id.to_string()).replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("method_id".to_string(), serde_json::to_value(&method_id)?);
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
    pub async fn shipping_tiers_get(&self, method_id: String, id: String) -> Result<crate::models::ShippingRateTier, Error> {
        let api_path = "/v1/shipping/methods/{method_id}/tiers/{id}".replace("{methodId}", &method_id.to_string()).replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("method_id".to_string(), serde_json::to_value(&method_id)?);
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn shipping_tiers_update(&self, method_id: String, id: String, from_value: Option<f64>, position: Option<i64>, price: Option<f64>) -> Result<crate::models::ShippingRateTier, Error> {
        let api_path = "/v1/shipping/methods/{method_id}/tiers/{id}".replace("{methodId}", &method_id.to_string()).replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("method_id".to_string(), serde_json::to_value(&method_id)?);
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &from_value {
            api_params.insert("from_value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &price {
            api_params.insert("price".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn shipping_rates(&self, attributes: Option<serde_json::Value>, country: Option<String>, currency: Option<String>, market_id: Option<String>, order_value: Option<f64>, quantity: Option<f64>, weight: Option<f64>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/shipping/rates".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &attributes {
            api_params.insert("attributes".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &country {
            api_params.insert("country".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &currency {
            api_params.insert("currency".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &market_id {
            api_params.insert("market_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &order_value {
            api_params.insert("order_value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &quantity {
            api_params.insert("quantity".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &weight {
            api_params.insert("weight".to_string(), serde_json::to_value(value)?);
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
}
