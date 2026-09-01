use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProductGridFilter {
    /// The attribute code to filter on.
    #[serde(rename = "code", default)]
    pub code: String,
    /// The attribute's i18n labels, for the filter's own caption.
    #[serde(rename = "label", default)]
    pub label: serde_json::Value,
    /// Which control the filter asks for — the same widget vocabulary the
    /// columns use.
    #[serde(rename = "type", default)]
    pub xtype: String,
}
