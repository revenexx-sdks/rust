use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EligiblePaymentMethod {
    #[serde(rename = "code", default)]
    pub code: String,
    #[serde(rename = "currency", default)]
    pub currency: String,
    #[serde(rename = "description", default)]
    pub description: String,
    #[serde(rename = "fee", default)]
    pub fee: f64,
    #[serde(rename = "fee_type", default)]
    pub fee_type: String,
    #[serde(rename = "kind", default)]
    pub kind: String,
    #[serde(rename = "labels", default)]
    pub labels: serde_json::Value,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "position", default)]
    pub position: i64,
    #[serde(rename = "provider", default)]
    pub provider: String,
}
