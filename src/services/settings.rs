use std::collections::HashMap;

use serde_json::Value;

use crate::client::Client;
use crate::error::Error;
use crate::input_file::InputFile;

/// Settings service
pub struct Settings {
    client: Client,
}

impl Settings {
    pub fn new(client: Client) -> Self {
        Settings { client }
    }
    /// The tenant's effective settings for the app — the declared schema's
    /// defaults merged with stored tenant/market values. Sensitive settings are
    /// masked (listed in `masked`, omitted from `settings`).
    pub async fn settings_get_app_settings(&self, app: String, market: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/settings/apps/{app}".replace("{app}", &app.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("app".to_string(), serde_json::to_value(&app)?);
        if let Some(value) = &market {
            api_params.insert("market".to_string(), serde_json::to_value(value)?);
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
