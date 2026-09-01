use serde::{Deserialize, Serialize};

/// Additive order facts for one organization. Average order value is
/// revenue_total / order_count.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderCustomerRollup {
    /// Every currency seen on the counted orders, sorted. MORE THAN ONE MEANS THE
    /// SUMS MIX CURRENCIES — nothing here converts, so a two-currency row's
    /// revenue_total is a sum of unlike numbers and should be shown per currency
    /// or not at all.
    #[serde(rename = "currencies", default)]
    pub currencies: Vec<String>,
    /// When this company first ordered — placed_at where there is one, otherwise
    /// created_at. Null cannot happen on a row that exists, but the field is
    /// nullable because the columns behind it are.
    #[serde(rename = "first_order_at", default)]
    pub first_order_at: String,
    /// When they last ordered. Together with as_of this is the recency a churn
    /// rule reads.
    #[serde(rename = "last_order_at", default)]
    pub last_order_at: String,
    /// How many orders of this company were counted — orders in one of the
    /// counted statuses, over all time.
    #[serde(rename = "order_count", default)]
    pub order_count: i64,
    /// Orders in the 30 days before as_of.
    #[serde(rename = "order_count_30d", default)]
    pub order_count_30d: i64,
    /// Orders in the 365 days before as_of — the rolling year a "still active"
    /// rule usually asks about.
    #[serde(rename = "order_count_365d", default)]
    pub order_count_365d: i64,
    /// Orders in the 90 days before as_of.
    #[serde(rename = "order_count_90d", default)]
    pub order_count_90d: i64,
    /// The company these facts belong to — the id the customers app knows it by.
    /// Every row of the answer carries one; orders without an organization are
    /// counted in orders_without_organization instead.
    #[serde(rename = "organization_id", default)]
    pub organization_id: String,
    /// Revenue in the 30 days before as_of.
    #[serde(rename = "revenue_30d", default)]
    pub revenue_30d: f64,
    /// Revenue in the 365 days before as_of.
    #[serde(rename = "revenue_365d", default)]
    pub revenue_365d: f64,
    /// Revenue in the 90 days before as_of.
    #[serde(rename = "revenue_90d", default)]
    pub revenue_90d: f64,
    /// Sum of grand_total over the counted orders. Gross: it includes tax and
    /// shipping, because grand_total does.
    #[serde(rename = "revenue_total", default)]
    pub revenue_total: f64,
}
