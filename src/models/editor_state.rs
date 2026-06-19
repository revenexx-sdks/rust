use serde::{Deserialize, Serialize};

/// The blökkli adapter state: page, translations, edit state + mutation log,
/// materialized field lists, mutated options/entity values, text field values,
/// droppable field values and violations.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EditorState {
    #[serde(rename = "currentUserIsOwner", default)]
    pub current_user_is_owner: bool,
    #[serde(rename = "droppableFieldValues", default)]
    pub droppable_field_values: Vec<serde_json::Value>,
    #[serde(rename = "editState", default)]
    pub edit_state: serde_json::Value,
    #[serde(rename = "fields", default)]
    pub fields: Vec<serde_json::Value>,
    #[serde(rename = "ignoredAnalyzeIdentifiers", default)]
    pub ignored_analyze_identifiers: Vec<String>,
    #[serde(rename = "langcode", default)]
    pub langcode: String,
    #[serde(rename = "mutatedEntity", default)]
    pub mutated_entity: serde_json::Value,
    #[serde(rename = "mutatedHostOptions", default)]
    pub mutated_host_options: serde_json::Value,
    #[serde(rename = "mutatedOptions", default)]
    pub mutated_options: serde_json::Value,
    #[serde(rename = "mutations", default)]
    pub mutations: Vec<serde_json::Value>,
    #[serde(rename = "page", default)]
    pub page: serde_json::Value,
    #[serde(rename = "textFieldValues", default)]
    pub text_field_values: Vec<serde_json::Value>,
    #[serde(rename = "translations", default)]
    pub translations: Vec<serde_json::Value>,
    #[serde(rename = "violations", default)]
    pub violations: Vec<serde_json::Value>,
}
