use serde::{Deserialize, Serialize};

/// Locale
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Locale {
    /// Continent name. This field support localization.
    #[serde(rename = "continent", default)]
    pub continent: String,
    /// Continent code. A two character continent code "AF" for Africa, "AN" for
    /// Antarctica, "AS" for Asia, "EU" for Europe, "NA" for North America, "OC"
    /// for Oceania, and "SA" for South America.
    #[serde(rename = "continentCode", default)]
    pub continent_code: String,
    /// Country name. This field support localization.
    #[serde(rename = "country", default)]
    pub country: String,
    /// Country code in [ISO 3166-1](http://en.wikipedia.org/wiki/ISO_3166-1)
    /// two-character format
    #[serde(rename = "countryCode", default)]
    pub country_code: String,
    /// Currency code in [ISO 4217-1](http://en.wikipedia.org/wiki/ISO_4217)
    /// three-character format
    #[serde(rename = "currency", default)]
    pub currency: String,
    /// True if country is part of the European Union.
    #[serde(rename = "eu", default)]
    pub eu: bool,
    /// User IP address.
    #[serde(rename = "ip", default)]
    pub ip: String,
}
