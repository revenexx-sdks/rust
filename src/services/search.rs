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
    /// The collections the tenant's installed apps have provisioned. Available on
    /// the API-gateway-trust path only — a `revx_` key authorises a single
    /// collection, so discovery is a gateway concern and a key-authenticated
    /// caller gets 403.
    pub async fn search_list_collections(&self) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/search/collections".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Returns the Typesense collection definition (fields, defaults, document
    /// count). Requires the `collections:read` action.
    pub async fn search_get_collection(&self, collection: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/search/collections/{collection}".replace("{collection}", &collection.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("collection".to_string(), serde_json::to_value(&collection)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Full-text search within one collection. Typesense search parameters are
    /// passed through verbatim as the query string, so parameters not listed here
    /// still reach Typesense. Requires the `documents:search` action.
    pub async fn search_search_documents_get(&self, collection: String, q: Option<String>, query_by: Option<String>, filter_by: Option<String>, sort_by: Option<String>, facet_by: Option<String>, max_facet_values: Option<i64>, group_by: Option<String>, include_fields: Option<String>, exclude_fields: Option<String>, highlight_full_fields: Option<String>, num_typos: Option<i64>, prefix: Option<String>, page: Option<i64>, per_page: Option<i64>) -> Result<crate::models::Error, Error> {
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
        if let Some(value) = &facet_by {
            api_params.insert("facet_by".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &max_facet_values {
            api_params.insert("max_facet_values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &group_by {
            api_params.insert("group_by".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &include_fields {
            api_params.insert("include_fields".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &exclude_fields {
            api_params.insert("exclude_fields".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &highlight_full_fields {
            api_params.insert("highlight_full_fields".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &num_typos {
            api_params.insert("num_typos".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &prefix {
            api_params.insert("prefix".to_string(), serde_json::to_value(value)?);
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

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Full-text search within one collection, with the Typesense search
    /// parameters in the body. Requires the `documents:search` action.
    pub async fn search_search_documents(&self, collection: String, exclude_fields: Option<String>, facet_by: Option<String>, filter_by: Option<String>, group_by: Option<String>, highlight_full_fields: Option<String>, include_fields: Option<String>, max_facet_values: Option<i64>, num_typos: Option<i64>, page: Option<i64>, per_page: Option<i64>, prefix: Option<String>, q: Option<String>, query_by: Option<String>, sort_by: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/search/collections/{collection}/documents/search".replace("{collection}", &collection.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("collection".to_string(), serde_json::to_value(&collection)?);
        if let Some(value) = &exclude_fields {
            api_params.insert("exclude_fields".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &facet_by {
            api_params.insert("facet_by".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &filter_by {
            api_params.insert("filter_by".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &group_by {
            api_params.insert("group_by".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &highlight_full_fields {
            api_params.insert("highlight_full_fields".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &include_fields {
            api_params.insert("include_fields".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &max_facet_values {
            api_params.insert("max_facet_values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &num_typos {
            api_params.insert("num_typos".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &page {
            api_params.insert("page".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &per_page {
            api_params.insert("per_page".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &prefix {
            api_params.insert("prefix".to_string(), serde_json::to_value(value)?);
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

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Fetch a single document by id. The document shape is the collection's own
    /// schema, so it is described as a free-form object. Requires the
    /// `documents:get` action.
    pub async fn search_get_document(&self, collection: String, document_id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/search/collections/{collection}/documents/{documentId}".replace("{collection}", &collection.to_string()).replace("{documentId}", &document_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("collection".to_string(), serde_json::to_value(&collection)?);
        api_params.insert("documentId".to_string(), serde_json::to_value(&document_id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Idempotent, and bounded by the tenant's own configuration: it can add
    /// no field for an attribute the tenant has not marked `is_filterable`,
    /// and drops only fields whose attribute it has itself un-marked. A run
    /// that changes nothing makes zero calls to Typesense.
    /// 
    /// Body (optional) narrows the sweep to one app:
    /// 
    /// {"vendor": "revenexx", "app": "products"}
    /// 
    /// Omitted, every app the tenant has installed is swept. Apps outside the
    /// facet-sync allowlist are included in the response with
    /// `skipped: app_not_enabled` rather than silently dropped — a caller
    /// asking for an app that cannot have facets deserves to be told so.
    /// 
    /// The response shape below is DECLARED rather than inferred. Its entries
    /// are built by spreading AttributeFacetSyncer::syncForCollection()'s
    /// summary, and the generator cannot see through an array spread: left to
    /// itself it emits an unnamed property and a null in `required`, which
    /// Spectral rejects as `"1" property must be string`.
    /// AppController::resyncFacets() carries the same declaration for the same
    /// reason — keep both in step with syncForApp()'s return type.
    pub async fn gateway_facet_resync(&self, app: Option<String>, vendor: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/search/facets/resync".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &app {
            api_params.insert("app".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &vendor {
            api_params.insert("vendor".to_string(), serde_json::to_value(value)?);
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
    /// Run several searches in one round trip — the endpoint the typesense-js
    /// `multiSearch` helper and the InstantSearch adapter use for every query. On
    /// the gateway-trust path each entry must name a collection the tenant owns.
    /// With a `revx_` key `collection_name` is optional and is forced to the key's
    /// own collection. Requires the `documents:search` action.
    pub async fn search_multi_search(&self, searches: Vec<crate::models::MultiSearchEntry>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/search/multi_search".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("searches".to_string(), serde_json::to_value(&searches)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
}
