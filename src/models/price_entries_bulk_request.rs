use serde::{Deserialize, Serialize};

/// A chunk of an import. Unlike the replace call it never wipes the list.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PriceEntriesBulkRequest {
    /// At most 5000 rows per call — send a large book in chunks.
    #[serde(rename = "entries", default)]
    pub entries: Vec<crate::models::PriceEntryReplaceItem>,
    /// Default 'upsert': a row naming a rung the list already has (same
    /// product/sku AND quantity_min) updates it. 'append' always inserts — a
    /// re-run then duplicates the ladder, which is what makes an ambiguous tier
    /// table.
    #[serde(rename = "mode", default)]
    pub mode: String,
}
