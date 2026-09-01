use serde::{Deserialize, Serialize};

/// One vocabulary and every value it permits.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PagesVocabulary {
    /// Always 'pages'.
    #[serde(rename = "app", default)]
    pub app: String,
    /// The set is exhaustive, so a value outside it is stale data rather than a
    /// missing label.
    #[serde(rename = "closed", default)]
    pub closed: bool,
    /// The badge colour a value nobody toned falls back to.
    #[serde(rename = "default_tone", default)]
    pub default_tone: String,
    /// What the set is for, or null. A plain string, or a locale map keyed by
    /// language tag ({ "en": …, "de": … }). Read the requested tag, fall back
    /// to `en`.
    #[serde(rename = "description", default)]
    pub description: serde_json::Value,
    /// The vocabulary name, echoed.
    #[serde(rename = "name", default)]
    pub name: String,
    /// Always 'schema' — the values are parsed from the column's CHECK
    /// constraint, which is why the served set cannot drift from the enforced one.
    #[serde(rename = "source", default)]
    pub source: String,
    /// What this set of values is called. A plain string, or a locale map keyed by
    /// language tag ({ "en": …, "de": … }). Read the requested tag, fall back
    /// to `en`.
    #[serde(rename = "title", default)]
    pub title: serde_json::Value,
    /// Every permitted value, in the order the constraint lists them — which is
    /// the order a select should offer.
    #[serde(rename = "values", default)]
    pub values: Vec<crate::models::PagesVocabularyValue>,
}
