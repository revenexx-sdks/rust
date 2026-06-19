use serde::{Deserialize, Serialize};

/// MFAType
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MfaType {
    /// Secret token used for TOTP factor.
    #[serde(rename = "secret", default)]
    pub secret: String,
    /// URI for authenticator apps.
    #[serde(rename = "uri", default)]
    pub uri: String,
}
