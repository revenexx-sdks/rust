use serde::{Deserialize, Serialize};

/// Countries List
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CountryList {
    /// List of countries.
    #[serde(rename = "countries", default)]
    pub countries: Vec<crate::models::Country>,
    /// Total number of countries that matched your query.
    #[serde(rename = "total", default)]
    pub total: i64,
}
