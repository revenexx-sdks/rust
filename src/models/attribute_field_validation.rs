use serde::{Deserialize, Serialize};

/// The limits the value has to satisfy, ready to hand to a form validator.
/// Only the seven keys below are republished; anything else the tenant stored
/// in `attributes.validation` stays there.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AttributeFieldValidation {
    /// Largest permitted number.
    #[serde(rename = "max", default)]
    pub max: f64,
    /// Most entries.
    #[serde(rename = "max_items", default)]
    pub max_items: i64,
    /// Longest permitted text.
    #[serde(rename = "max_length", default)]
    pub max_length: i64,
    /// Smallest permitted number, for a number or measure field.
    #[serde(rename = "min", default)]
    pub min: f64,
    /// Fewest entries, for a multi-select or a collection.
    #[serde(rename = "min_items", default)]
    pub min_items: i64,
    /// Shortest permitted text.
    #[serde(rename = "min_length", default)]
    pub min_length: i64,
    /// A regular expression the text has to match.
    #[serde(rename = "pattern", default)]
    pub pattern: String,
}
