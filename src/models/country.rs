use serde::{Deserialize, Serialize};

/// Country
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Country {
    /// Country two-character ISO 3166-1 alpha code.
    #[serde(rename = "code", default)]
    pub code: String,
    /// Country name.
    #[serde(rename = "name", default)]
    pub name: String,
}
