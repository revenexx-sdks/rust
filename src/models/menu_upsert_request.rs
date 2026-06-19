use serde::{Deserialize, Serialize};

/// Create or update the menu identified by menuKey (idempotent per tenant).
/// `items` is the ordered nav tree ([{ label, to, items? }]).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MenuUpsertRequest {
    /// Ordered menu entries ({ label, to?, items? }).
    #[serde(rename = "items", default)]
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "label", default)]
    pub label: String,
    /// Stable menu identifier, e.g. "main", "footer", "account".
    #[serde(rename = "menuKey", default)]
    pub menu_key: String,
}
