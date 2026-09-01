use std::collections::HashMap;

use serde_json::Value;

use crate::client::Client;
use crate::error::Error;
use crate::input_file::InputFile;

/// ProductsAssets service
pub struct ProductsAssets {
    client: Client,
}

impl ProductsAssets {
    pub fn new(client: Client) -> Self {
        ProductsAssets { client }
    }
    /// One piece of media in the decoupled asset domain. The bytes live either in
    /// this platform's object store (`source: "storage"` with a `storage_asset_id`
    /// that survives a rename) or on somebody else's host (`source: "external"`
    /// with an `external_url`), and the database enforces the pair so neither half
    /// can be stored alone. A product points at an asset by its code through a
    /// media attribute; there is no product-to-asset link table in this app.
    /// 
    /// Every column of `assets` is an exact-match query parameter, `order` sorts
    /// by one column, and `limit`/`offset` page through `page.total`. A query key
    /// that is NOT a column is dropped rather than refused, and the `filter`
    /// object echoes the ones that were understood — that echo is the only way
    /// to tell an unfiltered answer from an empty one. It reads rows exactly as
    /// they are stored: no join is resolved, no jsonb value is unpacked.
    /// 
    /// Answered from the gateway's tenant cache for up to 30 minutes and dropped
    /// the moment this entity is written, because the data model changes weekly at
    /// most and every product page asks the same question.
    pub async fn products_assets_list(&self, limit: Option<i64>, offset: Option<i64>, order: Option<String>, id: Option<String>, asset_family_id: Option<String>, code: Option<String>, source: Option<String>, storage_asset_id: Option<String>, delivery_path: Option<String>, external_url: Option<String>, attribute_values: Option<String>, created_at: Option<String>, updated_at: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/products/assets".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &limit {
            api_params.insert("limit".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &offset {
            api_params.insert("offset".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &order {
            api_params.insert("order".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &id {
            api_params.insert("id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &asset_family_id {
            api_params.insert("asset_family_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &code {
            api_params.insert("code".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &source {
            api_params.insert("source".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &storage_asset_id {
            api_params.insert("storage_asset_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &delivery_path {
            api_params.insert("delivery_path".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &external_url {
            api_params.insert("external_url".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &attribute_values {
            api_params.insert("attribute_values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &created_at {
            api_params.insert("created_at".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &updated_at {
            api_params.insert("updated_at".to_string(), serde_json::to_value(value)?);
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
    /// Creates one asset and answers 201 with the stored row, including the id and
    /// the timestamps the database filled in — a client never sends an id, it
    /// reads one back and uses it in the path of every later call.
    /// 
    /// One piece of media in the decoupled asset domain. The bytes live either in
    /// this platform's object store (`source: "storage"` with a `storage_asset_id`
    /// that survives a rename) or on somebody else's host (`source: "external"`
    /// with an `external_url`), and the database enforces the pair so neither half
    /// can be stored alone. A product points at an asset by its code through a
    /// media attribute; there is no product-to-asset link table in this app.
    /// 
    /// `asset_family_id` and `code` are the only columns the database refuses the
    /// row without; everything else has a default or is nullable. A second row
    /// with the same `asset_family_id` and `code` answers 409. This app owns the
    /// create, because it is the only place an external URL can enter the catalog:
    /// an asset with no family falls back to the `default_asset_family` setting,
    /// and an `external` one is refused unless the tenant allows external media
    /// and the URL's host is on its allow-list.
    pub async fn products_assets_create(&self, asset_family_id: String, code: String, attribute_values: Option<serde_json::Value>, delivery_path: Option<String>, external_url: Option<String>, source: Option<String>, storage_asset_id: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/assets".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("asset_family_id".to_string(), serde_json::to_value(&asset_family_id)?);
        api_params.insert("code".to_string(), serde_json::to_value(&code)?);
        if let Some(value) = &attribute_values {
            api_params.insert("attribute_values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &delivery_path {
            api_params.insert("delivery_path".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &external_url {
            api_params.insert("external_url".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &source {
            api_params.insert("source".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &storage_asset_id {
            api_params.insert("storage_asset_id".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Deletes one asset by id. It is a hard delete — the row is gone, and the
    /// answer is a confirmation rather than a result to branch on.
    /// 
    /// Nothing in this schema references it, so nothing else changes.
    /// 
    /// An id no asset of this tenant carries answers 404; there is no 409, because
    /// every foreign key pointing at this entity resolves itself on delete rather
    /// than blocking one.
    pub async fn products_assets_delete(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/assets/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("DELETE", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Reads one asset by its id — the whole row, every column, as it is stored.
    /// 
    /// One piece of media in the decoupled asset domain. The bytes live either in
    /// this platform's object store (`source: "storage"` with a `storage_asset_id`
    /// that survives a rename) or on somebody else's host (`source: "external"`
    /// with an `external_url`), and the database enforces the pair so neither half
    /// can be stored alone. A product points at an asset by its code through a
    /// media attribute; there is no product-to-asset link table in this app.
    /// 
    /// An id no asset of this tenant carries answers 404, and so does one
    /// belonging to another tenant: row-level security makes that row invisible
    /// rather than forbidden. A malformed id answers 400 before the route is
    /// reached.
    /// 
    /// Answered from the gateway's tenant cache for up to 30 minutes and dropped
    /// the moment this entity is written, because the data model changes weekly at
    /// most and every product page asks the same question.
    pub async fn products_assets_get(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/assets/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Updates one asset by id. A partial patch: the body names only the columns
    /// to change and every column it leaves out keeps its current value, so there
    /// is no read-modify-write and no way to blank a field by forgetting it.
    /// 
    /// One piece of media in the decoupled asset domain. The bytes live either in
    /// this platform's object store (`source: "storage"` with a `storage_asset_id`
    /// that survives a rename) or on somebody else's host (`source: "external"`
    /// with an `external_url`), and the database enforces the pair so neither half
    /// can be stored alone. A product points at an asset by its code through a
    /// media attribute; there is no product-to-asset link table in this app.
    /// 
    /// A body that names nothing writable is refused with 400 rather than answered
    /// as a no-op, an id nobody carries answers 404, and a value that collides on
    /// `asset_family_id` and `code` answers 409.
    pub async fn products_assets_update(&self, id: String, asset_family_id: Option<String>, attribute_values: Option<serde_json::Value>, code: Option<String>, delivery_path: Option<String>, external_url: Option<String>, source: Option<String>, storage_asset_id: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/assets/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &asset_family_id {
            api_params.insert("asset_family_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &attribute_values {
            api_params.insert("attribute_values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &code {
            api_params.insert("code".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &delivery_path {
            api_params.insert("delivery_path".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &external_url {
            api_params.insert("external_url".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &source {
            api_params.insert("source".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &storage_asset_id {
            api_params.insert("storage_asset_id".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
}
