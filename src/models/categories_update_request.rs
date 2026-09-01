use serde::{Deserialize, Serialize};

/// Partial update — omitted fields keep their current value.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CategoriesUpdateRequest {
    /// The category's stable identifier — what an import and a storefront join
    /// on, and what survives a rename of the label. Unique per tenant.
    #[serde(rename = "code", default)]
    pub code: String,
    /// The category name a person sees, per language tag. The catalog reads by
    /// name, not by code — a locale left blank falls back to the next filled
    /// one.
    #[serde(rename = "labels", default)]
    pub labels: serde_json::Value,
    /// The category this one hangs under. Null is a root of the tree. Deleting a
    /// parent lifts its children to the root rather than deleting them, so a
    /// mis-click never takes a subtree with it.
    #[serde(rename = "parent_id", default)]
    pub parent_id: String,
    /// A materialized position in the tree, kept for importers that carry one
    /// (`tools/power_tools/cordless_drills`). Nothing in this app writes or reads
    /// it — `parent_id` is the structure this app navigates.
    #[serde(rename = "path", default)]
    pub path: String,
    /// Order among the siblings under the same parent, ascending.
    #[serde(rename = "position", default)]
    pub position: i64,
    /// How the conditions combine: 'all' ANDs them (the default), 'any' ORs them.
    /// It is a column of its own rather than a key of `rules` because the compiler
    /// reads the two separately.
    #[serde(rename = "rule_match", default)]
    pub rule_match: String,
    /// The selector that makes this a RULE-DRIVEN category. Null means
    /// hand-picked. Matching products are MATERIALIZED as `product_categories`
    /// rows with source `rule`, next to the hand-picked ones a recompute never
    /// touches; `POST /products/categories/{category_id}/rules/preview` dry-runs
    /// this exact document before it is stored. Conditions address the `common`
    /// bucket of a product's values — a value held per locale or per channel has
    /// no single answer for a rule to test.
    #[serde(rename = "rules", default)]
    pub rules: serde_json::Value,
    /// When the rule last ran TO COMPLETION and its memberships were synced. Null
    /// means no pass has ever finished — a recompute is chunked, so a
    /// half-finished pass leaves this untouched.
    #[serde(rename = "rules_computed_at", default)]
    pub rules_computed_at: String,
    /// Whatever this catalog keeps on a category beyond the model — the keys
    /// belong to the tenant, not to this app, and nothing here reads them.
    #[serde(rename = "values", default)]
    pub values: serde_json::Value,
}
