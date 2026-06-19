use std::collections::HashMap;

use serde_json::Value;

use crate::client::Client;
use crate::error::Error;
use crate::input_file::InputFile;

/// Tokens service
pub struct Tokens {
    client: Client,
}

impl Tokens {
    pub fn new(client: Client) -> Self {
        Tokens { client }
    }
    /// List all the tokens created for a specific file or bucket. You can use the
    /// query params to filter your results.
    pub async fn tokens_list(&self, bucket_id: String, file_id: String, queries: Option<Vec<String>>, total: Option<bool>) -> Result<crate::models::ResourceTokenList, Error> {
        let api_path = "/v1/tokens/buckets/{bucketId}/files/{fileId}".replace("{bucketId}", &bucket_id.to_string()).replace("{fileId}", &file_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("bucketId".to_string(), serde_json::to_value(&bucket_id)?);
        api_params.insert("fileId".to_string(), serde_json::to_value(&file_id)?);
        if let Some(value) = &queries {
            api_params.insert("queries".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &total {
            api_params.insert("total".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Create a new token. A token is linked to a file. Token can be passed as a
    /// request URL search parameter.
    pub async fn tokens_create_file_token(&self, bucket_id: String, file_id: String, expire: Option<String>) -> Result<crate::models::ResourceToken, Error> {
        let api_path = "/v1/tokens/buckets/{bucketId}/files/{fileId}".replace("{bucketId}", &bucket_id.to_string()).replace("{fileId}", &file_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("bucketId".to_string(), serde_json::to_value(&bucket_id)?);
        api_params.insert("fileId".to_string(), serde_json::to_value(&file_id)?);
        if let Some(value) = &expire {
            api_params.insert("expire".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Delete a token by its unique ID.
    pub async fn tokens_delete(&self, token_id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/tokens/{tokenId}".replace("{tokenId}", &token_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("tokenId".to_string(), serde_json::to_value(&token_id)?);

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
    /// Get a token by its unique ID.
    pub async fn tokens_get(&self, token_id: String) -> Result<crate::models::ResourceToken, Error> {
        let api_path = "/v1/tokens/{tokenId}".replace("{tokenId}", &token_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("tokenId".to_string(), serde_json::to_value(&token_id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Update a token by its unique ID. Use this endpoint to update a token's
    /// expiry date.
    pub async fn tokens_update(&self, token_id: String, expire: Option<String>) -> Result<crate::models::ResourceToken, Error> {
        let api_path = "/v1/tokens/{tokenId}".replace("{tokenId}", &token_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("tokenId".to_string(), serde_json::to_value(&token_id)?);
        if let Some(value) = &expire {
            api_params.insert("expire".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PATCH", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
}
