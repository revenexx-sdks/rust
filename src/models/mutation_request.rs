use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MutationRequest {
    #[serde(rename = "langcode", default)]
    pub langcode: String,
    #[serde(rename = "payload", default)]
    pub payload: serde_json::Value,
    /// Mutation plugin id (add, move, delete, duplicate, update_field_value, ...).
    #[serde(rename = "plugin", default)]
    pub plugin: String,
}
