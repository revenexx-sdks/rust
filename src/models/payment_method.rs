use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaymentMethod {
    #[serde(rename = "code", default)]
    pub code: String,
    #[serde(rename = "countries", default)]
    pub countries: serde_json::Value,
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    #[serde(rename = "description", default)]
    pub description: String,
    #[serde(rename = "enabled", default)]
    pub enabled: bool,
    #[serde(rename = "fee_amount", default)]
    pub fee_amount: f64,
    #[serde(rename = "fee_currency", default)]
    pub fee_currency: String,
    #[serde(rename = "fee_type", default)]
    pub fee_type: String,
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "kind", default)]
    pub kind: String,
    #[serde(rename = "labels", default)]
    pub labels: serde_json::Value,
    #[serde(rename = "max_order_value", default)]
    pub max_order_value: f64,
    #[serde(rename = "metadata", default)]
    pub metadata: serde_json::Value,
    #[serde(rename = "min_order_value", default)]
    pub min_order_value: f64,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "position", default)]
    pub position: i64,
    #[serde(rename = "provider", default)]
    pub provider: String,
    #[serde(rename = "provider_method", default)]
    pub provider_method: String,
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
}
