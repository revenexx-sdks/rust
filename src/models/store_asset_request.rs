use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoreAssetRequest {
    #[serde(rename = "alt_text", default)]
    pub alt_text: String,
    #[serde(rename = "description", default)]
    pub description: String,
    #[serde(rename = "display_name", default)]
    pub display_name: String,
    #[serde(rename = "file", default)]
    pub file: String,
    #[serde(rename = "folder_id", default)]
    pub folder_id: String,
    #[serde(rename = "keep_archive", default)]
    pub keep_archive: bool,
    #[serde(rename = "tags", default)]
    pub tags: Vec<String>,
    /// Archives only: unpack the members after upload (see AssetController).
    #[serde(rename = "unpack", default)]
    pub unpack: bool,
    #[serde(rename = "visibility", default)]
    pub visibility: String,
}
