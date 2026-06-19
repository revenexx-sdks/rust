use std::collections::HashMap;

use serde_json::Value;

use crate::client::Client;
use crate::error::Error;
use crate::input_file::InputFile;

/// Storage service
pub struct Storage {
    client: Client,
}

impl Storage {
    pub fn new(client: Client) -> Self {
        Storage { client }
    }
    pub async fn asset_index(&self, search: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/storage/assets".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &search {
            api_params.insert("search".to_string(), serde_json::to_value(value)?);
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
    pub async fn asset_store(&self, file: String, alt_text: Option<String>, description: Option<String>, display_name: Option<String>, folder_id: Option<String>, keep_archive: Option<bool>, tags: Option<Vec<String>>, unpack: Option<bool>, visibility: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/storage/assets".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("file".to_string(), serde_json::to_value(&file)?);
        if let Some(value) = &alt_text {
            api_params.insert("alt_text".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &description {
            api_params.insert("description".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &display_name {
            api_params.insert("display_name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &folder_id {
            api_params.insert("folder_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &keep_archive {
            api_params.insert("keep_archive".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &tags {
            api_params.insert("tags".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &unpack {
            api_params.insert("unpack".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &visibility {
            api_params.insert("visibility".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "multipart/form-data".to_string());

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
    pub async fn asset_bulk(&self, folder_id: Option<String>, visibility: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/storage/assets/bulk".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &folder_id {
            api_params.insert("folder_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &visibility {
            api_params.insert("visibility".to_string(), serde_json::to_value(value)?);
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
    pub async fn asset_destroy(&self, id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/storage/assets/{id}".replace("{id}", &id.to_string());

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
    pub async fn asset_show(&self, id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/storage/assets/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

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
    pub async fn asset_update(&self, id: String, alt_text: Option<String>, description: Option<String>, display_name: Option<String>, folder_id: Option<String>, name: Option<String>, tags: Option<Vec<String>>, visibility: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/storage/assets/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &alt_text {
            api_params.insert("alt_text".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &description {
            api_params.insert("description".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &display_name {
            api_params.insert("display_name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &folder_id {
            api_params.insert("folder_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &name {
            api_params.insert("name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &tags {
            api_params.insert("tags".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &visibility {
            api_params.insert("visibility".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PATCH", &api_path, api_headers, api_params)
            .await?;

        if api_response.body.is_empty() {
            Ok(serde_json::Value::Null)
        } else {
            Ok(serde_json::from_slice(&api_response.body)?)
        }
    }
    pub async fn asset_download(&self, id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/storage/assets/{id}/download".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

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
    pub async fn asset_permanent(&self, id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/storage/assets/{id}/permanent".replace("{id}", &id.to_string());

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
    pub async fn asset_reprocess(&self, id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/storage/assets/{id}/reprocess".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

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
    pub async fn asset_restore(&self, id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/storage/assets/{id}/restore".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

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
    pub async fn asset_sign(&self, id: String, ttl_seconds: Option<i64>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/storage/assets/{id}/sign".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &ttl_seconds {
            api_params.insert("ttl_seconds".to_string(), serde_json::to_value(value)?);
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
    pub async fn asset_unpack(&self, id: String, keep_archive: Option<bool>, target_folder_id: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/storage/assets/{id}/unpack".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &keep_archive {
            api_params.insert("keep_archive".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &target_folder_id {
            api_params.insert("target_folder_id".to_string(), serde_json::to_value(value)?);
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
    pub async fn folder_index(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/storage/folders".to_string();

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
    pub async fn folder_store(&self, name: String, parent_id: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/storage/folders".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("name".to_string(), serde_json::to_value(&name)?);
        if let Some(value) = &parent_id {
            api_params.insert("parent_id".to_string(), serde_json::to_value(value)?);
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
    pub async fn folder_destroy(&self, id: String, recursive: Option<bool>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/storage/folders/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &recursive {
            api_params.insert("recursive".to_string(), serde_json::to_value(value)?);
        }

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
    pub async fn folder_show(&self, id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/storage/folders/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

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
    pub async fn folder_update(&self, id: String, name: Option<String>, parent_id: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/storage/folders/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &name {
            api_params.insert("name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &parent_id {
            api_params.insert("parent_id".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PATCH", &api_path, api_headers, api_params)
            .await?;

        if api_response.body.is_empty() {
            Ok(serde_json::Value::Null)
        } else {
            Ok(serde_json::from_slice(&api_response.body)?)
        }
    }
    pub async fn sync_rule_index(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/storage/sftp/rules".to_string();

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
    pub async fn sync_rule_store(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/storage/sftp/rules".to_string();

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
    pub async fn sync_rule_destroy(&self, id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/storage/sftp/rules/{id}".replace("{id}", &id.to_string());

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
    pub async fn sync_rule_show(&self, id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/storage/sftp/rules/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

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
    pub async fn sync_rule_update(&self, id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/storage/sftp/rules/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("PATCH", &api_path, api_headers, api_params)
            .await?;

        if api_response.body.is_empty() {
            Ok(serde_json::Value::Null)
        } else {
            Ok(serde_json::from_slice(&api_response.body)?)
        }
    }
    pub async fn sync_rule_run(&self, id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/storage/sftp/rules/{id}/run".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

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
    pub async fn sync_rule_run_protocol(&self, id: String, run_id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/storage/sftp/rules/{id}/runs/{runId}".replace("{id}", &id.to_string()).replace("{runId}", &run_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        api_params.insert("runId".to_string(), serde_json::to_value(&run_id)?);

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
    pub async fn sync_rule_history(&self, rule_id: Option<String>, from: Option<String>, to: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/storage/sftp/sync-history".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &rule_id {
            api_params.insert("rule_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &from {
            api_params.insert("from".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &to {
            api_params.insert("to".to_string(), serde_json::to_value(value)?);
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
    pub async fn tenant_stats(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/storage/tenant/stats".to_string();

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
    pub async fn tenant_usage(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/storage/tenant/usage".to_string();

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
