use serde::{Deserialize, Serialize};

/// One published page resolved for one language, ready to render: i18n
/// fallback applied per field, blocks outside their publish window removed,
/// library references expanded inline.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeliveryPage {
    /// The page's block tree, keyed by field name — `{ "content": [ … ] }`. A
    /// theme renders the field it knows and ignores the rest.
    #[serde(rename = "fields", default)]
    pub fields: serde_json::Value,
    /// The page frame — everything a theme needs before it starts rendering
    /// blocks.
    #[serde(rename = "page", default)]
    pub page: serde_json::Value,
}
