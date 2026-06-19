use serde::{Deserialize, Serialize};

/// Provider list
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProviderList {
    /// List of providers.
    #[serde(rename = "providers", default)]
    pub providers: Vec<crate::models::Provider>,
    /// Total number of providers that matched your query.
    #[serde(rename = "total", default)]
    pub total: i64,
}
