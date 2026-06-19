use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PriceEntriesReplaceRequest {
    /// The complete new entry set (set semantics).
    #[serde(rename = "entries", default)]
    pub entries: Vec<crate::models::PriceEntryReplaceItem>,
}
