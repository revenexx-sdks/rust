use serde::{Deserialize, Serialize};

/// Create or replace the menu identified by menuKey (idempotent per tenant).
/// `items` is written wholesale — there is no per-entry edit, so send the
/// whole tree every time.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MenuUpsertRequest {
    /// The ordered navigation tree. Replaces the stored one completely.
    #[serde(rename = "items", default)]
    pub items: Vec<crate::models::PageMenuItem>,
    /// What this menu is called for the people who edit it. Required on a create;
    /// an update keeps the label it had when this is left out.
    #[serde(rename = "label", default)]
    pub label: String,
    /// The stable slot the theme asks for this menu by. Idempotency is keyed on
    /// it: sending an existing key replaces that menu instead of creating a second
    /// one.
    #[serde(rename = "menuKey", default)]
    pub menu_key: String,
}
