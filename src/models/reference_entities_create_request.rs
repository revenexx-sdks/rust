use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReferenceEntitiesCreateRequest {
    /// The entity's stable identifier — a domain of records the catalog POINTS
    /// AT instead of duplicating, so a brand is edited once and not on nine
    /// thousand products. Unique per tenant.
    #[serde(rename = "code", default)]
    pub code: String,
    /// A delivery path or URL for the entity's own icon. Cosmetic — nothing in
    /// this app resolves it.
    #[serde(rename = "image", default)]
    pub image: String,
    /// What the entity is called, per language tag — the heading over its record
    /// list.
    #[serde(rename = "labels", default)]
    pub labels: serde_json::Value,
}
