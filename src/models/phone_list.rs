use serde::{Deserialize, Serialize};

/// Phones List
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PhoneList {
    /// List of phones.
    #[serde(rename = "phones", default)]
    pub phones: Vec<crate::models::Phone>,
    /// Total number of phones that matched your query.
    #[serde(rename = "total", default)]
    pub total: i64,
}
