use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderVocabulary {
    /// This app's name — the part before the dot in the qualified id.
    #[serde(rename = "app", default)]
    pub app: String,
    /// True when the values are the complete permitted set — always, since the
    /// routes enforce the ones the schema does not.
    #[serde(rename = "closed", default)]
    pub closed: bool,
    /// The tone an unlabelled value gets.
    #[serde(rename = "default_tone", default)]
    pub default_tone: String,
    /// Either one string, or a map of locale to string ({"en": …, "de": …}).
    #[serde(rename = "description", default)]
    pub description: String,
    /// Which vocabulary this is — echoed from the path, and the part after the
    /// dot in the qualified id.
    #[serde(rename = "name", default)]
    pub name: String,
    /// Who enforces the set: 'schema' = a CHECK constraint, 'app' = the routes.
    #[serde(rename = "source", default)]
    pub source: String,
    /// Either one string, or a map of locale to string ({"en": …, "de": …}).
    #[serde(rename = "title", default)]
    pub title: String,
    /// Every permitted value, in CONSTRAINT order — which for a status is
    /// lifecycle order, so a client can render them as a sequence without knowing
    /// one.
    #[serde(rename = "values", default)]
    pub values: Vec<crate::models::OrderVocabularyValue>,
}
