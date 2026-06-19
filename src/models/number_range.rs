use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NumberRange {
    #[serde(rename = "channel_id", default)]
    pub channel_id: String,
    #[serde(rename = "code", default)]
    pub code: String,
    #[serde(rename = "counter", default)]
    pub counter: i64,
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "metadata", default)]
    pub metadata: serde_json::Value,
    #[serde(rename = "padding", default)]
    pub padding: i64,
    #[serde(rename = "position_step", default)]
    pub position_step: i64,
    #[serde(rename = "prefix", default)]
    pub prefix: String,
    #[serde(rename = "step", default)]
    pub step: i64,
    #[serde(rename = "suffix", default)]
    pub suffix: String,
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
}
