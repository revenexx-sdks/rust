use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CategoryRuleCondition {
    /// A product column (sku, kind, enabled, family_id, parent_id) or
    /// 'attribute:<code>' for the common bucket of attribute_values. An attribute
    /// code is [A-Za-z0-9_]+. Locale-/channel-scoped attributes are not supported.
    #[serde(rename = "field", default)]
    pub field: String,
    /// How to compare. 'eq'/'neq' are equality, 'gt'/'gte'/'lt'/'lte' order
    /// (numerically for a number, as text for a string), 'in' membership,
    /// 'contains'/'starts_with'/'ends_with' substring, 'is_empty'/'is_not_empty'
    /// presence — those last two take no `value`.
    #[serde(rename = "operator", default)]
    pub operator: String,
    /// Comparison value. An array for 'in' — non-empty, at most 200 entries, all
    /// of the same type; omitted for 'is_empty'/'is_not_empty'; a non-empty string
    /// for 'contains'/'starts_with'/'ends_with'; a string or number for
    /// gt/gte/lt/lte. Numbers compare numerically (jsonb), strings as text.
    #[serde(rename = "value", default)]
    pub value: String,
}
