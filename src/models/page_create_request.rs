use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageCreateRequest {
    #[serde(rename = "bundle", default)]
    pub bundle: String,
    #[serde(rename = "hostOptions", default)]
    pub host_options: serde_json::Value,
    #[serde(rename = "meta", default)]
    pub meta: serde_json::Value,
    #[serde(rename = "slug", default)]
    pub slug: String,
    #[serde(rename = "sourceLanguage", default)]
    pub source_language: String,
    #[serde(rename = "title", default)]
    pub title: String,
}
