use serde::{Deserialize, Serialize};

/// The exact-column filters this call was understood to carry, verbatim as
/// they arrived. A query parameter that is not a column of `categories` —
/// `?status=`, a typo, a filter another entity has — is DROPPED and does not
/// appear here, and the list comes back unfiltered. This object is the only
/// way to tell that apart from "nothing matched".
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CategoriesFilter {
    /// The literal `?code=` value this call was understood to carry.
    #[serde(rename = "code", default)]
    pub code: String,
    /// The literal `?created_at=` value this call was understood to carry.
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    /// The literal `?id=` value this call was understood to carry.
    #[serde(rename = "id", default)]
    pub id: String,
    /// The literal `?labels=` value this call was understood to carry.
    #[serde(rename = "labels", default)]
    pub labels: String,
    /// The literal `?parent_id=` value this call was understood to carry.
    #[serde(rename = "parent_id", default)]
    pub parent_id: String,
    /// The literal `?path=` value this call was understood to carry.
    #[serde(rename = "path", default)]
    pub path: String,
    /// The literal `?position=` value this call was understood to carry.
    #[serde(rename = "position", default)]
    pub position: String,
    /// The literal `?rule_match=` value this call was understood to carry.
    #[serde(rename = "rule_match", default)]
    pub rule_match: String,
    /// The literal `?rules=` value this call was understood to carry.
    #[serde(rename = "rules", default)]
    pub rules: String,
    /// The literal `?rules_computed_at=` value this call was understood to carry.
    #[serde(rename = "rules_computed_at", default)]
    pub rules_computed_at: String,
    /// The literal `?updated_at=` value this call was understood to carry.
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
    /// The literal `?values=` value this call was understood to carry.
    #[serde(rename = "values", default)]
    pub values: String,
}
