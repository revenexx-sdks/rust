use serde::{Deserialize, Serialize};

/// AlgoMD5
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AlgoMd5 {
    /// Algo type.
    #[serde(rename = "type", default)]
    pub xtype: String,
}
