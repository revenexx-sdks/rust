use std::collections::HashMap;

use serde_json::Value;

use crate::client::Client;
use crate::error::Error;
use crate::input_file::InputFile;

/// Prices service
pub struct Prices {
    client: Client,
}

impl Prices {
    pub fn new(client: Client) -> Self {
        Prices { client }
    }
    pub async fn prices_lists_list(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/prices/lists".to_string();

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
    pub async fn prices_lists_create(&self, code: String, name: String, channel_id: Option<String>, contact_id: Option<String>, currency: Option<String>, description: Option<String>, is_default: Option<bool>, labels: Option<serde_json::Value>, market_id: Option<String>, metadata: Option<serde_json::Value>, organization_id: Option<String>, priority: Option<i64>, status: Option<String>, tax_included: Option<bool>, valid_from: Option<String>, valid_until: Option<String>) -> Result<crate::models::PriceList, Error> {
        let api_path = "/v1/prices/lists".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("code".to_string(), serde_json::to_value(&code)?);
        api_params.insert("name".to_string(), serde_json::to_value(&name)?);
        if let Some(value) = &channel_id {
            api_params.insert("channel_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &contact_id {
            api_params.insert("contact_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &currency {
            api_params.insert("currency".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &description {
            api_params.insert("description".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &is_default {
            api_params.insert("is_default".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &market_id {
            api_params.insert("market_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &metadata {
            api_params.insert("metadata".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &organization_id {
            api_params.insert("organization_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &priority {
            api_params.insert("priority".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &status {
            api_params.insert("status".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &tax_included {
            api_params.insert("tax_included".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &valid_from {
            api_params.insert("valid_from".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &valid_until {
            api_params.insert("valid_until".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn prices_lists_defaults(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/prices/lists/defaults".to_string();

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
    pub async fn prices_lists_delete(&self, id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/prices/lists/{id}".replace("{id}", &id.to_string());

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
    pub async fn prices_lists_get(&self, id: String) -> Result<crate::models::PriceList, Error> {
        let api_path = "/v1/prices/lists/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn prices_lists_update(&self, id: String, channel_id: Option<String>, code: Option<String>, contact_id: Option<String>, currency: Option<String>, description: Option<String>, is_default: Option<bool>, labels: Option<serde_json::Value>, market_id: Option<String>, metadata: Option<serde_json::Value>, name: Option<String>, organization_id: Option<String>, priority: Option<i64>, status: Option<String>, tax_included: Option<bool>, valid_from: Option<String>, valid_until: Option<String>) -> Result<crate::models::PriceList, Error> {
        let api_path = "/v1/prices/lists/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &channel_id {
            api_params.insert("channel_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &code {
            api_params.insert("code".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &contact_id {
            api_params.insert("contact_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &currency {
            api_params.insert("currency".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &description {
            api_params.insert("description".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &is_default {
            api_params.insert("is_default".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
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
        if let Some(value) = &organization_id {
            api_params.insert("organization_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &priority {
            api_params.insert("priority".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &status {
            api_params.insert("status".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &tax_included {
            api_params.insert("tax_included".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &valid_from {
            api_params.insert("valid_from".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &valid_until {
            api_params.insert("valid_until".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn prices_entries_list(&self, list_id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/prices/lists/{list_id}/entries".replace("{listId}", &list_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("list_id".to_string(), serde_json::to_value(&list_id)?);

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
    pub async fn prices_entries_create(&self, list_id: String, metadata: Option<serde_json::Value>, price_type: Option<String>, product_id: Option<String>, quantity_min: Option<f64>, sku: Option<String>, unit: Option<String>, unit_price: Option<f64>, valid_from: Option<String>, valid_until: Option<String>) -> Result<crate::models::PriceEntry, Error> {
        let api_path = "/v1/prices/lists/{list_id}/entries".replace("{listId}", &list_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("list_id".to_string(), serde_json::to_value(&list_id)?);
        if let Some(value) = &metadata {
            api_params.insert("metadata".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &price_type {
            api_params.insert("price_type".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &product_id {
            api_params.insert("product_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &quantity_min {
            api_params.insert("quantity_min".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &sku {
            api_params.insert("sku".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &unit {
            api_params.insert("unit".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &unit_price {
            api_params.insert("unit_price".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &valid_from {
            api_params.insert("valid_from".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &valid_until {
            api_params.insert("valid_until".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn prices_entries_replace(&self, list_id: String, entries: Vec<crate::models::PriceEntryReplaceItem>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/prices/lists/{list_id}/entries".replace("{listId}", &list_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("list_id".to_string(), serde_json::to_value(&list_id)?);
        api_params.insert("entries".to_string(), serde_json::to_value(&entries)?);

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
    pub async fn prices_entries_bulk(&self, list_id: String, entries: Vec<crate::models::PriceEntryReplaceItem>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/prices/lists/{list_id}/entries/bulk".replace("{listId}", &list_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("list_id".to_string(), serde_json::to_value(&list_id)?);
        api_params.insert("entries".to_string(), serde_json::to_value(&entries)?);

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
    pub async fn prices_entries_delete(&self, list_id: String, id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/prices/lists/{list_id}/entries/{id}".replace("{listId}", &list_id.to_string()).replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("list_id".to_string(), serde_json::to_value(&list_id)?);
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
    pub async fn prices_entries_get(&self, list_id: String, id: String) -> Result<crate::models::PriceEntry, Error> {
        let api_path = "/v1/prices/lists/{list_id}/entries/{id}".replace("{listId}", &list_id.to_string()).replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("list_id".to_string(), serde_json::to_value(&list_id)?);
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn prices_entries_update(&self, list_id: String, id: String, metadata: Option<serde_json::Value>, price_type: Option<String>, product_id: Option<String>, quantity_min: Option<f64>, sku: Option<String>, unit: Option<String>, unit_price: Option<f64>, valid_from: Option<String>, valid_until: Option<String>) -> Result<crate::models::PriceEntry, Error> {
        let api_path = "/v1/prices/lists/{list_id}/entries/{id}".replace("{listId}", &list_id.to_string()).replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("list_id".to_string(), serde_json::to_value(&list_id)?);
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &metadata {
            api_params.insert("metadata".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &price_type {
            api_params.insert("price_type".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &product_id {
            api_params.insert("product_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &quantity_min {
            api_params.insert("quantity_min".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &sku {
            api_params.insert("sku".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &unit {
            api_params.insert("unit".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &unit_price {
            api_params.insert("unit_price".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &valid_from {
            api_params.insert("valid_from".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &valid_until {
            api_params.insert("valid_until".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn prices_resolve(&self, items: Vec<crate::models::PriceResolveItem>, at: Option<String>, channel_id: Option<String>, contact_id: Option<String>, currency: Option<String>, market_id: Option<String>, organization_id: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/prices/resolve".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("items".to_string(), serde_json::to_value(&items)?);
        if let Some(value) = &at {
            api_params.insert("at".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &channel_id {
            api_params.insert("channel_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &contact_id {
            api_params.insert("contact_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &currency {
            api_params.insert("currency".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &market_id {
            api_params.insert("market_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &organization_id {
            api_params.insert("organization_id".to_string(), serde_json::to_value(value)?);
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
