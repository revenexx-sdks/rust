use serde::{Deserialize, Serialize};

/// AlgoSHA
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AlgoSha {
    /// Algo type.
    #[serde(rename = "type", default)]
    pub xtype: String,
}
