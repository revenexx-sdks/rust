use serde::{Deserialize, Serialize};

/// Currency
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Currency {
    /// Currency code in [ISO 4217-1](http://en.wikipedia.org/wiki/ISO_4217)
    /// three-character format.
    #[serde(rename = "code", default)]
    pub code: String,
    /// Number of decimal digits.
    #[serde(rename = "decimalDigits", default)]
    pub decimal_digits: i64,
    /// Currency name.
    #[serde(rename = "name", default)]
    pub name: String,
    /// Currency plural name
    #[serde(rename = "namePlural", default)]
    pub name_plural: String,
    /// Currency digit rounding.
    #[serde(rename = "rounding", default)]
    pub rounding: f64,
    /// Currency symbol.
    #[serde(rename = "symbol", default)]
    pub symbol: String,
    /// Currency native symbol.
    #[serde(rename = "symbolNative", default)]
    pub symbol_native: String,
}
