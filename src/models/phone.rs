use serde::{Deserialize, Serialize};

/// Phone
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Phone {
    /// Phone code.
    #[serde(rename = "code", default)]
    pub code: String,
    /// Country two-character ISO 3166-1 alpha code.
    #[serde(rename = "countryCode", default)]
    pub country_code: String,
    /// Country name.
    #[serde(rename = "countryName", default)]
    pub country_name: String,
}
