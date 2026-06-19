use serde::{Deserialize, Serialize};

/// Published page resolved for one language: nested block tree with i18n
/// fallback applied and scheduled blocks filtered.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeliveryPage {
    /// Field name → ordered block list ({ uuid, bundle, props, options, children
    /// }).
    #[serde(rename = "fields", default)]
    pub fields: serde_json::Value,
    #[serde(rename = "page", default)]
    pub page: serde_json::Value,
}
