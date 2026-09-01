use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InventoryVocabulary {
    /// This app's name — the part before the dot in the qualified id.
    #[serde(rename = "app", default)]
    pub app: String,
    /// True when these values are the complete permitted set, because they were
    /// read out of a CHECK constraint. A value outside a closed set is therefore
    /// stale data, not a missing label — which is what lets a client show it as
    /// an error instead of inventing a title for it.
    #[serde(rename = "closed", default)]
    pub closed: bool,
    /// The tone a value gets when nobody has labelled it — a value added to the
    /// CHECK constraint is served with its key humanized and this tone, rather
    /// than not being served at all.
    #[serde(rename = "default_tone", default)]
    pub default_tone: String,
    /// A plain string, or a locale map keyed by language tag ({ "en": …, "de":
    /// … }). Read the requested tag, fall back to `en`.
    #[serde(rename = "description", default)]
    pub description: serde_json::Value,
    /// The vocabulary name, echoed — the part after the dot in the qualified id.
    #[serde(rename = "name", default)]
    pub name: String,
    /// Where the words come from: 'schema' — the app's own, read from the
    /// constraint. Nothing here is renameable per tenant, so a client may cache it
    /// per app version.
    #[serde(rename = "source", default)]
    pub source: String,
    /// A plain string, or a locale map keyed by language tag ({ "en": …, "de":
    /// … }). Read the requested tag, fall back to `en`.
    #[serde(rename = "title", default)]
    pub title: serde_json::Value,
    /// Every permitted value, IN CONSTRAINT ORDER — which is lifecycle order for
    /// a status, so a UI can render the steps in the order they happen.
    #[serde(rename = "values", default)]
    pub values: Vec<serde_json::Value>,
}
