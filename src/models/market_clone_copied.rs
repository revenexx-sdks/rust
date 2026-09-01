use serde::{Deserialize, Serialize};

/// Child rows copied from the source, per collection. A flag left false is a
/// zero here, and so is a source that had none of that kind.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MarketCloneCopied {
    /// Traded currencies copied from the source market.
    #[serde(rename = "currencies", default)]
    pub currencies: i64,
    /// Locales copied from the source market.
    #[serde(rename = "locales", default)]
    pub locales: i64,
    /// Tax classes copied from the source market.
    #[serde(rename = "tax_classes", default)]
    pub tax_classes: i64,
}
