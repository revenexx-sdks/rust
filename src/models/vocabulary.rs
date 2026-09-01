use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Vocabulary {
    /// This app's name — the part before the dot in the qualified id.
    #[serde(rename = "app", default)]
    pub app: String,
    /// True when the values are the complete permitted set. For a CHECK-backed
    /// vocabulary the constraint guarantees it; for a table-backed one the app
    /// refuses a value outside the rows, and for `locales` outside the configured
    /// list — the same guarantee by three mechanisms.
    #[serde(rename = "closed", default)]
    pub closed: bool,
    /// The tone an unlabelled value gets.
    #[serde(rename = "default_tone", default)]
    pub default_tone: String,
    /// A plain string, or a locale map keyed by language tag ({ "en": …, "de":
    /// … }). Read the requested tag, fall back to `en`. A curated label is a
    /// map; a value nobody labelled is humanized into a plain string.
    #[serde(rename = "description", default)]
    pub description: serde_json::Value,
    /// The vocabulary this is.
    #[serde(rename = "name", default)]
    pub name: String,
    /// 'schema' — a CHECK constraint owns the set. 'table' — the tenant's own
    /// rows do. 'defaults' — a table-backed set the tenant never wrote down,
    /// answered from the built-ins. 'tenant' — the merchant configured the
    /// values through a setting (locales).
    #[serde(rename = "source", default)]
    pub source: String,
    /// A plain string, or a locale map keyed by language tag ({ "en": …, "de":
    /// … }). Read the requested tag, fall back to `en`. A curated label is a
    /// map; a value nobody labelled is humanized into a plain string.
    #[serde(rename = "title", default)]
    pub title: serde_json::Value,
    /// Every permitted value, in the order a select should offer them.
    #[serde(rename = "values", default)]
    pub values: Vec<serde_json::Value>,
}
