use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MarketContext {
    #[serde(rename = "currencies", default)]
    pub currencies: Vec<crate::models::MarketCurrency>,
    #[serde(rename = "locales", default)]
    pub locales: Vec<crate::models::MarketLocale>,
    #[serde(rename = "market", default)]
    pub market: crate::models::Market,
    #[serde(rename = "tax_classes", default)]
    pub tax_classes: Vec<crate::models::MarketTaxClass>,
}
