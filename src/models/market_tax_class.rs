use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MarketTaxClass {
    #[serde(rename = "code", default)]
    pub code: String,
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "is_default", default)]
    pub is_default: bool,
    #[serde(rename = "labels", default)]
    pub labels: serde_json::Value,
    #[serde(rename = "market_id", default)]
    pub market_id: String,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "position", default)]
    pub position: i64,
    #[serde(rename = "rate", default)]
    pub rate: f64,
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
}
