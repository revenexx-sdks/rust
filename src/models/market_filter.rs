use serde::{Deserialize, Serialize};

/// The exact-column filters this call applied, echoed back. Every value is the
/// raw query string, never the column's own type: `?is_default=true` comes
/// back as `"true"`. A `?column=value` naming a column this entity does not
/// have is DROPPED rather than refused — the call answers 200 with the
/// unfiltered list, and the key missing from here is the only way to find out.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MarketFilter {
    /// The `code` filter as it arrived, verbatim. Present only when the call sent
    /// it.
    #[serde(rename = "code", default)]
    pub code: String,
    /// The `created_at` filter as it arrived, verbatim. Present only when the call
    /// sent it. Any form the database accepts as a timestamp, including a bare
    /// date.
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    /// The `currency` filter as it arrived, verbatim. Present only when the call
    /// sent it.
    #[serde(rename = "currency", default)]
    pub currency: String,
    /// The `id` filter as it arrived, verbatim. Present only when the call sent
    /// it.
    #[serde(rename = "id", default)]
    pub id: String,
    /// The `is_default` filter as it arrived, verbatim. Present only when the call
    /// sent it.
    #[serde(rename = "is_default", default)]
    pub is_default: String,
    /// The `labels` filter as it arrived, verbatim. Present only when the call
    /// sent it.
    #[serde(rename = "labels", default)]
    pub labels: String,
    /// The `name` filter as it arrived, verbatim. Present only when the call sent
    /// it.
    #[serde(rename = "name", default)]
    pub name: String,
    /// The `position` filter as it arrived, verbatim. Present only when the call
    /// sent it.
    #[serde(rename = "position", default)]
    pub position: String,
    /// The `status` filter as it arrived, verbatim. Present only when the call
    /// sent it.
    #[serde(rename = "status", default)]
    pub status: String,
    /// The `updated_at` filter as it arrived, verbatim. Present only when the call
    /// sent it. Any form the database accepts as a timestamp, including a bare
    /// date.
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
}
