use serde::{Deserialize, Serialize};

/// One navigation menu of the tenant, addressed by the stable key a theme
/// looks it up under.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Menu {
    /// When the menu was created.
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    /// The user id that created the menu.
    #[serde(rename = "created_by", default)]
    pub created_by: String,
    /// The tombstone. A soft-deleted menu disappears from the renderer
    /// immediately.
    #[serde(rename = "deleted_at", default)]
    pub deleted_at: String,
    /// The menu row id. Used by the management routes; the renderer addresses a
    /// menu by its `menu_key` instead, because that is the thing a theme
    /// hard-codes.
    #[serde(rename = "id", default)]
    pub id: String,
    /// The ordered navigation tree itself. Stored exactly as it was sent, so the
    /// theme and the editor agree on the shape without this app enforcing one.
    #[serde(rename = "items", default)]
    pub items: Vec<crate::models::PageMenuItem>,
    /// What this menu is called for the people who edit it. Never rendered in the
    /// storefront.
    #[serde(rename = "label", default)]
    pub label: String,
    /// The stable name the theme asks for a menu by — `main`, `footer`,
    /// `account`. It is what makes seeding idempotent and what a header component
    /// looks up; renaming it detaches the menu from the theme slot.
    #[serde(rename = "menu_key", default)]
    pub menu_key: String,
    /// When the menu was last replaced. The upsert rewrites `items` wholesale, so
    /// this is the timestamp of the whole navigation, not of one entry.
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
}
