use serde::{Deserialize, Serialize};

/// Add one value to the payment terms set. It is available to
/// `organizations.payment_terms` immediately.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaymentTermCreateRequest {
    /// What `organizations.payment_terms` will store. Lowercase, starting with a
    /// letter; immutable afterwards.
    #[serde(rename = "code", default)]
    pub code: String,
    /// One line of help for whoever picks this value.
    #[serde(rename = "description", default)]
    pub description: String,
    /// Localized descriptions, keyed by language tag ({ "en": …, "de": … }).
    /// Null when nobody translated this value — a client then falls back to
    /// `description`.
    #[serde(rename = "descriptions", default)]
    pub descriptions: serde_json::Value,
    /// Promote this value; the previous default is demoted in the same call.
    #[serde(rename = "is_default", default)]
    pub is_default: bool,
    /// Localized titles, keyed by language tag ({ "en": …, "de": … }). Null
    /// when nobody translated this value — a client then falls back to `title`.
    #[serde(rename = "labels", default)]
    pub labels: serde_json::Value,
    /// Where it sits in the set, ascending. Default 0.
    #[serde(rename = "position", default)]
    pub position: i64,
    /// The fallback name shown when no locale matches.
    #[serde(rename = "title", default)]
    pub title: String,
    /// Semantic badge colour.
    #[serde(rename = "tone", default)]
    pub tone: String,
}
