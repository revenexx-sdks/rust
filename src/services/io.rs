use std::collections::HashMap;

use serde_json::Value;

use crate::client::Client;
use crate::error::Error;
use crate::input_file::InputFile;

/// Io service
pub struct Io {
    client: Client,
}

impl Io {
    pub fn new(client: Client) -> Self {
        Io { client }
    }
    /// The calling tenant's bulk jobs, newest first. Jobs are created by the
    /// feature blocks (import / export / A/B swap / tenant copy / sample) —
    /// never here; this surface is read-only.
    pub async fn list_bulk_jobs(&self, xtype: Option<serde_json::Value>, status: Option<serde_json::Value>, vendor: Option<String>, app: Option<String>, entity: Option<String>, limit: Option<i64>) -> Result<crate::models::ValidationFailedResponse, Error> {
        let api_path = "/v1/io/bulk-jobs".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &xtype {
            api_params.insert("type".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &status {
            api_params.insert("status".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &vendor {
            api_params.insert("vendor".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &app {
            api_params.insert("app".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &entity {
            api_params.insert("entity".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &limit {
            api_params.insert("limit".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Status, row counts, and progress for one bulk job.
    /// 
    /// Tenant-scoped: an id belonging to another tenant is filtered out and
    /// is therefore indistinguishable from a non-existent one — which is the
    /// intent.
    pub async fn get_bulk_job(&self, id: String) -> Result<crate::models::ValidationFailedResponse, Error> {
        let api_path = "/v1/io/bulk-jobs/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Flat list of the entities the calling tenant's installed apps expose,
    /// sorted by vendor, app, entity. Feeds the entity pickers of the
    /// Integration Studio I/O nodes.
    /// 
    /// The app set comes from `baseline.tenant_app_versions`. Per app the
    /// entity list is resolved from the tenant's pinned schema version; when
    /// that pointer is stale (missing or not applied) it falls back to the
    /// latest applied version of `(vendor, app)`. Apps with no applied
    /// schema at all contribute no entities.
    pub async fn list_io_entities(&self) -> Result<crate::models::ValidationFailedResponse, Error> {
        let api_path = "/v1/io/entities".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Creates a `bulk_job` and dispatches the engine to export the tenant's
    /// rows for an entity. CSV/XML stream row-by-row into an S3 multipart
    /// upload (flat RAM); JSON/XLSX are buffered. The response carries the
    /// object key the result will be written to.
    pub async fn create_export(&self, app: String, entity: String, vendor: String, format: Option<String>, profile_id: Option<String>) -> Result<crate::models::ValidationFailedResponse, Error> {
        let api_path = "/v1/io/exports".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("app".to_string(), serde_json::to_value(&app)?);
        api_params.insert("entity".to_string(), serde_json::to_value(&entity)?);
        api_params.insert("vendor".to_string(), serde_json::to_value(&vendor)?);
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

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Mints a short-TTL signed S3 `GET` URL for the object a completed
    /// export wrote. Tenant-scoped: an id belonging to another tenant — or
    /// to a job that is not an export — is indistinguishable from a
    /// non-existent one and answers `404`.
    /// 
    /// The job must have reached `completed` or `partial`; any earlier
    /// state answers `409` and carries the current `job_status`.
    pub async fn get_export_url(&self, id: String) -> Result<crate::models::ValidationFailedResponse, Error> {
        let api_path = "/v1/io/exports/{id}/url".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Creates a `bulk_job` and dispatches the engine to import a previously
    /// uploaded object into the named entity. The engine streams CSV
    /// row-by-row (flat RAM at 1M+ rows) and COPYs into the entity's staging
    /// sibling before a merge / content-hash delta into the target.
    pub async fn create_import(&self, app: String, entity: String, object_key: String, vendor: String, format: Option<String>, keys: Option<Vec<String>>, max_rejects: Option<i64>, mode: Option<String>, profile_id: Option<String>, target: Option<String>) -> Result<crate::models::ValidationFailedResponse, Error> {
        let api_path = "/v1/io/imports".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("app".to_string(), serde_json::to_value(&app)?);
        api_params.insert("entity".to_string(), serde_json::to_value(&entity)?);
        api_params.insert("object_key".to_string(), serde_json::to_value(&object_key)?);
        api_params.insert("vendor".to_string(), serde_json::to_value(&vendor)?);
        if let Some(value) = &format {
            api_params.insert("format".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &keys {
            api_params.insert("keys".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &max_rejects {
            api_params.insert("max_rejects".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &mode {
            api_params.insert("mode".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &profile_id {
            api_params.insert("profile_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &target {
            api_params.insert("target".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// The calling tenant's saved profiles, ordered by name.
    /// 
    /// When `X-Revenexx-Market` is present the listing is filtered to the
    /// profiles offered for that market — global profiles (`markets: null`)
    /// plus those whose `markets` contain it. Omit the header to get every
    /// profile, which is what the management view wants.
    pub async fn list_profiles(&self) -> Result<crate::models::ValidationFailedResponse, Error> {
        let api_path = "/v1/io/profiles".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// A tenant-secured, reusable mapping (field rename + transforms + keys)
    /// for a direction (`import`/`export`), format, and entity. Runnable
    /// on-click via `/io/profiles/{id}/run`.
    pub async fn create_profile(&self, app: String, direction: String, entity: String, format: String, name: String, vendor: String, apply_mode: Option<String>, mapping: Option<serde_json::Value>, markets: Option<Vec<String>>, options: Option<serde_json::Value>) -> Result<crate::models::ValidationFailedResponse, Error> {
        let api_path = "/v1/io/profiles".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("app".to_string(), serde_json::to_value(&app)?);
        api_params.insert("direction".to_string(), serde_json::to_value(&direction)?);
        api_params.insert("entity".to_string(), serde_json::to_value(&entity)?);
        api_params.insert("format".to_string(), serde_json::to_value(&format)?);
        api_params.insert("name".to_string(), serde_json::to_value(&name)?);
        api_params.insert("vendor".to_string(), serde_json::to_value(&vendor)?);
        if let Some(value) = &apply_mode {
            api_params.insert("apply_mode".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &mapping {
            api_params.insert("mapping".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &markets {
            api_params.insert("markets".to_string(), serde_json::to_value(value)?);
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
    /// Permanently remove a saved profile owned by the calling tenant.
    /// 
    /// Idempotent, and deliberately not a `404` path: deleting an id that
    /// does not belong to the tenant still answers `200`, with `deleted: 0`.
    pub async fn delete_profile(&self, id: String) -> Result<crate::models::ValidationFailedResponse, Error> {
        let api_path = "/v1/io/profiles/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("DELETE", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// A single saved profile. Tenant-scoped: an id owned by another tenant
    /// is indistinguishable from a non-existent one and answers `404`.
    pub async fn show_profile(&self, id: String) -> Result<crate::models::ValidationFailedResponse, Error> {
        let api_path = "/v1/io/profiles/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Replace a saved profile's mapping, format, or apply mode (tenant-scoped).
    pub async fn update_profile(&self, id: String, app: String, direction: String, entity: String, format: String, name: String, vendor: String, apply_mode: Option<String>, mapping: Option<serde_json::Value>, markets: Option<Vec<String>>, options: Option<serde_json::Value>) -> Result<crate::models::ValidationFailedResponse, Error> {
        let api_path = "/v1/io/profiles/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        api_params.insert("app".to_string(), serde_json::to_value(&app)?);
        api_params.insert("direction".to_string(), serde_json::to_value(&direction)?);
        api_params.insert("entity".to_string(), serde_json::to_value(&entity)?);
        api_params.insert("format".to_string(), serde_json::to_value(&format)?);
        api_params.insert("name".to_string(), serde_json::to_value(&name)?);
        api_params.insert("vendor".to_string(), serde_json::to_value(&vendor)?);
        if let Some(value) = &apply_mode {
            api_params.insert("apply_mode".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &mapping {
            api_params.insert("mapping".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &markets {
            api_params.insert("markets".to_string(), serde_json::to_value(value)?);
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
    /// Dispatches the engine using the saved profile. An import run requires
    /// `object_key` (upload first); an export run writes a generated key.
    pub async fn run_profile(&self, id: String, markets: Option<Vec<String>>, object_key: Option<String>) -> Result<crate::models::ValidationFailedResponse, Error> {
        let api_path = "/v1/io/profiles/{id}/run".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &markets {
            api_params.insert("markets".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &object_key {
            api_params.insert("object_key".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Returns a short-lived signed S3 `PUT` URL (+ required headers) and
    /// the `object_key` to reference in a subsequent `/io/imports`. The
    /// client uploads bytes directly to object storage — never through
    /// Baseline.
    pub async fn create_upload(&self, extension: Option<String>) -> Result<crate::models::ValidationFailedResponse, Error> {
        let api_path = "/v1/io/uploads".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &extension {
            api_params.insert("extension".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
}
