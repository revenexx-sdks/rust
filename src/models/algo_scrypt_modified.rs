use serde::{Deserialize, Serialize};

/// AlgoScryptModified
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AlgoScryptModified {
    /// Salt used to compute hash.
    #[serde(rename = "salt", default)]
    pub salt: String,
    /// Separator used to compute hash.
    #[serde(rename = "saltSeparator", default)]
    pub salt_separator: String,
    /// Key used to compute hash.
    #[serde(rename = "signerKey", default)]
    pub signer_key: String,
    /// Algo type.
    #[serde(rename = "type", default)]
    pub xtype: String,
}
