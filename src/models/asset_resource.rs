use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AssetResource {
    #[serde(rename = "alt_text", default)]
    pub alt_text: String,
    #[serde(rename = "content_hash", default)]
    pub content_hash: String,
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    #[serde(rename = "deleted_at", default)]
    pub deleted_at: String,
    #[serde(rename = "description", default)]
    pub description: String,
    #[serde(rename = "display_name", default)]
    pub display_name: String,
    #[serde(rename = "dominant_color", default)]
    pub dominant_color: String,
    #[serde(rename = "duration_ms", default)]
    pub duration_ms: i64,
    #[serde(rename = "folder_id", default)]
    pub folder_id: String,
    #[serde(rename = "height", default)]
    pub height: i64,
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "kind", default)]
    pub kind: String,
    #[serde(rename = "metadata", default)]
    pub metadata: Vec<serde_json::Value>,
    #[serde(rename = "mime_type", default)]
    pub mime_type: String,
    #[serde(rename = "original_name", default)]
    pub original_name: String,
    #[serde(rename = "page_count", default)]
    pub page_count: i64,
    #[serde(rename = "path_name", default)]
    pub path_name: String,
    #[serde(rename = "processed_at", default)]
    pub processed_at: String,
    #[serde(rename = "size_bytes", default)]
    pub size_bytes: i64,
    #[serde(rename = "status", default)]
    pub status: String,
    #[serde(rename = "tags", default)]
    pub tags: Vec<serde_json::Value>,
    #[serde(rename = "tenant_id", default)]
    pub tenant_id: String,
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
    #[serde(rename = "url", default)]
    pub url: String,
    #[serde(rename = "visibility", default)]
    pub visibility: String,
    #[serde(rename = "width", default)]
    pub width: i64,
}
