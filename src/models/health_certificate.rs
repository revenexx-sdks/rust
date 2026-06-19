use serde::{Deserialize, Serialize};

/// Health Certificate
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HealthCertificate {
    /// Issuer organisation
    #[serde(rename = "issuerOrganisation", default)]
    pub issuer_organisation: String,
    /// Certificate name
    #[serde(rename = "name", default)]
    pub name: String,
    /// Signature type SN
    #[serde(rename = "signatureTypeSN", default)]
    pub signature_type_sn: String,
    /// Subject SN
    #[serde(rename = "subjectSN", default)]
    pub subject_sn: String,
    /// Valid from
    #[serde(rename = "validFrom", default)]
    pub valid_from: String,
    /// Valid to
    #[serde(rename = "validTo", default)]
    pub valid_to: String,
}
