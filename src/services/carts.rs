use std::collections::HashMap;

use serde_json::Value;

use crate::client::Client;
use crate::error::Error;
use crate::input_file::InputFile;

/// Carts service
pub struct Carts {
    client: Client,
}

impl Carts {
    pub fn new(client: Client) -> Self {
        Carts { client }
    }
    pub async fn carts_list(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/carts".to_string();

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
    pub async fn carts_create(&self, channel_id: Option<String>, contact_id: Option<String>, currency: Option<String>, is_current: Option<bool>, market_id: Option<String>, metadata: Option<serde_json::Value>, name: Option<String>, session_key: Option<String>) -> Result<crate::models::Cart, Error> {
        let api_path = "/v1/carts".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &channel_id {
            api_params.insert("channel_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &contact_id {
            api_params.insert("contact_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &currency {
            api_params.insert("currency".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &is_current {
            api_params.insert("is_current".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &market_id {
            api_params.insert("market_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &metadata {
            api_params.insert("metadata".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &name {
            api_params.insert("name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &session_key {
            api_params.insert("session_key".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn carts_claim(&self, contact_id: String, session_key: String, target_cart_id: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/carts/claim".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("contact_id".to_string(), serde_json::to_value(&contact_id)?);
        api_params.insert("session_key".to_string(), serde_json::to_value(&session_key)?);
        if let Some(value) = &target_cart_id {
            api_params.insert("target_cart_id".to_string(), serde_json::to_value(value)?);
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
    pub async fn carts_import(&self, contact_id: Option<String>, csv: Option<String>, name: Option<String>, payload: Option<serde_json::Value>, profile_id: Option<String>, session_key: Option<String>, target_cart_id: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/carts/import".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &contact_id {
            api_params.insert("contact_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &csv {
            api_params.insert("csv".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &name {
            api_params.insert("name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &payload {
            api_params.insert("payload".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &profile_id {
            api_params.insert("profile_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &session_key {
            api_params.insert("session_key".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &target_cart_id {
            api_params.insert("target_cart_id".to_string(), serde_json::to_value(value)?);
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
    pub async fn carts_io_profiles_list(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/carts/io/profiles".to_string();

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
    pub async fn carts_io_profiles_create(&self, direction: String, name: String, apply_mode: Option<String>, entity: Option<String>, format: Option<String>, is_template: Option<bool>, mapping: Option<serde_json::Value>, options: Option<serde_json::Value>) -> Result<crate::models::IoProfile, Error> {
        let api_path = "/v1/carts/io/profiles".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("direction".to_string(), serde_json::to_value(&direction)?);
        api_params.insert("name".to_string(), serde_json::to_value(&name)?);
        if let Some(value) = &apply_mode {
            api_params.insert("apply_mode".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &entity {
            api_params.insert("entity".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &format {
            api_params.insert("format".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &is_template {
            api_params.insert("is_template".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &mapping {
            api_params.insert("mapping".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &options {
            api_params.insert("options".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn carts_io_profiles_defaults(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/carts/io/profiles/defaults".to_string();

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
    pub async fn carts_io_profiles_delete(&self, id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/carts/io/profiles/{id}".replace("{id}", &id.to_string());

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
    pub async fn carts_io_profiles_get(&self, id: String) -> Result<crate::models::IoProfile, Error> {
        let api_path = "/v1/carts/io/profiles/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn carts_io_profiles_update(&self, id: String, apply_mode: Option<String>, direction: Option<String>, entity: Option<String>, format: Option<String>, is_template: Option<bool>, mapping: Option<serde_json::Value>, name: Option<String>, options: Option<serde_json::Value>) -> Result<crate::models::IoProfile, Error> {
        let api_path = "/v1/carts/io/profiles/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &apply_mode {
            api_params.insert("apply_mode".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &direction {
            api_params.insert("direction".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &entity {
            api_params.insert("entity".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &format {
            api_params.insert("format".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &is_template {
            api_params.insert("is_template".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &mapping {
            api_params.insert("mapping".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &name {
            api_params.insert("name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &options {
            api_params.insert("options".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn carts_merge(&self, source_cart_id: String, target_cart_id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/carts/merge".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("source_cart_id".to_string(), serde_json::to_value(&source_cart_id)?);
        api_params.insert("target_cart_id".to_string(), serde_json::to_value(&target_cart_id)?);

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
    pub async fn carts_items_list(&self, cart_id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/carts/{cart_id}/items".replace("{cartId}", &cart_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("cart_id".to_string(), serde_json::to_value(&cart_id)?);

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
    pub async fn carts_items_create(&self, cart_id: String, configuration: Option<serde_json::Value>, currency: Option<String>, metadata: Option<serde_json::Value>, name: Option<String>, position: Option<i64>, product_id: Option<String>, quantity: Option<f64>, sku: Option<String>, snapshot: Option<serde_json::Value>, tax_rate: Option<f64>, xtype: Option<String>, unit: Option<String>, unit_price: Option<f64>) -> Result<crate::models::CartItem, Error> {
        let api_path = "/v1/carts/{cart_id}/items".replace("{cartId}", &cart_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("cart_id".to_string(), serde_json::to_value(&cart_id)?);
        if let Some(value) = &configuration {
            api_params.insert("configuration".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &currency {
            api_params.insert("currency".to_string(), serde_json::to_value(value)?);
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
        if let Some(value) = &product_id {
            api_params.insert("product_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &quantity {
            api_params.insert("quantity".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &sku {
            api_params.insert("sku".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &snapshot {
            api_params.insert("snapshot".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &tax_rate {
            api_params.insert("tax_rate".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &xtype {
            api_params.insert("type".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &unit {
            api_params.insert("unit".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &unit_price {
            api_params.insert("unit_price".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn carts_items_replace(&self, cart_id: String, items: Vec<crate::models::CartItemCreateRequest>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/carts/{cart_id}/items".replace("{cartId}", &cart_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("cart_id".to_string(), serde_json::to_value(&cart_id)?);
        api_params.insert("items".to_string(), serde_json::to_value(&items)?);

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
    pub async fn carts_items_delete(&self, cart_id: String, id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/carts/{cart_id}/items/{id}".replace("{cartId}", &cart_id.to_string()).replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("cart_id".to_string(), serde_json::to_value(&cart_id)?);
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
    pub async fn carts_items_get(&self, cart_id: String, id: String) -> Result<crate::models::CartItem, Error> {
        let api_path = "/v1/carts/{cart_id}/items/{id}".replace("{cartId}", &cart_id.to_string()).replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("cart_id".to_string(), serde_json::to_value(&cart_id)?);
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn carts_items_update(&self, cart_id: String, id: String, configuration: Option<serde_json::Value>, currency: Option<String>, metadata: Option<serde_json::Value>, name: Option<String>, position: Option<i64>, product_id: Option<String>, quantity: Option<f64>, sku: Option<String>, snapshot: Option<serde_json::Value>, tax_rate: Option<f64>, xtype: Option<String>, unit: Option<String>, unit_price: Option<f64>) -> Result<crate::models::CartItem, Error> {
        let api_path = "/v1/carts/{cart_id}/items/{id}".replace("{cartId}", &cart_id.to_string()).replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("cart_id".to_string(), serde_json::to_value(&cart_id)?);
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &configuration {
            api_params.insert("configuration".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &currency {
            api_params.insert("currency".to_string(), serde_json::to_value(value)?);
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
        if let Some(value) = &product_id {
            api_params.insert("product_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &quantity {
            api_params.insert("quantity".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &sku {
            api_params.insert("sku".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &snapshot {
            api_params.insert("snapshot".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &tax_rate {
            api_params.insert("tax_rate".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &xtype {
            api_params.insert("type".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &unit {
            api_params.insert("unit".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &unit_price {
            api_params.insert("unit_price".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn carts_delete(&self, id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/carts/{id}".replace("{id}", &id.to_string());

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
    pub async fn carts_get(&self, id: String) -> Result<crate::models::Cart, Error> {
        let api_path = "/v1/carts/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn carts_update(&self, id: String, channel_id: Option<String>, currency: Option<String>, market_id: Option<String>, metadata: Option<serde_json::Value>, name: Option<String>) -> Result<crate::models::Cart, Error> {
        let api_path = "/v1/carts/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &channel_id {
            api_params.insert("channel_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &currency {
            api_params.insert("currency".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &market_id {
            api_params.insert("market_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &metadata {
            api_params.insert("metadata".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &name {
            api_params.insert("name".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn carts_abandon(&self, id: String) -> Result<crate::models::Cart, Error> {
        let api_path = "/v1/carts/{id}/abandon".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn carts_activate(&self, id: String) -> Result<crate::models::Cart, Error> {
        let api_path = "/v1/carts/{id}/activate".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn carts_export(&self, id: String, format: Option<String>, profile_id: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/carts/{id}/export".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &format {
            api_params.insert("format".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &profile_id {
            api_params.insert("profile_id".to_string(), serde_json::to_value(value)?);
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
    pub async fn carts_order(&self, id: String, order_ref: Option<String>) -> Result<crate::models::Cart, Error> {
        let api_path = "/v1/carts/{id}/order".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &order_ref {
            api_params.insert("order_ref".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn carts_reopen(&self, id: String) -> Result<crate::models::Cart, Error> {
        let api_path = "/v1/carts/{id}/reopen".replace("{id}", &id.to_string());

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
