use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Page {
    #[serde(rename = "analyze_ignored", default)]
    pub analyze_ignored: serde_json::Value,
    #[serde(rename = "bundle", default)]
    pub bundle: String,
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    #[serde(rename = "created_by", default)]
    pub created_by: String,
    #[serde(rename = "deleted_at", default)]
    pub deleted_at: String,
    #[serde(rename = "host_options", default)]
    pub host_options: serde_json::Value,
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "meta", default)]
    pub meta: serde_json::Value,
    #[serde(rename = "published_revision_id", default)]
    pub published_revision_id: String,
    #[serde(rename = "slug", default)]
    pub slug: String,
    #[serde(rename = "source_language", default)]
    pub source_language: String,
    #[serde(rename = "status", default)]
    pub status: String,
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
    #[serde(rename = "updated_by", default)]
    pub updated_by: String,
}
