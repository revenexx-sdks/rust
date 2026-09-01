use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CategoryRulesRequest {
    /// Between 1 and 25 conditions — a rule is a selector, not a query language.
    /// An empty list is a 400, not "everything".
    #[serde(rename = "conditions", default)]
    pub conditions: Vec<crate::models::CategoryRuleCondition>,
    /// 'all' ANDs every condition (default), 'any' ORs them.
    #[serde(rename = "rule_match", default)]
    pub rule_match: String,
}
