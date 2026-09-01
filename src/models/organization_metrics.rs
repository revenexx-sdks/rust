use serde::{Deserialize, Serialize};

/// What an organization has BOUGHT, materialized from the orders app. One row
/// per organization — including all-zero rows for companies that never
/// ordered, so a 'never bought anything' rule has something to match.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrganizationMetrics {
    /// revenue_total / order_count, computed here from the sums rather than
    /// averaged upstream. Zero when there are no orders.
    #[serde(rename = "avg_order_value", default)]
    pub avg_order_value: f64,
    /// revenue_365d / order_count_365d. Zero when there were none in the window.
    #[serde(rename = "avg_order_value_365d", default)]
    pub avg_order_value_365d: f64,
    /// When this row was last written. The projection is materialized, so this is
    /// how stale the numbers are.
    #[serde(rename = "computed_at", default)]
    pub computed_at: String,
    /// When the projection row first appeared.
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    /// The single ISO 4217 currency all counted orders were in. NULL when there
    /// were none, and also when there were several — read `currency_mixed` to
    /// tell those two apart.
    #[serde(rename = "currency", default)]
    pub currency: String,
    /// True when this company ordered in more than one currency. The sums are
    /// still stored (dropping money is worse), but they are not comparable against
    /// a threshold, and a rule reading revenue should say so.
    #[serde(rename = "currency_mixed", default)]
    pub currency_mixed: bool,
    /// When this company first ordered. Null if it never has — that is what
    /// makes it usable as "is this a customer at all?".
    #[serde(rename = "first_order_at", default)]
    pub first_order_at: String,
    /// Primary key of the projection row.
    #[serde(rename = "id", default)]
    pub id: String,
    /// When this company last ordered. Null if it never has, which is why the
    /// virtual `days_since_last_order` rule field never matches those companies:
    /// use `last_order_at is_empty` for them.
    #[serde(rename = "last_order_at", default)]
    pub last_order_at: String,
    /// Orders ever counted for this company.
    #[serde(rename = "order_count", default)]
    pub order_count: i64,
    /// Orders in the 30 days before `orders_as_of`. A rolling window, not a
    /// calendar month.
    #[serde(rename = "order_count_30d", default)]
    pub order_count_30d: i64,
    /// Orders in the 365 days before `orders_as_of`.
    #[serde(rename = "order_count_365d", default)]
    pub order_count_365d: i64,
    /// Orders in the 90 days before `orders_as_of`.
    #[serde(rename = "order_count_90d", default)]
    pub order_count_90d: i64,
    /// The instant the rolling windows were measured from. Pinned across a chunked
    /// refresh, so a multi-call pass cannot let the windows slide underneath it.
    #[serde(rename = "orders_as_of", default)]
    pub orders_as_of: String,
    /// The company these numbers describe. One row per organization, and rows
    /// exist for companies that never ordered — all zeros rather than missing,
    /// so a "never bought" rule matches something.
    #[serde(rename = "organization_id", default)]
    pub organization_id: String,
    /// Revenue in the 30 days before `orders_as_of`.
    #[serde(rename = "revenue_30d", default)]
    pub revenue_30d: f64,
    /// Revenue in the 365 days before `orders_as_of`. The usual "how big is this
    /// customer" number, and the one a key-account rule should read.
    #[serde(rename = "revenue_365d", default)]
    pub revenue_365d: f64,
    /// Revenue in the 90 days before `orders_as_of`.
    #[serde(rename = "revenue_90d", default)]
    pub revenue_90d: f64,
    /// Revenue ever counted, in `currency`. Which orders count is the orders app's
    /// decision, not this app's.
    #[serde(rename = "revenue_total", default)]
    pub revenue_total: f64,
    /// The tenant this row belongs to — the store slug, not an id. Set by the
    /// platform from the authenticated context, never by a caller; a write that
    /// carries it is ignored, and no request can read another tenant's rows by
    /// sending a different one.
    #[serde(rename = "tenant_id", default)]
    pub tenant_id: String,
    /// When the row last changed. Unchanged numbers are not rewritten, so this can
    /// lag `computed_at`.
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
}
