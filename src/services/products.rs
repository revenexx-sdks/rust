use std::collections::HashMap;

use serde_json::Value;

use crate::client::Client;
use crate::error::Error;
use crate::input_file::InputFile;

/// Products service
pub struct Products {
    client: Client,
}

impl Products {
    pub fn new(client: Client) -> Self {
        Products { client }
    }
    pub async fn products_list(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/products".to_string();

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
    pub async fn products_create(&self, sku: String, attribute_values: Option<serde_json::Value>, completeness: Option<serde_json::Value>, deleted_at: Option<String>, enabled: Option<bool>, family_id: Option<String>, family_variant_id: Option<String>, kind: Option<String>, parent_id: Option<String>, quantified_associations: Option<serde_json::Value>, tax_class: Option<String>) -> Result<crate::models::Products, Error> {
        let api_path = "/v1/products".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("sku".to_string(), serde_json::to_value(&sku)?);
        if let Some(value) = &attribute_values {
            api_params.insert("attribute_values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &completeness {
            api_params.insert("completeness".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &deleted_at {
            api_params.insert("deleted_at".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &enabled {
            api_params.insert("enabled".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &family_id {
            api_params.insert("family_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &family_variant_id {
            api_params.insert("family_variant_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &kind {
            api_params.insert("kind".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &parent_id {
            api_params.insert("parent_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &quantified_associations {
            api_params.insert("quantified_associations".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &tax_class {
            api_params.insert("tax_class".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn products_asset_families_list(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/products/asset_families".to_string();

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
    pub async fn products_asset_families_create(&self, code: String, labels: Option<serde_json::Value>, naming_convention: Option<serde_json::Value>) -> Result<crate::models::AssetFamilies, Error> {
        let api_path = "/v1/products/asset_families".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("code".to_string(), serde_json::to_value(&code)?);
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &naming_convention {
            api_params.insert("naming_convention".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn products_asset_families_delete(&self, id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/products/asset_families/{id}".replace("{id}", &id.to_string());

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
    pub async fn products_asset_families_get(&self, id: String) -> Result<crate::models::AssetFamilies, Error> {
        let api_path = "/v1/products/asset_families/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn products_asset_families_update(&self, id: String, code: Option<String>, labels: Option<serde_json::Value>, naming_convention: Option<serde_json::Value>) -> Result<crate::models::AssetFamilies, Error> {
        let api_path = "/v1/products/asset_families/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &code {
            api_params.insert("code".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &naming_convention {
            api_params.insert("naming_convention".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn products_assets_list(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/products/assets".to_string();

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
    pub async fn products_assets_create(&self, asset_family_id: String, code: String, attribute_values: Option<serde_json::Value>, media_uuid: Option<String>) -> Result<crate::models::Assets, Error> {
        let api_path = "/v1/products/assets".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("asset_family_id".to_string(), serde_json::to_value(&asset_family_id)?);
        api_params.insert("code".to_string(), serde_json::to_value(&code)?);
        if let Some(value) = &attribute_values {
            api_params.insert("attribute_values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &media_uuid {
            api_params.insert("media_uuid".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn products_assets_delete(&self, id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/products/assets/{id}".replace("{id}", &id.to_string());

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
    pub async fn products_assets_get(&self, id: String) -> Result<crate::models::Assets, Error> {
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
    pub async fn products_assets_update(&self, id: String, asset_family_id: Option<String>, attribute_values: Option<serde_json::Value>, code: Option<String>, media_uuid: Option<String>) -> Result<crate::models::Assets, Error> {
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
        if let Some(value) = &media_uuid {
            api_params.insert("media_uuid".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn products_association_types_list(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/products/association_types".to_string();

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
    pub async fn products_association_types_create(&self, code: String, is_quantified: Option<bool>, is_two_way: Option<bool>, labels: Option<serde_json::Value>) -> Result<crate::models::AssociationTypes, Error> {
        let api_path = "/v1/products/association_types".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("code".to_string(), serde_json::to_value(&code)?);
        if let Some(value) = &is_quantified {
            api_params.insert("is_quantified".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &is_two_way {
            api_params.insert("is_two_way".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn products_association_types_delete(&self, id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/products/association_types/{id}".replace("{id}", &id.to_string());

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
    pub async fn products_association_types_get(&self, id: String) -> Result<crate::models::AssociationTypes, Error> {
        let api_path = "/v1/products/association_types/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn products_association_types_update(&self, id: String, code: Option<String>, is_quantified: Option<bool>, is_two_way: Option<bool>, labels: Option<serde_json::Value>) -> Result<crate::models::AssociationTypes, Error> {
        let api_path = "/v1/products/association_types/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &code {
            api_params.insert("code".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &is_quantified {
            api_params.insert("is_quantified".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &is_two_way {
            api_params.insert("is_two_way".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn products_attribute_groups_list(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/products/attribute_groups".to_string();

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
    pub async fn products_attribute_groups_create(&self, code: String, labels: Option<serde_json::Value>, position: Option<i64>) -> Result<crate::models::AttributeGroups, Error> {
        let api_path = "/v1/products/attribute_groups".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("code".to_string(), serde_json::to_value(&code)?);
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn products_attribute_groups_delete(&self, id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/products/attribute_groups/{id}".replace("{id}", &id.to_string());

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
    pub async fn products_attribute_groups_get(&self, id: String) -> Result<crate::models::AttributeGroups, Error> {
        let api_path = "/v1/products/attribute_groups/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn products_attribute_groups_update(&self, id: String, code: Option<String>, labels: Option<serde_json::Value>, position: Option<i64>) -> Result<crate::models::AttributeGroups, Error> {
        let api_path = "/v1/products/attribute_groups/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &code {
            api_params.insert("code".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn products_attribute_options_list(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/products/attribute_options".to_string();

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
    pub async fn products_attribute_options_create(&self, attribute_id: String, code: String, labels: Option<serde_json::Value>, position: Option<i64>, swatch: Option<serde_json::Value>) -> Result<crate::models::AttributeOptions, Error> {
        let api_path = "/v1/products/attribute_options".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("attribute_id".to_string(), serde_json::to_value(&attribute_id)?);
        api_params.insert("code".to_string(), serde_json::to_value(&code)?);
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &swatch {
            api_params.insert("swatch".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn products_attribute_options_delete(&self, id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/products/attribute_options/{id}".replace("{id}", &id.to_string());

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
    pub async fn products_attribute_options_get(&self, id: String) -> Result<crate::models::AttributeOptions, Error> {
        let api_path = "/v1/products/attribute_options/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn products_attribute_options_update(&self, id: String, attribute_id: Option<String>, code: Option<String>, labels: Option<serde_json::Value>, position: Option<i64>, swatch: Option<serde_json::Value>) -> Result<crate::models::AttributeOptions, Error> {
        let api_path = "/v1/products/attribute_options/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &attribute_id {
            api_params.insert("attribute_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &code {
            api_params.insert("code".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &swatch {
            api_params.insert("swatch".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn products_attributes_list(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/products/attributes".to_string();

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
    pub async fn products_attributes_create(&self, code: String, xtype: String, config: Option<serde_json::Value>, entity_ref: Option<String>, entity_type: Option<String>, group_id: Option<String>, is_filterable: Option<bool>, is_unique: Option<bool>, labels: Option<serde_json::Value>, localizable: Option<bool>, position: Option<i64>, scopable: Option<bool>, usable_in_grid: Option<bool>, validation: Option<serde_json::Value>) -> Result<crate::models::Attributes, Error> {
        let api_path = "/v1/products/attributes".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("code".to_string(), serde_json::to_value(&code)?);
        api_params.insert("type".to_string(), serde_json::to_value(&xtype)?);
        if let Some(value) = &config {
            api_params.insert("config".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &entity_ref {
            api_params.insert("entity_ref".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &entity_type {
            api_params.insert("entity_type".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &group_id {
            api_params.insert("group_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &is_filterable {
            api_params.insert("is_filterable".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &is_unique {
            api_params.insert("is_unique".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &localizable {
            api_params.insert("localizable".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &scopable {
            api_params.insert("scopable".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &usable_in_grid {
            api_params.insert("usable_in_grid".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &validation {
            api_params.insert("validation".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn products_attributes_delete(&self, id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/products/attributes/{id}".replace("{id}", &id.to_string());

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
    pub async fn products_attributes_get(&self, id: String) -> Result<crate::models::Attributes, Error> {
        let api_path = "/v1/products/attributes/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn products_attributes_update(&self, id: String, code: Option<String>, config: Option<serde_json::Value>, entity_ref: Option<String>, entity_type: Option<String>, group_id: Option<String>, is_filterable: Option<bool>, is_unique: Option<bool>, labels: Option<serde_json::Value>, localizable: Option<bool>, position: Option<i64>, scopable: Option<bool>, xtype: Option<String>, usable_in_grid: Option<bool>, validation: Option<serde_json::Value>) -> Result<crate::models::Attributes, Error> {
        let api_path = "/v1/products/attributes/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &code {
            api_params.insert("code".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &config {
            api_params.insert("config".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &entity_ref {
            api_params.insert("entity_ref".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &entity_type {
            api_params.insert("entity_type".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &group_id {
            api_params.insert("group_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &is_filterable {
            api_params.insert("is_filterable".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &is_unique {
            api_params.insert("is_unique".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &localizable {
            api_params.insert("localizable".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &scopable {
            api_params.insert("scopable".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &xtype {
            api_params.insert("type".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &usable_in_grid {
            api_params.insert("usable_in_grid".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &validation {
            api_params.insert("validation".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn products_batch(&self, ids: Option<Vec<String>>, skus: Option<Vec<String>>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/products/batch".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &ids {
            api_params.insert("ids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &skus {
            api_params.insert("skus".to_string(), serde_json::to_value(value)?);
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
    pub async fn products_categories_list(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/products/categories".to_string();

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
    pub async fn products_categories_create(&self, code: String, labels: Option<serde_json::Value>, parent_id: Option<String>, path: Option<String>, position: Option<i64>, values: Option<serde_json::Value>) -> Result<crate::models::Categories, Error> {
        let api_path = "/v1/products/categories".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("code".to_string(), serde_json::to_value(&code)?);
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &parent_id {
            api_params.insert("parent_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &path {
            api_params.insert("path".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &values {
            api_params.insert("values".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn products_categories_delete(&self, id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/products/categories/{id}".replace("{id}", &id.to_string());

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
    pub async fn products_categories_get(&self, id: String) -> Result<crate::models::Categories, Error> {
        let api_path = "/v1/products/categories/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn products_categories_update(&self, id: String, code: Option<String>, labels: Option<serde_json::Value>, parent_id: Option<String>, path: Option<String>, position: Option<i64>, values: Option<serde_json::Value>) -> Result<crate::models::Categories, Error> {
        let api_path = "/v1/products/categories/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &code {
            api_params.insert("code".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &parent_id {
            api_params.insert("parent_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &path {
            api_params.insert("path".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &values {
            api_params.insert("values".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn products_families_list(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/products/families".to_string();

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
    pub async fn products_families_create(&self, code: String, image_attribute: Option<String>, label_attribute: Option<String>, labels: Option<serde_json::Value>) -> Result<crate::models::Families, Error> {
        let api_path = "/v1/products/families".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("code".to_string(), serde_json::to_value(&code)?);
        if let Some(value) = &image_attribute {
            api_params.insert("image_attribute".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &label_attribute {
            api_params.insert("label_attribute".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn products_families_delete(&self, id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/products/families/{id}".replace("{id}", &id.to_string());

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
    pub async fn products_families_get(&self, id: String) -> Result<crate::models::Families, Error> {
        let api_path = "/v1/products/families/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn products_families_update(&self, id: String, code: Option<String>, image_attribute: Option<String>, label_attribute: Option<String>, labels: Option<serde_json::Value>) -> Result<crate::models::Families, Error> {
        let api_path = "/v1/products/families/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &code {
            api_params.insert("code".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &image_attribute {
            api_params.insert("image_attribute".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &label_attribute {
            api_params.insert("label_attribute".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn products_family_attributes_list(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/products/family_attributes".to_string();

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
    pub async fn products_family_attributes_create(&self, attribute_id: String, family_id: String, is_required: Option<bool>, position: Option<i64>, required_channels: Option<serde_json::Value>) -> Result<crate::models::FamilyAttributes, Error> {
        let api_path = "/v1/products/family_attributes".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("attribute_id".to_string(), serde_json::to_value(&attribute_id)?);
        api_params.insert("family_id".to_string(), serde_json::to_value(&family_id)?);
        if let Some(value) = &is_required {
            api_params.insert("is_required".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &required_channels {
            api_params.insert("required_channels".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn products_family_attributes_delete(&self, id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/products/family_attributes/{id}".replace("{id}", &id.to_string());

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
    pub async fn products_family_attributes_get(&self, id: String) -> Result<crate::models::FamilyAttributes, Error> {
        let api_path = "/v1/products/family_attributes/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn products_family_attributes_update(&self, id: String, attribute_id: Option<String>, family_id: Option<String>, is_required: Option<bool>, position: Option<i64>, required_channels: Option<serde_json::Value>) -> Result<crate::models::FamilyAttributes, Error> {
        let api_path = "/v1/products/family_attributes/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &attribute_id {
            api_params.insert("attribute_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &family_id {
            api_params.insert("family_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &is_required {
            api_params.insert("is_required".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &required_channels {
            api_params.insert("required_channels".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn products_family_variants_list(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/products/family_variants".to_string();

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
    pub async fn products_family_variants_create(&self, code: String, family_id: String, axes: Option<serde_json::Value>, labels: Option<serde_json::Value>) -> Result<crate::models::FamilyVariants, Error> {
        let api_path = "/v1/products/family_variants".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("code".to_string(), serde_json::to_value(&code)?);
        api_params.insert("family_id".to_string(), serde_json::to_value(&family_id)?);
        if let Some(value) = &axes {
            api_params.insert("axes".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn products_family_variants_delete(&self, id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/products/family_variants/{id}".replace("{id}", &id.to_string());

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
    pub async fn products_family_variants_get(&self, id: String) -> Result<crate::models::FamilyVariants, Error> {
        let api_path = "/v1/products/family_variants/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn products_family_variants_update(&self, id: String, axes: Option<serde_json::Value>, code: Option<String>, family_id: Option<String>, labels: Option<serde_json::Value>) -> Result<crate::models::FamilyVariants, Error> {
        let api_path = "/v1/products/family_variants/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &axes {
            api_params.insert("axes".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &code {
            api_params.insert("code".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &family_id {
            api_params.insert("family_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn products_measurement_families_list(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/products/measurement_families".to_string();

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
    pub async fn products_measurement_families_create(&self, code: String, standard_unit: String, labels: Option<serde_json::Value>, units: Option<serde_json::Value>) -> Result<crate::models::MeasurementFamilies, Error> {
        let api_path = "/v1/products/measurement_families".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("code".to_string(), serde_json::to_value(&code)?);
        api_params.insert("standard_unit".to_string(), serde_json::to_value(&standard_unit)?);
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &units {
            api_params.insert("units".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn products_measurement_families_delete(&self, id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/products/measurement_families/{id}".replace("{id}", &id.to_string());

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
    pub async fn products_measurement_families_get(&self, id: String) -> Result<crate::models::MeasurementFamilies, Error> {
        let api_path = "/v1/products/measurement_families/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn products_measurement_families_update(&self, id: String, code: Option<String>, labels: Option<serde_json::Value>, standard_unit: Option<String>, units: Option<serde_json::Value>) -> Result<crate::models::MeasurementFamilies, Error> {
        let api_path = "/v1/products/measurement_families/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &code {
            api_params.insert("code".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &standard_unit {
            api_params.insert("standard_unit".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &units {
            api_params.insert("units".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn products_product_associations_list(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/products/product_associations".to_string();

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
    pub async fn products_product_associations_create(&self, association_type_id: String, product_id: String, target_product_id: String, position: Option<i64>, quantity: Option<f64>) -> Result<crate::models::ProductAssociations, Error> {
        let api_path = "/v1/products/product_associations".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("association_type_id".to_string(), serde_json::to_value(&association_type_id)?);
        api_params.insert("product_id".to_string(), serde_json::to_value(&product_id)?);
        api_params.insert("target_product_id".to_string(), serde_json::to_value(&target_product_id)?);
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &quantity {
            api_params.insert("quantity".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn products_product_associations_delete(&self, id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/products/product_associations/{id}".replace("{id}", &id.to_string());

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
    pub async fn products_product_associations_get(&self, id: String) -> Result<crate::models::ProductAssociations, Error> {
        let api_path = "/v1/products/product_associations/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn products_product_associations_update(&self, id: String, association_type_id: Option<String>, position: Option<i64>, product_id: Option<String>, quantity: Option<f64>, target_product_id: Option<String>) -> Result<crate::models::ProductAssociations, Error> {
        let api_path = "/v1/products/product_associations/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &association_type_id {
            api_params.insert("association_type_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &product_id {
            api_params.insert("product_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &quantity {
            api_params.insert("quantity".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &target_product_id {
            api_params.insert("target_product_id".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn products_product_categories_list(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/products/product_categories".to_string();

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
    pub async fn products_product_categories_create(&self, category_id: String, product_id: String, position: Option<i64>) -> Result<crate::models::ProductCategories, Error> {
        let api_path = "/v1/products/product_categories".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("category_id".to_string(), serde_json::to_value(&category_id)?);
        api_params.insert("product_id".to_string(), serde_json::to_value(&product_id)?);
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn products_product_categories_delete(&self, id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/products/product_categories/{id}".replace("{id}", &id.to_string());

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
    pub async fn products_product_categories_get(&self, id: String) -> Result<crate::models::ProductCategories, Error> {
        let api_path = "/v1/products/product_categories/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn products_product_categories_update(&self, id: String, category_id: Option<String>, position: Option<i64>, product_id: Option<String>) -> Result<crate::models::ProductCategories, Error> {
        let api_path = "/v1/products/product_categories/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &category_id {
            api_params.insert("category_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &product_id {
            api_params.insert("product_id".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn products_reference_entities_list(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/products/reference_entities".to_string();

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
    pub async fn products_reference_entities_create(&self, code: String, image: Option<String>, labels: Option<serde_json::Value>) -> Result<crate::models::ReferenceEntities, Error> {
        let api_path = "/v1/products/reference_entities".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("code".to_string(), serde_json::to_value(&code)?);
        if let Some(value) = &image {
            api_params.insert("image".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn products_reference_entities_delete(&self, id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/products/reference_entities/{id}".replace("{id}", &id.to_string());

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
    pub async fn products_reference_entities_get(&self, id: String) -> Result<crate::models::ReferenceEntities, Error> {
        let api_path = "/v1/products/reference_entities/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn products_reference_entities_update(&self, id: String, code: Option<String>, image: Option<String>, labels: Option<serde_json::Value>) -> Result<crate::models::ReferenceEntities, Error> {
        let api_path = "/v1/products/reference_entities/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &code {
            api_params.insert("code".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &image {
            api_params.insert("image".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn products_reference_entity_records_list(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/products/reference_entity_records".to_string();

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
    pub async fn products_reference_entity_records_create(&self, code: String, reference_entity_id: String, attribute_values: Option<serde_json::Value>, labels: Option<serde_json::Value>) -> Result<crate::models::ReferenceEntityRecords, Error> {
        let api_path = "/v1/products/reference_entity_records".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("code".to_string(), serde_json::to_value(&code)?);
        api_params.insert("reference_entity_id".to_string(), serde_json::to_value(&reference_entity_id)?);
        if let Some(value) = &attribute_values {
            api_params.insert("attribute_values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn products_reference_entity_records_delete(&self, id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/products/reference_entity_records/{id}".replace("{id}", &id.to_string());

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
    pub async fn products_reference_entity_records_get(&self, id: String) -> Result<crate::models::ReferenceEntityRecords, Error> {
        let api_path = "/v1/products/reference_entity_records/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn products_reference_entity_records_update(&self, id: String, attribute_values: Option<serde_json::Value>, code: Option<String>, labels: Option<serde_json::Value>, reference_entity_id: Option<String>) -> Result<crate::models::ReferenceEntityRecords, Error> {
        let api_path = "/v1/products/reference_entity_records/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &attribute_values {
            api_params.insert("attribute_values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &code {
            api_params.insert("code".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &reference_entity_id {
            api_params.insert("reference_entity_id".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn products_delete(&self, id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/products/{id}".replace("{id}", &id.to_string());

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
    pub async fn products_get(&self, id: String) -> Result<crate::models::Products, Error> {
        let api_path = "/v1/products/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn products_update(&self, id: String, attribute_values: Option<serde_json::Value>, completeness: Option<serde_json::Value>, deleted_at: Option<String>, enabled: Option<bool>, family_id: Option<String>, family_variant_id: Option<String>, kind: Option<String>, parent_id: Option<String>, quantified_associations: Option<serde_json::Value>, sku: Option<String>, tax_class: Option<String>) -> Result<crate::models::Products, Error> {
        let api_path = "/v1/products/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &attribute_values {
            api_params.insert("attribute_values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &completeness {
            api_params.insert("completeness".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &deleted_at {
            api_params.insert("deleted_at".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &enabled {
            api_params.insert("enabled".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &family_id {
            api_params.insert("family_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &family_variant_id {
            api_params.insert("family_variant_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &kind {
            api_params.insert("kind".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &parent_id {
            api_params.insert("parent_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &quantified_associations {
            api_params.insert("quantified_associations".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &sku {
            api_params.insert("sku".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &tax_class {
            api_params.insert("tax_class".to_string(), serde_json::to_value(value)?);
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
