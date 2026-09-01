use serde::{Deserialize, Serialize};

/// Name the family either way — `family_id` wins when both are sent. The
/// family has to exist already; this route assigns one, it does not create
/// one.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProductFamilyAssignRequest {
    /// Alternative to family_id — a `families.code` this tenant holds, from `GET
    /// /products/families`. No example: a code is tenant data, and any value
    /// published here names a family somebody does not have.
    #[serde(rename = "family_code", default)]
    pub family_code: String,
    /// The family to assign.
    #[serde(rename = "family_id", default)]
    pub family_id: String,
}
