use serde::{Deserialize, Serialize};

/// LocaleCode
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LocaleCode {
    /// Locale codes in [ISO
    /// 639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes)
    #[serde(rename = "code", default)]
    pub code: String,
    /// Locale name
    #[serde(rename = "name", default)]
    pub name: String,
}
