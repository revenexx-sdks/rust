use serde::{Deserialize, Serialize};

/// AlgoPHPass
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AlgoPhpass {
    /// Algo type.
    #[serde(rename = "type", default)]
    pub xtype: String,
}
