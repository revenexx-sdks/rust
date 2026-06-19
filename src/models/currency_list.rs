use serde::{Deserialize, Serialize};

/// Currencies List
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CurrencyList {
    /// List of currencies.
    #[serde(rename = "currencies", default)]
    pub currencies: Vec<crate::models::Currency>,
    /// Total number of currencies that matched your query.
    #[serde(rename = "total", default)]
    pub total: i64,
}
