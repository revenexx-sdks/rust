use std::collections::HashMap;

use serde_json::Value;

use crate::client::Client;
use crate::error::Error;
use crate::input_file::InputFile;

/// Events service
pub struct Events {
    client: Client,
}

impl Events {
    pub fn new(client: Client) -> Self {
        Events { client }
    }
    /// Every event type this tenant's installed apps and platform services declare
    /// — what can be published and subscribed to, independent of whether one has
    /// fired yet. Each entry says what causes it (`trigger`) and what it carries
    /// (`sample`, `data_schema`).
    pub async fn events_get_catalog(&self, fields: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/events/catalog".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &fields {
            api_params.insert("fields".to_string(), serde_json::to_value(value)?);
        }

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
}
