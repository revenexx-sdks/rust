use std::collections::HashMap;

use serde_json::Value;

use crate::client::Client;
use crate::error::Error;
use crate::input_file::InputFile;

/// Search service
pub struct Search {
    client: Client,
}

impl Search {
    pub fn new(client: Client) -> Self {
        Search { client }
    }
    /// The collections the tenant's installed apps have provisioned.
    pub async fn search_list_collections(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/search/collections".to_string();

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
    /// Full-text search within one collection using Typesense query parameters as
    /// the query string.
    pub async fn search_search_documents_get(&self, collection: String, q: Option<String>, query_by: Option<String>, filter_by: Option<String>, sort_by: Option<String>, page: Option<i64>, per_page: Option<i64>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/search/collections/{collection}/documents/search".replace("{collection}", &collection.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("collection".to_string(), serde_json::to_value(&collection)?);
        if let Some(value) = &q {
            api_params.insert("q".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &query_by {
            api_params.insert("query_by".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &filter_by {
            api_params.insert("filter_by".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &sort_by {
            api_params.insert("sort_by".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &page {
            api_params.insert("page".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &per_page {
            api_params.insert("per_page".to_string(), serde_json::to_value(value)?);
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
    /// Full-text search within one collection. The body holds Typesense search
    /// parameters.
    pub async fn search_search_documents(&self, collection: String, facet_by: Option<String>, filter_by: Option<String>, page: Option<i64>, per_page: Option<i64>, q: Option<String>, query_by: Option<String>, sort_by: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/search/collections/{collection}/documents/search".replace("{collection}", &collection.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("collection".to_string(), serde_json::to_value(&collection)?);
        if let Some(value) = &facet_by {
            api_params.insert("facet_by".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &filter_by {
            api_params.insert("filter_by".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &page {
            api_params.insert("page".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &per_page {
            api_params.insert("per_page".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &q {
            api_params.insert("q".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &query_by {
            api_params.insert("query_by".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &sort_by {
            api_params.insert("sort_by".to_string(), serde_json::to_value(value)?);
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
    /// Fetch a single document by id from a collection the tenant has installed.
    pub async fn search_get_document(&self, collection: String, document_id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/search/collections/{collection}/documents/{documentId}".replace("{collection}", &collection.to_string()).replace("{documentId}", &document_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("collection".to_string(), serde_json::to_value(&collection)?);
        api_params.insert("documentId".to_string(), serde_json::to_value(&document_id)?);

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
    /// Run several searches in one request (the InstantSearch adapter uses this).
    /// Each entry names its collection.
    pub async fn search_multi_search(&self, searches: Vec<serde_json::Value>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/search/multi_search".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("searches".to_string(), serde_json::to_value(&searches)?);

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
