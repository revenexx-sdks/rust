use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MarketLocale {
    #[serde(rename = "code", default)]
    pub code: String,
    #[serde(rename = "country", default)]
    pub country: String,
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "is_default", default)]
    pub is_default: bool,
    #[serde(rename = "language", default)]
    pub language: String,
    #[serde(rename = "market_id", default)]
    pub market_id: String,
    #[serde(rename = "position", default)]
    pub position: i64,
}
