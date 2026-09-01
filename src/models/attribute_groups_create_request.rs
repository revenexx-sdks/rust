use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AttributeGroupsCreateRequest {
    /// The group's stable identifier, and the value an `AttributeField` carries as
    /// its `group` — a SECTION of the product form, not a label. Unique per
    /// tenant and the key an import joins on.
    #[serde(rename = "code", default)]
    pub code: String,
    /// The section heading a person sees, keyed by language tag. The code is never
    /// shown to an operator; a tag nobody translated falls back to the next filled
    /// one, then to English.
    #[serde(rename = "labels", default)]
    pub labels: serde_json::Value,
    /// Where this section sits in a form, ascending. Sections that tie keep the
    /// order the database returns them in.
    #[serde(rename = "position", default)]
    pub position: i64,
}
