use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrganizationMetricsFreshness {
    /// Companies with no metrics row yet. A rule reading revenue silently skips
    /// them, so this is the number to watch after an import.
    #[serde(rename = "missing", default)]
    pub missing: i64,
    /// The OLDEST computed_at in the table — the floor, not an average. Null
    /// when there are no rows at all.
    #[serde(rename = "oldest_computed_at", default)]
    pub oldest_computed_at: String,
    /// The anchor those oldest numbers were measured from.
    #[serde(rename = "orders_as_of", default)]
    pub orders_as_of: String,
    /// Companies in this tenant.
    #[serde(rename = "organizations", default)]
    pub organizations: i64,
    /// Metrics rows that exist — at most one per company.
    #[serde(rename = "rows", default)]
    pub rows: i64,
}
