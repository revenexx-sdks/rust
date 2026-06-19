use std::collections::HashMap;

use serde_json::Value;

use crate::client::Client;
use crate::error::Error;
use crate::input_file::InputFile;

/// Pages service
pub struct Pages {
    client: Client,
}

impl Pages {
    pub fn new(client: Client) -> Self {
        Pages { client }
    }
    pub async fn pages_delivery_menus(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/pages/delivery/menus".to_string();

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
    pub async fn pages_delivery_page(&self) -> Result<crate::models::DeliveryPage, Error> {
        let api_path = "/v1/pages/delivery/page".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn pages_delivery_pages(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/pages/delivery/pages".to_string();

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
    pub async fn pages_delivery_preview(&self, token: String) -> Result<crate::models::DeliveryPage, Error> {
        let api_path = "/v1/pages/delivery/preview/{token}".replace("{token}", &token.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("token".to_string(), serde_json::to_value(&token)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn pages_editor_edit_states(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/pages/editor/edit-states".to_string();

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
    pub async fn pages_editor_notifications_list(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/pages/editor/notifications".to_string();

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
    pub async fn pages_editor_notifications_mark_all_read(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/pages/editor/notifications/mark-all-read".to_string();

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
    pub async fn pages_editor_notifications_unread_count(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/pages/editor/notifications/unread-count".to_string();

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
    pub async fn pages_editor_translate(&self, items: Option<Vec<serde_json::Value>>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/pages/editor/translate".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &items {
            api_params.insert("items".to_string(), serde_json::to_value(value)?);
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
    pub async fn pages_editor_user_settings_get(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/pages/editor/user-settings".to_string();

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
    pub async fn pages_editor_user_settings_put(&self, settings: Option<serde_json::Value>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/pages/editor/user-settings".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &settings {
            api_params.insert("settings".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        if api_response.body.is_empty() {
            Ok(serde_json::Value::Null)
        } else {
            Ok(serde_json::from_slice(&api_response.body)?)
        }
    }
    pub async fn pages_editor_users(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/pages/editor/users".to_string();

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
    pub async fn pages_editor_comments_list(&self, page_id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/pages/editor/{page_id}/comments".replace("{pageId}", &page_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("page_id".to_string(), serde_json::to_value(&page_id)?);

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
    pub async fn pages_editor_comments_create(&self, page_id: String, body: String, block_uuids: Option<Vec<String>>, parent_uuid: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/pages/editor/{page_id}/comments".replace("{pageId}", &page_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("page_id".to_string(), serde_json::to_value(&page_id)?);
        api_params.insert("body".to_string(), serde_json::to_value(&body)?);
        if let Some(value) = &block_uuids {
            api_params.insert("blockUuids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &parent_uuid {
            api_params.insert("parentUuid".to_string(), serde_json::to_value(value)?);
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
    pub async fn pages_editor_comments_delete(&self, page_id: String, uuid: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/pages/editor/{page_id}/comments/{uuid}".replace("{pageId}", &page_id.to_string()).replace("{uuid}", &uuid.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("page_id".to_string(), serde_json::to_value(&page_id)?);
        api_params.insert("uuid".to_string(), serde_json::to_value(&uuid)?);

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
    pub async fn pages_editor_comments_update(&self, page_id: String, uuid: String, body: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/pages/editor/{page_id}/comments/{uuid}".replace("{pageId}", &page_id.to_string()).replace("{uuid}", &uuid.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("page_id".to_string(), serde_json::to_value(&page_id)?);
        api_params.insert("uuid".to_string(), serde_json::to_value(&uuid)?);
        api_params.insert("body".to_string(), serde_json::to_value(&body)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        if api_response.body.is_empty() {
            Ok(serde_json::Value::Null)
        } else {
            Ok(serde_json::from_slice(&api_response.body)?)
        }
    }
    pub async fn pages_editor_comments_resolve(&self, page_id: String, uuid: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/pages/editor/{page_id}/comments/{uuid}/resolve".replace("{pageId}", &page_id.to_string()).replace("{uuid}", &uuid.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("page_id".to_string(), serde_json::to_value(&page_id)?);
        api_params.insert("uuid".to_string(), serde_json::to_value(&uuid)?);

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
    pub async fn pages_editor_comments_toggle_task(&self, page_id: String, uuid: String, task_index: i64) -> Result<crate::models::Comment, Error> {
        let api_path = "/v1/pages/editor/{page_id}/comments/{uuid}/toggle-task".replace("{pageId}", &page_id.to_string()).replace("{uuid}", &uuid.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("page_id".to_string(), serde_json::to_value(&page_id)?);
        api_params.insert("uuid".to_string(), serde_json::to_value(&uuid)?);
        api_params.insert("taskIndex".to_string(), serde_json::to_value(&task_index)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn pages_editor_comments_unresolve(&self, page_id: String, uuid: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/pages/editor/{page_id}/comments/{uuid}/unresolve".replace("{pageId}", &page_id.to_string()).replace("{uuid}", &uuid.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("page_id".to_string(), serde_json::to_value(&page_id)?);
        api_params.insert("uuid".to_string(), serde_json::to_value(&uuid)?);

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
    pub async fn pages_editor_history(&self, page_id: String, index: i64, langcode: Option<String>) -> Result<crate::models::MutationResponse, Error> {
        let api_path = "/v1/pages/editor/{page_id}/history".replace("{pageId}", &page_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("page_id".to_string(), serde_json::to_value(&page_id)?);
        api_params.insert("index".to_string(), serde_json::to_value(&index)?);
        if let Some(value) = &langcode {
            api_params.insert("langcode".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn pages_editor_last_changed(&self, page_id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/pages/editor/{page_id}/last-changed".replace("{pageId}", &page_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("page_id".to_string(), serde_json::to_value(&page_id)?);

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
    pub async fn pages_editor_mutation_status(&self, page_id: String, enabled: bool, index: i64, langcode: Option<String>) -> Result<crate::models::MutationResponse, Error> {
        let api_path = "/v1/pages/editor/{page_id}/mutation-status".replace("{pageId}", &page_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("page_id".to_string(), serde_json::to_value(&page_id)?);
        api_params.insert("enabled".to_string(), serde_json::to_value(&enabled)?);
        api_params.insert("index".to_string(), serde_json::to_value(&index)?);
        if let Some(value) = &langcode {
            api_params.insert("langcode".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn pages_editor_mutate(&self, page_id: String, plugin: String, langcode: Option<String>, payload: Option<serde_json::Value>) -> Result<crate::models::MutationResponse, Error> {
        let api_path = "/v1/pages/editor/{page_id}/mutations".replace("{pageId}", &page_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("page_id".to_string(), serde_json::to_value(&page_id)?);
        api_params.insert("plugin".to_string(), serde_json::to_value(&plugin)?);
        if let Some(value) = &langcode {
            api_params.insert("langcode".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &payload {
            api_params.insert("payload".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn pages_editor_preview_grant(&self, page_id: String, ttl_hours: Option<i64>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/pages/editor/{page_id}/preview-grant".replace("{pageId}", &page_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("page_id".to_string(), serde_json::to_value(&page_id)?);
        if let Some(value) = &ttl_hours {
            api_params.insert("ttlHours".to_string(), serde_json::to_value(value)?);
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
    pub async fn pages_editor_publish(&self, page_id: String, force: Option<bool>, label: Option<String>) -> Result<crate::models::MutationResponse, Error> {
        let api_path = "/v1/pages/editor/{page_id}/publish".replace("{pageId}", &page_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("page_id".to_string(), serde_json::to_value(&page_id)?);
        if let Some(value) = &force {
            api_params.insert("force".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &label {
            api_params.insert("label".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn pages_editor_revert(&self, page_id: String) -> Result<crate::models::MutationResponse, Error> {
        let api_path = "/v1/pages/editor/{page_id}/revert".replace("{pageId}", &page_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("page_id".to_string(), serde_json::to_value(&page_id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn pages_editor_schedule(&self, page_id: String, scheduled_at: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/pages/editor/{page_id}/schedule".replace("{pageId}", &page_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("page_id".to_string(), serde_json::to_value(&page_id)?);
        api_params.insert("scheduledAt".to_string(), serde_json::to_value(&scheduled_at)?);

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
    pub async fn pages_editor_state(&self, page_id: String) -> Result<crate::models::EditorState, Error> {
        let api_path = "/v1/pages/editor/{page_id}/state".replace("{pageId}", &page_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("page_id".to_string(), serde_json::to_value(&page_id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn pages_editor_take_ownership(&self, page_id: String) -> Result<crate::models::MutationResponse, Error> {
        let api_path = "/v1/pages/editor/{page_id}/take-ownership".replace("{pageId}", &page_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("page_id".to_string(), serde_json::to_value(&page_id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn pages_editor_templates_create(&self, page_id: String, label: String, uuids: Vec<String>, description: Option<String>, field_name: Option<String>, is_default: Option<bool>, page_bundle: Option<String>) -> Result<crate::models::Template, Error> {
        let api_path = "/v1/pages/editor/{page_id}/templates".replace("{pageId}", &page_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("page_id".to_string(), serde_json::to_value(&page_id)?);
        api_params.insert("label".to_string(), serde_json::to_value(&label)?);
        api_params.insert("uuids".to_string(), serde_json::to_value(&uuids)?);
        if let Some(value) = &description {
            api_params.insert("description".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &field_name {
            api_params.insert("fieldName".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &is_default {
            api_params.insert("isDefault".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &page_bundle {
            api_params.insert("pageBundle".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn pages_editor_unschedule(&self, page_id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/pages/editor/{page_id}/unschedule".replace("{pageId}", &page_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("page_id".to_string(), serde_json::to_value(&page_id)?);

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
    pub async fn pages_library_list(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/pages/library".to_string();

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
    pub async fn pages_library_delete(&self, id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/pages/library/{id}".replace("{id}", &id.to_string());

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
    pub async fn pages_library_get(&self, id: String) -> Result<crate::models::LibraryItem, Error> {
        let api_path = "/v1/pages/library/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn pages_library_update(&self, id: String, bundle: Option<String>, label: Option<String>, tree: Option<serde_json::Value>) -> Result<crate::models::LibraryItem, Error> {
        let api_path = "/v1/pages/library/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &bundle {
            api_params.insert("bundle".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &label {
            api_params.insert("label".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &tree {
            api_params.insert("tree".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn pages_menus_list(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/pages/menus".to_string();

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
    pub async fn pages_menus_upsert(&self, label: String, menu_key: String, items: Option<Vec<serde_json::Value>>) -> Result<crate::models::Menu, Error> {
        let api_path = "/v1/pages/menus".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("label".to_string(), serde_json::to_value(&label)?);
        api_params.insert("menuKey".to_string(), serde_json::to_value(&menu_key)?);
        if let Some(value) = &items {
            api_params.insert("items".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn pages_menus_delete(&self, id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/pages/menus/{id}".replace("{id}", &id.to_string());

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
    pub async fn pages_menus_get(&self, id: String) -> Result<crate::models::Menu, Error> {
        let api_path = "/v1/pages/menus/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn pages_menus_update(&self, id: String, items: Option<Vec<serde_json::Value>>, label: Option<String>) -> Result<crate::models::Menu, Error> {
        let api_path = "/v1/pages/menus/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &items {
            api_params.insert("items".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &label {
            api_params.insert("label".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn pages_pages_list(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/pages/pages".to_string();

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
    pub async fn pages_pages_create(&self, title: String, bundle: Option<String>, host_options: Option<serde_json::Value>, meta: Option<serde_json::Value>, slug: Option<String>, source_language: Option<String>) -> Result<crate::models::Page, Error> {
        let api_path = "/v1/pages/pages".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("title".to_string(), serde_json::to_value(&title)?);
        if let Some(value) = &bundle {
            api_params.insert("bundle".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &host_options {
            api_params.insert("hostOptions".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &meta {
            api_params.insert("meta".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &slug {
            api_params.insert("slug".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &source_language {
            api_params.insert("sourceLanguage".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn pages_pages_delete(&self, id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/pages/pages/{id}".replace("{id}", &id.to_string());

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
    pub async fn pages_pages_get(&self, id: String) -> Result<crate::models::Page, Error> {
        let api_path = "/v1/pages/pages/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn pages_pages_update(&self, id: String, bundle: Option<String>, meta: Option<serde_json::Value>, slug: Option<String>, status: Option<String>, title: Option<String>) -> Result<crate::models::Page, Error> {
        let api_path = "/v1/pages/pages/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &bundle {
            api_params.insert("bundle".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &meta {
            api_params.insert("meta".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &slug {
            api_params.insert("slug".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &status {
            api_params.insert("status".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &title {
            api_params.insert("title".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn pages_pages_revisions(&self, id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/pages/pages/{id}/revisions".replace("{id}", &id.to_string());

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
    pub async fn pages_seed(&self, menus: Option<Vec<serde_json::Value>>, pages: Option<Vec<serde_json::Value>>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/pages/seed".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &menus {
            api_params.insert("menus".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &pages {
            api_params.insert("pages".to_string(), serde_json::to_value(value)?);
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
    pub async fn pages_templates_list(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/pages/templates".to_string();

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
    pub async fn pages_templates_delete(&self, id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/pages/templates/{id}".replace("{id}", &id.to_string());

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
    pub async fn pages_templates_get(&self, id: String) -> Result<crate::models::Template, Error> {
        let api_path = "/v1/pages/templates/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn pages_templates_update(&self, id: String, description: Option<String>, field_name: Option<String>, is_default: Option<bool>, label: Option<String>, page_bundle: Option<String>, tree: Option<Vec<serde_json::Value>>) -> Result<crate::models::Template, Error> {
        let api_path = "/v1/pages/templates/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &description {
            api_params.insert("description".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &field_name {
            api_params.insert("field_name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &is_default {
            api_params.insert("is_default".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &label {
            api_params.insert("label".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &page_bundle {
            api_params.insert("page_bundle".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &tree {
            api_params.insert("tree".to_string(), serde_json::to_value(value)?);
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
