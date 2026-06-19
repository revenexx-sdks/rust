use serde::{Deserialize, Serialize};

/// MFA Recovery Codes
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MfaRecoveryCodes {
    /// Recovery codes.
    #[serde(rename = "recoveryCodes", default)]
    pub recovery_codes: Vec<String>,
}
