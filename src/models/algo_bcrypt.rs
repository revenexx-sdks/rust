use serde::{Deserialize, Serialize};

/// AlgoBcrypt
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AlgoBcrypt {
    /// Algo type.
    #[serde(rename = "type", default)]
    pub xtype: String,
}
