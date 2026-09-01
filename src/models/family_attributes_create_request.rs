use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FamilyAttributesCreateRequest {
    /// The attribute the family carries. One row per (family, attribute); deleting
    /// either side deletes the link.
    #[serde(rename = "attribute_id", default)]
    pub attribute_id: String,
    /// The family this link belongs to — one side of the pair that makes an
    /// attribute part of a family's form.
    #[serde(rename = "family_id", default)]
    pub family_id: String,
    /// The attribute has to carry a value for a product of this family to count as
    /// complete. `POST /products/{id}/completeness` measures exactly these and
    /// nothing else.
    #[serde(rename = "is_required", default)]
    pub is_required: bool,
    /// The family's own ordering of this attribute, which overrides the
    /// attribute's default `position` in this family's form.
    #[serde(rename = "position", default)]
    pub position: i64,
    /// Narrows `is_required` to named channels. NULL or an empty list means
    /// required EVERYWHERE, not nowhere — that is how every required link in the
    /// wild is stored, and reading an empty list as "nowhere" reports a fully
    /// configured family as demanding nothing.
    #[serde(rename = "required_channels", default)]
    pub required_channels: serde_json::Value,
}
