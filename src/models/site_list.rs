use serde::{Deserialize, Serialize};

/// Sites List
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SiteList {
    /// List of sites.
    #[serde(rename = "sites", default)]
    pub sites: Vec<crate::models::Site>,
    /// Total number of sites that matched your query.
    #[serde(rename = "total", default)]
    pub total: i64,
}
