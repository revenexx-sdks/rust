use serde::{Deserialize, Serialize};

/// Partial update — omitted fields keep their current value.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MarketLocaleUpdateRequest {
    /// Locale code, e.g. 'de-DE' (unique per market).
    #[serde(rename = "code", default)]
    pub code: String,
    /// ISO 3166-1 alpha-2 country code.
    #[serde(rename = "country", default)]
    pub country: String,
    #[serde(rename = "is_default", default)]
    pub is_default: bool,
    /// ISO 639-1 language code.
    #[serde(rename = "language", default)]
    pub language: String,
    /// Sort position (default 0).
    #[serde(rename = "position", default)]
    pub position: i64,
}
