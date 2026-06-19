use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ShippingMethod {
    #[serde(rename = "carrier", default)]
    pub carrier: String,
    #[serde(rename = "code", default)]
    pub code: String,
    #[serde(rename = "countries", default)]
    pub countries: serde_json::Value,
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    #[serde(rename = "currency", default)]
    pub currency: String,
    #[serde(rename = "description", default)]
    pub description: String,
    #[serde(rename = "enabled", default)]
    pub enabled: bool,
    #[serde(rename = "eta_days_max", default)]
    pub eta_days_max: i64,
    #[serde(rename = "eta_days_min", default)]
    pub eta_days_min: i64,
    #[serde(rename = "free_above", default)]
    pub free_above: f64,
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "labels", default)]
    pub labels: serde_json::Value,
    #[serde(rename = "matrix_attribute", default)]
    pub matrix_attribute: String,
    #[serde(rename = "matrix_basis", default)]
    pub matrix_basis: String,
    #[serde(rename = "metadata", default)]
    pub metadata: serde_json::Value,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "position", default)]
    pub position: i64,
    #[serde(rename = "price", default)]
    pub price: f64,
    #[serde(rename = "pricing_type", default)]
    pub pricing_type: String,
    #[serde(rename = "tax_class", default)]
    pub tax_class: String,
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
}
