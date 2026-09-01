use serde::{Deserialize, Serialize};

/// One importable / exportable entity of an installed app.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IoEntity {
    #[serde(rename = "app", default)]
    pub app: String,
    #[serde(rename = "entity", default)]
    pub entity: String,
    /// Humanised entity name for pickers.
    #[serde(rename = "label", default)]
    pub label: String,
    /// The physical table name Baseline provisioned.
    #[serde(rename = "table", default)]
    pub table: String,
    #[serde(rename = "vendor", default)]
    pub vendor: String,
}
