use serde::{Deserialize, Serialize};

/// Locale codes list
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LocaleCodeList {
    /// List of localeCodes.
    #[serde(rename = "localeCodes", default)]
    pub locale_codes: Vec<crate::models::LocaleCode>,
    /// Total number of localeCodes that matched your query.
    #[serde(rename = "total", default)]
    pub total: i64,
}
