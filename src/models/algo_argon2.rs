use serde::{Deserialize, Serialize};

/// AlgoArgon2
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AlgoArgon2 {
    /// Memory used to compute hash.
    #[serde(rename = "memoryCost", default)]
    pub memory_cost: i64,
    /// Number of threads used to compute hash.
    #[serde(rename = "threads", default)]
    pub threads: i64,
    /// Amount of time consumed to compute hash
    #[serde(rename = "timeCost", default)]
    pub time_cost: i64,
    /// Algo type.
    #[serde(rename = "type", default)]
    pub xtype: String,
}
