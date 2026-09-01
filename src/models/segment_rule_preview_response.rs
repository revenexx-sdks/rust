use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SegmentRulePreviewResponse {
    /// The cap that applied (5000), or null when the rule was answered by a single
    /// count query and no cap was needed.
    #[serde(rename = "cap", default)]
    pub cap: i64,
    /// True when the combined evaluation hit the id cap, which makes `count` a
    /// lower bound.
    #[serde(rename = "capped", default)]
    pub capped: bool,
    /// How many organizations the rule selects. Exact when 'capped' is false; a
    /// LOWER BOUND when it is true.
    #[serde(rename = "count", default)]
    pub count: i64,
    /// How the conditions were combined for this preview.
    #[serde(rename = "rule_match", default)]
    pub rule_match: String,
    /// A handful of the organizations the rule selects — enough for an operator
    /// to recognise whether the rule means what they thought. Never the full set.
    #[serde(rename = "sample", default)]
    pub sample: Vec<serde_json::Value>,
    /// The segment named in the path. It is not read — the rule comes from the
    /// body — but it has to exist.
    #[serde(rename = "segment_id", default)]
    pub segment_id: String,
    /// What the rule selects. Only 'organizations' exists.
    #[serde(rename = "target", default)]
    pub target: String,
}
