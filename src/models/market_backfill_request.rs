use serde::{Deserialize, Serialize};

/// The path id is the market being REPAIRED; `source` is the market to copy
/// from (a uuid or a market code). The three flags default to true.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MarketBackfillRequest {
    /// Take the source's traded currencies for codes this market does not already
    /// carry. Default true.
    #[serde(rename = "currencies", default)]
    pub currencies: bool,
    /// Take the source's locales for codes this market does not already carry.
    /// Default true.
    #[serde(rename = "locales", default)]
    pub locales: bool,
    /// The market to copy the missing pieces FROM — a uuid or a market code.
    /// Must not be the market in the path. Pick a market that is already right;
    /// nothing about it is changed.
    #[serde(rename = "source", default)]
    pub source: String,
    /// Take the source's tax classes for codes this market does not already carry.
    /// An existing code keeps ITS rate — a backfill never re-rates a class the
    /// merchant already set. Default true.
    #[serde(rename = "tax_classes", default)]
    pub tax_classes: bool,
}
