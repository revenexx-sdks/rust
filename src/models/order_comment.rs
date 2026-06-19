use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderComment {
    #[serde(rename = "author", default)]
    pub author: String,
    #[serde(rename = "body", default)]
    pub body: String,
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "order_id", default)]
    pub order_id: String,
    #[serde(rename = "visibility", default)]
    pub visibility: String,
}
