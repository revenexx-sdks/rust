use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrganizationMetricsRefreshRequest {
    /// Anchor for the rolling windows — pass back the value the previous call
    /// returned.
    #[serde(rename = "as_of", default)]
    pub as_of: String,
    /// Continue an unfinished refresh: the value the previous call returned,
    /// verbatim. It is the id of the last organization processed, so only a value
    /// this API handed out ever resolves.
    #[serde(rename = "cursor", default)]
    pub cursor: String,
    /// Refresh exactly these organizations in one call instead of walking all of
    /// them.
    #[serde(rename = "organization_ids", default)]
    pub organization_ids: Vec<String>,
}
