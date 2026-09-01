use serde::{Deserialize, Serialize};

/// One navigation menu, ready to render.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeliveryMenu {
    /// The menu KEY (`main`, `footer`, `account`), not the row id — this is the
    /// handle a theme hard-codes.
    #[serde(rename = "id", default)]
    pub id: String,
    /// The ordered navigation tree, exactly as it is stored. Render it in order;
    /// nesting is `items` inside an entry.
    #[serde(rename = "items", default)]
    pub items: Vec<crate::models::PageMenuItem>,
    /// What the menu is called for the people who edit it. A theme rarely renders
    /// it.
    #[serde(rename = "label", default)]
    pub label: String,
}
