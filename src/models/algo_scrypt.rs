use serde::{Deserialize, Serialize};

/// AlgoScrypt
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AlgoScrypt {
    /// CPU complexity of computed hash.
    #[serde(rename = "costCpu", default)]
    pub cost_cpu: i64,
    /// Memory complexity of computed hash.
    #[serde(rename = "costMemory", default)]
    pub cost_memory: i64,
    /// Parallelization of computed hash.
    #[serde(rename = "costParallel", default)]
    pub cost_parallel: i64,
    /// Length used to compute hash.
    #[serde(rename = "length", default)]
    pub length: i64,
    /// Algo type.
    #[serde(rename = "type", default)]
    pub xtype: String,
}
