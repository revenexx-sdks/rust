use std::collections::HashMap;

use serde_json::Value;

use crate::client::Client;
use crate::error::Error;
use crate::input_file::InputFile;

/// Health service
pub struct Health {
    client: Client,
}

impl Health {
    pub fn new(client: Client) -> Self {
        Health { client }
    }
    /// Answers as long as the process is running. Never touches a dependency, so
    /// it stays 200 while the gateway is degraded — use readiness to decide
    /// whether to send traffic.
    pub async fn health_live(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/health/live".to_string();

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
    /// Answers 200 once the gateway's registry source is reachable, 503 until
    /// then.
    pub async fn health_ready(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/health/ready".to_string();

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
}
