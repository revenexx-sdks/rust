use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SeedRequest {
    #[serde(rename = "menus", default)]
    pub menus: Vec<serde_json::Value>,
    #[serde(rename = "pages", default)]
    pub pages: Vec<serde_json::Value>,
}
