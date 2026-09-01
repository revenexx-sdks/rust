use serde::{Deserialize, Serialize};

/// Partial update — omitted fields keep their current value.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FamilyVariantsUpdateRequest {
    /// The attribute codes a product model splits its variants on. Two shapes are
    /// in the wild and both are read: a bare list of codes, or one entry per
    /// level, outermost first — `[{"level": 1, "axes": ["colour"]}, {"level": 2,
    /// "axes": ["size"]}]`. An attribute named here is READ-ONLY on the model and
    /// set on each variant, which is what `AttributeField.readonly_reason`
    /// reports.
    #[serde(rename = "axes", default)]
    pub axes: serde_json::Value,
    /// The variant structure's stable identifier — how this family splits, not
    /// which product it splits. Unique per tenant.
    #[serde(rename = "code", default)]
    pub code: String,
    /// The family this variant structure belongs to. A family may carry several,
    /// and a product names the one it follows through `family_variant_id`.
    #[serde(rename = "family_id", default)]
    pub family_id: String,
    /// What the variant structure is called, per language tag.
    #[serde(rename = "labels", default)]
    pub labels: serde_json::Value,
}
