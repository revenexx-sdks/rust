use serde::{Deserialize, Serialize};

/// MFAFactors
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MfaFactors {
    /// Can email be used for MFA challenge for this account.
    #[serde(rename = "email", default)]
    pub email: bool,
    /// Can phone (SMS) be used for MFA challenge for this account.
    #[serde(rename = "phone", default)]
    pub phone: bool,
    /// Can recovery code be used for MFA challenge for this account.
    #[serde(rename = "recoveryCode", default)]
    pub recovery_code: bool,
    /// Can TOTP be used for MFA challenge for this account.
    #[serde(rename = "totp", default)]
    pub totp: bool,
}
