use serde::{Deserialize, Serialize};

/// Continents List
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContinentList {
    /// List of continents.
    #[serde(rename = "continents", default)]
    pub continents: Vec<crate::models::Continent>,
    /// Total number of continents that matched your query.
    #[serde(rename = "total", default)]
    pub total: i64,
}
