use serde::{Deserialize, Serialize};

/// One enum this app owns, with every permitted value.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaymentVocabulary {
    /// The app that owns this vocabulary — always `payments` here. Together with
    /// `name` it forms the platform-wide key `payments.statuses`.
    #[serde(rename = "app", default)]
    pub app: String,
    /// True when the set comes from a CHECK constraint and is therefore exhaustive
    /// — a client may treat anything outside it as stale data rather than a
    /// missing label.
    #[serde(rename = "closed", default)]
    pub closed: bool,
    /// The tone a permitted value nobody labelled falls back to, so every value is
    /// renderable.
    #[serde(rename = "default_tone", default)]
    pub default_tone: String,
    /// What this set of values is about. A plain string, or a locale map keyed by
    /// language tag ({ "en": …, "de": … }). Read the requested tag, fall back
    /// to `en`.
    #[serde(rename = "description", default)]
    pub description: serde_json::Value,
    /// The vocabulary name, as it appears in the URL.
    #[serde(rename = "name", default)]
    pub name: String,
    /// Where the values come from. `schema` means they were parsed out of the
    /// CHECK constraint, so what is served is what the database enforces.
    #[serde(rename = "source", default)]
    pub source: String,
    /// The vocabulary's own label, for a filter heading or a column title. A plain
    /// string, or a locale map keyed by language tag ({ "en": …, "de": … }).
    /// Read the requested tag, fall back to `en`.
    #[serde(rename = "title", default)]
    pub title: serde_json::Value,
    /// Every permitted value, in constraint order — which is the lifecycle order
    /// an author wrote, and the order a select should offer.
    #[serde(rename = "values", default)]
    pub values: Vec<crate::models::PaymentVocabularyValue>,
}
