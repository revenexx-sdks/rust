use serde::{Deserialize, Serialize};

/// Transaction List
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TransactionList {
    /// Total number of transactions that matched your query.
    #[serde(rename = "total", default)]
    pub total: i64,
    /// List of transactions.
    #[serde(rename = "transactions", default)]
    pub transactions: Vec<crate::models::Transaction>,
}
