use serde::{Deserialize, Serialize};

/// Continent
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Continent {
    /// Continent two letter code.
    #[serde(rename = "code", default)]
    pub code: String,
    /// Continent name.
    #[serde(rename = "name", default)]
    pub name: String,
}
